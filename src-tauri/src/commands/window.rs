#[tauri::command]
pub fn shutdown_window(exit_sender: tauri::State<crate::managed_state::ExitSignaller>) -> () {
    exit_sender.0.send(0usize).ok();
}
