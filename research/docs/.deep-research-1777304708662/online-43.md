# Online Research: extensions/tunnel-forwarding/

(no external research applicable)

The `extensions/tunnel-forwarding/` partition (4 files, 474 LOC) implements a TunnelProvider by spawning the native `code-tunnel` CLI binary and communicating over JSON stdin/stdout IPC using Node's built-in `child_process` module and VS Code's internal `tunnelFactory`/resolver proposed APIs. There are no significant external libraries beyond Node stdlib and VS Code's own extension host APIs, so no external documentation is central to understanding or porting this code.
