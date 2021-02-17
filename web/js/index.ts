(async () => {
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
