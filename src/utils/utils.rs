use std::io;
use std::io::{stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use rust_simple_tui::simpletui::ui::Menu;
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

    for entry in entries {
        paws.add_action(entry.to_string(), format!("{}:{}", entry.ssid, entry.active));
    }

    paws.add_label("".to_string());
    paws.add_action("back to main menu".to_string(), "simplewifi-exit-select".to_string());

    let res=paws.render()?;
    leave_select();

    Ok(res)
}