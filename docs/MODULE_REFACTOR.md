# GTE module refactor

## Goal

Give every Rust file a clear responsibility while preserving the browser app, terminal controls, 24 exercise targets, and generated function reference.

## Boundaries

- `main.rs` chooses the browser or terminal entry point.
- `api/` separates public archive selection, HTTP requests, JSON conversion, and URL helpers.
- `chess/` separates the game model, PGN parsing, board setup, SAN moves, and answer scoring.
- `cli/` separates terminal lifecycle, state, input/navigation, exercise checks, progress/reset, and screen rendering.
- `lessons/` separates lesson types, course ordering, and the three exercise records for each topic.
- `web/` separates TCP/HTTP responses, routes, request decoding, game sessions, handlers, game rendering, and the book reader.
- `assets/templates/` contains large HTML templates. `assets/styles/` contains browser stylesheets.

Each top-level module keeps a small facade. Other modules use its explicit public exports; implementation helpers use the narrowest useful visibility.

## Compatibility

The `gte` terminal entry point, `--web`, existing URLs/forms, session behavior, score rules, board snapshots, exercise IDs/order, and solution loading must behave the same after the move. Learner exercise source and saved progress are not part of this restructuring.

## Documentation and verification

Update the book and function appendix to use the new source paths. Discover Rust modules recursively so future files appear in the function reference. Check compilation and focused production behavior: demo game parsing, scoring, HTML escaping, Markdown safety, routes/session transitions, and raw-terminal line endings. Exercise source remains independent of production module layout.

## Completed layout

The largest Rust source file is 153 lines, including its local checks. The browser facade is 11 lines and the terminal entry module is 67 lines. Separate files own HTTP transport, URL routing, request decoding, session storage, game setup, move navigation, answer handling, each terminal screen, and each curriculum topic.

The regenerated book reference contains 111 production functions across 47 modules. Its downloadable appendix has 136 printed pages and 111 bookmarks. The generator excludes test modules and the PDF builder rejects stale function inventories or implementation excerpts.

Compilation succeeds with `cargo check --offline`. A review against the original browser module confirmed that moved production logic and embedded HTML/CSS preserve their behavior.
