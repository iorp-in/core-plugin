use chrono::Local;
use samp::amx::Amx;
use samp::cell::AmxString;
use samp::error::AmxResult;
use samp::native;
use std::fs;
use std::io::Write;

impl super::Plugin {
    #[native(name = "log")]
    pub fn native_log(
        &mut self,
        _amx: &Amx,
        log: AmxString,
        filename: AmxString,
        dateformat: AmxString,
    ) -> AmxResult<usize> {
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(filename.to_string())
            .unwrap();

        let now = Local::now();
        file.write_all(
            format!(
                "{}{}\n",
                now.format(&dateformat.to_string()),
                log.to_string()
            )
            .as_bytes(),
        )
        .unwrap();

        return Ok(1);
    }
}
