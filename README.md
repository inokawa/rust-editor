# rust-editor

![deploy](https://github.com/inokawa/rust-editor/workflows/check/badge.svg)

WIP

An implementation of text editor with Rust/WebAssembly.

This is a hobby project just for my study, but I'm trying to make it as much as practical.

This editor is roughly based on [kilo](https://github.com/antirez/kilo), but has some improvements.

- Support ASCII/UTF-8 encoded texts
- Support Undo/Redo
- Run on terminal in UNIX, and on browser with WebAssembly

NOTE: Some features are not implemented completely.

## Start

```sh
git clone git@github.com:inokawa/rust-editor.git
cd rust-editor
cargo run "path/to/file.txt"
```

## References

Thank you for this great tutorial of kilo:

- https://viewsourcecode.org/snaptoken/kilo/

And thank you for other great implementations of kilo:

- https://github.com/rhysd/kiro-editor
- https://github.com/ilai-deutel/kibi
- https://www.philippflenker.com/hecto/
- https://github.com/nkon/ked-texteditor
