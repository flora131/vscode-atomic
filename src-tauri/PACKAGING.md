# VS Code Atomic Tauri packaging

`code-tauri` packaging is intentionally separate from existing Electron packaging.

## Entrypoints

- Dry-run command plan: `npm run package:tauri:dry-run`
- Execute package build: `npm run package:tauri -- --target <rust-target-triple> --bundles <bundle-list>`
- Direct script help: `node scripts/tauri-package.mjs --help`

The script runs, in order:

1. `npm run compile-build`
2. `cargo build --manifest-path src-tauri/Cargo.toml --features runtime`
3. `cargo tauri build --config src-tauri/tauri.conf.json --bundles all`

Use `--skip-frontend` or `--skip-rust-check` only when release engineering has already produced those artifacts in the current workspace.

## Signing and notarization placeholders

No signing secrets live in this repository.

Release CI should provide platform credentials through environment variables or secret files mounted at build time:

- Tauri updater signatures: `TAURI_SIGNING_PRIVATE_KEY`, `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`
- Windows code signing: certificate path/password or signing service variables owned by release engineering
- macOS code signing/notarization: Apple Developer ID certificate, `APPLE_TEAM_ID`, and either `APPLE_ID`/`APPLE_PASSWORD` or App Store Connect API key variables
- Linux package signing: distro-specific signing key material from release CI

`node scripts/tauri-package.mjs --sign --notarize` records expected secret placeholders in the command plan without reading or printing secret values.
