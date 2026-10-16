use super::route;
use crate::web::http::HttpResponse;
use crate::web::session::sessions;

fn page(target: &str) -> String {
    match route(target) {
        HttpResponse::Page(body) => body,
        _ => panic!("expected an HTML page for {target}"),
    }
}

fn demo_session(mode: &str, target: &str) -> String {
    match route(&format!(
        "/load?mode={mode}&source=demo&difficulty=hard&target={target}"
    )) {
        HttpResponse::Redirect(location) => location
            .strip_prefix("/play?sid=")
            .expect("new sessions redirect to play")
            .to_owned(),
        _ => panic!("expected a new demo session"),
    }
}

#[test]
fn elo_round_preserves_navigation_selected_side_and_rating_reveal() {
    let id = demo_session("elo", "white");
    let (rating, final_ply) = {
        let store = sessions().lock().unwrap();
        let session = &store[&id];
        (session.game.white_rating.unwrap(), session.game.ply_count())
    };
    let initial = page(&format!("/play?sid={id}"));
    assert_eq!(initial.matches("class=\"square ").count(), 64);
    assert!(initial.contains("name=white"));
    assert!(!initial.contains("name=black"));
    assert!(!initial.contains(&format!("Rating · {rating}")));

    assert!(matches!(
        route(&format!("/move?sid={id}&ply=99999")),
        HttpResponse::Redirect(_)
    ));
    assert_eq!(sessions().lock().unwrap()[&id].ply, final_ply);
    route(&format!("/guess-elo?sid={id}&white={rating}"));
    let revealed = page(&format!("/play?sid={id}"));
    assert!(revealed.contains(&format!("Rating · {rating}")));
    assert!(revealed.contains("inside tolerance"));
    assert!(revealed.contains("Black · Rating hidden"));
    sessions().lock().unwrap().remove(&id);
}

#[test]
fn opening_round_clamps_moves_and_accepts_normalized_answer() {
    let id = demo_session("opening", "both");
    let answer = {
        let store = sessions().lock().unwrap();
        assert_eq!(store[&id].ply, 8);
        store[&id]
            .game
            .opening
            .to_ascii_uppercase()
            .replace(' ', "+")
    };
    route(&format!("/move?sid={id}&ply=99999"));
    assert_eq!(sessions().lock().unwrap()[&id].ply, 8);
    route(&format!("/guess-opening?sid={id}&guess=++{answer}++"));
    assert!(page(&format!("/play?sid={id}")).contains("Correct — the opening is"));
    sessions().lock().unwrap().remove(&id);
}

#[test]
fn book_downloads_and_unknown_routes_keep_their_contracts() {
    assert!(page("/guide").contains("Download Function Appendix PDF"));
    assert!(matches!(route("/unknown"), HttpResponse::NotFound));
    assert!(matches!(route("/guide/missing"), HttpResponse::NotFound));
    match route("/guide/download") {
        HttpResponse::Download(book) => {
            assert!(book.contains("# The GTE Book"));
            assert!(book.contains("Function page"));
        }
        _ => panic!("expected Markdown download"),
    }
    match route("/guide/functions.pdf") {
        HttpResponse::Pdf(bytes) => assert!(bytes.starts_with(b"%PDF-")),
        _ => panic!("expected PDF download"),
    }
}
