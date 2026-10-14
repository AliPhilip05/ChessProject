# Render a page from Rust

## Goal

Describe a Rust route that renders a selected board position, move controls, guess form, and Guidebook link.

## Check

Identify which application component owns the page output, and name the Guidebook route.

## Hint

The same Rust server handles `/`, `/play`, and `/guide`.

## NOTE

The expected implementation builds a move-navigation URL and safely escapes user text before rendering HTML.

## TODO

Insert both helpers in `exercises/src/server_rendering.rs`, at the functions marked with `TODO:` comments.

## Reference

Read [reference.md: Web server and rendered pages](reference.md#web-server-and-rendered-pages).
