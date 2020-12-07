#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use wtl::atl::{Dialog,DlgMsg};
use wtl::ctrls::{CButton,BtnMsg};
use winapi::um::winuser::IDOK;

// modal dialog should not auto created
pub struct AboutDialog<T> {
    pub this: Dialog<T>,
    btn_ok:CButton,
}

impl<T> AboutDialog<T> {
	pub fn new()->AboutDialog<T>{
		AboutDialog{
			this: Dialog::new(IDD_ABOUTBOX as u16),
			btn_ok:CButton::new(),
		}
	}

	////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T>{
		DlgMsg::new(&mut self.this.handlers)
	}

	pub fn btn_ok_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDOK as u16)
	}
}
