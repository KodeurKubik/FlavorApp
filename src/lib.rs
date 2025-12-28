use tauri::{WebviewUrl, WebviewWindowBuilder};

fn send_extension_ping() -> Result<(), String> {
    let client = reqwest::blocking::Client::new();
    client
        .get("https://flavortown.hackclub.com/")
        .header("X-Flavortown-Ext-2793", "true")
        .send()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(not(target_os = "android"))]
            {
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("https://flavortown.hackclub.com".parse().unwrap()),
                )
                .title("FlavorApp")
                .title_bar_style(tauri::TitleBarStyle::Transparent)
                .inner_size(1080.0, 720.0)
                .build()?;
            }

            #[cfg(target_os = "android")]
            {
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("https://flavortown.hackclub.com".parse().unwrap()),
                )
                .build()?;
            }

            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(60));
                send_extension_ping().ok();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
