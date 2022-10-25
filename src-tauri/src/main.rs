#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn search_anime(name: String) -> String {
    println!("Searching for {}", name);
    let url = format!("https://api.jikan.moe/v4/anime?q={}", name);
    let resp = reqwest::get(&url).await.unwrap();
    let body = resp.text().await.unwrap();

    let json: serde_json::Value = serde_json::from_str(&body).unwrap();

    let title = json["data"][0]["title"].as_str().unwrap();

    format!("Found {}", title)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, search_anime])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
