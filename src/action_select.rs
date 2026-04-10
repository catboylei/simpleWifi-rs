use std::io;
use simple_terminal_select::fancyterm::ui::Menu;
use crate::utils::network_manager::{handle_wifi_selection, rescan_wifi};
use crate::utils::utils::{enter_select, leave_select, prompt_select_from_vec};

pub fn select_action() -> io::Result<()> {
    enter_select();
    let mut paws: Menu = Menu::new();

    paws.add_label("Wifi Manager :3".to_string());
    paws.add_action("rescan wifi".to_string(), "scan".to_string());
    paws.add_action("connections".to_string(), "con".to_string());
    paws.add_action("test".to_string(), "test".to_string());

    let res=paws.render()?;
    leave_select();

    match res.as_str() {
        "scan" => {
            rescan_wifi();
            select_action().expect("TODO: panic message");
        },
        "con" => {
            loop { handle_wifi_selection(prompt_select_from_vec().unwrap_or("Error :c".to_string())) }; // todo dont use loop
        },
        "test" => println!("meow"),
        _ => println!("you fucked up g")
    }

    Ok(())
}