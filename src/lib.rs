//! wtl: GUI operation library ,include dialogs,buttons,cmd_bar,mdi frame
//! atl: Message loop and disptach


extern crate winapi;

pub use atl::*;
pub use wtl::*;

pub mod atl;
pub mod wtl;
