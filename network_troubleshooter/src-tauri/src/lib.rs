mod models;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod linux_parser;



#[cfg(target_os = "windows")]
mod windows;

#[tauri::command]
fn get_os_type() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "macos"
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[cfg(target_os = "linux")]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // linux commands
            get_os_type,
            linux::ip_link,
            linux::nmcli,
            linux::ip_neigh,
            linux::ping,
            linux::netcat,
            linux::curl,
            linux::dig,
            linux::traceroute,
            linux::ip_addr,
            linux::ip_route, 
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // windows commands
            get_os_type,
            windows::link_state,
            windows::get_neighbors,
            windows::get_ipconfig, 
            windows::resolve_dns_name, 
            windows::invoke_web_request,
            windows::get_route,
            windows::test_connection,
            windows::test_net_connection,
            windows::tracert, 
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}





