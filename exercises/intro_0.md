# Question 0: Find your way around GTE

This starter is already implemented. Run it and read the passing test output; you do not need to edit any code.

## Check

Run `cargo test --manifest-path exercises/Cargo.toml --test intro_0`. The tests confirm the GTE mark, the two process entry points, and the module map.

## What to notice

- `src/main.rs` sends the normal terminal launch to `cli::run()` and `--web` to `web::serve()`.
- `src/api.rs` retrieves public archive games; `src/chess.rs` interprets their PGN and answers.
- `src/web.rs` builds the browser game and Guidebook.
- `src/lessons.rs` describes this curriculum; `src/cli.rs` drives it.
- `PartialEq` lets `assert_eq!` compare the `Component` values in the tests.

## Hint

In GTE, `R` reruns the current check. The test names are a quick tour of what the starter already knows.

## NOTE

The code is intentionally filled in for the first question so you can begin with a passing run and learn the test loop before changing anything.

## TODO

Run the exercise, read the output, and open the listed files. There is no code insertion for Question 0.

## Reference

Read [Question 0: project map](reference.md#question-zero-project-map) in the walkthrough.
