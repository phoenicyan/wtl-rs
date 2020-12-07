
#![allow(non_snake_case,dead_code,unused_variables)]
///////////////////////////////////////////////////////////////////////////////
// CButton - client side for a Windows BUTTON control
use std::ops::{Deref,DerefMut};
use atl::{CWindow,NULL_HWND};
use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::ntdef::*;
use winapi::um::commctrl::*;
use winapi::um::winuser::*;

//use super::consts::*;

pub struct CButton {
    inner: CWindow,
}

impl Deref for CButton {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CButton {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CButton {
    pub fn new()->CButton{
        CButton{
            inner: CWindow::new(NULL_HWND),
        }
    }

    pub fn cwin(&self)->&CWindow {
		&self.inner
	}

    // pub fn Attach(&mut self,h: HWND) {
    //     self.inner.Attach(h)
    // }
    
    // pub fn Detach(&mut self)->HWND {
    //     self.inner.Detach()
    // }

	// pub fn cwin_mut(&mut self)->&mut CWindow {
	// 	&mut self.inner
	// }
	// pub fn cwin_mut(&mut self)->&mut CWindow {
	// 	&mut self.inner
	// }
}
/*
(1)
^\t(\w+)\s+(\w+)\((.*)\)
=>
\tpub fn \2 \(\3\)->\1

(2)
delete 
->void
const
(3)
ATLASSERT(::IsWindow(m_hWnd));
=>
self.inner.assert_window();

(4)
::SendMessage(m_hWnd,
=>
self.inner.SendMessage(

(5) parameter define
pub fn (\w+)\s*\((\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\)

(6) coercion
\(LPARAM\)(\w+)
=>
\1 as LPARAM
*/
impl CButton {
	
	// CButtonT(HWND hWnd = NULL) : TBase(hWnd)
	// { }

	// CButtonT< TBase >& operator =(HWND hWnd)
	// {
	// 	m_hWnd = hWnd;
	// 	return *this;
	// }

	// HWND Create(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
	// 		DWORD dwStyle = 0, DWORD dwExStyle = 0,
	// 		ATL::_U_MENUorID MenuOrID = 0U, LPVOID lpCreateParam = NULL)
	// {
	// 	return TBase::Create(GetWndClassName(), hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, MenuOrID.m_hMenu, lpCreateParam);
	// }

// Attributes
	// static LPCTSTR GetWndClassName()
	// {
	// 	return _T("BUTTON");
	// }
	pub fn GetState(&self) ->UINT {
		//self.inner.assert_window();
		//return (UINT)self.inner.SendMessage( winapi::um::winuser::BM_GETSTATE, 0, 0);
		self.inner.assert_window();
		self.inner.SendMessage(winapi::um::winuser::BM_GETSTATE, 0, 0) as UINT
	}

	pub fn SetState(&self,bHighlight: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_SETSTATE, bHighlight as WPARAM, 0);
	}

	pub fn GetCheck(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_GETCHECK, 0, 0) as c_int
	}

	pub fn SetCheck(&self,nCheck: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_SETCHECK, nCheck as WPARAM, 0);
	}

	pub fn GetButtonStyle(&self)->UINT {
		self.inner.assert_window();
		(self.inner.GetWindowLong(GWL_STYLE) & 0xFFFF) as UINT
	}

	pub fn SetButtonStyle(&self,nStyle:UINT, bRedraw:BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_SETSTYLE, nStyle as WPARAM, bRedraw as LPARAM);
	}

//#ifndef _WIN32_WCE
	pub fn GetIcon(&self)->HICON {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_GETIMAGE, IMAGE_ICON as WPARAM, 0) as HICON
	}

	pub fn SetIcon(&self,hIcon: HICON)->HICON {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_SETIMAGE, IMAGE_ICON as WPARAM, hIcon as LPARAM) as HICON
	}

	// pub fn GetBitmap ()->CBitmapHandle 
	// {
	// 	self.inner.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.inner.SendMessage( winapi::um::winuser::BM_GETIMAGE, IMAGE_BITMAP, 0));
	// }

	// pub fn SetBitmap(&self,hBitmap: HBITMAP)->CBitmapHandle
	// {
	// 	self.inner.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.inner.SendMessage( winapi::um::winuser::BM_SETIMAGE, IMAGE_BITMAP, hBitmap as LPARAM));
	// }
//#endif // !_WIN32_WCE

//#if (_WIN32_WINNT >= 0x0501)
	pub fn GetIdealSize(&self,lpSize: LPSIZE)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_GETIDEALSIZE, 0, lpSize as LPARAM) as BOOL
	}

	pub fn GetImageList(&self,pButtonImagelist: PBUTTON_IMAGELIST)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_GETIMAGELIST, 0, pButtonImagelist as LPARAM) as BOOL
	}

	pub fn SetImageList(&self,pButtonImagelist: PBUTTON_IMAGELIST)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_SETIMAGELIST, 0, pButtonImagelist as LPARAM) as BOOL
	}

	pub fn GetTextMargin(&self,lpRect: LPRECT)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_GETTEXTMARGIN, 0, lpRect as LPARAM) as BOOL
	}

	pub fn SetTextMargin(&self,lpRect: LPRECT)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_SETTEXTMARGIN, 0, lpRect as LPARAM) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0501)

//#if (WINVER >= 0x0600)
	pub fn SetDontClick(&self,bDontClick: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_SETDONTCLICK, bDontClick as WPARAM, 0);
	}
//#endif // (WINVER >= 0x0600)

//#if (_WIN32_WINNT >= 0x0600)
	pub fn SetDropDownState(&self,bDropDown: BOOL)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.inner.SendMessage( BCM_SETDROPDOWNSTATE, bDropDown as WPARAM, 0) as BOOL
	}

	pub fn GetSplitInfo(&self,pSplitInfo: PBUTTON_SPLITINFO)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.inner.SendMessage( BCM_GETSPLITINFO, 0, pSplitInfo as LPARAM) as BOOL
	}

	pub fn SetSplitInfo(&self,pSplitInfo: PBUTTON_SPLITINFO)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (BS_SPLITBUTTON | BS_DEFSPLITBUTTON)) != 0);
		self.inner.SendMessage( BCM_SETSPLITINFO, 0, pSplitInfo as LPARAM) as BOOL
	}

	pub fn GetNoteLength (&self)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
		self.inner.SendMessage( BCM_GETNOTELENGTH, 0, 0) as c_int
	}

	// pub fn GetNote (LPWSTR lpstrNoteText, int cchNoteText)->BOOL {
	// 	self.inner.assert_window();
	// 	debug_assert!((self.inner.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
	// 	self.inner.SendMessage( BCM_GETNOTE, cchNoteText, lpstrNoteText as LPARAM) as BOOL
	// }

	pub fn SetNote(&self,lpstrNoteText: LPCWSTR)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (BS_COMMANDLINK | BS_DEFCOMMANDLINK)) != 0);
		self.inner.SendMessage( BCM_SETNOTE, 0, lpstrNoteText as LPARAM) as BOOL
	}

	pub fn SetElevationRequiredState(&self,bSet: BOOL)->LRESULT {
		self.inner.assert_window();
		self.inner.SendMessage( BCM_SETSHIELD, 0, bSet as LPARAM)
	}
//#endif // (_WIN32_WINNT >= 0x0600)

// Operations
	pub fn Click (&self) {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::BM_CLICK, 0, 0);
	}
}

/////////////////////////////////////////////////////////
// expose all inner methods
// impl CButton {
// 	expose_cwindow!();
// }