(no external research applicable)

The scope is a 7-line bootstrap shim (`src/bootstrap-server.ts`) whose sole purpose is to delete the `ELECTRON_RUN_AS_NODE` environment variable. There are no external libraries, no APIs, no network calls, and no frameworks involved. The entire porting decision for this file is mechanical: the variable is Electron-specific and meaningless in a Tauri/Rust context, so the file and its logic are dropped entirely with no replacement. No external research is warranted.
