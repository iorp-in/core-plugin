use std::sync::{Arc, Mutex};

use log::error;
use samp::amx::{Amx, AmxIdent};
use samp::cell::AmxString;
use samp::error::{AmxError, AmxResult};
use samp::native;
use slab::Slab;

pub struct AlexaJob {
    job_completed: bool,
    ident: AmxIdent,
    _query: String,
    player_id: u32,
    offset: u32,
    response: Option<String>,
}

pub struct AlexaPlugin {
    pub jobs: Arc<Mutex<Slab<AlexaJob>>>,
}

impl AlexaPlugin {
    fn executor(params: &mut AlexaJob) -> Result<(), Box<dyn std::error::Error>> {
        params.response = Some("Hi, this is an invalid instruction. See /help.".to_string());

        Ok(())
    }

    pub fn add_job(&mut self, amx: &Amx, player_id: u32, query: AmxString, offset: u32) {
        let ident = AmxIdent::from(amx.amx().as_ptr());

        let data = AlexaJob {
            job_completed: false,
            ident,
            _query: query.to_string(),
            player_id,
            offset,
            response: None,
        };

        let key = self.jobs.lock().unwrap().insert(data);
        let arc_key = Arc::new(key);
        let share_arc_key = Arc::clone(&arc_key);
        let slab_handle = Arc::clone(&self.jobs);

        std::thread::spawn(move || {
            let mut slab = slab_handle.lock().unwrap();
            let key = *share_arc_key;
            let params = slab.get_mut(key).unwrap();

            match AlexaPlugin::executor(params) {
                Err(_e) => error!("{}", _e),
                Ok(_) => (),
            };

            params.job_completed = true;
        });
    }

    pub fn process_tick(&mut self) -> Result<(), AmxError> {
        let mut slab = self.jobs.lock().map_err(|_| AmxError::NotFound)?;

        // Collect keys of jobs to be removed
        let mut to_remove = Vec::new();

        for (_key, params) in slab.iter_mut() {
            if params.job_completed {
                let amx = samp::amx::get(params.ident).ok_or(AmxError::NotFound)?;
                let index = amx.find_public("OnAlexaReply")?;
                let allocator = amx.allocator();

                let push_string = |value: &Option<String>| {
                    let binding = "NaN".to_string();
                    let str: &String = value.as_ref().unwrap_or(&binding);
                    amx.push(allocator.allot_string(str)?)
                };

                amx.push(params.offset)?;
                push_string(&params.response)?;
                amx.push(params.player_id)?;
                amx.exec(index)?;

                // Add key to to_remove
                to_remove.push(_key);
            }
        }

        // Remove processed items
        for key in to_remove {
            slab.remove(key);
        }

        Ok(())
    }
}

impl super::Plugin {
    #[native(name = "Alexa")]
    pub fn native_alexa(
        &mut self,
        amx: &Amx,
        player_id: u32,
        query: AmxString,
        offset: u32,
    ) -> AmxResult<bool> {
        AlexaPlugin::add_job(&mut self.alexa, amx, player_id, query, offset);
        Ok(true)
    }
}
