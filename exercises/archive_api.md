# Load a public archive safely

## Goal

Describe the Chess.com PubAPI flow from username to a randomly selected archived game.

## Check

Identify the endpoint that returns a player's available monthly archive URLs. Then explain why requests are made one at a time and how the UI should respond to an API error.

## Hint

The archive index is under `/pub/player/{username}/games/archives`.

## NOTE

The expected implementation selects recent archive URLs in newest-first order and validates a username before requesting data.

## TODO

Insert both implementations in `exercises/src/archive_api.rs`, at the functions marked with `TODO:` comments.

## Reference

Read [reference.md: Chess.com archive data](reference.md#chesscom-archive-data).
