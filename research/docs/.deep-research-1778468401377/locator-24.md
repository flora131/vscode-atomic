# File Location Map: `extensions/github-authentication/`

## Implementation

### Core Extension Logic
- `extensions/github-authentication/src/extension.ts` — Extension activation and provider registration
- `extensions/github-authentication/src/config.ts` — Configuration management for GitHub and GitHub Enterprise

### Authentication Flows
- `extensions/github-authentication/src/flows.ts` — OAuth flow implementations (authorization code, device flow)
- `extensions/github-authentication/src/githubServer.ts` — GitHub API server interactions and token management
- `extensions/github-authentication/src/github.ts` — GitHub-specific API client and session handling

### Platform-Specific Implementation

#### Node.js Runtime
- `extensions/github-authentication/src/node/authServer.ts` — Local HTTP server for OAuth callback (Node.js)
- `extensions/github-authentication/src/node/crypto.ts` — Cryptographic utilities (Node.js)
- `extensions/github-authentication/src/node/buffer.ts` — Buffer utilities (Node.js)
- `extensions/github-authentication/src/node/fetch.ts` — Fetch implementation wrapper (Node.js)

#### Browser Runtime
- `extensions/github-authentication/src/browser/authServer.ts` — Auth server for browser context
- `extensions/github-authentication/src/browser/crypto.ts` — Cryptographic utilities (browser)
- `extensions/github-authentication/src/browser/buffer.ts` — Buffer utilities (browser)
- `extensions/github-authentication/src/browser/fetch.ts` — Fetch implementation wrapper (browser)

### Common/Shared Logic
- `extensions/github-authentication/src/common/env.ts` — Environment detection and configuration
- `extensions/github-authentication/src/common/errors.ts` — Custom error classes and definitions
- `extensions/github-authentication/src/common/keychain.ts` — Secure credential storage abstraction
- `extensions/github-authentication/src/common/logger.ts` — Logging utilities
- `extensions/github-authentication/src/common/utils.ts` — General utility functions
- `extensions/github-authentication/src/common/experimentationService.ts` — A/B testing integration

## Tests

- `extensions/github-authentication/src/test/flows.test.ts` — OAuth flow unit tests
- `extensions/github-authentication/src/test/node/authServer.test.ts` — Auth server tests (Node.js)
- `extensions/github-authentication/src/test/node/fetch.test.ts` — Fetch wrapper tests (Node.js)

## Configuration

### Build Configuration
- `extensions/github-authentication/esbuild.mts` — Build configuration (Node.js bundle)
- `extensions/github-authentication/esbuild.browser.mts` — Build configuration (browser bundle)
- `extensions/github-authentication/tsconfig.json` — TypeScript configuration
- `extensions/github-authentication/tsconfig.browser.json` — TypeScript configuration (browser)

### Package Management
- `extensions/github-authentication/package.json` — Extension manifest and dependencies
- `extensions/github-authentication/package-lock.json` — Locked dependency versions
- `extensions/github-authentication/package.nls.json` — Localization strings
- `extensions/github-authentication/.npmrc` — NPM configuration
- `extensions/github-authentication/.gitignore` — Git ignore rules
- `extensions/github-authentication/.vscodeignore` — VS Code packaging ignore rules

## Examples / Fixtures

### Web Assets
- `extensions/github-authentication/media/index.html` — Auth flow UI (browser)
- `extensions/github-authentication/media/auth.css` — Styles for auth UI
- `extensions/github-authentication/media/code-icon.svg` — VS Code logo SVG
- `extensions/github-authentication/media/sessions-icon.svg` — Sessions icon
- `extensions/github-authentication/media/favicon.ico` — Favicon for auth page
- `extensions/github-authentication/media/icon.png` — Icon asset

### Extension Icons
- `extensions/github-authentication/images/icon.png` — Extension icon

## Documentation

- `extensions/github-authentication/README.md` — Extension overview and features

## Notable Clusters

**Platform Abstraction Pattern**: The extension implements a clean platform abstraction with `src/node/` and `src/browser/` directories, each containing parallel implementations of:
- `authServer.ts` — OAuth callback handling (local HTTP vs. browser-based)
- `crypto.ts` — PKCE and token encryption (Node.js native vs. WebCrypto API)
- `buffer.ts` — Binary data handling
- `fetch.ts` — HTTP client wrapper

**OAuth Flow Support**: Multiple authentication flows are supported:
- Standard OAuth 2.0 authorization code flow
- Device flow (for environments without local server)
- GitHub Enterprise Server configuration

**Dual-Build Architecture**: The extension builds to two targets:
- `out/extension.js` — Node.js/Electron runtime
- `dist/browser/extension.js` — Web/browser runtime

---

## Summary

The GitHub Authentication extension is a foundational auth provider for VS Code, implementing complete OAuth 2.0 flows with dual runtime support (Node.js and browser). It contains **~1,140 TypeScript lines** across 24 files, with clean platform abstraction patterns allowing the same authentication logic to work in both Electron (Node.js) and web contexts. The extension handles credential storage, session management, token refresh, and GitHub Enterprise integration. Authentication flows are heavily tested, and the codebase demonstrates sophisticated patterns for managing cross-platform compatibility, secure credential handling, and OAuth security practices (PKCE, state validation).
