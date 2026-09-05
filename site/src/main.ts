import "./style.css";

type Policy = "overwrite" | "skip" | "keep-both";

const scenario = {
  alwaysWriteLower: 3.01,
  alwaysWriteUpper: 19,
  conflictWrite: 2,
  reclaimable: 1,
};

function query<T extends Element>(selector: string): T | null {
  return document.querySelector<T>(selector);
}

function mib(value: number): string {
  return `${value.toFixed(1)} MiB`;
}

const form = query<HTMLFormElement>("#policy-form");
const freeInput = query<HTMLInputElement>("#free-space");
const error = query<HTMLElement>("#free-error");
const result = query<HTMLElement>("#result");

function renderPlan(): void {
  if (!form || !freeInput || !error || !result) return;
  const data = new FormData(form);
  const policy = (data.get("policy") ?? "overwrite") as Policy;
  const free = Number(freeInput.value);
  const invalid = !Number.isFinite(free) || free < 0 || free > 9999;
  freeInput.setAttribute("aria-invalid", String(invalid));
  error.textContent = invalid ? "Enter free space from 0 to 9,999 MiB." : "";
  if (invalid) {
    result.setAttribute("aria-busy", "true");
    return;
  }
  result.removeAttribute("aria-busy");

  const writesConflict = policy !== "skip";
  const headroom = scenario.alwaysWriteUpper + (writesConflict ? scenario.conflictWrite : 0);
  const reclaimed = policy === "overwrite" ? scenario.reclaimable : 0;
  const net = headroom - reclaimed;
  const safe = free >= headroom;
  const difference = Math.abs(free - headroom);

  const available = query<HTMLElement>("#available-value");
  const headroomValue = query<HTMLElement>("#headroom-value");
  const netValue = query<HTMLElement>("#net-value");
  const status = query<HTMLElement>("#result-status");
  const explanation = query<HTMLElement>("#result-explanation");
  const actions = query<HTMLOListElement>("#action-list");
  const fill = query<HTMLElement>("#capacity-fill");
  const marker = query<HTMLElement>("#capacity-marker");
  if (!available || !headroomValue || !netValue || !status || !explanation || !actions || !fill || !marker) return;

  available.textContent = mib(free);
  headroomValue.textContent = mib(headroom);
  netValue.textContent = `+${mib(net)}`;
  status.className = `result-status ${safe ? "is-safe" : "is-danger"}`;
  status.replaceChildren();
  const icon = document.createElement("span");
  icon.className = "status-icon";
  icon.setAttribute("aria-hidden", "true");
  icon.textContent = safe ? "✓" : "!";
  const statusCopy = document.createElement("span");
  const statusLabel = document.createElement("small");
  statusLabel.textContent = "Preflight result";
  const statusValue = document.createElement("strong");
  statusValue.textContent = safe ? "Safe to start" : "Do not start";
  statusCopy.append(statusLabel, statusValue);
  status.append(icon, statusCopy);

  const policyNote = policy === "overwrite"
    ? `The overwrite is staged before ${mib(scenario.reclaimable)} can be reclaimed.`
    : policy === "skip"
      ? "The existing photo stays untouched and adds no write cost."
      : "The photo gets a new name, so both files remain.";
  const spaceNote = safe
    ? `The upper bound fits with ${mib(difference)} to spare.`
    : `The upper bound exceeds available space by ${mib(difference)}.`;
  explanation.textContent = `${policyNote} ${spaceNote}`;

  const conflictAction = policy === "overwrite"
    ? ["OVERWRITE", "photos.raw", "2.0 MiB"]
    : policy === "skip"
      ? ["SKIP", "photos.raw", "0 B"]
      : ["COPY", "photos (copy 1).raw", "2.0 MiB"];
  const plan = [
    ["MKDIR", "archive", "0 B"],
    ["COPY", "archive/interview.mov", "3.0 MiB"],
    ["COPY", "archive/project-notes.txt", "4.0 KiB"],
    ["MKDIR", "disk-images", "0 B"],
    ["COPY", "disk-images/field-laptop.img", "4 KiB–16 MiB"],
    conflictAction,
  ];
  actions.replaceChildren(...plan.map(([operation, path, size]) => {
    const item = document.createElement("li");
    const operationNode = document.createElement("b");
    const pathNode = document.createElement("code");
    const sizeNode = document.createElement("span");
    operationNode.textContent = operation;
    pathNode.textContent = path;
    sizeNode.textContent = size;
    item.append(operationNode, pathNode, sizeNode);
    return item;
  }));

  const scale = Math.max(headroom, free, 1);
  fill.style.width = `${Math.min(100, (headroom / scale) * 100)}%`;
  marker.style.left = `${Math.min(100, (free / scale) * 100)}%`;
  result.dataset.verdict = safe ? "safe" : "danger";
}

form?.addEventListener("input", renderPlan);
renderPlan();

query<HTMLButtonElement>("#reset-demo")?.addEventListener("click", () => {
  form?.reset();
  renderPlan();
  const message = query<HTMLElement>("#demo-message");
  if (message) message.textContent = "Demo reset to overwrite with 16 MiB free.";
  query<HTMLInputElement>('input[name="policy"]:checked')?.focus();
});

query<HTMLButtonElement>("#replay-recording")?.addEventListener("click", (event) => {
  const recording = query<HTMLElement>(".terminal-recording");
  if (!recording) return;
  recording.classList.remove("is-replaying");
  void recording.offsetWidth;
  recording.classList.add("is-replaying");
  const button = event.currentTarget as HTMLButtonElement;
  button.textContent = "Playing demo";
  window.setTimeout(() => { button.textContent = "Replay terminal demo"; }, 2400);
});

const networkState = query<HTMLElement>("#network-state");
function renderNetwork(): void {
  if (!networkState) return;
  networkState.textContent = navigator.onLine ? "● Ready offline" : "○ Offline · sample works";
}
window.addEventListener("online", renderNetwork);
window.addEventListener("offline", renderNetwork);
renderNetwork();

if ("serviceWorker" in navigator && import.meta.env.PROD) {
  window.addEventListener("load", () => {
    void navigator.serviceWorker.register("/sw.js");
  });
}
