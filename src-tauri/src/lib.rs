// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command] // Redundant method...
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
} // Serves no real purpose for now since it is straight
//   from a template.
//   If I ever decide to add any rust functions, at least
//   I have a starting base, although I'm better off deleting this
//   function and looking through the docs instead lol
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
