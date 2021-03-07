import * as Comlink from "comlink";

let keys: string[] = [];

let wasm: typeof import("../pkg/index.js");
const worker = {
  init: async (write: (str: string) => void) => {
    (self as any).term = {
      write,
      get_key: () => {
        const key = keys.shift();
        return key;
      },
    };
    wasm = await import("../pkg/index.js");
  },
  send_key: async (...args: any[]) => {
    keys.push(args[0]);
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
