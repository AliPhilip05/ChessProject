# Chess.com Public Data

The app uses Chess.com's public, read-only Published-Data API. It does not sign into an account or submit moves.

## Archive request flow

1. The user enters a Chess.com username.
2. `api.rs` requests `/pub/player/{username}/games/archives`.
3. The response lists monthly archive URLs. The app requests one archive URL at a time.
4. Game records are filtered to standard chess and converted from PGN and metadata.
5. A candidate is selected for a round. If the request fails, the app explains the error and loads its demo game.

The API exposes archives by player, not a global random-game feed. “Random game” therefore means a game sampled from a selected player's public archives.

## Pasted game links

A public game URL is resolved by scanning recent monthly archives for that selected username and comparing the game URL or its final path segment. Enter a username associated with the game. If the game is not in the scanned archives, the page explains that and offers the demo path.

## Data and failure handling

The game record may provide a PGN, usernames, ratings, opening metadata, result, time control, and a URL. Missing optional fields remain unknown. The interface does not invent ratings. Invalid usernames, unsupported links, missing archives, HTTP errors, and unreadable JSON become readable messages.

The serial request flow is deliberate: it avoids launching a burst of archive requests when one result will do. For the official endpoint details, read the [Chess.com Published-Data API documentation](https://www.chess.com/news/view/published-data-api).
