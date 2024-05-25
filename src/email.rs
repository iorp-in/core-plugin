use log::info;
use reqwest;
use samp::amx::Amx;
use samp::cell::AmxString;
use samp::error::AmxResult;
use samp::native;

pub fn send_post(url: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .post(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(format!("{}", body))
        .send()?;
    Ok(())
}

pub fn send_get(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    client
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .send()?;
    Ok(())
}

impl super::Plugin {
    #[native(name = "sendHttpGet")]
    pub fn native_send_http_get(&mut self, _amx: &Amx, url: AmxString) -> AmxResult<bool> {
        let input_url = url.to_string();
        std::thread::spawn(move || {
            match send_get(&input_url) {
                Err(_e) => info!("{}", _e),
                Ok(_) => (),
            };
        });
        Ok(true)
    }

    #[native(name = "sendHttpPost")]
    pub fn native_send_http_post(
        &mut self,
        _amx: &Amx,
        url: AmxString,
        body: AmxString,
    ) -> AmxResult<bool> {
        let input_url = url.to_string();
        let input_body = body.to_string();
        std::thread::spawn(move || {
            match send_post(&input_url, &input_body) {
                Err(_e) => info!("{}", _e),
                Ok(_) => (),
            };
        });
        Ok(true)
    }
}
