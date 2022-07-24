pub(super) fn execute() {
	tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}