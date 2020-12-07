#![allow(non_snake_case,dead_code,unused_variables)]

use atl::{CWindow,NULL_HWND};
use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::windowsx::*;
use winapi::shared::ntdef::*;
use winapi::um::commctrl::*;
use winapi::um::winuser::*;

use std::ops::{Deref,DerefMut};

pub struct CEdit {
    inner: CWindow,
}

impl Deref for CEdit {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CEdit {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CEdit {
    pub fn new()->CEdit{
        CEdit{
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
}

/*
(1)
delete const
) const
=>
)

convert fn format
^\t(\w+)\s+(\w+)\((.*)\)\s+\{
=>
\tpub fn \2 \(\3\)->\1 {

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

// no parameter
pub fn (\w+)\s*\(\)
=>
pub fn \1\(&self)

//one parameter
pub fn (\w+)\s*\((\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\)
// two parameter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\, \5: \4)

// three paramter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+), (\w+) (\w+)\)
=>
pub fn \1\(&self,\3: \2\, \5: \4,\7: \6)

//four parameter
pub fn (\w+)\s*\((\w+) (\w+), (\w+) (\w+), (\w+) (\w+), (\w+) (\w+)\s*\)
=>
pub fn \1\(&self,\3: \2\, \5: \4,\7: \6,\9: \8)

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

(9) replace manually
WPARAM coercion
(self.inner.SendMessage\(\s*\w+,\s*)(\w+)(.*)
=>
\1 \2 as WPARAM\3

LPARAM coercion
(self.inner.SendMessage\(.*,.*,\s*)(\w+)(.*)
=>
\1 \2 as LPARAM\3

*/

impl CEdit{
// Constructors
// 	CEditT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CEditT< TBase >& operator =(HWND hWnd)
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
// 		return _T("EDIT");
// 	}

	pub fn CanUndo(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_CANUNDO.into(), 0, 0) as BOOL
	}

	pub fn GetLineCount(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETLINECOUNT.into(), 0, 0) as c_int
	}

	pub fn GetModify(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETMODIFY.into(), 0, 0) as BOOL
	}

	pub fn SetModify (&self,bModified: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETMODIFY.into(),  bModified as WPARAM, 0);
	}

	pub fn GetRect(&self,lpRect: LPRECT) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETRECT.into(), 0, lpRect as LPARAM);
	}

	pub fn GetSel(&self)->DWORD {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETSEL.into(), 0, 0) as DWORD
	}

	pub fn GetSel2 (&self,nStartChar: &mut c_int, nEndChar: &mut c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETSEL.into(), nStartChar as *mut _ as *mut c_void as WPARAM, nEndChar as *mut _ as *mut c_void as LPARAM);
	}

//#ifndef _WIN32_WCE
	pub fn GetHandle(&self)->HLOCAL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETHANDLE.into(), 0, 0) as HLOCAL
	}

	pub fn SetHandle(&self,hBuffer: HLOCAL) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETHANDLE.into(), hBuffer as WPARAM, 0);
	}
//#endif // !_WIN32_WCE

	pub fn GetMargins(&self)->DWORD {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETMARGINS.into(), 0, 0) as DWORD
	}

	pub fn GetMargins2 (&self,nLeft: &mut UINT, nRight: &mut UINT) {
		self.inner.assert_window();
		let dwRet = self.inner.SendMessage( EM_GETMARGINS.into(), 0, 0) as DWORD;
		*nLeft = LOWORD(dwRet) as UINT;
		*nRight = HIWORD(dwRet) as UINT;
	}

	// WORD wFlags = EC_LEFTMARGIN | EC_RIGHTMARGIN
	pub fn SetMargins (&self, nLeft: UINT, nRight: UINT, wFlags: WORD) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETMARGINS.into(),  wFlags as WPARAM, MAKELONG(nLeft as WORD, nRight as WORD) as LPARAM);
	}

	pub fn GetLimitText(&self)->UINT {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETLIMITTEXT.into(), 0, 0) as UINT
	}

	pub fn SetLimitText(&self,nMax: UINT) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETLIMITTEXT.into(),  nMax as WPARAM, 0);
	}

	pub fn PosFromChar(&self,nChar: UINT)->POINT {
		self.inner.assert_window();
		let dwRet = self.inner.SendMessage( EM_POSFROMCHAR.into(),  nChar as WPARAM, 0) as DWORD;
		POINT{ x:GET_X_LPARAM(dwRet as LPARAM), y:GET_Y_LPARAM(dwRet as LPARAM) }
	}

	// pLine = NULL
	pub fn CharFromPos (&self, pt: POINT, pLine: Option<&mut c_int>)->c_int {
		self.inner.assert_window();
		let dwRet = self.inner.SendMessage( EM_CHARFROMPOS.into(), 0, MAKELONG(pt.x as WORD, pt.y as WORD) as LPARAM) as DWORD;
		// if pLine != None{
		// 	*pLine = (c_int)(short)HIWORD(dwRet);
		// }
		if let Some(p) = pLine {
			*p = HIWORD(dwRet) as c_short as c_int;
		}
		LOWORD(dwRet) as c_short as c_int
	}

	// NOTE: first word in lpszBuffer must contain the size of the buffer!
	// pub fn GetLine(&self,nIndex: c_int, lpszBuffer: LPTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( EM_GETLINE, nIndex, lpszBuffer as LPARAM) as c_int
	// }

	// pub fn GetLine(&self,nIndex: c_int, lpszBuffer: LPTSTR,nMaxLength: c_int)->c_int {
	// 	self.inner.assert_window();
	// 	*(LPWORD)lpszBuffer = (WORD)nMaxLength;
	// 	self.inner.SendMessage( EM_GETLINE, nIndex, lpszBuffer as LPARAM) as c_int
	// }

	// pub fn GetPasswordChar(&self)->TCHAR {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( EM_GETPASSWORDCHAR, 0, 0) as TCHAR
	// }

	// pub fn SetPasswordChar(&self,ch: TCHAR) {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( EM_SETPASSWORDCHAR, ch, 0);
	// }

//#ifndef _WIN32_WCE
	pub fn GetWordBreakProc(&self)->EDITWORDBREAKPROCW {
		self.inner.assert_window();
		unsafe{
			::std::mem::transmute(self.inner.SendMessage( EM_GETWORDBREAKPROC.into(), 0, 0))
		}
	}

	pub fn SetWordBreakProc(&self,ewbprc: EDITWORDBREAKPROCW) {
		self.inner.assert_window();
		match ewbprc {
			None =>
				self.inner.SendMessage( EM_SETWORDBREAKPROC.into(), 0, 0),
			Some(p) =>
				self.inner.SendMessage( EM_SETWORDBREAKPROC.into(), 0, p as LPARAM),
		};
	}
//#endif // !_WIN32_WCE

	pub fn GetFirstVisibleLine(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETFIRSTVISIBLELINE.into(), 0, 0) as c_int
	}

//#ifndef _WIN32_WCE
	pub fn GetThumb(&self)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & ES_MULTILINE) != 0);
		self.inner.SendMessage( EM_GETTHUMB.into(), 0, 0) as c_int
	}
//#endif // !_WIN32_WCE

	pub fn SetReadOnly (&self, bReadOnly: BOOL)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETREADONLY.into(),  bReadOnly as WPARAM, 0) as BOOL
	}

//#if (WINVER >= 0x0500) && !defined(_WIN32_WCE)
	pub fn GetImeStatus(&self,uStatus: UINT)->UINT {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETIMESTATUS.into(),  uStatus as WPARAM, 0) as UINT
	}

	pub fn SetImeStatus(&self,uStatus: UINT, uData: UINT)->UINT {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETIMESTATUS.into(),  uStatus as WPARAM,  uData as LPARAM) as UINT
	}
//#endif // (WINVER >= 0x0500) && !defined(_WIN32_WCE)

//#if (_WIN32_WINNT >= 0x0501)
	pub fn GetCueBannerText(&self,lpstrText: LPCWSTR, cchText: c_int)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETCUEBANNER.into(), lpstrText as WPARAM,  cchText as LPARAM) as BOOL
	}

	// bKeepWithFocus - Vista only
	// pub fn SetCueBannerText (LPCWSTR lpstrText, BOOL bKeepWithFocus = winapi::shared::minwindef::p)->BOOL {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( EM_SETCUEBANNER, bKeepWithFocus as WPARAM, (LPARAM)(lpstrText)) as BOOL
	// }
//#endif // (_WIN32_WINNT >= 0x0501)

// Operations
	pub fn EmptyUndoBuffer(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_EMPTYUNDOBUFFER.into(), 0, 0);
	}

	pub fn FmtLines(&self,bAddEOL: BOOL)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_FMTLINES.into(),  bAddEOL as WPARAM, 0) as BOOL
	}

	pub fn LimitText (&self,nChars: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_LIMITTEXT.into(),  nChars as WPARAM, 0);
	}

	pub fn LineFromChar(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( EM_LINEFROMCHAR.into(),  nIndex as WPARAM, 0) as c_int
	}

	pub fn LineIndex(&self,nLine: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( EM_LINEINDEX.into(),  nLine as WPARAM, 0) as c_int
	}

	pub fn LineLength(&self,nLine: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( EM_LINELENGTH.into(),  nLine as WPARAM, 0) as c_int
	}

	pub fn LineScroll(&self,nLines: c_int, nChars: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_LINESCROLL.into(),  nChars as WPARAM,  nLines as LPARAM);
	}

	// pub fn ReplaceSel(&self,lpszNewText: LPCTSTR, bCanUndo: BOOL) {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( EM_REPLACESEL, bCanUndo as WPARAM, lpszNewText as LPARAM);
	// }

	pub fn SetRect(&self,lpRect: LPCRECT) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETRECT.into(), 0, lpRect as LPARAM);
	}

	pub fn SetRectNP(&self,lpRect: LPCRECT) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETRECTNP.into(), 0, lpRect as LPARAM);
	}

	pub fn SetSel(&self,dwSelection: DWORD, bNoScroll: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETSEL.into(), LOWORD(dwSelection) as WPARAM, HIWORD(dwSelection) as LPARAM);
		if bNoScroll == winapi::shared::minwindef::FALSE {
			self.inner.SendMessage( EM_SCROLLCARET.into(), 0, 0);
		}
	}

	pub fn SetSel2(&self,nStartChar: c_int, nEndChar: c_int,bNoScroll: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETSEL.into(),  nStartChar as WPARAM,  nEndChar as LPARAM);
		if bNoScroll == winapi::shared::minwindef::FALSE{
			self.inner.SendMessage( EM_SCROLLCARET.into(), 0, 0);
		}
	}

	pub fn SetSelAll(&self,bNoScroll: BOOL) {
		self.SetSel2(0, -1, bNoScroll);
	}

	pub fn SetSelNone(&self,bNoScroll: BOOL) {
		self.SetSel2(-1, 0, bNoScroll);
	}

	pub fn SetTabStops3(&self,nTabStops: c_int, rgTabStops: LPINT)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETTABSTOPS.into(),  nTabStops as WPARAM, rgTabStops as LPARAM) as BOOL
	}

	pub fn SetTabStops(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETTABSTOPS.into(), 0, 0) as BOOL
	}

	pub fn SetTabStops2(&self,cxEachStop: *const c_int)->BOOL	{
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETTABSTOPS.into(), 1,cxEachStop as LPARAM) as BOOL
	}

	pub fn ScrollCaret(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SCROLLCARET.into(), 0, 0);
	}

	pub fn Scroll(&self,nScrollAction: c_int)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & ES_MULTILINE) != 0);
		let lRet = self.inner.SendMessage( EM_SCROLL.into(),  nScrollAction as WPARAM, 0);
		if HIWORD(lRet as DWORD) as BOOL == winapi::shared::minwindef::FALSE {
			return -1;   // failed
		}
		LOWORD(lRet as DWORD) as c_short as c_int
		
	}

	// pub fn InsertText(&self,nInsertAfterChar: c_int, lpstrText: LPCTSTR,bNoScroll: BOOL,bCanUndo: BOOL) {
	// 	SetSel(nInsertAfterChar, nInsertAfterChar, bNoScroll);
	// 	ReplaceSel(lpstrText, bCanUndo);
	// }

	// pub fn AppendText (LPCTSTR lpstrText, BOOL bNoScroll, BOOL bCanUndo ) {
	// 	InsertText(GetWindowTextLength(), lpstrText, bNoScroll, bCanUndo);
	// }

//#if (_WIN32_WINNT >= 0x0501)
	pub fn ShowBalloonTip(&self,pEditBaloonTip: PEDITBALLOONTIP)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SHOWBALLOONTIP.into(), 0, pEditBaloonTip as LPARAM) as BOOL
	}

	pub fn HideBalloonTip(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_HIDEBALLOONTIP.into(), 0, 0) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0501)

//#if (_WIN32_WINNT >= 0x0600)
	pub fn GetHilite(&self)->DWORD {
		self.inner.assert_window();
		self.inner.SendMessage( EM_GETHILITE.into(), 0, 0) as DWORD
	}

	pub fn GetHilite2(&self, nStartChar: &mut c_int, nEndChar: &mut c_int) {
		self.inner.assert_window();
		let dwRet = self.inner.SendMessage( EM_GETHILITE.into(), 0, 0) as DWORD;
		*nStartChar = LOWORD(dwRet) as c_short as c_int;
		*nEndChar = HIWORD(dwRet) as c_short as c_int;
	}

	pub fn SetHilite(&self,nStartChar: c_int, nEndChar: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( EM_SETHILITE.into(),  nStartChar as WPARAM,  nEndChar as LPARAM);
	}
//#endif // (_WIN32_WINNT >= 0x0600)

	// Clipboard operations
	pub fn Undo(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( EM_UNDO.into(), 0, 0) as BOOL
	}

	pub fn Clear(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( WM_CLEAR, 0, 0);
	}

	pub fn Copy(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( WM_COPY, 0, 0);
	}

	pub fn Cut(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( WM_CUT, 0, 0);
	}

	pub fn Paste(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( WM_PASTE, 0, 0);
	}

//#ifdef WIN32_PLATFORM_WFSP   // SmartPhone only messages
	// pub fn GetExtendedStyle(&self)->DWORD {
	// 	return SendMessage(EM_GETEXTENDEDSTYLE);
	// }

	// pub fn SetExtendedStyle(&self,dwMask: DWORD, dwExStyle: DWORD)->DWORD {
	// 	return SendMessage(EM_SETEXTENDEDSTYLE, dwMask as WPARAM, dwExStyle as LPARAM);
	// }

	// pub fn GetInputMode(&self,bCurrentMode: BOOL)->DWORD {
	// 	return SendMessage(EM_GETINPUTMODE, 0, bCurrentMode as LPARAM);
	// }

	// pub fn SetInputMode(&self,dwMode: DWORD)->BOOL {
	// 	return SendMessage(EM_SETINPUTMODE, 0, dwMode as LPARAM);
	// }

	// pub fn SetSymbols(&self,szSymbols: LPCTSTR)->BOOL {
	// 	return SendMessage(EM_SETSYMBOLS, 0, szSymbols as LPARAM);
	// }

	// pub fn ResetSymbols(&self)->BOOL {
	// 	return SendMessage(EM_SETSYMBOLS);
	// }
//#endif // WIN32_PLATFORM_WFSP
}

//typedef CEditT<ATL::CWindow>   CEdit;


/////////////////////////////////////////////////////////
// expose all inner methods
// impl CEdit {
// 	expose_cwindow!();
// }
