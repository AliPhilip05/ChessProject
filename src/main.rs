mod api;
mod chess;
mod cli;
mod lessons;
mod web;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--web") {
        web::serve();
    } else {
        cli::run();
    }
}
