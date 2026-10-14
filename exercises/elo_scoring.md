# Score an Elo estimate

## Goal

Implement a tolerance lookup and absolute-error comparison for one rating guess.

## Check

For a 1500 rating at Medium difficulty, an estimate from 1400 through 1600 is accepted, including both endpoints.

## Hint

Easy = 200, Medium = 100, Hard = 50.

## NOTE

The expected implementation converts a difficulty into its tolerance and accepts guesses whose absolute error is within that value.

## TODO

Insert the tolerance and comparison logic in `exercises/src/elo_scoring.rs`, where the `TODO:` comments point to both functions.

## Reference

Read [reference.md: Elo scoring](reference.md#elo-scoring).
