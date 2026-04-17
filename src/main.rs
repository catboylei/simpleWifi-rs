pub mod utils;
mod action_select;
mod constants;

use std::io;
use crate::action_select::select_action;

fn main() -> io::Result<()> {
    loop { if select_action()? { break } }
    Ok(())
}