# Terminal Lessons

The terminal interface borrows the focused exercise loop used by Rustlings: select one topic, read a small task, edit a Rust stub, run a check, and inspect a solution when useful.

## Keys and screens

On the lesson list, use Up and Down to select a topic, Enter to open it, `G` to print the Guidebook address, `?` for shortcuts, and `Q` to quit. Lesson status is shown as not started, in progress, or complete.

Inside a lesson:

- `H` toggles the hint.
- `C` opens a short project-understanding question.
- `T` runs the matching integration-test target with Cargo. When it passes, the lesson completes and its solution opens automatically; compiler or test errors remain visible when it fails.
- `S` opens the reference solution in the terminal.
- `Esc` returns to the lesson list.

The exercise test only marks a lesson complete when Cargo reports success. The concept check helps recall the design; the Rust test checks the behavior of the code you wrote.

## Exercise files

Each `exercises/src/*.rs` stub has a `NOTE:` comment describing the expected result and a `TODO:` comment pointing to the insertion point. Exercise markdown gives the goal, check, hint, NOTE, TODO, and a link to the matching section of [reference.md](/guide/reference). Inside a lesson, `G` prints a deep link to that lesson's reference section.

The exercises are in their own crate. Running a focused test from the repository root looks like:

```bash
cargo test --manifest-path exercises/Cargo.toml --test opening_answer
```

The test target imports only that practice file. It does not contact the network.
