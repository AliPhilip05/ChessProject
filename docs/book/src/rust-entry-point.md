# Rust Entry Points and Types

This chapter follows execution from Cargo into Rust modules and explains the types used throughout GTE. It assumes you've run [Question 0](/guide/question-zero) and seen the two passing tests.

## Two Cargo packages

The root `Cargo.toml` defines the user-facing binary. Its explicit binary target is named `gte` and points at `src/main.rs`. `default-run = "gte"` makes `cargo run` select that target. `autobins = false` prevents Cargo from treating other files as implicit binaries.

The root package depends on:

- `crossterm` for terminal input, colors, cursor movement, raw mode, and screen switching;
- `ureq` for synchronous HTTP requests to the public archive API;
- `serde_json` for reading the API's JSON response values.

The second manifest, `exercises/Cargo.toml`, is a separate package named `chessproject-learning`. Keeping learner code in a separate crate means a `todo!()` in a practice file does not stop the main app from compiling. Each file under `exercises/tests/` is compiled as its own integration-test binary.

The root and exercise lockfiles record dependency resolution for their respective packages. Exercise tests use only the standard library and local fixture strings, so they do not need a network connection.

## The process entry point

Rust always starts the binary at `fn main()` in `src/main.rs`:

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--web") {
        web::serve();
    } else {
        cli::run();
    }
}
```

`std::env::args()` yields the executable name followed by the arguments. Collecting it into `Vec<String>` gives the program an owned list it can search. `any` returns as soon as an argument matches `--web`. The function then chooses one long-running interface.

The `mod` declarations at the top of `main.rs` compile `api`, `chess`, `cli`, `lessons`, and `web` as sibling modules in this binary crate. `use` statements inside a module bring selected names into scope; they do not create another executable or another copy of the data.

## Model known choices with enums

`web.rs` uses a `GameMode` enum with two variants: `Elo` and `Opening`. That prevents the session from needing to treat arbitrary strings such as `"openng"` as a valid mode.

```rust
#[derive(Clone, Copy, PartialEq, Eq)]
enum GameMode {
    Elo,
    Opening,
}
```

The derives provide standard behavior. `Copy` lets this small, data-free enum be passed by value without moving it. `PartialEq` supports `==` and `!=`. `Eq` marks equality as reflexive for all its values. `Clone` provides an explicit copy operation where needed. `Debug` is useful for diagnostic formatting.

The Rust-basics exercise uses `GameMode::GuessElo` and `GameMode::GuessOpening` to practice the same idea. Its tests use both `assert_eq!` and `assert_ne!`; those macros require `PartialEq`. Question 0's `Component` derives equality too, but it begins already complete.

## `String`, `&str`, `Option`, and `Result`

GTE uses owned `String` when a value is built from a request or parsed from a game: a username, SAN move, opening name, or player name. It uses `&str` for borrowed input and fixed labels. A `String` owns a heap buffer; an `&str` is a view into text owned elsewhere.

Optional game metadata uses `Option<T>`. For example, `white_rating: Option<i32>` can be `Some(1180)` or `None`. A missing rating is not the same thing as a rating of zero. The UI must decide how to explain the absence rather than invent a value.

Operations that can fail use `Result<T, E>`. `api::random_public_game` returns `Result<ChessGame, String>`: success carries a game and failure carries a readable message. The `?` operator returns early from the current function when a `Result` is an error, keeping validation and HTTP code direct.

## Structs and ownership

`ChessGame` groups metadata, the ordered move list, and board snapshots into one owned value. A `Session` owns a game plus the settings and view state for one browser round. When a session is inserted into the global map, the map owns it until the process exits.

The renderer borrows a session (`&Session`) because it only reads it. A route that updates the current ply borrows the map mutably (`get_mut`) because it changes state. Rust makes these read and write paths visible in the function signatures.

## Reading code and running the next check

After Question 0, continue through the terminal lesson list. The exercise stubs isolate one behavior at a time; the production files show where that behavior fits into the app. The [full reference walkthrough](/guide/reference) names every source module, helper, test, and solution.
