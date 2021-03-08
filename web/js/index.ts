import { Terminal } from "xterm";
import "xterm/css/xterm.css";
import * as Comlink from "comlink";
import { WasmWorker } from "./worker";

const term = new Terminal();
term.open(document.getElementById("terminal") as HTMLElement);
(window as any).term = term;

const wasm = Comlink.wrap(
  new Worker("./worker.ts", { name: "wasm", type: "module" })
) as WasmWorker;

(async () => {
  term.onKey(async (e) => {
    const event = e.domEvent;
    if (event.isComposing) return;
    await wasm.send_key(event.key);
  });
  term.onData((e) => {
    console.log(e);
  });

  await wasm.init(
    Comlink.proxy((data) => {
      term.write(data);
    })
  );
})();
