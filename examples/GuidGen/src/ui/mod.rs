/// code in this mod are all generated automatically ,DO NOT modify any code here

pub use self::message_loop::{MessageLoop};
pub use self::root::{Root,DialogHandler};
pub use self::about_dialog::AboutDialog;

pub mod root;

mod about_dialog;
mod main_dialog;
mod message_loop;