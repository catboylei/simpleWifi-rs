pub mod utils;
mod action_select;

use std::io;
use crate::action_select::select_action;
// todo handle using up on something while another is already up
// todo clean up all of the expects to have sensible messages and remove unnecesary ones

fn main() -> io::Result<()> {
    loop { if select_action()? { break } }
    Ok(())
}