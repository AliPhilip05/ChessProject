# Capstone: build a compact guessing game from scratch

## Goal

Create a small implementation from an empty Rust module. Use the included demo record before wiring the live archive API.

## Sample record

```text
white=QuietKnight
white_rating=1180
black=TacticalTea
black_rating=1432
opening=Italian Game
moves=e4 e5 Nf3 Nc6 Bc4 Bc5
```

## Required behavior

1. Represent the game record and the two game modes with Rust types.
2. Parse the sample move list and store a position for each ply.
3. Clamp previous/next navigation to the first and last positions.
4. Score White, Black, or both with Easy ±200, Medium ±100, and Hard ±50.
5. Normalize opening guesses without case sensitivity.
6. Render a useful result with the actual rating/opening only after submission.
7. Add a public archive loader as the next step, making serial API requests and showing a readable error when loading fails.

## Completion check

The learner can demonstrate move navigation, both guessing modes, boundary tolerance behavior, selected-player scoring, and a clear result state. The reference implementation is in `solutions/capstone.rs`.

## NOTE

The expected implementation connects game data, move navigation, selected-player scoring, and case-insensitive opening feedback into one compact flow.

## TODO

Build the model and functions in `exercises/src/capstone.rs` from the marked `TODO:` comments, then run the capstone exercise target.

## Reference

Read [reference.md: Capstone and end-to-end flow](reference.md#capstone-and-end-to-end-flow).
