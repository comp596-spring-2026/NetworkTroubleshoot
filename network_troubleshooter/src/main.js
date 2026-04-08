const { invoke } = window.__TAURI__.core;

const DEFAULT_URL = "google.com";
const DEFAULT_IP = "8.8.8.8";

let currentArgs = null;

function readArgsFromInputsOrDefault() {
  const urlInput =
    document.getElementById("dest-url")?.value.trim() ||
    document.getElementById("target-url")?.value.trim() ||
    "";

  const ipInput =
    document.getElementById("dest-ip-add")?.value.trim() ||
    document.getElementById("target-host")?.value.trim() ||
    "";

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

function normalizeUrl(input) {
  const s = (input || "").trim();
  if (!s) return `https://${DEFAULT_URL}`;
  return s.includes("://") ? s : `https://${s}`;
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
      setOutput(curlMsgEl, await invoke("curl", { url: normalizeUrl(url) }));
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
  const tracertMsgEl = document.querySelector("#tracert-output");

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
      setOutput(webMsgEl, await invoke("invoke_web_request", { url: normalizeUrl(url) }));
    } catch (err) {
      setOutput(webMsgEl, `Error: ${err}`);
    }
  });

  document.querySelector("#run-tracert")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    setOutput(tracertMsgEl, "Running...");
    try {
      const { url } = getArgs();
      const host = toHost(url);
      setOutput(tracertMsgEl, await invoke("tracert", { host }));
    } catch (err) {
      setOutput(tracertMsgEl, `Error: ${err}`);
    }
  });
}

function normalizeStatus(status) {
  const s = String(status || "").toLowerCase();
  if (s.includes("fail")) return "error";
  if (s.includes("warn")) return "warning";
  return "healthy";
}

function setStatusChip(chipId, cardId, state, label) {
  const chip = document.getElementById(chipId);
  const card = document.getElementById(cardId);
  if (!chip || !card) return;

  chip.className = "status-chip";
  card.classList.remove("status-healthy", "status-warning", "status-error");

  if (state === "error") {
    chip.classList.add("error");
    chip.textContent = label || "Problem";
    card.classList.add("status-error");
  } else if (state === "warning") {
    chip.classList.add("warning");
    chip.textContent = label || "Attention";
    card.classList.add("status-warning");
  } else {
    chip.classList.add("healthy");
    chip.textContent = label || "Healthy";
    card.classList.add("status-healthy");
  }
}

function renderLayerDiagnostics(layerName, diagnostics) {
  const layerMap = {
    LayerOne: { output: "layer-1-output", chip: "layer-1-chip", card: "layer-1-card" },
    LayerTwo: { output: "layer-2-output", chip: "layer-2-chip", card: "layer-2-card" },
    LayerThree: { output: "layer-3-output", chip: "layer-3-chip", card: "layer-3-card" },
    LayerFour: { output: "layer-4-output", chip: "layer-4-chip", card: "layer-4-card" },
    LayerSeven: { output: "layer-7-output", chip: "layer-7-chip", card: "layer-7-card" },
  };

  const meta = layerMap[layerName];
  if (!meta) return "healthy";

  const outputEl = document.getElementById(meta.output);
  if (!outputEl) return "healthy";

  if (!diagnostics.length) {
    outputEl.textContent = "No issues reported for this layer.";
    setStatusChip(meta.chip, meta.card, "healthy", "Healthy");
    return "healthy";
  }

  let worst = "healthy";

  outputEl.textContent = diagnostics
    .map((d) => {
      const state = normalizeStatus(d.status);
      if (state === "error") worst = "error";
      else if (state === "warning" && worst !== "error") worst = "warning";

      return `[${d.title}]\n${d.message}`;
    })
    .join("\n\n");

  setStatusChip(
    meta.chip,
    meta.card,
    worst,
    worst === "error" ? "Problem" : worst === "warning" ? "Attention" : "Healthy"
  );

  return worst;
}

function setupFullDiagnosticsHandler() {
  const runBtn = document.getElementById("run-scan-btn");
  if (!runBtn) return;

  runBtn.addEventListener("click", async () => {
    const helper = document.getElementById("scan-helper");
    const summaryText = document.getElementById("summary-text");
    const summaryOutput = document.getElementById("summary-output");
    const nextStepText = document.getElementById("next-step-text");
    const nextStepOutput = document.getElementById("next-step-output");

    currentArgs = readArgsFromInputsOrDefault();

    const { url, ip_address } = getArgs();
    const host = toHost(url) || ip_address;
    const finalUrl = normalizeUrl(url);

    runBtn.disabled = true;

    if (helper) helper.textContent = "Running diagnostics...";
    if (summaryText) summaryText.textContent = "Scan in progress...";
    if (summaryOutput) {
      summaryOutput.textContent = "Collecting results from the diagnostics engine...";
    }

    setStatusChip("overall-status-chip", "summary-card", "warning", "Running");

    ["1", "2", "3", "4", "7"].forEach((n) => {
      const out = document.getElementById(`layer-${n}-output`);
      if (out) out.textContent = "Waiting for scan results...";
      setStatusChip(`layer-${n}-chip`, `layer-${n}-card`, "warning", "Running");
    });

    if (nextStepText) nextStepText.textContent = "Scan in progress.";
    if (nextStepOutput) nextStepOutput.textContent = "Collecting diagnostics...";

    try {
      const diagnostics = await invoke("run_full_diagnostics", {
        host,
        url: finalUrl,
      });

      const grouped = {
        LayerOne: [],
        LayerTwo: [],
        LayerThree: [],
        LayerFour: [],
        LayerSeven: [],
      };

      for (const d of diagnostics) {
        if (grouped[d.layer]) grouped[d.layer].push(d);
      }

      const states = [
        renderLayerDiagnostics("LayerOne", grouped.LayerOne),
        renderLayerDiagnostics("LayerTwo", grouped.LayerTwo),
        renderLayerDiagnostics("LayerThree", grouped.LayerThree),
        renderLayerDiagnostics("LayerFour", grouped.LayerFour),
        renderLayerDiagnostics("LayerSeven", grouped.LayerSeven),
      ];

      let overall = "healthy";
      if (states.includes("error")) overall = "error";
      else if (states.includes("warning")) overall = "warning";

      setStatusChip(
        "overall-status-chip",
        "summary-card",
        overall,
        overall === "error" ? "Problem" : overall === "warning" ? "Attention" : "Healthy"
      );

      if (overall === "error") {
        if (summaryText) summaryText.textContent = "A network problem was detected.";
        if (summaryOutput) {
          summaryOutput.textContent = "At least one OSI layer reported a failure.";
        }
      } else if (overall === "warning") {
        if (summaryText) summaryText.textContent = "The network may be partially working.";
        if (summaryOutput) {
          summaryOutput.textContent = "At least one OSI layer reported a warning.";
        }
      } else {
        if (summaryText) summaryText.textContent = "Your network looks healthy.";
        if (summaryOutput) {
          summaryOutput.textContent = "No major problems were detected.";
        }
      }

      const firstBad = diagnostics.find((d) => {
        const s = normalizeStatus(d.status);
        return s === "error" || s === "warning";
      });

      if (!firstBad) {
        if (nextStepText) nextStepText.textContent = "No urgent next step needed.";
        if (nextStepOutput) {
          nextStepOutput.textContent = "The scan did not find a clear failure.";
        }
      } else {
        if (nextStepText) nextStepText.textContent = `Focus on ${firstBad.title}.`;
        if (nextStepOutput) nextStepOutput.textContent = firstBad.message;
      }

      if (helper) helper.textContent = "Scan complete.";
    } catch (err) {
      console.error("run_full_diagnostics failed:", err);
      setStatusChip("overall-status-chip", "summary-card", "error", "Error");

      if (summaryText) summaryText.textContent = "The scan could not complete.";
      if (summaryOutput) summaryOutput.textContent = String(err);
      if (helper) helper.textContent = "The scan failed.";
    } finally {
      runBtn.disabled = false;
    }
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  setupSharedInputs();
  setupFullDiagnosticsHandler();
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