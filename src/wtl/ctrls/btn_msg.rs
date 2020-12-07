
// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775941(v=vs.85).aspx
// Handling Messages from a Button

// Notifications from a button are sent as either WM_COMMAND or WM_NOTIFY messages. 
// Information about which message is used can be found on the reference page for each notification.


// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775983(v=vs.85).aspx

use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

//use super::super::cwindow::*;
//use super::consts::*;
//use super::Event;

use atl::{Handler,HandlerPriority,Event};
//use atl::CWindow;

pub struct BtnMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> BtnMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->BtnMsg<'a,T>{
		BtnMsg{
			id:id,
			h:h,
		}
	}
}

impl<'a,T> BtnMsg<'a,T> {
	// BN_CLICKED
	// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761825(v=vs.85).aspx
	pub fn on_click<F>(&mut self,f:F)->HandlerPriority<T> where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, BN_CLICKED, 100, f));
        let l = self.h.len()-1;
		HandlerPriority::new(&mut self.h[l])
    }
}