import { Terminal } from "xterm";
import "xterm/css/xterm.css";
import * as Comlink from "comlink";
import { WasmWorker } from "./worker";

const term = new Terminal();
term.open(document.getElementById("terminal") as HTMLElement);
term.resize(100, 40);
(window as any).term = term;

const wasm = Comlink.wrap(
  new Worker("./worker.ts", { name: "wasm", type: "module" })
) as WasmWorker;

(async () => {
  let prevKey = "";
  term.onKey(async (e) => {
    prevKey = e.key;
    const event = e.domEvent;
    if (event.isComposing) return;
    await wasm.send_key(event.key);
  });
  term.onData(async (data) => {
    if (data === prevKey) return;
    for (const d of data.split("")) {
      await wasm.send_key(d);
    }
  });

  await wasm.init(
    Comlink.proxy((data) => {
      term.write(data);
    })
  );
})();
