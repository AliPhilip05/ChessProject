# Route a request to its owner

## Goal

Implement a complete route lookup that separates a URL path from its query and identifies the responsible GTE module.

## Check

Run the `codebase_hard` test. The cases cover game routes, the Guidebook, query strings, and an unknown path.

## Hint

Split once at `?`, then match the path. A Guidebook child path and a game route both belong to `src/web.rs`.

## NOTE

This is the hard version of the project orientation example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/codebase_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: web-server-and-rendered-pages](reference.md#web-server-and-rendered-pages) in the codebase walkthrough.
