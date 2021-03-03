import { Terminal } from "xterm";
import "xterm/css/xterm.css";
import * as Comlink from "comlink";
import { WasmWorker } from "./worker";

const term = new Terminal();
const wasm = Comlink.wrap(
  new Worker("./worker.ts", { name: "wasm", type: "module" })
) as WasmWorker;

(async () => {
  term.open(document.getElementById("terminal") as HTMLElement);
  (window as any).term = term;

  term.onKey(async (e) => {
    console.log(e);
    const event = e.domEvent;
    await wasm.send_key(
      event.code,
      event.ctrlKey,
      event.shiftKey,
      event.altKey,
      event.metaKey
    );
    console.log("fin");
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
