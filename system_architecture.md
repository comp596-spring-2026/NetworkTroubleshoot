# System Architecture

This diagram shows the high-level structure of the **Network Troubleshooter** application, from the user-facing Tauri frontend down to the OS-native networking tools used for diagnostics.

## Mermaid Diagram

```mermaid
flowchart TD
    User[User]

    subgraph UI[Frontend UI / Tauri Webview]
        Dashboard[Dashboard / Quick Scan]
        LayerCards[Per-Layer Diagnostic Cards]
        Details[Selected Layer Details]
        Repair[Quick Repair Actions]
    end

    Bridge[Tauri Command Bridge]

    subgraph Backend[Rust Backend]
        LinuxCollectors[Linux Collectors]
        WindowsCollectors[Windows Collectors]
        Parsers[Parsers]
        Models[Normalized Models]
        Engine[Diagnostics Engine]
    end

    subgraph SystemTools[OS Native Tools]
        LinuxTools[ip, nmcli, ping, nc, dig, curl, traceroute]
        WindowsTools[PowerShell cmdlets, netsh, ping]
    end

    User -->|uses| Dashboard
    User -->|reviews| LayerCards
    User -->|triggers| Repair

    UI -->|invoke commands| Bridge
    Bridge -->|Linux path| LinuxCollectors
    Bridge -->|Windows path| WindowsCollectors

    LinuxCollectors -->|execute| LinuxTools
    WindowsCollectors -->|execute| WindowsTools

    LinuxCollectors -->|raw output| Parsers
    WindowsCollectors -->|raw output| Parsers
    Parsers -->|normalize| Models
    Models -->|evaluate| Engine
    Engine -->|results + explanation| Details
    Engine -->|summarized status| LayerCards
```

## Explanation

The **frontend UI** is responsible for interaction with the user. It provides the main dashboard, the per-layer summary cards, detailed explanations for the selected layer, and quick repair actions.

The **Tauri command bridge** connects the frontend to the Rust backend. When the user starts a scan or repair action, the frontend invokes backend commands through this bridge.

The **Rust backend** is the core of the application. It contains:
- platform-specific collectors for Linux and Windows
- parsers that convert raw command output into structured data
- normalized models shared across platforms
- a diagnostics engine that evaluates the collected data and determines status, severity, and explanations

The **OS native tools** remain the actual source of networking information. The application does not invent data; it reads and interprets results from trusted system utilities such as `ip`, `nmcli`, `ping`, `dig`, `curl`, `traceroute`, PowerShell networking cmdlets, and `netsh`.

This architecture keeps the project modular:
- the UI stays focused on presentation
- platform-specific logic stays isolated
- shared diagnostic reasoning stays reusable
- future platforms such as macOS can be added more easily
