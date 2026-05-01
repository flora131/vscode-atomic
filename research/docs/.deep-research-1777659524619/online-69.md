(no external research applicable)

The `extensions/types/` directory contains only ambient `.d.ts` declaration files for globals such as `URL`, `TextEncoder`, and `TextDecoder`. Because ambient declarations are purely compile-time constructs that instruct the TypeScript compiler about the shapes of values already provided by the runtime environment, they carry zero runtime code and pull in no libraries or packages at build or execution time, making external dependency research entirely inapplicable to this scope.
