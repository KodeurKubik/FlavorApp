#[cfg(desktop)]
pub async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    use tauri_plugin_updater::UpdaterExt;

    log::info!("Checking for updates...");
    if let Some(update) = app.updater()?.check().await? {
        use tauri_plugin_dialog::{DialogExt, MessageDialogButtons};

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

        app.dialog()
            .message(format!(
                "The update {} has been downloaded and is ready to install. Please restart to apply the update. You are currently on version {}",
                update.version, update.current_version
            ))
            .title("Update Ready!")
            .buttons(MessageDialogButtons::OkCancelCustom(
                "Restart Now".to_string(),
                "Later".to_string(),
            ))
            .show(move |result| match result {
                true => app.restart(),
                false => {}
            });
    }

    Ok(())
}
