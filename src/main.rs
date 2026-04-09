pub mod utils;
mod action_select;

use std::io;
use crate::action_select::select_action;
// SHIFT TODO
// implement clean enter and leave selection options

fn main() -> io::Result<()> {
    select_action()
}