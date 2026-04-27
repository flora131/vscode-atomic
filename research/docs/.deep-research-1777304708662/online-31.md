(no external research applicable)

The `extensions/media-preview/` extension declares only two runtime dependencies — `@vscode/extension-telemetry` and `vscode-uri` — which are VS Code infrastructure utilities unrelated to media rendering; all image, audio, and video display is handled entirely by HTML5 browser-native APIs (`<img>`, `<audio>`, `<video>`) inside a VS Code webview, with no third-party media library whose documentation would be central to a Tauri/Rust porting analysis.
