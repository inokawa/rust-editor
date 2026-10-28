// Keep in sync with `web/src/input.rs`, which decodes them into `core::Key`.

export type SpecialKey =
  | "Escape"
  | "Backspace"
  | "Delete"
  | "Enter"
  | "Home"
  | "End"
  | "PageUp"
  | "PageDown"
  | "ArrowUp"
  | "ArrowDown"
  | "ArrowLeft"
  | "ArrowRight"
  | "Tab";

export type CommandName = "find" | "undo" | "redo" | "save" | "exit";

export type InputEvent =
  | { kind: "key"; value: SpecialKey }
  | { kind: "char"; value: string }
  | { kind: "command"; value: CommandName }
  // value is a filename already stored in the worker's virtual fs
  | { kind: "open"; value: string };

export interface MainThreadApi {
  write: (data: string) => void;
  download: (filename: string, content: string) => void;
}
