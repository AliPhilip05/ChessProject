# Getting Started

GTE has two entry points in one Rust binary: the interactive terminal lessons and the browser chess app. The first time you start the terminal interface, it runs Question 0, whose implementation is already filled in.

## Requirements

- A current Rust toolchain with Cargo.
- A terminal for the keyboard-driven curriculum.
- A browser for the game and Guidebook.
- Network access only for public Chess.com archive lookups.

## Start the terminal curriculum

From the repository root, run:

```bash
cargo run
```

The terminal shows the GTE logo and current question. It checks the first incomplete exercise on launch and watches the active file for changes. Question 0 starts complete in source, so its first run gives you a green result without editing anything.

The main keys follow the Rustlings-style loop:

- `R` runs the current exercise check. On success, the matching reference implementation loads automatically alongside the passing test output.
- `N` advances after the current exercise is complete; `P` returns to the previous exercise.
- `H` shows a hint. `L` opens the exercise list. Arrow keys move through that list, and Enter runs the selected check.
- `C` runs all exercise targets. `X` asks before restoring the current exercise from Git and saves a backup in `.gte-backup/`.
- `Q` exits. `?` shows the available shortcuts.

If you want the short command after installing the binary, run `cargo install --path . --bin gte` once and then type `gte` in this repository. `cargo run` is the direct development command and does not require a separate installation.

## Start the browser app

Run the local Rust server:

```bash
cargo run -- --web
```

Open `http://127.0.0.1:7878/` for the app or `http://127.0.0.1:7878/guide` for this book. Select **Download Markdown** in the Guidebook header or open `/guide/download` to download the chapters and full codebase walkthrough. The server listens on localhost and handles one connection at a time.

Use the included demo game to explore either mode without a network call. The public archive forms use Chess.com's read-only Published-Data API.

## Run a focused exercise

Each integration-test target imports its associated source file directly. For example:

```bash
cargo test --manifest-path exercises/Cargo.toml --test intro_0
cargo test --manifest-path exercises/Cargo.toml --test rust_basics
```

Question 0 is already complete. Later stubs contain `todo!()` until you implement the requested behavior. A compiler error is useful feedback: for instance, the Rust-basics test compares two `GameMode` values, so the enum needs `PartialEq` before the test can compile.

Continue to [Project Architecture](/guide/architecture) to see why the app and exercises live in separate Cargo packages.
