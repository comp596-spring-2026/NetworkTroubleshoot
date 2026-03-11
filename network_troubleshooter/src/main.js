const { invoke } = window.__TAURI__.core;

window.addEventListener("DOMContentLoaded", () => {
  const ipLinkMsgEl = document.querySelector("#ip-link-output");
  const nmcliMsgEl = document.querySelector("#nmcli-output");
  const ipNeighMsgEl = document.querySelector("#ip-neigh-output");
  const ipPingMsgEl = document.querySelector("#ping-output");
  const netcatMsgEl = document.querySelector("#netcat-output");
  const curlMsgEl = document.querySelector("#curl-output");
  const digMsgEl = document.querySelector("#dig-output");
  const traceRouteMsgEl = document.querySelector("#traceroute-output");

  const DEFAULT_URL = "www.archlinux.org";
  const DEFAULT_IP = "8.8.8.8";

  let currentArgs = null;

  function readArgsFromInputsOrDefault() {
    const urlInput = document.getElementById("dest-url").value.trim();
    const ipInput = document.getElementById("dest-ip-add").value.trim();

    return {
      url: urlInput || DEFAULT_URL,
      ip_address: ipInput || DEFAULT_IP,
    };
  }

  function getArgs() {
    // If user never hit Submit, still run with defaults (or current inputs)
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

  document.getElementById("submitBtn").addEventListener("click", async () => {
    currentArgs = readArgsFromInputsOrDefault();
    console.log("Saved target:", currentArgs);

  });

  document.querySelector("#run-ip-link").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipLinkMsgEl.textContent = "Running...";
    try {
      ipLinkMsgEl.textContent = await invoke("ip_link");
    } catch (err) {
      ipLinkMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-nmcli").addEventListener("submit", async (e) => {
    e.preventDefault();
    nmcliMsgEl.textContent = "Running...";
    try {
      nmcliMsgEl.textContent = await invoke("nmcli");
    } catch (err) {
      nmcliMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-ip-neigh").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipNeighMsgEl.textContent = "Running...";
    try {
      ipNeighMsgEl.textContent = await invoke("ip_neigh");
    } catch (err) {
      ipNeighMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-ping").addEventListener("submit", async (e) => {
    e.preventDefault();
    ipPingMsgEl.textContent = "Running...";
    try {
      const { ip_address } = getArgs();
      // Requires Rust: ping(ip: String)
      ipPingMsgEl.textContent = await invoke("ping", { ip: ip_address });
    } catch (err) {
      ipPingMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-netcat").addEventListener("submit", async (e) => {
    e.preventDefault();
    netcatMsgEl.textContent = "Running...";
    try {
      const { url } = getArgs();
      const host = toHost(url);
      netcatMsgEl.textContent = await invoke("netcat", { host});
    } catch (err) {
      netcatMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-curl").addEventListener("submit", async (e) => {
    e.preventDefault();
    curlMsgEl.textContent = "Running...";
    try {
      const { url } = getArgs();
      // Requires Rust: curl(url: String)
      curlMsgEl.textContent = await invoke("curl", { url });
    } catch (err) {
      curlMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-dig").addEventListener("submit", async (e) => {
    e.preventDefault();
    digMsgEl.textContent = "Running...";
    try {
      const { url } = getArgs();
      const host = toHost(url);
      // Requires Rust: dig(host: String)
      digMsgEl.textContent = await invoke("dig", { host });
    } catch (err) {
      digMsgEl.textContent = `Error: ${err}`;
    }
  });

  document.querySelector("#run-traceroute").addEventListener("submit", async (e) => {
    e.preventDefault();
    traceRouteMsgEl.textContent = "Running...";
    try {
      const { url } = getArgs();
      const host = toHost(url);
      // Requires Rust: traceroute(host: String)
      traceRouteMsgEl.textContent = await invoke("traceroute", { host });
    } catch (err) {
      traceRouteMsgEl.textContent = `Error: ${err}`;
    }
  });
});