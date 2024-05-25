use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest;
use samp::amx::{Amx, AmxIdent};
use samp::cell::AmxString;
use samp::error::AmxResult;
use samp::native;

const TRANSLATE: &'static str = "http://localhost:7333/translate";

pub fn encode(string: &str) -> String {
    utf8_percent_encode(string, NON_ALPHANUMERIC).to_string()
}

pub fn translate(
    amxident: AmxIdent,
    playerid: u32,
    input: &str,
    language: &str,
    encoded: u32,
    offset: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let search;
    if encoded == 1 {
        search = format!(
            "{}?lang={}&q={}&encoded={}",
            TRANSLATE,
            encode(language),
            input,
            true
        );
    } else {
        search = format!(
            "{}?lang={}&q={}&encoded={}",
            TRANSLATE,
            encode(language),
            encode(input),
            false
        );
    }
    let response = reqwest::blocking::get(&search)?.text()?;
    let amx = samp::amx::get(amxident).ok_or(samp::error::AmxError::NotFound)?;
    let index = amx.find_public("OnTranslateResponse").unwrap();
    let allocator = amx.allocator();
    amx.push(offset)?;
    amx.push(allocator.allot_string(&response)?)?;
    amx.push(playerid)?; // playerid
    match amx.exec(index) {
        Ok(_res) => (),
        Err(_e) => (),
    }
    Ok(())
}

impl super::Plugin {
    #[native(name = "Translate")]
    pub fn native_translater(
        &mut self,
        _amx: &Amx,
        playerid: u32,
        input_data: AmxString,
        inpt_lang: AmxString,
        encoded: u32,
        offset: u32,
    ) -> AmxResult<bool> {
        let input_string = input_data.to_string();
        let input_lang = inpt_lang.to_string();
        let amx_identifier = AmxIdent::from(_amx.amx().as_ptr());
        std::thread::spawn(move || {
            match translate(
                amx_identifier,
                playerid,
                &input_string,
                &input_lang,
                encoded,
                offset,
            ) {
                Err(_e) => (),
                Ok(_) => (),
            };
        });
        Ok(true)
    }
}
