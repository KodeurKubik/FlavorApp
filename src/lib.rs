use tauri::{WebviewUrl, WebviewWindowBuilder};
mod cmds;
mod inject;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut app = tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(inject::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init());

    #[cfg(not(target_os = "android"))]
    {
        app = app
            .plugin(tauri_plugin_window_state::Builder::new().build())
            .plugin(tauri_plugin_updater::Builder::new().build());
    }

    app = app
        .setup(|app| {
            #[cfg(not(target_os = "android"))]
            {
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External("https://flavortown.hackclub.com".parse().unwrap()),
                )
                .title("FlavorApp")
                .devtools(true)
                .inner_size(1080.0, 720.0)
                .visible(false)
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

            #[cfg(all(dev, not(target_os = "android")))]
            {
                use tauri::Manager;

                let window = app
                    .get_webview_window("main")
                    .expect("main window not found");

                window.open_devtools();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![cmds::notify]);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
