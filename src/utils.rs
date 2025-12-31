#[cfg(desktop)]
pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    use crate::cmds;
    use tauri_plugin_updater::UpdaterExt;

    log::info!("Checking for updates...");
    if let Some(update) = app.updater()?.check().await? {
        log::info!(
            "Update found: {} (current: {})",
            update.version,
            update.current_version
        );
        let mut downloaded = 0;

        update
            .download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    log::info!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                    log::info!("download finished");
                },
            )
            .await?;

        log::info!("update installed");

        if let Err(o) = cmds::notify(
            app,
            "Update downloaded and installed!".to_string(),
            Some("Please restart the app for the update to apply".to_string()),
        )
        .await
        {
            log::error!("Could not notify: {o:?}");
        }
    }

    Ok(())
}
