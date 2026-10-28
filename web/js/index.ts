import { Terminal } from "xterm";
import { FitAddon } from "xterm-addon-fit";
import "xterm/css/xterm.css";
import * as Comlink from "comlink";
import type { WasmWorker } from "./worker";
import type { MainThreadApi } from "./protocol";
import { dataToInputEvents, resolveKey } from "./keymap";
import { download, listenFileDrop } from "./files";

const term = new Terminal({ scrollback: 0 });
const fitAddon = new FitAddon();
term.loadAddon(fitAddon);
term.open(document.getElementById("terminal") as HTMLElement);
fitAddon.fit();
(window as any).term = term;

const wasm = Comlink.wrap<WasmWorker>(
  new Worker(new URL("./worker.ts", import.meta.url), { name: "wasm" })
);

term.attachCustomKeyEventHandler((e) => {
  const action = resolveKey(e);
  if (!action) return true;
  if (action !== "native") {
    e.preventDefault();
    // called for keydown, keypress and keyup
    if (e.type === "keydown") wasm.send(action);
  }
  return false;
});
term.onData((data) => {
  for (const event of dataToInputEvents(data)) {
    wasm.send(event);
  }
});

listenFileDrop(document.body, async (file) => {
  await wasm.open(file.name, await file.text());
  term.focus();
});

let resizeFrame = 0;
window.addEventListener("resize", () => {
  cancelAnimationFrame(resizeFrame);
  resizeFrame = requestAnimationFrame(() => fitAddon.fit());
});
term.onResize(({ cols, rows }) => {
  wasm.resize(cols, rows);
});

wasm.init(
  Comlink.proxy<MainThreadApi>({ write: (data) => term.write(data), download }),
  term.cols,
  term.rows
);
term.focus();
