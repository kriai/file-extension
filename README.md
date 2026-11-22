# File Type Counter

A simple native Windows tool built in Rust. Drag files into the app window: the left pane lists them and the right pane shows totals by file extension.

## Run the executable

After a release build, open `target\release\file-type-counter.exe`.

## Build it again later

Rust is installed once for the Windows account. From this folder, run:

```powershell
cargo run
# or make an optimized executable
cargo build --release
```
