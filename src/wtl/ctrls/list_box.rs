#![allow(non_snake_case,dead_code,unused_variables,unused_imports)]

use atl::{CWindow,NULL_HWND};
use winapi::*;
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::ntdef::*;
use winapi::um::winuser::*;

//use super::consts::*;
use wtl::ToCU16Str;
use std::ops::{Deref,DerefMut};
/*
(1)
^\t(\w+)\s+(\w+)\((.*)\)\s+\{
=>
\tpub fn \2 \(\3\)->\1 {

(2)
delete 


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

pub struct CListBox {
    inner: CWindow,
}

impl Deref for CListBox {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CListBox {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CListBox {
// Constructors
// 	CListBoxT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CListBoxT< TBase >& operator =(HWND hWnd)
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
// 		return _T("LISTBOX");
// 	}
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
	
	pub fn new()->CListBox{
		CListBox{
			inner: CWindow::new(NULL_HWND),
		}
	}
	// for entire listbox
	pub fn GetCount (&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETCOUNT, 0, 0) as c_int
	}

//#ifndef _WIN32_WCE
	pub fn SetCount(&self,cItems: c_int)->c_int {
		self.inner.assert_window();
		debug_assert!(((self.inner.GetStyle() & LBS_NODATA) != 0) && ((self.inner.GetStyle() & LBS_HASSTRINGS) == 0));
		self.inner.SendMessage( LB_SETCOUNT, cItems as WPARAM, 0) as c_int
	}
//#endif // !_WIN32_WCE

	pub fn GetHorizontalExtent (&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETHORIZONTALEXTENT, 0, 0) as c_int
	}

	pub fn SetHorizontalExtent(&self,cxExtent: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETHORIZONTALEXTENT, cxExtent as WPARAM, 0);
	}

	pub fn GetTopIndex (&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETTOPINDEX, 0, 0) as c_int
	}

	pub fn SetTopIndex(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETTOPINDEX, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetLocale (&self)->LCID {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETLOCALE, 0, 0) as LCID
	}

	pub fn SetLocale(&self,nNewLocale: LCID)->LCID {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETLOCALE, nNewLocale as WPARAM, 0) as LCID
	}

//#if (WINVER >= 0x0500) && !defined(_WIN32_WCE)
	// Windows XP _WIN32_WINNT>=0x0501
	pub fn GetListBoxInfo (&self)->DWORD {
		self.inner.assert_window();
//#if (_WIN32_WINNT >= 0x0501)
		self.inner.SendMessage( LB_GETLISTBOXINFO, 0, 0) as DWORD
//#else // !(_WIN32_WINNT >= 0x0501)
//		return ::GetListBoxInfo(m_hWnd);
//#endif // !(_WIN32_WINNT >= 0x0501)
	}
//#endif // (WINVER >= 0x0500) && !defined(_WIN32_WCE)

	// for single-selection listboxes
	pub fn GetCurSel (&self)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) == 0);
		self.inner.SendMessage( LB_GETCURSEL, 0, 0) as c_int
	}

	pub fn SetCurSel(&self,nSelect: c_int)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) == 0);
		self.inner.SendMessage( LB_SETCURSEL, nSelect as WPARAM, 0) as c_int
	}

	// for multiple-selection listboxes
	// also works for single-selection
	pub fn GetSel(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETSEL, nIndex as WPARAM, 0) as c_int
	}

	pub fn SetSel (&self,nIndex: c_int, bSelect: BOOL)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		self.inner.SendMessage( LB_SETSEL, bSelect as WPARAM, nIndex as LPARAM) as c_int
	}

	pub fn GetSelCount (&self)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		self.inner.SendMessage( LB_GETSELCOUNT, 0, 0) as c_int
	}

	pub fn GetSelItems (&self,nMaxItems: c_int, rgIndex: LPINT)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		self.inner.SendMessage( LB_GETSELITEMS, nMaxItems as WPARAM, rgIndex as LPARAM) as c_int
	}

	pub fn GetAnchorIndex (&self)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		self.inner.SendMessage( LB_GETANCHORINDEX, 0, 0) as c_int
	}

	pub fn SetAnchorIndex(&self,nIndex: c_int) {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		self.inner.SendMessage( LB_SETANCHORINDEX, nIndex as WPARAM, 0);
	}

	pub fn GetCaretIndex (&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETCARETINDEX, 0, 0) as c_int
	}

	pub fn SetCaretIndex (&self,nIndex: c_int, bScroll: BOOL)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETCARETINDEX, nIndex as WPARAM, MAKELONG(bScroll as WORD, 0) as LPARAM) as c_int
	}

	// for listbox items
	pub fn GetItemData(&self,nIndex: c_int)->DWORD_PTR {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETITEMDATA, nIndex as WPARAM, 0) as DWORD_PTR
	}

	pub fn SetItemData (&self,nIndex: c_int, dwItemData: DWORD_PTR)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETITEMDATA, nIndex as WPARAM, dwItemData as LPARAM) as c_int
	}

	pub fn GetItemDataPtr(&self,nIndex: c_int)->*const c_void {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETITEMDATA, nIndex as WPARAM, 0) as *const c_void
	}

	pub fn SetItemDataPtr (&self,nIndex: c_int, pData: *const c_void)->c_int {
		self.inner.assert_window();
		self.SetItemData(nIndex, pData as DWORD_PTR)
	}

	pub fn GetItemRect (&self,nIndex: c_int, lpRect: LPRECT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETITEMRECT, nIndex as WPARAM, lpRect as LPARAM) as c_int
	}

	pub fn GetText (&self, nIndex: c_int)->String {
		self.inner.assert_window();
		let nLen = self.GetTextLen(nIndex) as usize;
		//println!("{}", nLen);
		let mut buff:Vec<u16> = Vec::with_capacity(nLen+1);
		unsafe{buff.set_len(nLen+1)};
		let nRead = self.inner.SendMessage( LB_GETTEXT, nIndex as WPARAM, buff.as_mut_ptr() as LPARAM);
		//println!("{}", nRead);
		//buff[nRead as usize] = 0;
		String::from_utf16_lossy(&buff[..nRead as usize].as_ref())
	}

// #ifndef _ATL_NO_COM
// #ifdef _OLEAUTO_H_
// 	pub fn GetTextBSTR (c_int nIndex, BSTR& bstrText)->BOOL {
// 		USES_CONVERSION;
// 		self.inner.assert_window();
// 		ATLASSERT(bstrText == NULL);

// 		c_int nLen = GetTextLen(nIndex);
// 		if(nLen == LB_ERR)
// 			return FALSE;

// 		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 		LPTSTR lpstrText = buff.Allocate(nLen + 1);
// 		if(lpstrText == NULL)
// 			return FALSE;

// 		if(GetText(nIndex, lpstrText) == LB_ERR)
// 			return FALSE;

// 		bstrText = ::SysAllocString(T2OLE(lpstrText));
// 		return (bstrText != NULL) ? TRUE : FALSE;
// 	}
// #endif // _OLEAUTO_H_
// #endif // !_ATL_NO_COM

// #if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	pub fn GetText (c_int nIndex, _CSTRING_NS::CString& strText)->c_int {
// 		self.inner.assert_window();
// 		c_int cchLen = GetTextLen(nIndex);
// 		if(cchLen == LB_ERR)
// 			return LB_ERR;
// 		c_int nRet = LB_ERR;
// 		LPTSTR lpstr = strText.GetBufferSetLength(cchLen);
// 		if(lpstr != NULL)
// 		{
// 			nRet = GetText(nIndex, lpstr);
// 			strText.ReleaseBuffer();
// 		}
// 		return nRet;
// 	}
// #endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn GetTextLen(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETTEXTLEN, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetItemHeight(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_GETITEMHEIGHT, nIndex as WPARAM, 0) as c_int
	}

	pub fn SetItemHeight (&self,nIndex: c_int, cyItemHeight: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETITEMHEIGHT, nIndex as WPARAM, MAKELONG(cyItemHeight as WORD, 0) as LPARAM) as c_int
	}

	// Settable only attributes
	pub fn SetColumnWidth(&self,cxWidth: c_int) {
		self.inner.assert_window();
		self.inner.SendMessage( LB_SETCOLUMNWIDTH, cxWidth as WPARAM, 0);
	}

	pub fn SetTabStops3(&self,nTabStops: c_int, rgTabStops: LPINT)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & LBS_USETABSTOPS) != 0);
		self.inner.SendMessage( LB_SETTABSTOPS, nTabStops as WPARAM, rgTabStops as LPARAM) as BOOL
	}

	pub fn SetTabStops (&self)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & LBS_USETABSTOPS) != 0);
		self.inner.SendMessage( LB_SETTABSTOPS, 0, 0) as BOOL
	}

	pub fn SetTabStops2(&self,cxEachStop: &mut c_int)->BOOL {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & LBS_USETABSTOPS) != 0);
		self.inner.SendMessage( LB_SETTABSTOPS, 1, cxEachStop as *mut _ as LPARAM) as BOOL
	}

// Operations
	pub fn InitStorage (&self,nItems: c_int, nBytes: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_INITSTORAGE, nItems as WPARAM, nBytes as LPARAM) as c_int
	}

	pub fn ResetContent (&self) {
		self.inner.assert_window();
		self.inner.SendMessage( LB_RESETCONTENT, 0, 0);
	}

	pub fn ItemFromPoint (&self,pt: POINT, bOutside: &mut BOOL)->UINT {
		self.inner.assert_window();
		let dw = self.inner.SendMessage( LB_ITEMFROMPOINT, 0, MAKELONG(pt.x as WORD, pt.y as WORD) as LPARAM) as DWORD;
		*bOutside = HIWORD(dw) as BOOL;
		LOWORD(dw) as UINT
	}

	// manipulating listbox items
	pub fn AddString(&self,lpszItem: &str)->c_int {
		self.inner.assert_window();
		let s = lpszItem.to_c_u16();
		self.inner.SendMessage( LB_ADDSTRING, 0, s.as_ptr() as LPARAM) as c_int
	}

	pub fn DeleteString(&self,nIndex: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( LB_DELETESTRING, nIndex as WPARAM, 0) as c_int
	}

	pub fn InsertString (&self, nIndex: c_int, lpszItem: &str)->c_int {
		self.inner.assert_window();
		let s = lpszItem.to_c_u16();
		self.inner.SendMessage( LB_INSERTSTRING, nIndex as WPARAM, s.as_ptr() as LPARAM) as c_int
	}

//#ifndef _WIN32_WCE
	pub fn Dir (&self, attr: UINT, lpszWildCard: &str)->c_int {
		self.inner.assert_window();
		let s = lpszWildCard.to_c_u16();
		self.inner.SendMessage( LB_DIR, attr as WPARAM, s.as_ptr() as LPARAM) as c_int
	}

	pub fn AddFile(&self,lpstrFileName: &str)->c_int {
		self.inner.assert_window();
		let s = lpstrFileName.to_c_u16();
		self.inner.SendMessage( LB_ADDFILE, 0, s.as_ptr() as LPARAM) as c_int
	}
//#endif // !_WIN32_WCE

	// selection helpers
	pub fn FindString (&self, nStartAfter: c_int, lpszItem: &str)->c_int {
		self.inner.assert_window();
		let s = lpszItem.to_c_u16();
		self.inner.SendMessage( LB_FINDSTRING, nStartAfter as WPARAM, s.as_ptr() as LPARAM) as c_int
	}

	pub fn FindStringExact (&self, nIndexStart: c_int, lpszFind: &str)->c_int {
		self.inner.assert_window();
		let s = lpszFind.to_c_u16();
		self.inner.SendMessage( LB_FINDSTRINGEXACT, nIndexStart as WPARAM, s.as_ptr() as LPARAM) as c_int
	}

	pub fn SelectString (&self, nStartAfter: c_int, lpszItem: &str)->c_int {
		self.inner.assert_window();
		let s = lpszItem.to_c_u16();
		self.inner.SendMessage( LB_SELECTSTRING, nStartAfter as WPARAM, s.as_ptr() as LPARAM) as c_int
	}

	pub fn SelItemRange (&self,bSelect: BOOL, nFirstItem: c_int, nLastItem: c_int)->c_int {
		self.inner.assert_window();
		debug_assert!((self.inner.GetStyle() & (LBS_MULTIPLESEL | LBS_EXTENDEDSEL)) != 0);
		debug_assert!(nFirstItem <= nLastItem);
		if bSelect == winapi::shared::minwindef::TRUE {
			self.inner.SendMessage( LB_SELITEMRANGEEX, nFirstItem as WPARAM, nLastItem as LPARAM) as c_int
		}else{
			self.inner.SendMessage( LB_SELITEMRANGEEX, nLastItem as WPARAM, nFirstItem as LPARAM) as c_int
		}
	}

// #ifdef WIN32_PLATFORM_WFSP   // SmartPhone only messages
// 	pub fn GetInputMode (BOOL bCurrentMode = winapi::shared::minwindef::TRUE)->DWORD {
// 		return SendMessage(LB_GETINPUTMODE, 0, bCurrentMode as LPARAM);
// 	}

// 	pub fn SetInputMode(&self,dwMode: DWORD)->BOOL {
// 		return SendMessage(LB_SETINPUTMODE, 0, dwMode as LPARAM);
// 	}
// #endif // WIN32_PLATFORM_WFSP
}

//typedef CListBoxT<ATL::CWindow>   CListBox;

/////////////////////////////////////////////////////////
// expose all inner methods
// impl CListBox {
// 	expose_cwindow!();
// }
