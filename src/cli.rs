use crate::lessons::{LESSONS, Lesson};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Lesson,
    Check,
    Solution,
    TestResult,
}

struct UiState {
    screen: Screen,
    selected: usize,
    lesson_index: usize,
    check_selected: usize,
    show_hint: bool,
    started: HashSet<&'static str>,
    completed: HashSet<&'static str>,
    message: String,
    solution: Vec<String>,
    solution_offset: usize,
    test_output: Vec<String>,
    test_passed: bool,
}

impl UiState {
    fn new() -> Self {
        Self {
            screen: Screen::Home,
            selected: 0,
            lesson_index: 0,
            check_selected: 0,
            show_hint: false,
            started: HashSet::new(),
            completed: HashSet::new(),
            message: "Choose a lesson. Press Enter to open it, or G to open the Guidebook.".into(),
            solution: Vec::new(),
            solution_offset: 0,
            test_output: Vec::new(),
            test_passed: false,
        }
    }

    fn lesson(&self) -> &'static Lesson {
        &LESSONS[self.lesson_index]
    }
}

pub fn run() {
    if let Err(error) = terminal::enable_raw_mode() {
        plain_fallback(&format!("Raw mode is unavailable ({error})."));
        return;
    }
    let mut stdout = io::stdout();
    if execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All)).is_err() {
        let _ = terminal::disable_raw_mode();
        plain_fallback("The terminal could not enter the interactive screen.");
        return;
    }

    let mut state = UiState::new();
    let mut running = true;
    while running {
        draw(&mut stdout, &state);
        match event::poll(Duration::from_millis(200)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => running = handle_key(key, &mut state),
                Ok(_) => {}
                Err(_) => running = false,
            },
            Ok(false) => {}
            Err(_) => running = false,
        }
    }

    let _ = execute!(stdout, Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
}

fn handle_key(key: KeyEvent, state: &mut UiState) -> bool {
    match state.screen {
        Screen::Home => match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            KeyCode::Up => state.selected = state.selected.saturating_sub(1),
            KeyCode::Down => state.selected = (state.selected + 1).min(LESSONS.len() - 1),
            KeyCode::Enter => {
                state.lesson_index = state.selected;
                state.started.insert(state.lesson().id);
                state.screen = Screen::Lesson;
                state.show_hint = false;
                state.message.clear();
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                state.message =
                    "Book: start `cargo run -- --web`, then open http://127.0.0.1:7878/guide"
                        .into();
            }
            KeyCode::Char('?') => {
                state.message = "↑/↓ choose a lesson · Enter open · G Guidebook · Q quit".into();
            }
            _ => {}
        },
        Screen::Lesson => match key.code {
            KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Home,
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
            KeyCode::Char('t') | KeyCode::Char('T') => {
                run_current_test(state);
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                state.screen = Screen::Check;
                state.check_selected = 0;
                state.message =
                    "Select an answer and press Enter to check your understanding.".into();
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                let path = format!(
                    "{}/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    state.lesson().solution_file
                );
                let source = fs::read_to_string(path)
                    .unwrap_or_else(|error| format!("Could not open reference solution: {error}"));
                state.solution = source.lines().map(str::to_owned).collect();
                state.solution_offset = 0;
                state.screen = Screen::Solution;
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                state.message = format!(
                    "Exercise reference: start `cargo run -- --web`, then open http://127.0.0.1:7878/guide/reference#{}",
                    state.lesson().reference_anchor
                );
            }
            _ => {}
        },
        Screen::Check => match key.code {
            KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Lesson,
            KeyCode::Up => state.check_selected = state.check_selected.saturating_sub(1),
            KeyCode::Down => state.check_selected = (state.check_selected + 1).min(2),
            KeyCode::Char('1') => state.check_selected = 0,
            KeyCode::Char('2') => state.check_selected = 1,
            KeyCode::Char('3') => state.check_selected = 2,
            KeyCode::Enter => {
                if state.check_selected == state.lesson().answer {
                    state.message = "Concept check passed. Run the Rust exercise tests with T when you're ready.".into();
                    state.screen = Screen::Lesson;
                } else {
                    state.message = "Not quite. Revisit the prompt or hint, then try again.".into();
                }
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            _ => {}
        },
        Screen::Solution => match key.code {
            KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Lesson,
            KeyCode::Up => state.solution_offset = state.solution_offset.saturating_sub(1),
            KeyCode::Down => {
                state.solution_offset =
                    (state.solution_offset + 1).min(state.solution.len().saturating_sub(1))
            }
            KeyCode::PageUp => state.solution_offset = state.solution_offset.saturating_sub(12),
            KeyCode::PageDown => {
                state.solution_offset =
                    (state.solution_offset + 12).min(state.solution.len().saturating_sub(1))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            _ => {}
        },
        Screen::TestResult => match key.code {
            KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Lesson,
            KeyCode::Char('r') | KeyCode::Char('R') => run_current_test(state),
            KeyCode::Char('s') | KeyCode::Char('S') => {
                let path = format!(
                    "{}/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    state.lesson().solution_file
                );
                let source = fs::read_to_string(path)
                    .unwrap_or_else(|error| format!("Could not open reference solution: {error}"));
                state.solution = source.lines().map(str::to_owned).collect();
                state.solution_offset = 0;
                state.screen = Screen::Solution;
            }
            KeyCode::Up => state.solution_offset = state.solution_offset.saturating_sub(1),
            KeyCode::Down => {
                state.solution_offset =
                    (state.solution_offset + 1).min(state.test_output.len().saturating_sub(1))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            _ => {}
        },
    }
    true
}

fn draw(stdout: &mut io::Stdout, state: &UiState) {
    let _ = queue!(stdout, MoveTo(0, 0), Clear(ClearType::All));
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(" GTE · RUSTLINGS PATH\n")
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(" learn the pieces behind the chess app\n\n"),
        ResetColor
    );

    match state.screen {
        Screen::Home => draw_home(stdout, state),
        Screen::Lesson => draw_lesson(stdout, state),
        Screen::Check => draw_check(stdout, state),
        Screen::Solution => draw_solution(stdout, state),
        Screen::TestResult => draw_test_result(stdout, state),
    }

    if !state.message.is_empty() {
        let _ = queue!(stdout, Print(format!("\n{}\n", state.message)));
    }
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print("\nQ quit · Esc back · ? shortcuts"),
        ResetColor
    );
    let _ = stdout.flush();
}

fn draw_home(stdout: &mut io::Stdout, state: &UiState) {
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print("PROJECT LESSONS\n"),
        ResetColor
    );
    for (index, lesson) in LESSONS.iter().enumerate() {
        let pointer = if index == state.selected { "›" } else { " " };
        let status = if state.completed.contains(lesson.id) {
            "✓"
        } else if state.started.contains(lesson.id) {
            "◉"
        } else {
            "○"
        };
        if index == state.selected {
            let _ = queue!(stdout, SetForegroundColor(Color::Green));
        }
        let _ = queue!(
            stdout,
            Print(format!(
                " {pointer} {status} {:<28} {}\n",
                lesson.title, lesson.track
            ))
        );
        let _ = queue!(stdout, ResetColor);
    }
    let _ = queue!(
        stdout,
        Print(
            "\n○ not started · ◉ in progress · ✓ complete\nEach lesson includes a runnable Rust test, hint, concept check, and reference solution in `solutions/`.\n"
        )
    );
    let _ = queue!(
        stdout,
        Print(
            "The final capstone asks you to assemble a guessing flow from a blank implementation.\n"
        )
    );
}

fn draw_lesson(stdout: &mut io::Stdout, state: &UiState) {
    let lesson = state.lesson();
    let exercise_file = lesson.id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", lesson.track)),
        ResetColor
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print(format!("{}\n\n", lesson.title)),
        ResetColor
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "EDIT exercises/src/{exercise_file}.rs\nREFERENCE exercises/reference.md#{}\n\n",
            lesson.reference_anchor
        )),
        ResetColor
    );
    let _ = queue!(
        stdout,
        Print(format!("{}\n\nTASK\n{}\n", lesson.concept, lesson.prompt))
    );
    if state.show_hint {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("\nHINT\n{}\n", lesson.hint)),
            ResetColor
        );
    }
    if state.completed.contains(lesson.id) {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("\n✓ Rust exercise tests passed\n"),
            ResetColor
        );
    }
    let _ = queue!(
        stdout,
        Print(
            "\nH show hint · T run Rust tests · C concept check · S reference solution · G Guidebook · Esc lesson list\n"
        )
    );
}

fn draw_check(stdout: &mut io::Stdout, state: &UiState) {
    let lesson = state.lesson();
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print("KNOWLEDGE CHECK\n"),
        ResetColor
    );
    let _ = queue!(
        stdout,
        Print(format!("{}\n\n{}\n", lesson.title, lesson.question))
    );
    for (index, choice) in lesson.choices.iter().enumerate() {
        let pointer = if index == state.check_selected {
            "›"
        } else {
            " "
        };
        if index == state.check_selected {
            let _ = queue!(stdout, SetForegroundColor(Color::Green));
        }
        let _ = queue!(
            stdout,
            Print(format!(" {pointer} {}. {choice}\n", index + 1))
        );
        let _ = queue!(stdout, ResetColor);
    }
    let _ = queue!(
        stdout,
        Print("\n↑/↓ or 1–3 select · Enter check · Esc return to lesson\n")
    );
}

fn draw_solution(stdout: &mut io::Stdout, state: &UiState) {
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("REFERENCE · {}\n", state.lesson().title)),
        ResetColor
    );
    if state.completed.contains(state.lesson().id) {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("✓ Exercise tests passed · matching solution loaded\n\n"),
            ResetColor
        );
    }
    for line in state.solution.iter().skip(state.solution_offset).take(20) {
        let _ = queue!(stdout, Print(format!("{line}\n")));
    }
    let _ = queue!(
        stdout,
        Print("\n↑/↓ scroll · PgUp/PgDn page · Esc return to lesson\n")
    );
}

fn draw_test_result(stdout: &mut io::Stdout, state: &UiState) {
    let (label, color) = if state.test_passed {
        ("TESTS PASSED", Color::Green)
    } else {
        ("TESTS NEED WORK", Color::Yellow)
    };
    let _ = queue!(
        stdout,
        SetForegroundColor(color),
        Print(format!("{label} · {}\n", state.lesson().title)),
        ResetColor
    );
    for line in state
        .test_output
        .iter()
        .skip(state.solution_offset)
        .take(20)
    {
        let _ = queue!(stdout, Print(format!("{line}\n")));
    }
    let _ = queue!(
        stdout,
        Print("\nR rerun this test · S open the reference solution · Esc return to lesson\n")
    );
}

fn run_current_test(state: &mut UiState) {
    let lesson = state.lesson();
    let manifest = format!("{}/exercises/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let target = lesson.id.replace('-', "_");
    let output = Command::new("cargo")
        .arg("test")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--test")
        .arg(&target)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    let (passed, text) = match output {
        Ok(output) => {
            let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            (output.status.success(), text)
        }
        Err(error) => (false, format!("Could not start Cargo: {error}")),
    };
    state.test_passed = passed;
    state.test_output = text.lines().map(str::to_owned).collect();
    state.solution_offset = 0;
    if passed {
        state.completed.insert(lesson.id);
        let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), lesson.solution_file);
        match fs::read_to_string(path) {
            Ok(source) => {
                state.solution = source.lines().map(str::to_owned).collect();
                state.screen = Screen::Solution;
                state.message =
                    "Exercise tests passed. The matching reference solution loaded automatically."
                        .into();
            }
            Err(error) => {
                state.screen = Screen::TestResult;
                state.message = format!(
                    "Exercise tests passed, but the reference solution could not load: {error}"
                );
            }
        }
    } else {
        state.screen = Screen::TestResult;
        state.message = "Edit the exercise code, check the hint, and run the test again.".into();
    }
}

fn plain_fallback(reason: &str) {
    println!("ChessProject interactive lesson UI\n{reason}\n");
    for (index, lesson) in LESSONS.iter().enumerate() {
        println!("{}. {} — {}", index + 1, lesson.title, lesson.concept);
    }
    println!(
        "\nStart the browser Guidebook with `cargo run -- --web` at http://127.0.0.1:7878/guide."
    );
}
