use tauri::{
    Runtime,
    plugin::{Builder, TauriPlugin},
};
use tauri_plugin_opener::OpenerExt;

const INJECT_SCRIPT: &str = include_str!("inject.js");

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::<R>::new("flavorapp")
        .js_init_script(INJECT_SCRIPT.to_string())
        .on_navigation(|window, url| {
            if let Some(host) = url.host_str() {
                if host.ends_with("hackclub.com") {
                    return true;
                }

                log::info!("Opening external URL in browser: {}", url);
                let _ = window.opener().open_url(url.as_str(), None::<&str>);
                return false;
            }
            true
        })
        .build()
}
