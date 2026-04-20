use std::io::stdout;
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
// i dont need error handling if this fails i kill myself
pub fn enter_select() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
}

pub fn leave_select() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn split_escaped(input: &str) -> Vec<String> { // custom splitter with escape chars
    let mut result = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                // take next char literally
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ':' => {
                result.push(current);
                current = String::new();
            }
            _ => current.push(c)
        }
    }

    result.push(current);
    result
}