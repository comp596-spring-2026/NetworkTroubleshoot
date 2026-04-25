mod diagnostic_engine;
mod models;
mod solve;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
mod linux_parser;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
mod windows_parser;

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
            get_os_type,
            // linux collector commands
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
            linux::run_full_diagnostics,
            // solve.rs linux commands
            solve::get_candidate_linux_interfaces,
            solve::up_linux_interface,
            solve::down_linux_interface,
            solve::dhcp_request_linux,
            solve::flush_dns_cache_linux,
            solve::repair_linux_interface,
            solve::repair_first_candidate_linux,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_os_type,
            // windows collector commands
            windows::link_state,
            windows::get_neighbors,
            windows::get_ipconfig,
            windows::resolve_dns_name,
            windows::invoke_web_request,
            windows::get_route,
            windows::test_connection,
            windows::test_net_connection,
            windows::tracert,
            windows::run_full_diagnostics,
            // solve.rs windows commands
            solve::get_candidate_windows_interfaces,
            solve::up_windows_interface,
            solve::down_windows_interface,
            solve::dhcp_request_windows,
            solve::flush_dns_cache_windows,
            solve::repair_windows_interface,
            solve::repair_first_candidate_windows,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
