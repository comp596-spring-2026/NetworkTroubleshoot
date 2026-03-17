const { invoke } = window.__TAURI__.core;

const DEFAULT_URL = "example.com";
const DEFAULT_IP = "8.8.8.8";

let currentArgs = null;

function readArgsFromInputsOrDefault() {
  const urlInput = document.getElementById("dest-url")?.value.trim() || "";
  const ipInput = document.getElementById("dest-ip-add")?.value.trim() || "";

  return {
    url: urlInput || DEFAULT_URL,
    ip_address: ipInput || DEFAULT_IP,
  };
}

function getArgs() {
  return currentArgs ?? readArgsFromInputsOrDefault();
}

function toHost(input) {
  const s = (input || "").trim();
  if (!s) return "";

  try {
    const u = s.includes("://") ? new URL(s) : new URL("https://" + s);
    return u.hostname;
  } catch (_) {
    return s.replace(/^https?:\/\//, "").split("/")[0];
  }
}

function hideAllPanels() {
  const linuxPanel = document.getElementById("linux");
  const windowsPanel = document.getElementById("windows");

  if (linuxPanel) linuxPanel.style.display = "none";
  if (windowsPanel) windowsPanel.style.display = "none";
}

function showLinuxPanel() {
  const linuxPanel = document.getElementById("linux");
  if (linuxPanel) linuxPanel.style.display = "block";
}

function showWindowsPanel() {
  const windowsPanel = document.getElementById("windows");
  if (windowsPanel) windowsPanel.style.display = "block";
}

function setOutput(element, value) {
  if (element) {
    element.textContent = value;
  }
}

function setupSharedInputs() {
  const submitBtn = document.getElementById("submitBtn");

  submitBtn?.addEventListener("click", () => {
    currentArgs = readArgsFromInputsOrDefault();
    console.log("Saved target:", currentArgs);
  });
}

function setupLinuxHandlers() {
  const ipLinkMsgEl = document.querySelector("#ip-link-output");
  const ipAddrMsgEl = document.querySelector("#ip-addr-output");
  const ipRouteMsgEl = document.querySelector("#ip-route-output");
  const nmcliMsgEl = document.querySelector("#nmcli-output");
  const ipNeighMsgEl = document.querySelector("#ip-neigh-output");
  const ipPingMsgEl = document.querySelector("#ping-output");
  const netcatMsgEl = document.querySelector("#netcat-output");
  const curlMsgEl = document.querySelector("#curl-output");
  const digMsgEl = document.querySelector("#dig-output");
  const traceRouteMsgEl = document.querySelector("#traceroute-output");

  document.querySelector("#run-ip-link")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipLinkMsgEl, "Running...");
    try {
      setOutput(ipLinkMsgEl, await invoke("ip_link"));
    } catch (err) {
      setOutput(ipLinkMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-addr")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipAddrMsgEl, "Running...");
    try {
      setOutput(ipAddrMsgEl, await invoke("ip_addr"));
    } catch (err) {
      setOutput(ipAddrMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-route")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipRouteMsgEl, "Running...");
    try {
      setOutput(ipRouteMsgEl, await invoke("ip_route"));
    } catch (err) {
      setOutput(ipRouteMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-nmcli")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(nmcliMsgEl, "Running...");
    try {
      setOutput(nmcliMsgEl, await invoke("nmcli"));
    } catch (err) {
      setOutput(nmcliMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ip-neigh")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipNeighMsgEl, "Running...");
    try {
      setOutput(ipNeighMsgEl, await invoke("ip_neigh"));
    } catch (err) {
      setOutput(ipNeighMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-ping")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipPingMsgEl, "Running...");
    try {
      const { ip_address } = getArgs();
      setOutput(ipPingMsgEl, await invoke("ping", { ip: ip_address }));
    } catch (err) {
      setOutput(ipPingMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-netcat")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(netcatMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(netcatMsgEl, await invoke("netcat", { host }));
    } catch (err) {
      setOutput(netcatMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-curl")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(curlMsgEl, "Running...");
    try {
      const { url } = getArgs();
      setOutput(curlMsgEl, await invoke("curl", { url }));
    } catch (err) {
      setOutput(curlMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-dig")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(digMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(digMsgEl, await invoke("dig", { host }));
    } catch (err) {
      setOutput(digMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-traceroute")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(traceRouteMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(traceRouteMsgEl, await invoke("traceroute", { host }));
    } catch (err) {
      setOutput(traceRouteMsgEl, `Error: ${err}`);
    }
  });
}

function setupWindowsHandlers() {
  const linkStateMsgEl = document.querySelector("#link-state-output");
  const neighborsMsgEl = document.querySelector("#neighbors-output");
  const ipconfigMsgEl = document.querySelector("#ipconfig-output");
  const routeMsgEl = document.querySelector("#route-output");
  const pingMsgEl = document.querySelector("#win-ping-output");
  const netConnectionMsgEl = document.querySelector("#net-connection-output");
  const dnsMsgEl = document.querySelector("#dns-output");
  const webMsgEl = document.querySelector("#web-output");

  document.querySelector("#run-link-state")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(linkStateMsgEl, "Running...");
    try {
      setOutput(linkStateMsgEl, await invoke("link_state"));
    } catch (err) {
      setOutput(linkStateMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-neighbors")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(neighborsMsgEl, "Running...");
    try {
      setOutput(neighborsMsgEl, await invoke("get_neighbors"));
    } catch (err) {
      setOutput(neighborsMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-ipconfig")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(ipconfigMsgEl, "Running...");
    try {
      setOutput(ipconfigMsgEl, await invoke("get_ipconfig"));
    } catch (err) {
      setOutput(ipconfigMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-get-route")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(routeMsgEl, "Running...");
    try {
      setOutput(routeMsgEl, await invoke("get_route"));
    } catch (err) {
      setOutput(routeMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-test-connection")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(pingMsgEl, "Running...");
    try {
      const { ip_address } = getArgs();
      setOutput(pingMsgEl, await invoke("test_connection", { host: ip_address }));
    } catch (err) {
      setOutput(pingMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-test-net-connection")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(netConnectionMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(netConnectionMsgEl, await invoke("test_net_connection", { host }));
    } catch (err) {
      setOutput(netConnectionMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-resolve-dns")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(dnsMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(dnsMsgEl, await invoke("resolve_dns_name", { host }));
    } catch (err) {
      setOutput(dnsMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-web-request")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(webMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const finalUrl = url.includes("://") ? url : `https://${url}`;
      setOutput(webMsgEl, await invoke("invoke_web_request", { url: finalUrl }));
    } catch (err) {
      setOutput(webMsgEl, `Error: ${err}`);
    }
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  setupSharedInputs();
  hideAllPanels();

  try {
    const os = await invoke("get_os_type");
    console.log("Detected OS:", os);

    if (os === "linux") {
      showLinuxPanel();
      setupLinuxHandlers();
    } else if (os === "windows") {
      showWindowsPanel();
      setupWindowsHandlers();
    } else {
      showLinuxPanel();
      setupLinuxHandlers();
    }
  } catch (err) {
    console.error("Failed to detect OS:", err);
  }
});