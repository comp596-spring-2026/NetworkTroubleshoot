mod linux;
mod windows;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // linux commands
            linux::ip_link,
            linux::nmcli,
            linux::ip_neigh,
            linux::ping,
            linux::netcat,
            linux::curl,
            linux::dig,
            linux::traceroute,

            // windows commands
            windows::link_state,
            windows::get_neighbors,
            windows::get_ipconfig,

            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
