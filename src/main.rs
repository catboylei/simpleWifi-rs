pub mod utils;
mod action_select;
mod constants;

use std::io;
use crate::action_select::select_action;
use crate::utils::network_manager::rescan_cache;

fn main() -> io::Result<()> {
    rescan_cache();
    loop { if select_action()? { break } }
    Ok(())
}