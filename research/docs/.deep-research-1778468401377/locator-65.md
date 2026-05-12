# Locator Partition 65: bootstrap-meta.ts

## Scope
`src/bootstrap-meta.ts` — Metadata constants for bootstrap (30 LOC)

## Implementation

**File**: `/home/norinlavaee/projects/vscode-atomic/src/bootstrap-meta.ts`

Single TypeScript module that handles product and package metadata initialization at bootstrap time. The file:

- Imports `IProductConfiguration` type from `./vs/base/common/product.js`
- Uses dynamic `require()` via `createRequire` to load configuration files during runtime
- Initializes two module-level objects:
  - `productObj`: Product configuration loaded from `../product.json` when running from sources; includes a build-time patching placeholder
  - `pkgObj`: Package metadata loaded from `../package.json`
- Supports development overrides via `product.overrides.json` when `VSCODE_DEV` environment variable is set
- Exports `product` and `pkg` objects for use by other modules

This file bridges the gap between build-time configuration insertion and runtime metadata availability, serving as a central point for accessing product version and package information during VS Code initialization.

