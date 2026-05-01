(no external research applicable)

<one-sentence justification: LESS grammar extension is host-agnostic JSON with no runtime dependencies central to the port>

The `extensions/less/` directory contains only a TextMate grammar definition expressed as static JSON, along with standard VS Code extension manifest metadata. Because there is no JavaScript or TypeScript runtime code, no npm dependencies, and no API surface that interacts with the host environment, the extension operates identically regardless of which execution context or platform it is deployed to. There are no external libraries to audit, no runtime behaviors to port, and no compatibility concerns to research — the grammar simply declares syntax-highlighting rules that the editor engine applies universally.
