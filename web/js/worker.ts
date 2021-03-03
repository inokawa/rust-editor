import * as Comlink from "comlink";

let wasm: typeof import("../pkg/index.js");
const worker = {
  init: async (write: (str: string) => void) => {
    (self as any).term = { write };
    wasm = await import("../pkg/index.js");
  },
  send_key: async (...args: Parameters<typeof wasm["send_key"]>) => {
    wasm.send_key(...args);
    (self as any).term["write"](args[0]);
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
