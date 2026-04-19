pub mod utils;
mod action_select;
mod constants;

use std::io;
use crate::action_select::select_action;
use crate::utils::network_manager::NetworkManager;

fn main() -> io::Result<()> {
    let nm = NetworkManager::new();
    loop { if select_action()? { break } }
    Ok(())
}