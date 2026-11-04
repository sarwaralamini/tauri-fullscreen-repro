fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("failed to run fullscreen test app");
}
