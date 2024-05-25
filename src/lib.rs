use alexa::AlexaPlugin;
use ip_info::IpInfoPlugin;
use log::info;
use math::MathPlugin;
use samp::amx::Amx;
use samp::initialize_plugin;
use samp::plugin::SampPlugin;
use slab::Slab;
use std::io;
use std::sync::{Arc, Mutex};

mod alexa;
mod email;
mod ip_info;
mod math;
mod native_string;

struct Plugin {
    alexa: AlexaPlugin,
    ip: IpInfoPlugin,
    math: MathPlugin,
}

impl SampPlugin for Plugin {
    fn on_load(&mut self) {
        info!("IORP Core. Loaded");
    }

    fn on_unload(&mut self) {
        info!("IORP Core. unloaded");
    }

    fn on_amx_unload(&mut self, _unloaded_amx: &Amx) {}

    fn process_tick(&mut self) {
        let _ = self.alexa.process_tick();
        let _ = self.ip.process_tick();
        let _ = self.math.process_tick();
    }
}

initialize_plugin!(
    natives: [
        Plugin::native_alexa,
        Plugin::native_math,
        Plugin::native_ip_info,
        Plugin::native_is_string_contain_words,
        Plugin::native_sort_string,
        Plugin::native_trim_string,
        Plugin::native_get_menu_list,
        Plugin::native_get_heder_menu_list,
        Plugin::native_get_menu_string,
        Plugin::native_get_word,
        Plugin::native_get_substring,
        Plugin::native_reg_match,
        Plugin::native_unix_to_human,
        Plugin::native_get_percentage,
        Plugin::native_get_percentage_of,
        Plugin::native_reg_match_count,
        Plugin::native_send_http_post,
        Plugin::native_send_http_get,
    ],
    {
        samp::plugin::enable_process_tick();
        let _ = fern::Dispatch::new()
            .format(|callback, message, record| {
                callback.finish(format_args!("\t[Indian Ocean Roleplay] {}: {}", record.level().to_string().to_lowercase(), message))
            })
             .chain(
                fern::Dispatch::new()
                    .level(log::LevelFilter::Info)
                    .chain(io::stdout()),
            ).chain(
                fern::Dispatch::new()
                    .level(log::LevelFilter::Error)
                    .chain(io::stdout()),
            )
            .apply();

        return Plugin {
            alexa: AlexaPlugin {
                jobs: Arc::new(Mutex::new(Slab::new()))
            },
            ip: IpInfoPlugin {
                jobs: Arc::new(Mutex::new(Slab::new()))
            },
            math: MathPlugin {
                jobs: Arc::new(Mutex::new(Slab::new()))
            }

        }
    }
);
