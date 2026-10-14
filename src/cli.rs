use crate::lessons::{LESSONS, Lesson};
use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{execute, queue};
use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, SystemTime};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    List,
    Lesson,
    Check,
    Solution,
    TestResult,
    ResetConfirm,
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
    last_modified: Option<SystemTime>,
}

/// Raw mode disables the terminal's usual LF-to-CRLF conversion. Restore it
/// for rendered text so diagnostics and prompts always begin at column one.
struct CrLfWriter<W> {
    inner: W,
    last_was_carriage_return: bool,
}

impl<W> CrLfWriter<W> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            last_was_carriage_return: false,
        }
    }
}

impl<W: Write> Write for CrLfWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut normalized = Vec::with_capacity(bytes.len() + 8);
        for &byte in bytes {
            if byte == b'\n' && !self.last_was_carriage_return {
                normalized.push(b'\r');
            }
            normalized.push(byte);
            self.last_was_carriage_return = byte == b'\r';
        }
        self.inner.write_all(&normalized)?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

impl UiState {
    fn new() -> Self {
        let completed = load_progress();
        let lesson_index = LESSONS
            .iter()
            .position(|lesson| !completed.contains(lesson.id))
            .unwrap_or(0);
        Self {
            screen: Screen::Home,
            selected: lesson_index,
            lesson_index,
            check_selected: 0,
            show_hint: false,
            started: completed.clone(),
            completed,
            message: String::new(),
            solution: Vec::new(),
            solution_offset: 0,
            test_output: Vec::new(),
            test_passed: false,
            last_modified: None,
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
    let mut stdout = CrLfWriter::new(io::stdout());
    if execute!(stdout, EnterAlternateScreen, Hide, Clear(ClearType::All)).is_err() {
        let _ = terminal::disable_raw_mode();
        plain_fallback("The terminal could not enter the interactive screen.");
        return;
    }

    let mut state = UiState::new();
    run_current_test(&mut state);
    let mut running = true;
    while running {
        draw(&mut stdout, &state);
        match event::poll(Duration::from_millis(200)) {
            Ok(true) => match event::read() {
                Ok(Event::Key(key)) => running = handle_key(key, &mut state),
                Ok(_) => {}
                Err(_) => running = false,
            },
            Ok(false) => {
                if matches!(state.screen, Screen::Home | Screen::Solution)
                    && source_modified(&state).is_some_and(|modified| {
                        state.last_modified.is_some_and(|last| modified > last)
                    })
                {
                    run_current_test(&mut state);
                }
            }
            Err(_) => running = false,
        }
    }

    let _ = execute!(stdout, Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
    println!(
        "We hope you're enjoying learning Rust! If you want to continue working on the exercises at a later point, you can simply run `gte` again in this directory."
    );
}

fn handle_key(key: KeyEvent, state: &mut UiState) -> bool {
    if matches!(key.code, KeyCode::Char('x') | KeyCode::Char('X'))
        && state.screen != Screen::ResetConfirm
    {
        if state.screen == Screen::List {
            state.lesson_index = state.selected;
        }
        state.screen = Screen::ResetConfirm;
        state.message.clear();
        return true;
    }
    if key.code == KeyCode::Char('?') {
        state.message = match state.screen {
            Screen::Home => "h:hint / l:list / c:check all / r:run / x:reset / n:next / p:previous / q:quit ?".into(),
            Screen::List => "n/p or ↑/↓ select / Enter: run exercise / l:back / q:quit".into(),
            Screen::Lesson => {
                "N/P next/previous · R run · H hint · C check · S solution · G Guidebook · L list · Q quit".into()
            }
            Screen::Check => "↑/↓ or 1–3 select · Enter check · R run · N/P next/previous · Esc/L lesson list · Q quit".into(),
            Screen::Solution => "↑/↓ or PgUp/PgDn scroll · N/P next/previous · Esc lesson · L list · Q quit".into(),
            Screen::TestResult => "R rerun · S solution · N/P next/previous · Esc lesson · L list · Q quit".into(),
            Screen::ResetConfirm => "y:reset current exercise / n:cancel".into(),
        };
        return true;
    }
    match state.screen {
        Screen::Home => match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            KeyCode::Char('p') | KeyCode::Char('P') => advance_previous(state),
            KeyCode::Char('n') | KeyCode::Char('N') => advance_next(state),
            KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
            KeyCode::Char('l') | KeyCode::Char('L') => open_lesson_list(state),
            KeyCode::Char('c') | KeyCode::Char('C') => run_all_tests(state),
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
                run_current_test(state);
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                state.message =
                    "Book: start `cargo run -- --web`, then open http://127.0.0.1:7878/guide"
                        .into();
            }
            _ => {}
        },
        Screen::List => match key.code {
            KeyCode::Esc | KeyCode::Backspace | KeyCode::Char('l') | KeyCode::Char('L') => {
                state.screen = Screen::Home;
                state.message.clear();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            KeyCode::Up | KeyCode::Char('p') | KeyCode::Char('P') => {
                state.selected = state.selected.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('n') | KeyCode::Char('N') => {
                state.selected = (state.selected + 1).min(LESSONS.len() - 1);
            }
            KeyCode::Enter => {
                state.lesson_index = state.selected;
                run_current_test(state);
            }
            KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
            _ => {}
        },
        Screen::Lesson => match key.code {
            KeyCode::Esc | KeyCode::Backspace => state.screen = Screen::Home,
            KeyCode::Char('l') | KeyCode::Char('L') => open_lesson_list(state),
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            KeyCode::Char('h') | KeyCode::Char('H') => state.show_hint = !state.show_hint,
            KeyCode::Char('n') | KeyCode::Char('N') => {
                advance_next(state);
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                advance_previous(state);
            }
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
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
            KeyCode::Char('l') | KeyCode::Char('L') => {
                open_lesson_list(state);
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                advance_next(state);
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                advance_previous(state);
            }
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
                run_current_test(state)
            }
            KeyCode::Up => state.check_selected = state.check_selected.saturating_sub(1),
            KeyCode::Down => state.check_selected = (state.check_selected + 1).min(2),
            KeyCode::Char('1') => state.check_selected = 0,
            KeyCode::Char('2') => state.check_selected = 1,
            KeyCode::Char('3') => state.check_selected = 2,
            KeyCode::Enter => {
                if state.check_selected == state.lesson().answer {
                    state.message =
                        "Concept check passed. Press R to run the Rust exercise check.".into();
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
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
                run_current_test(state)
            }
            KeyCode::Char('l') | KeyCode::Char('L') => {
                open_lesson_list(state);
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                advance_next(state);
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                advance_previous(state);
            }
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
            KeyCode::Char('l') | KeyCode::Char('L') => {
                open_lesson_list(state);
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                advance_next(state);
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                advance_previous(state);
            }
            KeyCode::Char('r') | KeyCode::Char('R') | KeyCode::Char('t') | KeyCode::Char('T') => {
                run_current_test(state)
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
            KeyCode::Up => state.solution_offset = state.solution_offset.saturating_sub(1),
            KeyCode::Down => {
                state.solution_offset =
                    (state.solution_offset + 1).min(state.test_output.len().saturating_sub(1))
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            _ => {}
        },
        Screen::ResetConfirm => match key.code {
            KeyCode::Char('y') | KeyCode::Char('Y') => reset_current_exercise(state),
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc | KeyCode::Backspace => {
                state.screen = Screen::Home;
                state.message = "Reset cancelled.".into();
            }
            KeyCode::Char('q') | KeyCode::Char('Q') => return false,
            _ => {}
        },
    }
    true
}

fn open_lesson_list(state: &mut UiState) {
    state.selected = state.lesson_index;
    state.screen = Screen::List;
    state.message.clear();
}

fn advance_next(state: &mut UiState) {
    if !state.completed.contains(state.lesson().id) {
        state.message =
            "This exercise is not complete yet. Edit it, then press R to run it.".into();
        return;
    }
    let next = (state.lesson_index + 1..LESSONS.len())
        .chain(0..state.lesson_index)
        .find(|index| !state.completed.contains(LESSONS[*index].id));
    match next {
        Some(index) => {
            state.lesson_index = index;
            state.selected = index;
            state.show_hint = false;
            state.message.clear();
            run_current_test(state);
        }
        None => {
            state.message = "All exercises are complete. Use L to review the exercise list.".into();
        }
    }
}

fn advance_previous(state: &mut UiState) {
    if state.lesson_index == 0 {
        state.message = "This is the first exercise.".into();
        return;
    }
    state.lesson_index -= 1;
    state.selected = state.lesson_index;
    state.show_hint = false;
    state.message.clear();
    run_current_test(state);
}

fn source_path(state: &UiState) -> PathBuf {
    let file_name = state.lesson().id.replace('-', "_");
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("exercises/src")
        .join(format!("{file_name}.rs"))
}

fn source_modified(state: &UiState) -> Option<SystemTime> {
    fs::metadata(source_path(state)).ok()?.modified().ok()
}

fn reset_current_exercise(state: &mut UiState) {
    let relative = format!("exercises/src/{}.rs", state.lesson().id.replace('-', "_"));
    let source_path = source_path(state);
    let backup_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".gte-backup")
        .join(state.lesson().id.replace('-', "_") + ".rs");
    let original = Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output();
    let original = match original {
        Ok(output) if output.status.success() => output.stdout,
        Ok(output) => {
            state.screen = Screen::Home;
            state.message = format!(
                "Could not restore the exercise from Git: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
            return;
        }
        Err(error) => {
            state.screen = Screen::Home;
            state.message = format!("Could not start Git to reset the exercise: {error}");
            return;
        }
    };
    if let Err(error) = fs::create_dir_all(backup_path.parent().expect("backup parent"))
        .and_then(|()| fs::copy(&source_path, &backup_path).map(|_| ()))
        .and_then(|()| fs::write(&source_path, original))
    {
        state.screen = Screen::Home;
        state.message =
            format!("Reset failed; the current file is backed up when possible: {error}");
        return;
    }
    state.completed.remove(state.lesson().id);
    state.started.remove(state.lesson().id);
    let _ = save_progress(&state.completed);
    state.screen = Screen::Home;
    state.show_hint = false;
    state.message = format!(
        "Reset {}. Previous contents saved in .gte-backup/.",
        state.lesson().title
    );
    run_current_test(state);
}

fn progress_path() -> String {
    format!("{}/.gte-state.txt", env!("CARGO_MANIFEST_DIR"))
}

fn load_progress() -> HashSet<&'static str> {
    let Ok(saved) = fs::read_to_string(progress_path()) else {
        return HashSet::new();
    };
    LESSONS
        .iter()
        .filter(|lesson| saved.lines().any(|line| line == lesson.id))
        .map(|lesson| lesson.id)
        .collect()
}

fn save_progress(completed: &HashSet<&'static str>) -> io::Result<()> {
    let saved = LESSONS
        .iter()
        .filter(|lesson| completed.contains(lesson.id))
        .map(|lesson| lesson.id)
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(progress_path(), saved)
}

fn draw<W: Write>(stdout: &mut W, state: &UiState) {
    let _ = queue!(stdout, MoveTo(0, 0), Clear(ClearType::All));
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(
            " █████    ███████   ██████\n██        ██        ██\n██ ███    █████     █████\n██  ██    ██        ██\n █████    ███████   ██████\n\n  GUESS THE ELO · RUST CODEBASE WALKTHROUGH\n\n"
        )
    );
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(" Rustlings-style project exercises\n\n"),
        ResetColor
    );

    match state.screen {
        Screen::Home => draw_home(stdout, state),
        Screen::List => draw_list(stdout, state),
        Screen::Lesson => draw_lesson(stdout, state),
        Screen::Check => draw_check(stdout, state),
        Screen::Solution => draw_solution(stdout, state),
        Screen::TestResult => draw_test_result(stdout, state),
        Screen::ResetConfirm => draw_reset_confirm(stdout, state),
    }

    if !state.message.is_empty() {
        let _ = queue!(stdout, Print(format!("\n{}\n", state.message)));
    }
    if state.screen != Screen::Home {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::DarkGrey),
            Print("\nQ quit · Esc back · ? shortcuts"),
            ResetColor
        );
    }
    let _ = stdout.flush();
}

fn draw_home<W: Write>(stdout: &mut W, state: &UiState) {
    let lesson = state.lesson();
    let bar_width = terminal::size()
        .map(|(columns, _)| (columns as usize).saturating_sub(28).clamp(10, 94))
        .unwrap_or(52);
    let complete_count = state.completed.len().min(LESSONS.len());
    let filled = complete_count * bar_width / LESSONS.len();
    let marker_width = if complete_count < LESSONS.len() { 1 } else { 0 };
    let exercise_file = lesson.id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print("Progress: ["),
        SetForegroundColor(Color::Green),
        Print("#".repeat(filled)),
        SetForegroundColor(Color::Yellow),
        Print(if complete_count < LESSONS.len() {
            ">"
        } else {
            ""
        }),
        SetForegroundColor(Color::DarkGrey),
        Print("-".repeat(bar_width.saturating_sub(filled + marker_width))),
        SetForegroundColor(Color::White),
        Print(format!("] {complete_count}/{}\n", LESSONS.len())),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "Current exercise: exercises/src/{exercise_file}.rs\n\n"
        )),
        ResetColor,
        SetForegroundColor(Color::White),
        Print(format!("{}\n{}\n", lesson.title, lesson.prompt)),
        ResetColor
    );
    if state.show_hint {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("\nHINT\n{}\n", lesson.hint)),
            ResetColor
        );
    }
    let rows = terminal::size()
        .map(|(_, rows)| rows as usize)
        .unwrap_or(24);
    let output_lines = state.test_output.len().min(rows.saturating_sub(12));
    if !state.test_output.is_empty() {
        let _ = queue!(
            stdout,
            SetForegroundColor(if state.test_passed {
                Color::Green
            } else {
                Color::Yellow
            }),
            Print(if state.test_passed {
                "\n✓ Exercise check passed\n"
            } else {
                "\nExercise check output\n"
            }),
            ResetColor
        );
        for line in state
            .test_output
            .iter()
            .skip(state.test_output.len().saturating_sub(output_lines))
        {
            let _ = queue!(stdout, Print(format!("{line}\n")));
        }
        if output_lines < state.test_output.len() {
            let _ = queue!(
                stdout,
                SetForegroundColor(Color::DarkGrey),
                Print("… output shortened to fit terminal\n"),
                ResetColor
            );
        }
    }
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::DarkGrey),
        Print(if state.completed.contains(lesson.id) {
            "\nh:hint / l:list / c:check all / x:reset / n:next / q:quit ?\n"
        } else {
            "\nh:hint / l:list / c:check all / x:reset / q:quit ?\n"
        }),
        ResetColor
    );
}

fn draw_reset_confirm<W: Write>(stdout: &mut W, state: &UiState) {
    let path = state.lesson().id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("Reset current exercise?\n\n"),
        ResetColor,
        Print(format!(
            "This restores exercises/src/{path}.rs from Git HEAD.\n"
        )),
        Print("A copy of the current file will be saved in .gte-backup/.\n\n"),
        SetForegroundColor(Color::White),
        Print("Reset (y/n)? "),
        ResetColor
    );
}

fn draw_list<W: Write>(stdout: &mut W, state: &UiState) {
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::White),
        Print("EXERCISE LIST\n"),
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
            "\n○ not started · ◉ in progress · ✓ complete\nN/P or ↑/↓ select · Enter open · L/Esc return\n"
        )
    );
}

fn draw_lesson<W: Write>(stdout: &mut W, state: &UiState) {
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
            "\nN next · P previous · R run check · H hint · C concept check · S solution · G Guidebook · L lesson list\n"
        )
    );
}

fn draw_check<W: Write>(stdout: &mut W, state: &UiState) {
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
        Print(
            "\n↑/↓ or 1–3 select · Enter check · R run · N/P next/previous · L list · Esc lesson\n"
        )
    );
}

fn draw_solution<W: Write>(stdout: &mut W, state: &UiState) {
    let complete_count = state.completed.len().min(LESSONS.len());
    let exercise_file = state.lesson().id.replace('-', "_");
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("REFERENCE · {}\n", state.lesson().title)),
        ResetColor,
        SetForegroundColor(Color::DarkGrey),
        Print(format!(
            "Progress: {complete_count}/{} · exercises/src/{exercise_file}.rs\n\n",
            LESSONS.len()
        )),
        ResetColor
    );
    if state.completed.contains(state.lesson().id) {
        let _ = queue!(
            stdout,
            SetForegroundColor(Color::Green),
            Print("✓ Exercise tests passed · matching solution loaded\n"),
            ResetColor
        );
        let test_lines = state
            .test_output
            .iter()
            .filter(|line| {
                line.starts_with("running ")
                    || line.starts_with("test ")
                    || line.starts_with("test result:")
            })
            .take(7);
        for line in test_lines {
            let _ = queue!(stdout, Print(format!("  {line}\n")));
        }
        let _ = queue!(stdout, Print("\n"));
    }
    let _ = queue!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print("MATCHING REFERENCE SOLUTION\n"),
        ResetColor
    );
    for line in state.solution.iter().skip(state.solution_offset).take(20) {
        let _ = queue!(stdout, Print(format!("{line}\n")));
    }
    let _ = queue!(
        stdout,
        Print("\n↑/↓ scroll · PgUp/PgDn page · N/P next/previous · L lesson list · Esc return\n")
    );
}

fn draw_test_result<W: Write>(stdout: &mut W, state: &UiState) {
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
        Print("\nR rerun · S solution · N/P next/previous · L lesson list · Esc return\n")
    );
}

fn run_current_test(state: &mut UiState) {
    let lesson = state.lesson();
    state.started.insert(lesson.id);
    let (passed, text) = run_lesson_test(lesson);
    state.last_modified = source_modified(state);
    state.test_passed = passed;
    state.test_output = text.lines().map(str::to_owned).collect();
    state.solution_offset = 0;
    if passed {
        state.completed.insert(lesson.id);
        let _ = save_progress(&state.completed);
        let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), lesson.solution_file);
        match fs::read_to_string(path) {
            Ok(source) => {
                state.solution = source.lines().map(str::to_owned).collect();
                state.screen = Screen::Solution;
                state.message =
                    "Exercise check passed. The matching reference solution loaded automatically."
                        .into();
            }
            Err(error) => {
                state.screen = Screen::TestResult;
                state.message = format!(
                    "Exercise check passed, but the reference solution could not load: {error}"
                );
            }
        }
    } else {
        state.completed.remove(lesson.id);
        let _ = save_progress(&state.completed);
        state.screen = Screen::Home;
        state.message = "Edit the exercise code, check the hint, and run the check again.".into();
    }
}

fn run_all_tests(state: &mut UiState) {
    let mut summary = Vec::new();
    let mut passed_count = 0;
    for lesson in &LESSONS {
        state.started.insert(lesson.id);
        let (passed, output) = run_lesson_test(lesson);
        if passed {
            state.completed.insert(lesson.id);
            passed_count += 1;
            summary.push(format!("✓ {}", lesson.title));
        } else {
            state.completed.remove(lesson.id);
            summary.push(format!("✗ {}", lesson.title));
            summary.extend(
                output
                    .lines()
                    .rev()
                    .take(5)
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev()
                    .map(|line| format!("  {line}")),
            );
        }
    }
    let _ = save_progress(&state.completed);
    state.test_passed = passed_count == LESSONS.len();
    state.test_output = summary;
    state.solution_offset = 0;
    state.screen = Screen::TestResult;
    state.message = format!(
        "Check all finished: {passed_count}/{} exercises passed.",
        LESSONS.len()
    );
}

fn run_lesson_test(lesson: &Lesson) -> (bool, String) {
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
    match output {
        Ok(output) => {
            let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            (output.status.success(), text)
        }
        Err(error) => (false, format!("Could not start Cargo: {error}")),
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
