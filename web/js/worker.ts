import * as Comlink from "comlink";

const keys: { key: string; ctrl: boolean }[] = [];

let wasm: typeof import("../pkg/index.js");
const worker = {
  init: async (write: (str: string) => void) => {
    (self as any).term = {
      write,
      get_key: () => {
        const key = keys.shift();
        return key?.key;
      },
    };
    wasm = await import("../pkg/index.js");
  },
  send_key: async (key: string, ctrl: boolean) => {
    keys.push({ key, ctrl });
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
