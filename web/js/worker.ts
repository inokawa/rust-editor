import * as Comlink from "comlink";

const keys: [key: string, ctrl: boolean][] = [];

let wasm: typeof import("../pkg/index.js");
const worker = {
  init: async (write: (str: string) => void, cols: number, rows: number) => {
    (self as any).term = {
      write,
      read_key: () => {
        return keys[0]?.[0];
      },
      read_ctrl: () => {
        return keys[0]?.[1] ?? false;
      },
      read_end: () => {
        keys.shift();
      },
      get_col_size: () => cols,
      get_row_size: () => rows,
    };
    wasm = await import("../pkg/index.js");
  },
  send_key: async (key: string, ctrl: boolean) => {
    keys.push([key, ctrl]);
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
