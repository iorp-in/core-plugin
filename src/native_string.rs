use chrono::prelude::*;
use regex::Regex;
use regex::RegexBuilder;
use samp::amx::Amx;
use samp::cell::{AmxString, UnsizedBuffer};
use samp::error::AmxResult;
use samp::native;
use voca_rs::*;

impl super::Plugin {
    #[native(name = "regexMatchCount")]
    pub fn native_reg_match_count(
        &mut self,
        _amx: &Amx,
        input: AmxString,
        string: AmxString,
    ) -> AmxResult<usize> {
        let re = RegexBuilder::new(&input.to_string())
            .case_insensitive(true)
            .build()
            .expect("Invalid Regex");
        let results_count = re.find_iter(&string.to_string()).count();
        return Ok(results_count);
    }

    #[native(name = "GetPercentage")]
    pub fn native_get_percentage(
        &mut self,
        _amx: &Amx,
        value: usize,
        maximum: usize,
    ) -> AmxResult<usize> {
        if value <= 0 {
            return Ok(0);
        }
        if value >= maximum {
            return Ok(0);
        }
        let percent = value * 100 / maximum;
        Ok(percent)
    }

    #[native(name = "GetPercentageOf")]
    pub fn native_get_percentage_of(
        &mut self,
        _amx: &Amx,
        percent: usize,
        value: usize,
    ) -> AmxResult<usize> {
        if percent <= 0 {
            return Ok(0);
        }
        if percent >= 100 {
            return Ok(value);
        }
        let percent = value * percent / 100;
        Ok(percent)
    }

    #[native(name = "IsStringContainWords")]
    pub fn native_is_string_contain_words(
        &mut self,
        _amx: &Amx,
        string: AmxString,
        words: AmxString,
    ) -> AmxResult<bool> {
        let mut response = true;
        let rust_string = format!(" {} ", string.to_string());
        let rust_word = words.to_string();
        let word_split = rust_word.split(", ");
        for i in word_split {
            let word_split_sub = i.split(" ");
            for j in word_split_sub {
                let formatted_word = format!(" {} ", j);
                let contain_status = rust_string.contains(&formatted_word);
                if !contain_status {
                    response = false;
                    break;
                } else {
                    response = true;
                }
            }
            if response {
                break;
            }
        }
        Ok(response)
    }

    #[native(name = "SortString")]
    pub fn native_sort_string(
        &mut self,
        _amx: &Amx,
        query: AmxString,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let input = query.to_string();
        let split = input.split("\n");
        let mut vec: Vec<&str> = split.collect();
        vec.sort();
        let joined = vec.join("\n");
        let trim_ed = joined.trim();
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &trim_ed);
        Ok(true)
    }

    #[native(name = "GetMenuList")]
    pub fn native_get_menu_list(
        &mut self,
        _amx: &Amx,
        string: AmxString,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let mut owned_string: String = "".to_owned();
        let input = string.to_string();
        let split = input.split("\n");
        for s in split {
            let text = s.split("\t").nth(0).unwrap();
            if text.len() > 0 {
                let re = Regex::new(r"\{(.*?)\}").unwrap();
                let result = re.replace_all(text, "");
                owned_string.push_str(&result);
                owned_string.push_str("\n");
            }
        }
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &owned_string.trim());
        Ok(true)
    }

    #[native(name = "GetHeaderMenuList")]
    pub fn native_get_heder_menu_list(
        &mut self,
        _amx: &Amx,
        string: AmxString,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let mut owned_string: String = "".to_owned();
        let input = string.to_string();
        let split = input.split("\n");
        let mut count = 0;
        for s in split {
            let text = s.split("\t").nth(0).unwrap();
            if text.len() > 0 && count != 0 {
                let re = Regex::new(r"\{(.*?)\}").unwrap();
                let result = re.replace_all(text, "");
                owned_string.push_str(&result);
                owned_string.push_str("\n");
            }
            count += 1;
        }
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &owned_string.trim());
        Ok(true)
    }

    #[native(name = "GetMenuString")]
    pub fn native_get_menu_string(
        &mut self,
        _amx: &Amx,
        string: AmxString,
        position: usize,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let result;
        let input = string.to_string();
        let total = input.split("\n").count();
        if position >= total {
            result = "null";
        } else {
            result = input.split("\n").nth(position).unwrap();
        }
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &result);
        Ok(true)
    }

    #[native(name = "TrimString")]
    pub fn native_trim_string(
        &mut self,
        _amx: &Amx,
        input: AmxString,
        output: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let string_input = input.to_string();
        let trim_ed = string_input.trim();
        let mut buffer = output.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &trim_ed);
        Ok(true)
    }

    #[native(name = "GetWord")]
    pub fn native_get_word(
        &mut self,
        _amx: &Amx,
        query: AmxString,
        position: usize,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let input = query.to_string();
        let split = input.split(" ");
        let vec: Vec<&str> = split.collect();
        let total_words = count::count_words(&input, " ");
        let result;
        if total_words < position + 1 || vec[position].len() == 0 {
            result = "none";
        } else {
            result = vec[position];
        }
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &result);
        Ok(true)
    }

    #[native(name = "GetSubString")]
    pub fn native_get_substring(
        &mut self,
        _amx: &Amx,
        query: AmxString,
        search: AmxString,
        response: UnsizedBuffer,
        size: usize,
    ) -> AmxResult<bool> {
        let input = query.to_string();
        let word = search.to_string();
        let word_size = word.len() + 1;
        let index_of = index::index_of(&input, &word, 0);
        let result;
        if index_of == -1 {
            result = input;
        } else {
            result = chop::substring(&input, index_of as usize + word_size, 0);
        }
        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &result);
        Ok(true)
    }

    #[native(name = "RegMatch")]
    pub fn native_reg_match(
        &mut self,
        _amx: &Amx,
        pattern: AmxString,
        name: AmxString,
    ) -> AmxResult<bool> {
        let input = name.to_string();
        let re_pattern = pattern.to_string();
        let re = Regex::new(&re_pattern).unwrap();
        Ok(re.is_match(&input))
    }

    #[native(name = "UnixToHuman")]
    pub fn native_unix_to_human(
        &mut self,
        _amx: &Amx,
        unix: u32,
        response: UnsizedBuffer,
        format: AmxString,
        size: usize,
    ) -> AmxResult<bool> {
        // read unix timestamp
        let unix_time = unix.to_string();
        let format_time = format.to_string();
        // Convert the timestamp string into an i64
        let timestamp = unix_time.parse::<i64>().unwrap();

        // Create a NaiveDateTime from the timestamp
        let naive = NaiveDateTime::from_timestamp(timestamp, 0);

        // Create a normal DateTime from the NaiveDateTime
        let date_time: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        let date_time_local: DateTime<Local> = DateTime::from(date_time);

        // Format the date_time_local how you want
        let new_date = date_time_local.format(&format_time);

        let mut buffer = response.into_sized_buffer(size);
        let _ = samp::cell::string::put_in_buffer(&mut buffer, &format!("{}", new_date));
        Ok(true)
    }
}
