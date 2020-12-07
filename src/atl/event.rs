
#![allow(dead_code,non_snake_case)]
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;

pub struct Event<'a> {
    uMsg:u32,
    hwnd  :HWND,
    wParam:WPARAM,
    lParam:LPARAM,
    lResult:&'a mut LRESULT,
}

impl<'a> Event<'a> {
	#[inline(always)]
	pub fn new(hwnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,l:&'a mut LRESULT) ->Event {
		Event{
			hwnd:hwnd,
			uMsg:uMsg,
			wParam:wParam,
			lParam:lParam,
			lResult:l,
		}
	}

	#[inline(always)]
	pub fn set_result(&mut self,l:LRESULT){
		*self.lResult = l;
	}

	#[inline(always)]
	pub fn get_msg(&self)->u32{
		self.uMsg
	}

	#[inline(always)]
	pub fn get_hwnd(&self)->HWND{
		self.hwnd
	}

	#[inline(always)]
	pub fn get_wparam(&self)->WPARAM{
		self.wParam
	}

	#[inline(always)]
	pub fn get_lparam(&self)->LPARAM{
		self.lParam
	}
}
