# The Web Server and Rendered Pages

`src/web.rs` exposes the local HTTP interface, with focused child modules for transport, routing, sessions, handlers, rendering, and the book. The prototype uses Rust-generated HTML and CSS with ordinary links and forms; it has no JavaScript application bundle or database.

## Accepting a request

`web::serve` binds a `TcpListener` to `127.0.0.1:7878` and accepts connections one at a time. `handle_connection` reads the first part of one request, extracts its method and target, and accepts `GET`. Other methods receive a 405 response. It dispatches a route to a page, redirect, download, or 404 response, then closes the connection.

The route function splits the target at `?`. `parse_query` turns the query string into a `HashMap<String, String>`, decoding `+` as space and `%HH` as a byte. The route table is:

- `/` — render the overview and challenge forms;
- `/load` — load a demo, archive game, or game link and create a session;
- `/play?sid=…` — render the current game state;
- `/move?sid=…&ply=…` — change the visible position;
- `/guess-elo` and `/guess-opening` — accept an answer and return to the session;
- `/guide` and `/guide/{chapter}` — render the Guidebook;
- `/guide/download` — return all chapter Markdown as a download;
- `/guide/functions.pdf` — download the function appendix;
- `/new` — show the setup page for another round.

Unknown routes return 404. A missing or expired session id returns the home page with an explanation.

The route table is deliberately a function call rather than a framework router. `route` returns an `HttpResponse`, so handlers do not write bytes to the socket themselves. That keeps the game rules testable as ordinary functions and leaves one place (`handle_connection`) responsible for translating a response value into HTTP.

Action handlers redirect to the session page after processing a GET form:

```text
GET /load?mode=elo&source=demo
  -> create_session(...)
  -> 303 Location: /play?sid=1
  -> GET /play?sid=1
  -> render current session as HTML
```

The forms are simple to inspect, but their fields become URL query values. That is why the Guidebook calls this a local learning prototype rather than an account service.

## Sessions are state for one round

`Session` groups the loaded `ChessGame`, `GameMode`, difficulty, player target, selected ply, explanatory note, answer feedback, and whether ratings or an opening have been revealed. A `Mutex<HashMap<String, Session>>` protects the process-wide session store. `OnceLock` initializes it once; an `AtomicU64` creates numeric identifiers.

Creating a session starts Guess the Elo at ply zero. Guess the Opening starts at `min(8, game.ply_count())`, so it reveals only a short sequence. A session lives in memory only: restarting the program clears it. These ids are not accounts and do not persist.

The mutex is held only while reading or changing the map. `render_session` borrows a session immutably for rendering; `update_move` and answer handlers use `get_mut` to change the ply or feedback. In the current server loop requests are processed serially, so the lock mostly makes the ownership boundary explicit and keeps the store safe if request handling later becomes concurrent. A poisoned lock is surfaced as a readable page instead of unwrapped.

## Move navigation is a normal link

Each board control and SAN move links to `/move` with a session id and requested ply. The handler parses the requested number, uses the current ply if parsing fails, and clamps it to the current mode's limit. The next render reads `ChessGame::position_at(session.ply)` and draws that saved snapshot. Moving also clears old answer feedback so a stale result does not look attached to the new position.

There are two clamps. `ChessGame::position_at` protects its private snapshot vector from an out-of-range index. `max_ply` applies the product rule before the session is updated: Elo can reach the end of the game, while Opening stays within the first eight plies. The move list uses the same limit, so a hidden later move cannot be selected from its links.

## Submitting answers

`submit_elo_guess` confirms the session is in Elo mode, then scores only the selected player or players. `score_one` parses an integer, restricts it to `0..=4000`, handles a missing actual rating, computes absolute error, and compares against the mode's difficulty tolerance. It builds one feedback line per requested player and marks the session revealed.

`submit_opening_guess` confirms the session mode, then calls `chess::opening_matches`. An empty input gets a prompt; a match or mismatch reveals the stored opening. This is case-insensitive after whitespace normalization.

Both form handlers redirect back to `/play`. The session map retains the result, and `render_game` shows feedback and revealed metadata on the returned page.

The `target` value is normalized when the game is loaded. For `both`, both calls to `score_one` run; for `white` or `black`, only that player is scored. The renderer and handler use the same setting, so the page only displays an input for a rating that will actually be checked. `score_one` appends plain text built from a color label, bounded integer, and rating; opening feedback escapes the API-provided opening name before it is inserted into HTML.

## Render the home page and game

`render_home` writes the overview, GothamChess inspiration link, formal description, mode forms, and demo links. The archive form includes username, difficulty, and target; the pasted-link form also asks for the game URL. The opening form starts an opening challenge from a selected player's archive or the included demo.

`render_game` chooses the appropriate mode panel, computes the mode's maximum visible ply, creates the board HTML and move list, renders player rows and game metadata, and includes a notice for demo or fallback data. `render_elo_panel` omits the unselected side's input. `render_opening_panel` shows the first move sequence and answer field.

`render_board` makes 64 labeled squares from a `[char; 64]` position. Its `aria-label` describes each occupied square. `chess_symbol` maps the internal piece letter to a Unicode chess character, and `piece_name` supplies an accessible name. `mini_board` uses the same piece mapping for the overview illustration.

The CSS embedded from `assets/styles/` defines the charcoal, green, and cream chess-app layout, board-square colors, navigation, forms, and responsive columns. The book stylesheet is separate because book pages need a readable paper-like reading surface rather than the game palette.

`page` is the shared document wrapper. It escapes the title, inserts the fixed stylesheet, and inserts a notice that handlers have already escaped when they construct it. A renderer should treat every dynamic value as text: player names, PGN metadata, and API error messages can come from outside the program. `render_move_list` escapes each SAN token; player rows escape names and ratings; game metadata is escaped before interpolation.

## Keep untrusted text as text

Game names, API messages, answer feedback, and metadata can contain user- or service-provided text. `html_escape` replaces ampersands, angle brackets, quotation marks, and apostrophes before a value is inserted into markup. Most render helpers escape their dynamic text at the point of use.

The Guidebook parser first escapes ordinary Markdown text, then adds only the markup it supports: headings, paragraphs, lists, fenced code, inline code, emphasis, blockquotes, and approved links. `safe_markdown_href` allows HTTP(S), local Guidebook paths, and fragments; other link targets are rendered as text. This is a small documentation renderer, not a full CommonMark implementation.

`inline_markdown` scans from left to right. When it finds a matching pair of backticks, it escapes the contents and emits `<code>`. It handles `**strong text**` recursively and parses `[label](target)` only if the destination passes `safe_markdown_href`. All other characters go through `html_escape`. This order matters: ordinary Markdown characters cannot smuggle raw tags into a chapter.

## HTTP response helpers

`write_response` emits the status line, content type, byte length, `Connection: close`, and `X-Content-Type-Options: nosniff`, then writes the HTML body. `write_download` uses the Markdown content type and attachment filename. `url_decode` consumes valid percent escapes, leaves malformed percent characters in place, and converts the resulting bytes using lossy UTF-8.

`parse_query` splits on ampersands, then splits each pair once at `=`. It URL-decodes both the key and value and inserts them into a `HashMap`; if a key repeats, the last insertion wins. `normalize_difficulty` accepts Easy, Medium, Hard, and the historical spelling `harm`, then defaults unknown values to Medium. `normalize_target` accepts White and Black and treats all other inputs as Both. Those normalization functions are the boundary between untrusted URL text and a small set of values used by session logic.

When adding a route, follow the same four-step path: parse the URL field in `route`, validate and normalize incoming values in a handler, update or create one `Session`, and render the resulting state. Keep output writing in `handle_connection` and use `HttpResponse` for redirects, page bodies, downloads, and missing routes. For exact helper responsibilities, jump to [the web module walkthrough](/guide/reference#web-server-and-rendered-pages).

For the code map and route-by-route details, see [Codebase Walkthrough — reference.md](/guide/reference#web-server-and-rendered-pages). The next topic is [Terminal Lessons](/guide/terminal-learning).
