#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Component {
    pub path: &'static str,
    pub purpose: &'static str,
}

pub const COMPONENTS: [Component; 6] = [
    Component {
        path: "src/main.rs",
        purpose: "selects the terminal or browser entry point",
    },
    Component {
        path: "src/cli.rs",
        purpose: "runs the keyboard-driven Rust exercise UI",
    },
    Component {
        path: "src/lessons.rs",
        purpose: "defines exercise prompts, hints, and test targets",
    },
    Component {
        path: "src/api.rs",
        purpose: "loads public Chess.com archive games",
    },
    Component {
        path: "src/chess.rs",
        purpose: "parses PGN and calculates chess-game answers",
    },
    Component {
        path: "src/web.rs",
        purpose: "serves the game app and the GTE Book",
    },
];

pub fn product_mark() -> &'static str {
    "GTE"
}

pub fn entrypoint(web_mode: bool) -> &'static str {
    if web_mode {
        "web::serve"
    } else {
        "cli::run"
    }
}

pub fn component(path: &str) -> Option<&'static Component> {
    COMPONENTS.iter().find(|item| item.path == path)
}
