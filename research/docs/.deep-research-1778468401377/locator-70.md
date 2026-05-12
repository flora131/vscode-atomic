# Partition 70: Shared Type Aliases in extensions/types/

## Types / Configuration

### Type Declaration Files

- `/home/norinlavaee/projects/vscode-atomic/extensions/types/lib.url.d.ts` — Global URL type declaration
  - Declares `const URL: typeof import('url').URL;`
  - Provides URL constructor for browser and Node.js runtime compatibility
  
- `/home/norinlavaee/projects/vscode-atomic/extensions/types/lib.textEncoder.d.ts` — Global TextEncoder/TextDecoder type declarations
  - Declares `TextDecoder: typeof import('util').TextDecoder;`
  - Declares `TextEncoder: typeof import('util').TextEncoder;`
  - Provides text encoding/decoding globals for browser and Node.js runtime compatibility

## Summary

The `extensions/types/` directory contains two ambient type declaration files that establish shared global type definitions for cross-runtime compatibility (browser and Node.js). These are not `export type` aliases but rather global constant/variable declarations that provide TypeScript definitions for runtime APIs. Both files include copyright headers referencing Microsoft and use patterns drawn from DefinitelyTyped to bridge runtime API availability across execution environments.
