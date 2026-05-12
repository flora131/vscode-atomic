(no external research applicable)

The single file `src/bootstrap-import.ts` implements a Node.js ESM loader hook (`initialize` and `resolve`) using only built-in Node.js module system APIs — specifically the `node:module` customization hooks protocol — with no third-party library dependencies. All constructs used (URL manipulation via the `URL` global, JSON parsing of `package.json`, and string-based specifier mapping) are part of the Node.js standard runtime. There is no external library or framework documentation that is central to understanding or porting this file.
