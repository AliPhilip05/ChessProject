# Terminal Lessons

The terminal path is an interactive workbook about the codebase. It borrows Rustlings' focused exercise loop: open one problem, inspect its NOTE and TODO, run the small check, read the compiler or test result, and compare with a reference implementation when the check passes.

## The first launch is Question 0

The screen starts with the GTE logo and Question 0, `Find your way around GTE`. This starter is already implemented. The CLI runs its `intro_0` integration-test target and shows the passing test names before displaying the matching reference code. Learners can begin by reading the result, not by debugging a failing first compile.

The current exercise is selected from the ordered list in `src/lessons.rs`. Progress is saved in `.gte-state.txt`; a new `intro-0` lesson is automatically treated as incomplete for an existing learner, so it becomes the first exercise on the next launch. `advance_next` moves to an incomplete exercise only after the focused check passes.

## Keys and state transitions

- `R` runs the active exercise test. The CLI runs it on startup too, and polls the active source file's modification time so an external save reruns the check.
- `N` advances after success; `P` runs the previous exercise again.
- `H` toggles the lesson hint. `L` opens the list; use `N`/`P` or arrow keys and press Enter to check the selected item.
- `C` runs every exercise test target and summarizes the results.
- `X` opens a reset confirmation. If you confirm, the active exercise is restored from Git HEAD after its current source is copied into `.gte-backup/`.
- `?` displays the current screen's shortcuts and `Q` exits.

The UI has home/watch, exercise list, lesson, concept check, solution, test result, and reset confirmation screens. Passing a check updates progress and loads the corresponding `solutions/*.rs` file. The solution screen also shows the test names and final `test result:` line so you can see what just passed.

## How a focused check works

The exercise crate is separate from the app. A file such as `exercises/tests/elo_scoring.rs` uses:

```rust
#[path = "../src/elo_scoring.rs"]
mod exercise;
```

That compiles the learner file directly into an integration-test binary. `run_lesson_test` invokes `cargo test --manifest-path exercises/Cargo.toml --test elo_scoring`. It captures the child's standard output and diagnostics, uses the process exit status as the pass/fail result, and leaves the main app independent from unfinished learner code.

The separate `solutions/` folder is not compiled in place of the learner file. On a pass, the CLI reads the matching solution text for display. This means the exercise remains the code under test while a passing solution becomes available for comparison.

The mapping is data-driven. A `Lesson` record in `src/lessons.rs` connects an id to its tier, prompt, test-derived check, solution path, and book anchor. For example, `archive-api` is transformed to `archive_api` for both `exercises/src/archive_api.rs` and `cargo test --test archive_api`; its solution is `solutions/archive_api.rs`. When adding an exercise, keep all names consistent and add one integration-test target. `LESSONS` order determines the progress bar and normal next/previous order.

The runner captures both streams from the Cargo child process. A zero exit status means the exercise passed. Cargo's standard test lines are kept so the solution screen can show the names of the tests and the `test result:` summary. A nonzero status returns to the exercise screen with compiler errors or failed assertions intact. This means `PartialEq` diagnostics from Rust itself remain visible to the learner instead of being replaced by a custom message.

The source watcher compares the active file's modification time during the UI's short event-poll interval. Saving an edit reruns the same focused check without leaving the TUI. The watcher runs only on the home/watch or solution screen, avoiding a surprise transition while the learner is choosing an answer or navigating the lesson list.

## NOTE and TODO are part of each problem

Every problem source function begins with a `NOTE:` comment about the expected behavior and a `TODO:` marker where the requested implementation belongs. For example:

```rust
// NOTE: A guess succeeds when the absolute error is within the chosen limit.
// TODO: Compare guess and rating using the difficulty tolerance.
pub fn within_tolerance(guess: i32, actual: i32, difficulty: &str) -> bool {
    todo!("check the absolute error")
}
```

Question 0 is the special introduction: its TODO says to run and read the test because the implementation is prefilled. The exercise markdown contains the task, test, hint, NOTE/TODO explanation, and a `reference.md` link to the related production code.

The exercises are small stand-ins for production rules, not imports of the production modules. That separation lets each test focus on a single contract. For example, the learner's `rating_tolerance` returns a threshold from a difficulty string; production `chess::rating_tolerance` uses the same rule from the web scoring handler. The learner should use the exercise test to discover the contract, then use the `reference.md` anchor to see where the production code applies it.

The course has 24 questions arranged as eight topics with Easy, Medium, and Hard tiers. Question 0 is the already-solved Easy introduction, so it starts with a passing check. Project orientation then adds a guided mode-to-module exercise and a full route lookup. The remaining seven topics each have a guided Easy exercise, a partially scaffolded Medium exercise, and a Hard exercise that asks for a complete function. Those topics are Rust types, Chess.com archives, PGN and board state, Elo scoring, opening answers, server rendering, and the capstone.

Easy problems give step-by-step context and a small insertion TODO. Medium problems provide part of the implementation and ask for a specific behavior. Hard problems describe the contract and leave the whole function body to implement. Every source has a `NOTE:` and `TODO:` comment, and every exercise guide ends with its related `reference.md` link. The capstone Hard task combines move bounds, answer normalization, and result reveal state.

## Compiler feedback and equality traits

The Rust-basics exercise compares two `GameMode` variants with `assert_eq!` and `assert_ne!`. The macros require `PartialEq`. If the derive is missing, Rust points at the comparison and suggests the trait. Add `PartialEq` to the enum's derive, rerun the target, and the check advances.

The exercise also makes the function return a label for each variant. The matching solution is automatically shown after the focused tests pass. Later exercises use `Option`, slices, iterators, normalization, formatting, and selected-side logic in the same way.

## Use the book next to the terminal

Press `G` to get the current exercise's Guidebook reference address. Every prompt's anchor points to `exercises/reference.md`, which is embedded as the final Guidebook chapter. The book describes the full production modules; the exercise target is a deliberately small practice version of one behavior.

Read the [exercise-to-code map](/guide/reference#practice-files-tests-and-solutions) for all targets, or continue to the [capstone](/guide/capstone).
