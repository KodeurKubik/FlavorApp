use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

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

            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            
            std::thread::spawn(move || loop {
                std::thread::sleep(std::time::Duration::from_secs(60));

                let _ = window.eval("fetch('https://flavortown.hackclub.com/projects', { headers: { 'X-Flavortown-Ext-2793': true } })");
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
