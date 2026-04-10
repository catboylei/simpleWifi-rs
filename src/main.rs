pub mod utils;
mod action_select;

use std::io;
use crate::action_select::select_action;
// todo handle using up on something while another is already up


fn main() -> io::Result<()> {
    select_action()
}