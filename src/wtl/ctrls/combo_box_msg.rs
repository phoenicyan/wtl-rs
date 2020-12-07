#![allow(dead_code)]

// https://msdn.microsoft.com/en-us/library/windows/desktop/ff485968(v=vs.85).aspx
use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

//use super::consts::*;
use atl::{Handler,Event};

pub struct CmbBoxMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> CmbBoxMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->CmbBoxMsg<'a,T>{
		CmbBoxMsg{
			id:id,
			h:h,
		}
	}
}

// This notification code is sent only by a list box that has the LBS_NOTIFY style.
impl<'a,T> CmbBoxMsg<'a,T> {
	// CBN_SELCHANGE
	// https://msdn.microsoft.com/en-us/library/windows/desktop/bb775821(v=vs.85).aspx
	pub fn on_sel_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_SELCHANGE, priority, f));
    }

    // CBN_SELENDCANCEL
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775823(v=vs.85).aspx
    pub fn on_sel_end_cancel<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_SELENDCANCEL, priority, f));
    }

    // CBN_SELENDOK
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775825(v=vs.85).aspx
    pub fn on_sel_end_ok<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_SELENDOK, priority, f));
    }

    // CBN_EDITCHANGE
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775812(v=vs.85).aspx
    pub fn on_edit_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_EDITCHANGE, priority, f));
    }

    // CBN_EDITUPDATE
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775814(v=vs.85).aspx
    pub fn on_edit_update<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_EDITUPDATE, priority, f));
    }

    // CBN_CLOSEUP
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775806(v=vs.85).aspx
    pub fn on_closeup<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_CLOSEUP, priority, f));
    }

    // CBN_DROPDOWN
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775810(v=vs.85).aspx
    pub fn on_dropdown<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_DROPDOWN, priority, f));
    }

    // CBN_DBLCLK
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775808(v=vs.85).aspx
    pub fn on_db_click<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_DBLCLK, priority, f));
    }

    // CBN_KILLFOCUS
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775818(v=vs.85).aspx
	pub fn on_kill_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_KILLFOCUS, priority, f));
    }

    // CBN_SETFOCUS
    // https://msdn.microsoft.com/en-us/library/windows/desktop/bb775827(v=vs.85).aspx
	pub fn on_set_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, CBN_SETFOCUS, priority, f));
    }
}