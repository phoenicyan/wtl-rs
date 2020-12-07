#![allow(dead_code)]

use winapi::shared::minwindef::*;
use winapi::um::winuser::*;

//use super::super::cwindow::*;
//use super::consts::*;
//use super::Event;

use atl::{Handler,Event};
//use atl::CWindow;

pub struct EdtMsg <'a,T:'a> {
	id:WORD,
    h:&'a mut Vec<Handler<T>>,
}

impl<'a,T> EdtMsg<'a,T> {
	pub fn new(id:WORD,h:&'a mut Vec<Handler<T>>)->EdtMsg<'a,T>{
		EdtMsg{
			id:id,
			h:h,
		}
	}
}

impl<'a,T> EdtMsg<'a,T> {
	/// EN_CHANGE
	///
	/// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761676(v=vs.85).aspx
	///
	/// Sent when the user has taken an action that may have altered text in an edit control. 
	/// Unlike the EN_UPDATE notification code, this notification code is sent after the system updates the screen
	pub fn on_change<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_CHANGE, priority, f));
    }

    /// EN_UPDATE
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761687(v=vs.85).aspx
    /// Sent when an edit control is about to redraw itself. This notification code is sent after the control has formatted the text, 
    /// but before it displays the text. This makes it possible to resize the edit control window, if necessary
	pub fn on_update<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_UPDATE, priority, f));
    }

    /// EN_SETFOCUS
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761685(v=vs.85).aspx
    ///
    /// Sent when an edit control receives the keyboard focus
	pub fn on_set_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_SETFOCUS, priority, f));
    }

    /// EN_KILLFOCUS
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761682(v=vs.85).aspx
    ///
    /// Sent when an edit control loses the keyboard focus.
	pub fn on_kill_focus<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_KILLFOCUS, priority, f));
    }

    /// EN_VSCROLL
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761689(v=vs.85).aspx
    ///
    /// Sent when the user clicks an edit control's vertical scroll bar or when the user scrolls the mouse wheel over the edit control
	pub fn on_v_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_VSCROLL, priority, f));
    }

    /// EN_HSCROLL
    ///
    /// https://msdn.microsoft.com/en-us/library/windows/desktop/bb761680(v=vs.85).aspx
    ///
    /// Sent when the user clicks an edit control's horizontal scroll bar.
    /// The parent window is notified before the screen is updated.
	pub fn on_h_scroll<F>(&mut self,priority:u16,f:F) where F:Fn(&mut Event,&mut T) + 'static {
        self.h.push(Handler::new(WM_COMMAND, self.id, EN_HSCROLL, priority, f));
    }
}