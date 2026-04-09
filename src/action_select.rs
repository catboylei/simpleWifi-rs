use std::io;
use simple_terminal_select::fancyterm::ui::Menu;
use crate::utils::network_manager::{handle_wifi_selection, wifi_as_vec};
use crate::utils::utils::{enter_select, leave_select, prompt_select_from_vec};

pub fn select_action() -> io::Result<()> {
    enter_select();
    let mut paws: Menu = Menu::new();

    paws.add_label("Wifi Manager :3".to_string());
    paws.add_action("rescan wifi".to_string(), "scan".to_string());
    paws.add_action("connect".to_string(), "con".to_string());
    paws.add_action("test".to_string(), "test".to_string());

    let res=paws.render()?;
    leave_select();

    match res.as_str() {
        "scan" => for meow in wifi_as_vec().iter() {
            println!("{}", meow)
        },
        "con" => handle_wifi_selection(prompt_select_from_vec(wifi_as_vec()).unwrap_or("Error :c".to_string())),
        "test" => println!("meow"),
        _ => println!("you fucked up g")
    }

    Ok(())
}