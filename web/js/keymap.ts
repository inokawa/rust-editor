import type { CommandName, InputEvent, SpecialKey } from "./protocol";

const isMac = /Mac|iPhone|iPad/.test(navigator.platform);

const COMMANDS = new Map<string, CommandName>([
  ["f", "find"],
  ["z", "undo"],
  ["y", "redo"],
  ["s", "save"],
  ["q", "exit"],
]);
const CTRL_ALIASES = new Map<string, SpecialKey>([
  ["h", "Backspace"],
  ["l", "Escape"],
]);
// Leave clipboard shortcuts to the browser so paste reaches the terminal as data.
const NATIVE = new Set(["c", "v", "x"]);

const SPECIAL_KEYS = new Set<string>([
  "Escape",
  "Backspace",
  "Delete",
  "Enter",
  "Home",
  "End",
  "PageUp",
  "PageDown",
  "ArrowUp",
  "ArrowDown",
  "ArrowLeft",
  "ArrowRight",
  "Tab",
] satisfies SpecialKey[]);

/**
 * - InputEvent: handled by the editor; browser default and xterm are suppressed
 * - "native": let the browser handle it (e.g. clipboard)
 * - undefined: let xterm handle it; printable text arrives via onData
 */
export const resolveKey = (e: KeyboardEvent): InputEvent | "native" | undefined => {
  if (e.isComposing || e.keyCode === 229) return;

  if ((e.ctrlKey || (isMac && e.metaKey)) && !e.altKey) {
    const k = e.key.toLowerCase();
    const command = COMMANDS.get(k);
    if (command) return { kind: "command", value: command };
    const alias = CTRL_ALIASES.get(k);
    if (alias) return { kind: "key", value: alias };
    if (NATIVE.has(k)) return "native";
    return;
  }

  if (!e.ctrlKey && !e.metaKey && SPECIAL_KEYS.has(e.key)) {
    return { kind: "key", value: e.key as SpecialKey };
  }
};

export const dataToInputEvents = (data: string): InputEvent[] => {
  if (data.startsWith("\x1b")) return [];
  const events: InputEvent[] = [];
  for (const ch of data) {
    if (ch === "\r" || ch === "\n") {
      events.push({ kind: "key", value: "Enter" });
    } else if (ch === "\t") {
      events.push({ kind: "key", value: "Tab" });
    } else if (ch >= " " && ch !== "\x7f") {
      events.push({ kind: "char", value: ch });
    }
  }
  return events;
};
