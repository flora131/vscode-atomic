/**
 * TDD tests for build pipeline plugins (NLS rewrite, mangle-privates, source-map rewrite).
 * Run: pnpm -C tauri/frontend test
 */

import { describe, it, expect } from 'vitest';

// ─── NLS rewrite ─────────────────────────────────────────────────────────────

import { nlsRewrite } from '../build-plugins.js';

describe('nlsRewrite', () => {
  it('replaces nls.localize(key, default) with bundle message when key present', () => {
    const code = `var x = nls.localize("my.key", "Default text");`;
    const bundle = { 'my.key': 'Localized text' };
    const result = nlsRewrite(code, bundle);
    expect(result).toBe(`var x = "Localized text";`);
  });

  it('falls back to default string when key absent from bundle', () => {
    const code = `var x = nls.localize("missing.key", "Fallback");`;
    const bundle: Record<string, string> = {};
    const result = nlsRewrite(code, bundle);
    expect(result).toBe(`var x = "Fallback";`);
  });

  it('handles multiple calls in same file', () => {
    const code = [
      `var a = nls.localize("k1", "D1");`,
      `var b = nls.localize("k2", "D2");`,
    ].join('\n');
    const bundle = { k1: 'T1', k2: 'T2' };
    const result = nlsRewrite(code, bundle);
    expect(result).toContain('"T1"');
    expect(result).toContain('"T2"');
    expect(result).not.toContain('nls.localize');
  });

  it('returns code unchanged when no nls.localize calls', () => {
    const code = `var x = 42;`;
    const result = nlsRewrite(code, {});
    expect(result).toBe(code);
  });
});

// ─── mangle-privates ─────────────────────────────────────────────────────────

import { manglePrivates } from '../build-plugins.js';

describe('manglePrivates', () => {
  it('renames _private fields to mangled names', () => {
    const code = `class Foo { _secret = 1; get() { return this._secret; } }`;
    const result = manglePrivates(code);
    expect(result).not.toContain('_secret');
    // mangled name should appear twice (declaration + usage)
    const mangledName = result.match(/\b(\$[a-z0-9]+)\b/)?.[1];
    if (mangledName) {
      const count = (result.match(new RegExp(mangledName.replace('$', '\\$'), 'g')) ?? []).length;
      expect(count).toBeGreaterThanOrEqual(2);
    }
  });

  it('returns code unchanged when no _private fields', () => {
    const code = `class Foo { public = 1; }`;
    const result = manglePrivates(code);
    expect(result).toBe(code);
  });
});

// ─── source-map rewrite ───────────────────────────────────────────────────────

import { rewriteSourceMapUrls } from '../build-plugins.js';

describe('rewriteSourceMapUrls', () => {
  it('rewrites JS sourceMappingURL comment to CDN base', () => {
    const code = `var x=1;\n//# sourceMappingURL=index.js.map`;
    const result = rewriteSourceMapUrls(code, 'https://cdn.example.com/assets', 'index.js');
    expect(result).toContain('//# sourceMappingURL=https://cdn.example.com/assets/index.js.map');
    expect(result).not.toContain('//# sourceMappingURL=index.js.map');
  });

  it('rewrites CSS sourceMappingURL comment to CDN base', () => {
    const css = `.foo{color:red}\n/*# sourceMappingURL=style.css.map*/`;
    const result = rewriteSourceMapUrls(css, 'https://cdn.example.com/assets', 'style.css');
    expect(result).toContain('/*# sourceMappingURL=https://cdn.example.com/assets/style.css.map*/');
  });

  it('returns code unchanged when no sourceMappingURL present', () => {
    const code = `var x=1;`;
    const result = rewriteSourceMapUrls(code, 'https://cdn.example.com', 'index.js');
    expect(result).toBe(code);
  });

  it('leaves code unchanged when cdnBase is empty string', () => {
    const code = `var x=1;\n//# sourceMappingURL=index.js.map`;
    const result = rewriteSourceMapUrls(code, '', 'index.js');
    expect(result).toBe(code);
  });
});

// ─── VITE_TARGET gating ───────────────────────────────────────────────────────

import { resolveTarget } from '../build-plugins.js';

describe('resolveTarget', () => {
  it('returns desktop when VITE_TARGET=desktop', () => {
    expect(resolveTarget('desktop')).toBe('desktop');
  });

  it('returns server when VITE_TARGET=server', () => {
    expect(resolveTarget('server')).toBe('server');
  });

  it('returns server-web when VITE_TARGET=server-web', () => {
    expect(resolveTarget('server-web')).toBe('server-web');
  });

  it('returns web when VITE_TARGET=web', () => {
    expect(resolveTarget('web')).toBe('web');
  });

  it('defaults to desktop when VITE_TARGET is undefined', () => {
    expect(resolveTarget(undefined)).toBe('desktop');
  });

  it('throws on unknown target string', () => {
    expect(() => resolveTarget('bogus')).toThrow(/Unknown VITE_TARGET/);
  });
});
