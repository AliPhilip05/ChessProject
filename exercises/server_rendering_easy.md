# Build a move-navigation URL

## Goal

Complete a guided URL that carries both the session id and requested ply back to the Rust server.

## Check

Run `server_rendering_easy`. Exact-string assertions verify both query parameters and their order.

## Hint

Append the `ply` query parameter to the already-built session URL using `format!`.

## NOTE

This is the easy version of the web app and guidebook example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/server_rendering_easy.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: web-server-and-rendered-pages](reference.md#web-server-and-rendered-pages) in the codebase walkthrough.
