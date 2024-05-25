use std::sync::{Arc, Mutex};

use log::error;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use samp::amx::{Amx, AmxIdent};
use samp::cell::AmxString;
use samp::error::{AmxError, AmxResult};
use samp::native;
use slab::Slab;

pub struct MathJob {
    job_completed: bool,
    ident: AmxIdent,
    query: String,
    player_id: u32,
    offset: u32,
    response: Option<String>,
}

pub struct MathPlugin {
    pub jobs: Arc<Mutex<Slab<MathJob>>>,
}

impl MathPlugin {
    fn executor(params: &mut MathJob) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = utf8_percent_encode(&params.query, NON_ALPHANUMERIC).to_string();
        let search = format!("https://api.mathjs.org/v4/?expr={}", encoded);
        let response = reqwest::blocking::get(&search)?.text()?;
        params.response = Some(response);

        Ok(())
    }

    pub fn add_job(&mut self, amx: &Amx, player_id: u32, query: AmxString, offset: u32) {
        let ident = AmxIdent::from(amx.amx().as_ptr());

        let data = MathJob {
            job_completed: false,
            ident,
            query: query.to_string(),
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

            match MathPlugin::executor(params) {
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
                let index = amx.find_public("OnMathResponse")?;
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
    #[native(name = "Math")]
    pub fn native_math(
        &mut self,
        amx: &Amx,
        player_id: u32,
        query: AmxString,
        offset: u32,
    ) -> AmxResult<bool> {
        MathPlugin::add_job(&mut self.math, amx, player_id, query, offset);
        Ok(true)
    }
}
