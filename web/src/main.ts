import "./style.css";
import init, {
  registry_json,
  generate_json,
  verify_json,
  odds_json,
  version,
} from "../pkg/drawlab_wasm.js";

type Profile = {
  id: string;
  name: string;
  status: "laboratory" | "unverified";
  pool: number;
  pick: number;
};
type RecordData = {
  label: string;
  profile: {
    id: string;
    version: string;
    name: string;
    extra: { kind: string };
  };
  mode: string;
  boards: { main: number[]; extra: number | null }[];
};
function el<T extends HTMLElement>(id: string): T {
  const e = document.getElementById(id);
  if (!e) throw new Error("Missing element " + id);
  return e as T;
}
const select = el<HTMLSelectElement>("profile");
const count = el<HTMLInputElement>("count");
const status = el("status");
const form = el<HTMLFormElement>("generate-form");
let latest: string | null = null;
let profiles: Profile[] = [];
function setStatus(message: string, error = false) {
  status.textContent = message;
  status.classList.toggle("error", error);
}
function clearOutput() {
  latest = null;
  el("boards").replaceChildren();
  el("record").textContent = "No record yet.";
  for (const id of ["download-json", "download-text", "download-csv"])
    el<HTMLButtonElement>(id).disabled = true;
}
function refresh() {
  clearOutput();
  const p = profiles.find((p) => p.id === select.value);
  if (!p) return;
  const o = JSON.parse(odds_json(p.id)) as { denominator: string };
  el("odds").textContent =
    "1 in " + BigInt(o.denominator).toLocaleString("en-ZA");
  el("profile-note").textContent =
    p.pick +
    " distinct main numbers from 1–" +
    p.pool +
    ". Laboratory model only.";
  const mode = new FormData(form).get("mode");
  el("generate").textContent =
    mode === "draw" ? "Simulate draws ↗" : "Generate selections ↗";
  el("mode-tag").textContent = mode === "draw" ? "DRAW MODE" : "SELECTION MODE";
  setStatus("Ready. Generated boards stay on this device.");
}
function render(json: string) {
  const r = JSON.parse(json) as RecordData;
  const fragment = document.createDocumentFragment();
  r.boards.forEach((b, i) => {
    const article = document.createElement("article");
    article.className = "board";
    const label = document.createElement("div");
    label.className = "board-label";
    label.textContent =
      (r.mode === "draw" ? "SIMULATED DRAW " : "SELECTION ") +
      String(i + 1).padStart(2, "0");
    const balls = document.createElement("div");
    balls.className = "balls";
    for (const n of b.main) {
      const ball = document.createElement("span");
      ball.className = "ball";
      ball.textContent = String(n).padStart(2, "0");
      balls.append(ball);
    }
    if (b.extra !== null) {
      const label = document.createElement("span");
      label.className = "extra-label";
      label.textContent =
        r.profile.extra.kind === "remaining" ? "Bonus" : "Extra";
      const ball = document.createElement("span");
      ball.className = "ball extra";
      ball.textContent = String(b.extra).padStart(2, "0");
      balls.append(label, ball);
    }
    article.append(label, balls);
    fragment.append(article);
  });
  el("boards").replaceChildren(fragment);
  el("record").textContent = json;
  for (const id of ["download-json", "download-text", "download-csv"])
    el<HTMLButtonElement>(id).disabled = false;
  setStatus(
    r.boards.length +
      " " +
      (r.mode === "draw" ? "draw(s)" : "selection(s)") +
      " generated. Valid combinations are equally likely; duplicates across boards are possible.",
  );
}
function download(data: string, type: string, extension: string) {
  const url = URL.createObjectURL(new Blob([data], { type }));
  const a = document.createElement("a");
  a.href = url;
  a.download = "drawlab-laboratory." + extension;
  a.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
}
form.addEventListener("submit", (e) => {
  e.preventDefault();
  clearOutput();
  try {
    const amount = count.valueAsNumber;
    if (!Number.isInteger(amount) || amount < 1 || amount > 100)
      throw new Error("Choose a whole number of boards from 1 to 100.");
    const mode = String(new FormData(form).get("mode"));
    const json = generate_json(select.value, mode, amount);
    verify_json(json);
    latest = json;
    render(json);
  } catch (e) {
    setStatus(String(e), true);
  }
});
select.addEventListener("change", refresh);
form
  .querySelectorAll<HTMLInputElement>("input[name=mode]")
  .forEach((input) => input.addEventListener("change", refresh));
count.addEventListener("input", () => {
  clearOutput();
  setStatus("Configuration changed. Generate a new record.");
});
el("download-json").addEventListener("click", () => {
  if (latest) download(latest, "application/json", "json");
});
el("download-text").addEventListener("click", () => {
  if (!latest) return;
  const r = JSON.parse(latest) as RecordData;
  const rows = r.boards.map(
    (b, i) =>
      i +
      1 +
      ": " +
      b.main.join(" ") +
      (b.extra === null ? "" : " | extra: " + b.extra),
  );
  download(
    [
      r.label,
      "Profile: " + r.profile.id + " / " + r.profile.version,
      "Mode: " + r.mode,
      ...rows,
    ].join("\n"),
    "text/plain",
    "txt",
  );
});
el("download-csv").addEventListener("click", () => {
  if (!latest) return;
  const r = JSON.parse(latest) as RecordData;
  // Trusted fixed identifiers and bounded numbers only; no imported spreadsheet formulas.
  const rows = r.boards.map((b, i) =>
    [
      "laboratory",
      r.profile.id,
      r.mode,
      i + 1,
      '"' + b.main.join(" ") + '"',
      b.extra ?? "",
    ].join(","),
  );
  download(
    ["classification,profile,mode,board,main,extra", ...rows].join("\r\n"),
    "text/csv",
    "csv",
  );
});
el<HTMLInputElement>("import").addEventListener("change", async (e) => {
  const message = el("verification");
  message.textContent = "";
  message.classList.remove("error");
  const file = (e.target as HTMLInputElement).files?.[0];
  if (!file) return;
  try {
    if (file.size > 1048576) throw new Error("RECORD_TOO_LARGE: maximum 1 MiB");
    message.textContent = verify_json(await file.text());
  } catch (error) {
    message.classList.add("error");
    message.textContent = String(error);
  }
});
async function start() {
  try {
    await init();
    profiles = JSON.parse(registry_json()) as Profile[];
    for (const p of profiles) {
      if (p.status === "laboratory") {
        const option = document.createElement("option");
        option.value = p.id;
        option.textContent = p.name;
        select.append(option);
      } else {
        const li = document.createElement("li");
        li.textContent = p.name + ": disabled — rules unverified";
        el("coverage").append(li);
      }
    }
    select.value = "lab-5of36";
    select.disabled = false;
    el<HTMLButtonElement>("generate").disabled = false;
    el<HTMLInputElement>("import").disabled = false;
    el("version").textContent = "v" + version();
    refresh();
  } catch (error) {
    el("version").textContent = "Engine unavailable";
    setStatus(
      "Engine could not start. No fallback generator is used. " + String(error),
      true,
    );
  }
}
void start();
