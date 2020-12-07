#![allow(non_snake_case,dead_code,unused_variables)]

use atl::{CWindow};
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::ops::{Deref,DerefMut};
/*
(1)
^\t(\w+)\s+(\w+)\((.*)\)
=>
\tpub fn \2 \(\3\)->\1

(2)
delete 
->void

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

\(WPARAM\)(\w+)
=>
\1 as WPARAM

(7)  define
#define (\w+)\s+(\w+)
=>
 \1: UINT = \2;

 (8) return convert
 return \((\w+)\)(.*);
 =>
 \2 as \1
*/
pub struct CStatic {
    inner: CWindow,
}

impl Deref for CStatic {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CStatic {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CStatic {
	pub fn new()->CStatic {
        CStatic{
            inner: CWindow::new(0 as HWND),
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
// ructors
// 	CStaticT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CStaticT< TBase >& operator =(HWND hWnd)
// 	{
// 		m_hWnd = hWnd;
// 		return *this;
// 	}

// 	HWND Create(HWND hWndParent, ATL::_U_RECT rect = NULL, LPCTSTR szWindowName = NULL,
// 			DWORD dwStyle = 0, DWORD dwExStyle = 0,
// 			ATL::_U_MENUorID MenuOrID = 0U, LPVOID lpCreateParam = NULL)
// 	{
// 		return TBase::Create(GetWndClassName(), hWndParent, rect.m_lpRect, szWindowName, dwStyle, dwExStyle, MenuOrID.m_hMenu, lpCreateParam);
// 	}

// // Attributes
// 	static LPCTSTR GetWndClassName()
// 	{
// 		return _T("STATIC");
// 	}

//#ifndef _WIN32_WCE

	pub fn GetIcon (&self)->HICON {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_GETICON, 0, 0) as HICON
	}

	pub fn SetIcon(&self,hIcon: HICON)->HICON {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_SETICON, hIcon as WPARAM, 0) as HICON
	}

	pub fn GetEnhMetaFile (&self)->HENHMETAFILE {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_GETIMAGE, IMAGE_ENHMETAFILE as WPARAM, 0) as HENHMETAFILE
	}

	pub fn SetEnhMetaFile(&self,hMetaFile: HENHMETAFILE)->HENHMETAFILE {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_SETIMAGE, IMAGE_ENHMETAFILE as WPARAM, hMetaFile as LPARAM) as HENHMETAFILE
	}
//#else // CE specific
	// pub fn GetIcon (&self)->HICON {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( winapi::um::winuser::STM_GETIMAGE, IMAGE_ICON, 0) as HICON
	// }

	// pub fn SetIcon(&self,hIcon: HICON)->HICON {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( winapi::um::winuser::STM_SETIMAGE, IMAGE_ICON, hIcon as LPARAM) as HICON
	// }
//#endif // _WIN32_WCE

	// pub fn GetBitmap ()->CBitmapHandle {
	// 	self.inner.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.inner.SendMessage( winapi::um::winuser::STM_GETIMAGE, IMAGE_BITMAP, 0));
	// }

	// pub fn SetBitmap(&self,hBitmap: HBITMAP)->CBitmapHandle {
	// 	self.inner.assert_window();
	// 	return CBitmapHandle((HBITMAP)self.inner.SendMessage( winapi::um::winuser::STM_SETIMAGE, IMAGE_BITMAP, hBitmap as LPARAM));
	// }

	pub fn GetCursor (&self)->HCURSOR {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_GETIMAGE, IMAGE_CURSOR as WPARAM, 0) as HCURSOR
	}

	pub fn SetCursor(&self,hCursor: HCURSOR)->HCURSOR {
		self.inner.assert_window();
		self.inner.SendMessage( winapi::um::winuser::STM_SETIMAGE, IMAGE_CURSOR as WPARAM, hCursor as LPARAM) as HCURSOR
	}
}

//typedef CStaticT<ATL::CWindow>   CStatic;




/////////////////////////////////////////////////////////
// expose all inner methods
// impl CStatic {
//     expose_cwindow!();
// }