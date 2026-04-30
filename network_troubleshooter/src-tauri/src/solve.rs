#[cfg(target_os = "linux")]
use crate::linux;
#[cfg(target_os = "linux")]
use crate::linux_parser;

#[cfg(target_os = "windows")]
use crate::windows;
#[cfg(target_os = "windows")]
use crate::windows_parser;

#[cfg(any(target_os = "linux", target_os = "windows"))]
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RepairStep {
    pub action: String,
    pub success: bool,
    pub details: String,
}

#[derive(Debug, Serialize)]
pub struct RepairReport {
    pub interface: Option<String>,
    pub steps: Vec<RepairStep>,
}

impl RepairReport {
    fn new(interface: Option<String>) -> Self {
        Self {
            interface,
            steps: Vec::new(),
        }
    }

    fn push_ok(&mut self, action: &str, details: String) {
        self.steps.push(RepairStep {
            action: action.to_string(),
            success: true,
            details,
        });
    }

    fn push_err(&mut self, action: &str, details: String) {
        self.steps.push(RepairStep {
            action: action.to_string(),
            success: false,
            details,
        });
    }
}

#[cfg(target_os = "linux")]
fn is_candidate_interface_linux(name: &str) -> bool {
    let n = name.to_lowercase();

    let excluded_prefixes = [
        "lo",
        "docker",
        "br-",
        "veth",
        "virbr",
        "vmnet",
        "vboxnet",
        "zt",
        "tailscale",
        "wg",
        "tun",
        "tap",
        "dummy",
        "macvtap",
        "macvlan",
        "sit",
        "ip6tnl",
        "gre",
        "gretap",
        "erspan",
        "nlmon",
        "ifb",
        "can",
    ];

    if excluded_prefixes.iter().any(|p| n.starts_with(p)) {
        return false;
    }

    n.starts_with("eth")
        || n.starts_with("en")
        || n.starts_with("wl")
        || n.starts_with("wlan")
        || n.starts_with("usb")
        || n.starts_with("wwan")
        || n.starts_with("bond")
        || n.starts_with("team")
        || n.starts_with("ppp")
}

#[cfg(target_os = "windows")]
fn is_candidate_interface_windows(name: &str) -> bool {
    let n = name.to_lowercase();

    let excluded = [
        "virtual",
        "vmware",
        "vbox",
        "hyper-v",
        "loopback",
        "bluetooth",
        "tunnel",
        "pseudo",
        "npcap",
        "vethernet",
        "wsl",
        "zerotier",
        "tailscale",
        "wireguard",
    ];

    if excluded.iter().any(|e| n.contains(e)) {
        return false;
    }

    n.contains("ethernet")
        || n.contains("wi-fi")
        || n.contains("wifi")
        || n.contains("wireless")
        || n.contains("wlan")
        || n.contains("lan")
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn get_candidate_linux_interfaces() -> Result<Vec<String>, String> {
    let output = linux::run_cmd("ip", &["-j", "link"])?;
    let parsed = linux_parser::parse_ip_link(&output)?;

    let mut up_interfaces = Vec::new();
    let mut down_interfaces = Vec::new();

    for iface in parsed {
        if !is_candidate_interface_linux(&iface.name) {
            continue;
        }

        if iface.is_up {
            up_interfaces.push(iface.name);
        } else {
            down_interfaces.push(iface.name);
        }
    }

    up_interfaces.extend(down_interfaces);

    if up_interfaces.is_empty() {
        Err("No likely Linux network interfaces found".to_string())
    } else {
        Ok(up_interfaces)
    }
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn get_candidate_windows_interfaces() -> Result<Vec<String>, String> {
    let adapter_output = windows::run_powershell("Get-NetAdapter | ConvertTo-Json -Depth 4")?;
    let adapters = windows_parser::parse_net_adapter(&adapter_output)?;

    let ip_output = windows::run_powershell("Get-NetIPAddress | ConvertTo-Json -Depth 4")?;
    let ip_data = windows_parser::parse_net_ip_address(&ip_output)?;

    let mut with_ip = Vec::new();
    let mut without_ip = Vec::new();
    let mut down_candidates = Vec::new();

    for adapter in adapters {
        if !is_candidate_interface_windows(&adapter.name) {
            continue;
        }

        let has_ip = ip_data.iter().any(|iface| {
            iface.name == adapter.name && (iface.ipv4.is_some() || iface.ipv6.is_some())
        });

        if adapter.is_up {
            if has_ip {
                with_ip.push(adapter.name);
            } else {
                without_ip.push(adapter.name);
            }
        } else {
            down_candidates.push(adapter.name);
        }
    }

    with_ip.extend(without_ip);
    with_ip.extend(down_candidates);

    if with_ip.is_empty() {
        Err("No likely Windows network interfaces found".to_string())
    } else {
        Ok(with_ip)
    }
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn up_linux_interface(interface: String) -> Result<String, String> {
    linux::run_cmd("nmcli", &["networking", "on"])?;
    Ok(format!("Brought interface '{}' up", interface))
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn down_linux_interface(interface: String) -> Result<String, String> {
    linux::run_cmd("nmcli", &["networking", "off"])?;
    Ok(format!("Brought interface '{}' down", interface))
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn up_windows_interface(interface: String) -> Result<String, String> {
    let cmd = format!(
        "Enable-NetAdapter -Name '{}' -Confirm:$false",
        interface.replace('\'', "''")
    );
    windows::run_powershell(&cmd)?;
    Ok(format!("Enabled interface '{}'", interface))
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn down_windows_interface(interface: String) -> Result<String, String> {
    let cmd = format!(
        "Disable-NetAdapter -Name '{}' -Confirm:$false",
        interface.replace('\'', "''")
    );
    windows::run_powershell(&cmd)?;
    Ok(format!("Disabled interface '{}'", interface))
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn dhcp_request_linux(interface: String) -> Result<String, String> {
    // Try dhclient first
    if linux::run_cmd("dhclient", &["-r", &interface]).is_ok() {
        match linux::run_cmd("dhclient", &[&interface]) {
            Ok(out) => {
                return Ok(format!(
                    "Renewed DHCP lease on '{}' using dhclient.\n{}",
                    interface, out
                ))
            }
            Err(e) => return Err(format!("dhclient renew failed on '{}': {}", interface, e)),
        }
    }

    // Fallback: nmcli device reapply
    if linux::run_cmd("nmcli", &["device", "reapply", &interface]).is_ok() {
        return Ok(format!(
            "Requested DHCP/network reapply on '{}' using nmcli device reapply",
            interface
        ));
    }

    // Fallback: disconnect/reconnect via nmcli
    linux::run_cmd("nmcli", &["device", "disconnect", &interface]).ok();
    linux::run_cmd("nmcli", &["device", "connect", &interface]).map_err(|e| {
        format!(
            "Could not renew DHCP on '{}' using dhclient or nmcli: {}",
            interface, e
        )
    })?;

    Ok(format!(
        "Requested DHCP/network reconnect on '{}' using nmcli",
        interface
    ))
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn dhcp_request_windows(interface: String) -> Result<String, String> {
    let release_cmd = format!("ipconfig /release \"{}\"", interface.replace('"', ""));
    let renew_cmd = format!("ipconfig /renew \"{}\"", interface.replace('"', ""));

    windows::run_powershell(&release_cmd).ok();
    let out = windows::run_powershell(&renew_cmd)?;

    Ok(format!("Renewed DHCP lease on '{}'.\n{}", interface, out))
}

#[cfg(target_os = "linux")]
fn try_linux_dns_flush() -> Result<String, String> {
    let attempts = [
        ("resolvectl", vec!["flush-caches"]),
        ("systemd-resolve", vec!["--flush-caches"]),
        ("resolvconf", vec!["-i"]),
        ("service", vec!["nscd", "restart"]),
        ("service", vec!["dnsmasq", "restart"]),
        ("nmcli", vec!["general", "reload"]),
    ];

    let mut errors = Vec::new();

    for (cmd, args) in attempts {
        match linux::run_cmd(cmd, &args) {
            Ok(out) => {
                return Ok(if out.trim().is_empty() {
                    format!("Ran '{} {}'", cmd, args.join(" "))
                } else {
                    format!("Ran '{} {}'\n{}", cmd, args.join(" "), out)
                });
            }
            Err(e) => {
                errors.push(format!("{} {} -> {}", cmd, args.join(" "), e));
            }
        }
    }

    Err(format!(
        "Could not flush DNS cache on Linux with the common methods tried:\n{}",
        errors.join("\n")
    ))
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn flush_dns_cache_linux() -> Result<String, String> {
    try_linux_dns_flush()
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn flush_dns_cache_windows() -> Result<String, String> {
    let out = windows::run_powershell("ipconfig /flushdns")?;
    Ok(format!("Flushed DNS cache.\n{}", out))
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn repair_linux_interface(interface: String) -> Result<RepairReport, String> {
    let mut report = RepairReport::new(Some(interface.clone()));

    match linux::run_cmd("nmcli", &["networking", "on"]) {
        Ok(out) => report.push_ok(
            "Bring interface up",
            if out.trim().is_empty() {
                format!("Brought '{}' up", interface)
            } else {
                out
            },
        ),
        Err(e) => report.push_err("Bring interface up", e),
    }

    match dhcp_request_linux(interface.clone()).await {
        Ok(out) => report.push_ok("Renew DHCP lease", out),
        Err(e) => report.push_err("Renew DHCP lease", e),
    }

    match flush_dns_cache_linux().await {
        Ok(out) => report.push_ok("Flush DNS cache", out),
        Err(e) => report.push_err("Flush DNS cache", e),
    }

    Ok(report)
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn repair_windows_interface(interface: String) -> Result<RepairReport, String> {
    let mut report = RepairReport::new(Some(interface.clone()));

    let enable_cmd = format!(
        "Enable-NetAdapter -Name '{}' -Confirm:$false",
        interface.replace('\'', "''")
    );

    match windows::run_powershell(&enable_cmd) {
        Ok(out) => report.push_ok(
            "Enable interface",
            if out.trim().is_empty() {
                format!("Enabled '{}'", interface)
            } else {
                out
            },
        ),
        Err(e) => report.push_err("Enable interface", e),
    }

    match dhcp_request_windows(interface.clone()).await {
        Ok(out) => report.push_ok("Renew DHCP lease", out),
        Err(e) => report.push_err("Renew DHCP lease", e),
    }

    match flush_dns_cache_windows().await {
        Ok(out) => report.push_ok("Flush DNS cache", out),
        Err(e) => report.push_err("Flush DNS cache", e),
    }

    Ok(report)
}

#[cfg(target_os = "linux")]
#[tauri::command]
pub async fn repair_first_candidate_linux() -> Result<RepairReport, String> {
    let interfaces = get_candidate_linux_interfaces().await?;
    let first = interfaces
        .into_iter()
        .next()
        .ok_or_else(|| "No Linux interface available for repair".to_string())?;

    repair_linux_interface(first).await
}

#[cfg(target_os = "windows")]
#[tauri::command]
pub async fn repair_first_candidate_windows() -> Result<RepairReport, String> {
    let interfaces = get_candidate_windows_interfaces().await?;
    let first = interfaces
        .into_iter()
        .next()
        .ok_or_else(|| "No Windows interface available for repair".to_string())?;

    repair_windows_interface(first).await
}
