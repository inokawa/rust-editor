import * as Comlink from "comlink";
import type { InputEvent, MainThreadApi } from "./protocol";

const events: InputEvent[] = [];
const size = { cols: 0, rows: 0 };
const files = new Map<string, string>();

const worker = {
  init: async (main: MainThreadApi, cols: number, rows: number) => {
    worker.resize(cols, rows);
    // Globals bound by `web/src/*.rs` through `js_namespace`
    Object.assign(self, {
      term: {
        write: (data: string) => main.write(data),
        get_col_size: () => size.cols,
        get_row_size: () => size.rows,
      },
      input: {
        next: (): InputEvent | undefined => events.shift(),
      },
      fs: {
        read: (filename: string): string | undefined => files.get(filename),
        write: (filename: string, content: string) => {
          files.set(filename, content);
          main.download(filename, content);
        },
      },
    });
    await import("../pkg/index.js");
  },
  resize: (cols: number, rows: number) => {
    size.cols = cols;
    size.rows = rows;
  },
  send: (event: InputEvent) => {
    events.push(event);
  },
  open: (filename: string, content: string) => {
    files.set(filename, content);
    events.push({ kind: "open", value: filename });
  },
};

Comlink.expose(worker);

export type WasmWorker = typeof worker;
