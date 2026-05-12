# Locator Partition 62: esbuild-extension-common.mts Analysis

## Scope
- `extensions/esbuild-extension-common.mts` (50 LOC)

## Implementation

### Build Configuration Module
- `/home/norinlavaee/projects/vscode-atomic/extensions/esbuild-extension-common.mts` — Common build script for VS Code extensions using esbuild. Exports `run()` function that configures esbuild for bundling extensions targeting either 'node' or 'browser' platforms. Includes platform-specific resolution logic (mainFields, aliases for browser environment, process polyfills). Target ES2024, with tree-shaking, minification, and source maps enabled.

## Types / Interfaces

- `ExtensionRunConfig` interface (lines 11-14) — Extends `RunConfig` with `platform: 'node' | 'browser'` and optional `format: 'cjs' | 'esm'`

## Configuration

- esbuild.BuildOptions configuration applied per platform:
  - Node platform: mainFields = ['module', 'main']
  - Browser platform: mainFields = ['browser', 'module', 'main'], includes alias for 'path' -> 'path-browserify', defines process.platform, process.env, and BROWSER_ENV
  - Common options: ES2024 target, external vscode module, bundling with minification and tree-shaking

## Research Relevance

This file demonstrates VS Code's current build infrastructure for extensions—a TypeScript/esbuild-based configuration that handles dual-platform (Node/browser) extension bundling. A Tauri/Rust port would require equivalent build tooling for extensions running in a Rust runtime environment rather than Node.js or browser contexts. The platform abstraction pattern here (resolving different entry points per platform) is relevant for understanding how VS Code maintains compatibility across execution environments.

