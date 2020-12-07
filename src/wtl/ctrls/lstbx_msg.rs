#![allow(dead_code)]

// https://msdn.microsoft.com/en-us/library/windows/desktop/ff485968(v=vs.85).aspx
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

use atl::{Handler,Event};

pub struct LstBxMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> LstBxMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->LstBxMsg<'a,T>{
		LstBxMsg{
			id:id,
			h:h,
		}
	}
}

// This notification code is sent only by a list box that has the LBS_NOTIFY style.
impl<'a,T> LstBxMsg<'a,T> {
	// LBN_DBLCLK
	// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775153(v=vs.85).aspx
	pub fn on_db_click<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, LBN_DBLCLK, priority, f));
    }

    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775161(v=vs.85).aspx
    pub fn on_sel_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, LBN_SELCHANGE, priority, f));
    }

    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775159(v=vs.85).aspx
    pub fn on_sel_cancel<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, LBN_SELCANCEL, priority, f));
    }

    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775164(v=vs.85).aspx
    pub fn on_set_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, LBN_SETFOCUS, priority, f));
    }

    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775158(v=vs.85).aspx
    pub fn on_kill_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, LBN_KILLFOCUS, priority, f));
    }
}