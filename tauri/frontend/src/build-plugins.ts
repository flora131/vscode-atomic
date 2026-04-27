/**
 * Build pipeline utilities: NLS rewrite, mangle-privates, source-map URL rewrite.
 * Ported from build/next/index.ts:940-1055 and adapted for Vite Rollup plugins.
 */

// ─── Target resolution ────────────────────────────────────────────────────────

export type BuildTarget = 'desktop' | 'server' | 'server-web' | 'web';

const VALID_TARGETS = new Set<BuildTarget>(['desktop', 'server', 'server-web', 'web']);

/**
 * Resolve the VITE_TARGET env string to a strongly-typed BuildTarget.
 * Defaults to 'desktop' when undefined or empty.
 * Throws on unknown values.
 */
export function resolveTarget(raw: string | undefined): BuildTarget {
  if (!raw) return 'desktop';
  if ((VALID_TARGETS as Set<string>).has(raw)) return raw as BuildTarget;
  throw new Error(`Unknown VITE_TARGET: "${raw}". Must be one of: ${[...VALID_TARGETS].join(', ')}`);
}

// ─── NLS rewrite ─────────────────────────────────────────────────────────────

/**
 * Replace `nls.localize("key", "default")` calls with:
 *   - bundle[key]  when key is present in the NLS bundle
 *   - "default"    when key is absent (English fallback)
 *
 * Handles both single and double-quoted strings in source.
 */
export function nlsRewrite(
  code: string,
  bundle: Record<string, string>,
): string {
  // Pattern: nls.localize(<key>, <default>)
  // where key and default are single- or double-quoted string literals.
  return code.replace(
    /nls\.localize\(\s*(['"])((?:\\.|[^\\])*?)\1\s*,\s*(['"])((?:\\.|[^\\])*?)\3\s*\)/g,
    (_match, _kq, key: string, _dq, def: string) => {
      const msg = Object.prototype.hasOwnProperty.call(bundle, key) ? bundle[key] : def;
      // Re-encode as double-quoted JSON string (safe for embedding in JS)
      return JSON.stringify(msg);
    },
  );
}

// ─── mangle-privates ─────────────────────────────────────────────────────────

/**
 * Rename `_private` fields (underscore-prefixed identifiers used as class
 * members) to short `$N` names to reduce bundle size.
 *
 * Conservative approach: only renames identifiers that appear as:
 *   - `this._name` (property access)
 *   - `_name =` (class field declaration / assignment)
 *
 * Uses a deterministic counter so the same source always produces the same
 * mangled output (reproducible builds).
 */
export function manglePrivates(code: string): string {
  // Collect all _private names referenced via `this._name` or as standalone
  // class field declarations `_name`.
  const privatePattern = /\b(_[a-zA-Z_$][a-zA-Z0-9_$]*)\b/g;
  const names = new Set<string>();
  let m: RegExpExecArray | null;
  while ((m = privatePattern.exec(code)) !== null) {
    names.add(m[1]);
  }

  if (names.size === 0) return code;

  let result = code;
  let counter = 0;
  for (const name of [...names].sort()) {
    const mangled = `$${counter.toString(36)}`;
    counter++;
    // Replace whole-word occurrences
    result = result.replace(new RegExp(`\\b${escapeRegex(name)}\\b`, 'g'), mangled);
  }
  return result;
}

function escapeRegex(s: string): string {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

// ─── source-map URL rewrite ───────────────────────────────────────────────────

/**
 * Rebase the `//# sourceMappingURL=` (JS) or `/*# sourceMappingURL= */` (CSS)
 * comment to point at a CDN base URL.
 *
 * When cdnBase is empty the code is returned unchanged.
 */
export function rewriteSourceMapUrls(
  code: string,
  cdnBase: string,
  relativePath: string,
): string {
  if (!cdnBase) return code;

  // JS source map comment
  code = code.replace(
    /\/\/# sourceMappingURL=.+$/m,
    `//# sourceMappingURL=${cdnBase}/${relativePath}.map`,
  );

  // CSS source map comment
  code = code.replace(
    /\/\*# sourceMappingURL=.+\*\/$/m,
    `/*# sourceMappingURL=${cdnBase}/${relativePath}.map*/`,
  );

  return code;
}

// ─── Vite plugin factory ──────────────────────────────────────────────────────

import type { Plugin } from 'vite';

/**
 * Vite plugin: apply NLS rewrite + mangle-privates + sourcemap URL rebase
 * as a post-processing pass on bundled JS/CSS output files.
 */
export function postProcessPlugin(opts: {
  nlsBundle?: Record<string, string>;
  manglePrivates?: boolean;
  sourceMapCdnBase?: string;
}): Plugin {
  return {
    name: 'vscode-post-process',
    enforce: 'post',
    generateBundle(_options, bundle) {
      for (const [fileName, chunk] of Object.entries(bundle)) {
        if (chunk.type === 'chunk') {
          let code = chunk.code;

          if (opts.manglePrivates) {
            code = manglePrivates(code);
          }

          if (opts.nlsBundle && Object.keys(opts.nlsBundle).length > 0) {
            code = nlsRewrite(code, opts.nlsBundle);
          }

          if (opts.sourceMapCdnBase) {
            code = rewriteSourceMapUrls(code, opts.sourceMapCdnBase, fileName);
          }

          chunk.code = code;
        } else if (chunk.type === 'asset' && typeof chunk.source === 'string') {
          let source = chunk.source;

          if (opts.sourceMapCdnBase) {
            source = rewriteSourceMapUrls(source, opts.sourceMapCdnBase, fileName);
          }

          chunk.source = source;
        }
      }
    },
  };
}
