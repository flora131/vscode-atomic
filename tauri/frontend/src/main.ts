/**
 * main.ts — Vite entry point.
 *
 * Mounts a Monaco editor into #workbench, opens a sample untitled buffer,
 * and registers basic key bindings. Signals the Rust back-end that the
 * workbench is ready via bridge.workbenchReady().
 */

import * as monaco from 'monaco-editor';
import { workbenchReady } from './bridge.js';

// ─────────────────────────────────────────────────────────────────────────────
// Monaco worker config (required for full language support)
// ─────────────────────────────────────────────────────────────────────────────
// Vite's default asset handling makes monaco workers available as blobs.
// @see https://github.com/vitejs/vite/discussions/1791
(self as unknown as { MonacoEnvironment: monaco.Environment }).MonacoEnvironment = {
  getWorker(_workerId: string, _label: string): Worker {
    // For this skeleton we use the editor worker for all labels.
    // Full language servers can be added per-label later.
    const workerUrl = new URL(
      'monaco-editor/esm/vs/editor/editor.worker',
      import.meta.url,
    );
    return new Worker(workerUrl, { type: 'module' });
  },
};

// ─────────────────────────────────────────────────────────────────────────────
// DOM bootstrap
// ─────────────────────────────────────────────────────────────────────────────

function bootstrap(): void {
  const container = document.getElementById('workbench');
  if (!container) {
    throw new Error('Missing #workbench element');
  }

  // ── Editor ────────────────────────────────────────────────────────────────
  const editor = monaco.editor.create(container, {
    value: [
      '// VSCode Tauri — untitled scratch buffer',
      '// Open a folder with Cmd/Ctrl+K Cmd/Ctrl+O',
      '',
      'function hello(): string {',
      '  return "world";',
      '}',
    ].join('\n'),
    language: 'typescript',
    theme: 'vs-dark',
    automaticLayout: true,
    minimap: { enabled: false },
    fontSize: 14,
    wordWrap: 'on',
    scrollBeyondLastLine: false,
  });

  // ── Key bindings ──────────────────────────────────────────────────────────

  // Cmd/Ctrl + S — placeholder save action
  editor.addCommand(
    monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS,
    () => {
      const model = editor.getModel();
      if (model) {
        console.info('[workbench] save triggered, uri:', model.uri.toString());
        // TODO: pipe through writeFileChannel once Rust side is wired
      }
    },
  );

  // Cmd/Ctrl + Shift + P — placeholder command palette
  editor.addCommand(
    monaco.KeyMod.CtrlCmd | monaco.KeyMod.Shift | monaco.KeyCode.KeyP,
    () => {
      console.info('[workbench] command palette triggered (not yet implemented)');
    },
  );

  // ── Resize guard ─────────────────────────────────────────────────────────
  window.addEventListener('resize', () => editor.layout());

  // ── IPC handshake ─────────────────────────────────────────────────────────
  workbenchReady({ version: '0.0.1' }).catch((err: unknown) => {
    // Expected to fail until the Rust command is registered; log only.
    console.warn('[workbench] workbench_ready not handled by Rust side yet:', err);
  });
}

// Run after DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', bootstrap);
} else {
  bootstrap();
}
