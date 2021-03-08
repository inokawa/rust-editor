import * as Comlink from "comlink";

const keys: string[] = [];

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
  send_key: async (key: string) => {
    keys.push(key);
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
