# Render an escaped notice

## Goal

Write a complete notice renderer and escape untrusted title and message text before placing it in HTML.

## Check

Run `server_rendering_hard`. It checks all five escaped HTML characters and confirms trusted wrapper markup remains intact.

## Hint

Write an escape loop over characters; map `&`, `<`, `>`, double quote, and apostrophe, then format the wrapper.

## NOTE

This is the hard version of the web app and guidebook example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/server_rendering_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: web-server-and-rendered-pages](reference.md#web-server-and-rendered-pages) in the codebase walkthrough.
