import { Terminal } from "xterm";
import "xterm/css/xterm.css";

(async () => {
  const term = new Terminal();
  term.open(document.getElementById("terminal") as HTMLElement);
  term.write("Hello from \x1B[1;3;31mxterm.js\x1B[0m $ ");

  const wasm = await import("../pkg/index.js");
  console.log(wasm);

  document
    .querySelector(".textarea")
    ?.addEventListener("keydown", (ev: Event) => {
      let e = ev as KeyboardEvent;
      console.log(e);
      console.log(e.code);
      console.log(e.key);
      console.log(e.shiftKey);
      console.log(e.ctrlKey);
    });
})();
