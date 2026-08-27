import "./style.css";

type Policy = "overwrite" | "skip" | "keep-both";

const scenario = {
  alwaysWriteLower: 8.4,
  alwaysWriteUpper: 15.2,
  conflictWrite: 6.4,
  reclaimable: 5.9,
};

const form = document.querySelector<HTMLFormElement>("#policy-form");
const freeInput = document.querySelector<HTMLInputElement>("#free-space");
const error = document.querySelector<HTMLElement>("#free-error");
const result = document.querySelector<HTMLElement>("#result");

function query<T extends Element>(selector: string): T {
  const node = document.querySelector<T>(selector);
  if (!node) throw new Error(`Missing required element: ${selector}`);
  return node;
}

const els = {
  available: query<HTMLElement>("#available-value"),
  headroom: query<HTMLElement>("#headroom-value"),
  net: query<HTMLElement>("#net-value"),
  status: query<HTMLElement>("#result-status"),
  explanation: query<HTMLElement>("#result-explanation"),
  actions: query<HTMLOListElement>("#action-list"),
  fill: query<HTMLElement>("#capacity-fill"),
  marker: query<HTMLElement>("#capacity-marker"),
};

function gb(value: number): string {
  return `${value.toFixed(1)} GB`;
}

function render(): void {
  if (!form || !freeInput || !error || !result) return;
  const data = new FormData(form);
  const policy = (data.get("policy") ?? "overwrite") as Policy;
  const free = Number(freeInput.value);
  const invalid = !Number.isFinite(free) || free < 0 || free > 9999;
  freeInput.setAttribute("aria-invalid", String(invalid));
  error.textContent = invalid ? "Enter free space from 0 to 9,999 GB." : "";
  if (invalid) {
    result.setAttribute("aria-busy", "true");
    return;
  }
  result.removeAttribute("aria-busy");

  const writesConflict = policy !== "skip";
  const headroom = scenario.alwaysWriteUpper + (writesConflict ? scenario.conflictWrite : 0);
  const lower = scenario.alwaysWriteLower + (writesConflict ? scenario.conflictWrite : 0);
  const reclaimed = policy === "overwrite" ? scenario.reclaimable : 0;
  const net = headroom - reclaimed;
  const safe = free >= headroom;
  const difference = Math.abs(free - headroom);

  els.available.textContent = gb(free);
  els.headroom.textContent = gb(headroom);
  els.net.textContent = `${net >= 0 ? "+" : "−"}${gb(Math.abs(net))}`;
  els.status.className = `result-status ${safe ? "is-safe" : "is-danger"}`;
  els.status.innerHTML = `<span class="status-icon" aria-hidden="true">${safe ? "✓" : "!"}</span><span><small>Preflight result</small><strong>${safe ? "Safe to start" : "Do not start"}</strong></span>`;

  const policyNote = policy === "overwrite"
    ? `The overwrite is staged before ${gb(scenario.reclaimable)} can be reclaimed.`
    : policy === "skip"
      ? "The conflicting file stays untouched and adds no write cost."
      : "The conflict gets a new name, so both full files remain.";
  els.explanation.textContent = `${policyNote} The upper bound ${safe ? "fits with" : "exceeds available space by"} ${gb(difference)} ${safe ? "to spare" : ""}.`;

  const actionForConflict = policy === "overwrite"
    ? ["OVERWRITE", "photos.raw", "6.4 GB"]
    : policy === "skip"
      ? ["SKIP", "photos.raw", "0 B"]
      : ["COPY", "photos (copy 1).raw", "6.4 GB"];
  const actions = [
    ["COPY", "archive/video.mov", "7.2 GB"],
    actionForConflict,
    ["COPY", "sparse.image", "1.2–8.0 GB"],
  ];
  els.actions.replaceChildren(...actions.map(([operation, path, size]) => {
    const item = document.createElement("li");
    item.innerHTML = `<b>${operation}</b><code>${path}</code><span>${size}</span>`;
    return item;
  }));

  const scale = Math.max(headroom, free, 1);
  els.fill.style.width = `${Math.min(100, (headroom / scale) * 100)}%`;
  els.marker.style.left = `${Math.min(100, (free / scale) * 100)}%`;
  result.dataset.verdict = safe ? "safe" : "danger";
}

form?.addEventListener("input", render);
render();

document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const target = document.querySelector<HTMLElement>(button.dataset.copy ?? "");
    if (!target) return;
    try {
      await navigator.clipboard.writeText(target.textContent ?? "");
      button.textContent = "Copied";
      window.setTimeout(() => { button.textContent = "Copy"; }, 1600);
    } catch {
      const selection = window.getSelection();
      const range = document.createRange();
      range.selectNodeContents(target);
      selection?.removeAllRanges();
      selection?.addRange(range);
      button.textContent = "Selected";
    }
  });
});

const networkState = document.querySelector<HTMLElement>("#network-state");
function renderNetwork(): void {
  if (!networkState) return;
  networkState.textContent = navigator.onLine ? "● Local-only" : "○ Offline · demo works";
}
window.addEventListener("online", renderNetwork);
window.addEventListener("offline", renderNetwork);
renderNetwork();

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => {
    void navigator.serviceWorker.register("/sw.js");
  });
}
