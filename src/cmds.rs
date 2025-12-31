use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub async fn notify(
    app_handle: tauri::AppHandle,
    title: String,
    description: Option<String>,
) -> Result<(), String> {
    // Permission Check
    let permission_state = app_handle
        .notification()
        .permission_state()
        .map_err(|e| format!("Failed to check permission: {:?}", e))?;

    // Request Permission
    if permission_state != tauri_plugin_notification::PermissionState::Granted {
        println!("Requesting notification permission...");
        app_handle
            .notification()
            .request_permission()
            .map_err(|e| format!("Failed to request permission: {:?}", e))?;
    }

    // Send Notification
    app_handle
        .notification()
        .builder()
        .title(title)
        .body(description.unwrap_or_default())
        .show()
        .map_err(|e| format!("Failed to show notification: {:?}", e))?;

    Ok(())
}
