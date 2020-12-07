#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate wtl;
extern crate winapi;

use std::mem;
use winapi::um::commctrl::*;
use winapi::um::winuser::*;

mod ui;
mod main_dlg;
mod about;

use ui::DialogHandler;

fn main() {
    let iccx = INITCOMMONCONTROLSEX { dwSize: mem::size_of::<INITCOMMONCONTROLSEX>() as u32, dwICC: ICC_BAR_CLASSES };
    unsafe { InitCommonControlsEx(&iccx); }

    let mut root = ui::Root::new();

    let main_dlg_handler = main_dlg::MainDialogHandler::new();
    main_dlg_handler.register_handler(&mut root);

    root.create();

    ui::MessageLoop::run();
}
