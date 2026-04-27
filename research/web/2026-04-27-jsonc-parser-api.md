---
source_url: https://raw.githubusercontent.com/microsoft/node-jsonc-parser/main/README.md
fetched_at: 2026-04-27
fetch_method: html-parse
topic: jsonc-parser v3.2.0 API reference
---

# jsonc-parser — Scanner and parser for JSON with comments

Repository: https://github.com/microsoft/node-jsonc-parser

## Key exports

- `createScanner(text, ignoreTrivia)` — tokenizes JSON+comments into `SyntaxKind` tokens
- `parse(text, errors?, options?)` — fault-tolerant parse, returns plain JS value
- `visit(text, visitor, options?)` — SAX-style visitor (onObjectBegin, onObjectProperty, onLiteralValue, onComment, etc.)
- `parseTree(text, errors?, options?)` — builds offset-annotated DOM (`Node` tree)
- `getLocation(text, offset)` — returns `Location` with `path: JSONPath`, `isAtPropertyKey`, `matches(pattern)`
- `findNodeAtLocation(root, path)` / `findNodeAtOffset(root, offset)`
- `getNodePath(node)` / `getNodeValue(node)`
- `format(text, range, options)` → `EditResult` (array of `Edit`)
- `modify(text, path, value, options)` → `EditResult`
- `applyEdits(text, edits)` → string
- `stripComments(text, replaceCh?)` — remove JS-style comments from JSONC

## ParseOptions
- `disallowComments?: boolean`
- `allowTrailingComma?: boolean`
- `allowEmptyContent?: boolean`

## Location interface
```ts
interface Location {
  previousNode?: Node;
  path: JSONPath;           // (string|number)[]
  matches(patterns: JSONPath): boolean;
  isAtPropertyKey: boolean;
}
```

## Node interface
```ts
interface Node {
  type: NodeType;  // "object"|"array"|"property"|"string"|"number"|"boolean"|"null"
  value?: any;
  offset: number;
  length: number;
  colonOffset?: number;
  parent?: Node;
  children?: Node[];
}
```

## Note
Published as ESM-only package (v3.x).
