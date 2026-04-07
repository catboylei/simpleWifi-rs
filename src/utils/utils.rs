use std::io;
use std::io::{stdout};
use crossterm::ExecutableCommand;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use simple_terminal_select::fancyterm::ui::Menu;

// i dont need error handling if this fails i kill myself
pub fn enter_select() {
    enable_raw_mode().unwrap();
    stdout().execute(EnterAlternateScreen).unwrap();
}

pub fn leave_select() {
    stdout().execute(LeaveAlternateScreen).unwrap();
    disable_raw_mode().unwrap();
}

pub fn prompt_select_from_vec(entries: Vec<String>) -> io::Result<String> {
    enter_select(); // add error handling to this util
    let mut paws: Menu = Menu::new();

    for entry in entries {
        paws.add_action(entry.clone(), entry) // this is terrible but it works
    }

    let res=paws.render()?;
    leave_select(); // this one too

    Ok(res)
}