# Represent the two game modes

## Goal

Use a Rust enum so app state can be either Guess the Elo or Guess the Opening. Derive `PartialEq` and `Eq` so tests can compare the variants with `assert_eq!` and `assert_ne!`.

## Check

Choose the Rust type that represents a fixed set of named alternatives, then make its variants comparable. The terminal check asks which trait `assert_eq!` needs.

## Hint

Declare one variant per mode. The compiler error on `assert_eq!` points to the missing equality trait.

## NOTE

The expected implementation is a `GameMode` enum with readable labels. It also derives `PartialEq` so Rust can check equality in tests.

## TODO

Add `PartialEq` and `Eq` to the derive list, then implement the function in `exercises/src/rust_basics.rs` where the `TODO:` comment marks the insertion point.

## Reference

Read [reference.md: Rust types and mode selection](reference.md#rust-types-and-mode-selection).
