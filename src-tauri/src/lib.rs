use tauri::Manager;

#[tauri::command]
async fn login(app: tauri::AppHandle) -> Result<(), String> {
    let resource_path = app.path().resource_dir().unwrap();
    let test_doc = resource_path.join("resources/test.txt");
    let test_doc = test_doc.to_str().unwrap();
    let test_doc = std::fs::read_to_string(test_doc).unwrap();
    println!("{}", test_doc);

    Ok(())
}
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
