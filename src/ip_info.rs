use std::sync::{Arc, Mutex};

use log::error;
use samp::amx::{Amx, AmxIdent};
use samp::cell::AmxString;
use samp::error::{AmxError, AmxResult};
use samp::native;
use slab::Slab;

#[derive(serde_derive::Deserialize)]
struct Ip {
    ip: Option<String>,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    loc: Option<String>,
    org: Option<String>,
    postal: Option<String>,
    timezone: Option<String>,
}

pub struct IpInfoJob {
    job_completed: bool,
    ident: AmxIdent,
    player_id: u32,
    ip: String,
    token: String,
    offset: u32,
    response: Option<Ip>,
}

pub struct IpInfoPlugin {
    pub jobs: Arc<Mutex<Slab<IpInfoJob>>>,
}

impl IpInfoPlugin {
    fn executor(params: &mut IpInfoJob) -> Result<(), Box<dyn std::error::Error>> {
        let search = format!("http://ipinfo.io/{}?token={}", params.ip, params.token);
        let ip: Ip = reqwest::blocking::get(&search)?.json()?;
        params.response = Some(ip);

        Ok(())
    }

    fn add_job(&mut self, amx: &Amx, player_id: u32, ip: AmxString, token: AmxString, offset: u32) {
        let ident = AmxIdent::from(amx.amx().as_ptr());

        let data = IpInfoJob {
            job_completed: false,
            ident,
            player_id,
            ip: ip.to_string(),
            token: token.to_string(),
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

            match IpInfoPlugin::executor(params) {
                Err(_e) => {
                    error!("{}", _e);
                }
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
                let index = amx.find_public("OnIpInfoResponse")?;
                let allocator = amx.allocator();

                let push_string = |value: &Option<String>| {
                    let binding = "NaN".to_string();
                    let str = value.as_ref().unwrap_or(&binding);
                    amx.push(allocator.allot_string(str)?)
                };

                let response = &params.response.as_ref().ok_or(AmxError::NotFound)?;

                amx.push(params.offset)?;
                push_string(&response.postal)?;
                push_string(&response.timezone)?;
                push_string(&response.org)?;
                push_string(&response.city)?;
                push_string(&response.region)?;
                push_string(&response.country)?;
                push_string(&response.loc)?;
                push_string(&response.ip)?;
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
    #[native(name = "IpInfo")]
    pub fn native_ip_info(
        &mut self,
        amx: &Amx,
        player_id: u32,
        ip: AmxString,
        token: AmxString,
        offset: u32,
    ) -> AmxResult<bool> {
        IpInfoPlugin::add_job(&mut self.ip, amx, player_id, ip, token, offset);
        Ok(true)
    }
}
