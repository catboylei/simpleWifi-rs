use std::io;
use std::io::{stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use rust_simple_tui::simpletui::ui::Menu;
use crate::constants::LABEL;
use crate::utils::network_manager::{wifi_as_vec};

// i dont need error handling if this fails i kill myself
pub fn enter_select() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
}

pub fn leave_select() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn prompt_select_from_vec() -> io::Result<String> {
    let entries = wifi_as_vec();

    enter_select();
    let mut paws: Menu = Menu::new();

    paws.add_label(LABEL.to_string());

    for entry in entries {
        paws.add_action(entry.to_string(), format!("{}:{}:{}", entry.ssid, entry.active, entry.bssid));
    }

    paws.add_label("".to_string());
    paws.add_action("refresh selection".to_string(), "simplewifi-refresh-select".to_string());
    paws.add_action("back to main menu".to_string(), "simplewifi-exit-select".to_string());

    let res=paws.render()?;
    leave_select();

    Ok(res)
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
            _ => current.push(c),
        }
    }

    result.push(current);
    result
}
