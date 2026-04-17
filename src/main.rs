pub mod utils;
mod action_select;

use std::io;
use crate::action_select::select_action;
// todo clean up all of the expects to have sensible messages and remove unnecesary ones -> use string consts for the errors

fn main() -> io::Result<()> {
    loop { if select_action()? { break } }
    Ok(())
}