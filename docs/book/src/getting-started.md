# Getting Started

The repository contains one main Cargo package and a second, dependency-light package for learner exercises.

## Requirements

- A current Rust toolchain with Cargo.
- A terminal for the learning UI.
- A browser for the web app and book.
- Internet access only when requesting Chess.com public archives.

## Run the terminal lessons

From the repository root, run:

```bash
cargo run
```

Use the arrow keys to select a lesson and press Enter. Inside a lesson, `H` shows a hint, `C` opens a concept check, `T` runs that lesson's focused Rust exercise tests, `S` shows the solution, and `G` prints the local Guidebook URL. Edit the file shown at the top of the lesson before running its tests.

## Run the browser app

```bash
cargo run -- --web
```

Open [http://127.0.0.1:7878](http://127.0.0.1:7878) for the app and [http://127.0.0.1:7878/guide](http://127.0.0.1:7878/guide) for this book. The built-in demo game works without a network request. Public archive modes contact Chess.com's Published-Data API.

## Run a focused exercise

The exercise package is separate from the browser app. A focused test can be run from the repository root:

```bash
cargo test --manifest-path exercises/Cargo.toml --test rust_basics
```

The learner files begin with `TODO!()` placeholders, so the tests fail until you implement the requested behavior. Copying the corresponding file from `solutions/` is a way to inspect a completed version.

Continue to [Project Architecture](/guide/architecture) to see how the two Cargo packages fit together.
