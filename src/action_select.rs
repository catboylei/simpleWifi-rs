use std::io;
use rust_simple_tui::simpletui::ui::Menu;
use crate::constants::ABOUT;
use crate::utils::network_manager::{handle_wifi_selection, prompt_select_from_cache};
use crate::utils::utils::{enter_select, leave_select};

pub fn select_action() -> io::Result<bool> {
    enter_select();
    let mut paws: Menu = Menu::new();

    paws.add_label("Wifi Manager :3".to_string());
    paws.add_action("connections".to_string(), "con".to_string());
    paws.add_action("test".to_string(), "test".to_string());
    paws.add_label("".to_string());
    paws.add_action("about".to_string(), "about".to_string());
    paws.add_action("exit".to_string(), "exit".to_string());


    let res=paws.render()?;
    leave_select();

    match res.as_str() {
        "about" => {
            println!("{}", ABOUT);
            return Ok(true)
        }
        "con" => {
            loop {
                if handle_wifi_selection(prompt_select_from_cache().unwrap_or("".to_string())) { break; }
            };
        },
        "test" => {
        },
        "exit" => { return Ok(true) } // stop looping
        _ => println!("you fucked up g") // this is for the compiler but if you somehow manage to proc it ill send u 10 bucks
    }

    Ok(false) // return false to keep looping
}