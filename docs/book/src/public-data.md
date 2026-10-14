# Chess.com Public Data

`src/api.rs` is the network boundary between GTE and Chess.com. It reads public game archives through Chess.com's Published-Data API. It does not sign in, change profiles, or submit moves.

## Why archives come before games

The API exposes monthly archives for one player. It does not expose a global “random game” endpoint. GTE therefore asks for a username, fetches that player's archive index, and samples a game from one of the returned months.

The request flow in `random_public_game` is:

1. `validate_username` trims and lowercases the name and allows 1–25 ASCII letters, digits, or underscores.
2. `archive_urls` requests `/pub/player/{username}/games/archives`.
3. The code looks through up to four newest archive URLs, newest first.
4. `get_json` requests one archive at a time. Each returned game is passed through `to_chess_game`, which ignores nonstandard variants and records without PGN.
5. The first archive with usable candidates ends the search. A time-derived index selects one candidate from that set.
6. The function returns `Ok(ChessGame)` or a readable `Err(String)`.

The candidate index is a lightweight prototype choice, not cryptographic randomness. Choosing among one recent month also favors recent games, which makes loading quick.

## Resolve a pasted game URL

`game_from_archive_link` first validates the username and checks that the supplied URL begins with the public Chess.com game URL prefix. It then scans up to twelve returned monthly archives. `same_game_id` compares the final path segment after removing a query string, so equivalent URL forms can identify the same archive record.

A link must appear in the selected player's archive. If it does not, GTE returns an explanation and the web layer offers its built-in demo game so the learner can continue.

## Decode the JSON record

`get_json` sets a descriptive `User-Agent`, performs the GET request with `ureq`, and decodes a `serde_json::Value`. It turns request and JSON failures into error messages rather than panicking.

`to_chess_game` uses `Value` accessors such as `get`, `as_str`, and `as_i64` to read optional fields. It gets the PGN, ratings, names, opening/ECO, URL, time control, and variant. Missing ratings remain `None`; they are not silently converted to zero. The parser in `ChessGame::with_metadata` can use metadata from either the JSON fields or PGN headers.

If `opening` is absent, the API converter may turn an ECO URL such as `.../C50-Italian-Game` into an approximate readable label with `opening_from_eco`. If the API loader fails, `web::load_game` keeps the message and uses `ChessGame::demo()`.

## Error path examples

```rust
match api::random_public_game("erik") {
    Ok(game) => println!("Loaded {} plies", game.ply_count()),
    Err(message) => eprintln!("Could not load a public game: {message}"),
}
```

The production web route uses the same result but renders a notice and a demo fallback. Errors include invalid usernames, absent archive lists, empty public archives, HTTP failures, unreadable JSON, and no usable chess games.

## Privacy and limits

The API client sends only the requested public username and URL to Chess.com's public endpoint. No account credentials are needed. The implementation is serial to keep the request flow small and avoid launching an unnecessary burst of requests.

The archive endpoint can be stale or unavailable; the prototype has no cache. It only accepts standard chess records with PGN. Recent-archive link lookup can miss older games. The exercise `archive_api` tests the list slicing and username rules with local strings, while the full app uses the network functions in `api.rs`.

For the rest of the record-to-board path, continue to [PGN and Board State](/guide/board-state). For exact function responsibilities, see [the API section of reference.md](/guide/reference#chesscom-archive-data).
