#![allow(non_snake_case,dead_code,unused_variables)]

use atl::{CWindow,NULL_HWND};
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::ntdef::*;
use winapi::um::commctrl::*;
use winapi::um::winuser::*;
use std::ops::{Deref,DerefMut};

pub struct CComboBox {
    inner: CWindow,
}

impl Deref for CComboBox {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CComboBox {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CComboBox {
    pub fn new()->CComboBox{
        CComboBox{
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
) const
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



impl CComboBox{
// Constructors
// 	CComboBoxT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CComboBoxT< TBase >& operator =(HWND hWnd)
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
// 		return _T("COMBOBOX");
// 	}

	// for entire combo box
	pub fn GetCount(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETCOUNT, 0, 0) as c_int
	}

	pub fn GetCurSel(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETCURSEL, 0, 0) as c_int
	}

	pub fn SetCurSel(&self,nSelect: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETCURSEL, nSelect as WPARAM, 0) as c_int
	}

	pub fn GetLocale(&self)->LCID {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETLOCALE, 0, 0) as LCID
	}

	pub fn SetLocale(&self,nNewLocale: LCID)->LCID {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETLOCALE, nNewLocale as WPARAM, 0) as LCID
	}

	pub fn GetTopIndex(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETTOPINDEX, 0, 0) as c_int
	}

	pub fn SetTopIndex(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETTOPINDEX, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetHorizontalExtent(&self)->UINT {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETHORIZONTALEXTENT, 0, 0) as UINT
	}

	pub fn SetHorizontalExtent(&self,nExtent: UINT) {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETHORIZONTALEXTENT, nExtent as WPARAM, 0);
	}

	pub fn GetDroppedWidth(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETDROPPEDWIDTH, 0, 0) as c_int
	}

	pub fn SetDroppedWidth(&self,nWidth: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETDROPPEDWIDTH, nWidth as WPARAM, 0) as c_int
	}

//#if ((WINVER >= 0x0500) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
	pub fn GetComboBoxInfo(&self,pComboBoxInfo: PCOMBOBOXINFO)->BOOL {
		self.inner.assert_window();
//#if ((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
		self.inner.SendMessage( CB_GETCOMBOBOXINFO, 0, pComboBoxInfo as LPARAM) as BOOL
//#else // !((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
//		return ::GetComboBoxInfo(m_hWnd, pComboBoxInfo);
//#endif // !((_WIN32_WINNT >= 0x0501) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))
	}
//#endif // ((WINVER >= 0x0500) && !defined(_WIN32_WCE)) || (defined(_WIN32_WCE) && (_WIN32_WCE >= 420))

	// for edit control
	pub fn GetEditSel(&self)->DWORD {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETEDITSEL, 0, 0) as DWORD
	}

	pub fn SetEditSel(&self,nStartChar: c_int, nEndChar: c_int)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETEDITSEL, 0, MAKELONG(nStartChar as WORD, nEndChar as WORD) as LPARAM) as BOOL
	}

	// for combobox item
	pub fn GetItemData(&self,nIndex: c_int)->DWORD_PTR {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETITEMDATA, nIndex as WPARAM, 0) as DWORD_PTR
	}

	pub fn SetItemData(&self,nIndex: c_int, dwItemData: DWORD_PTR)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETITEMDATA, nIndex as WPARAM, dwItemData as LPARAM) as c_int
	}

	pub fn GetItemDataPtr(&self,nIndex: c_int)->*const c_void {
		self.inner.assert_window();
		self.GetItemData(nIndex) as *const c_void
	}

	pub fn SetItemDataPtr (&self, nIndex: c_int, pData: *const c_void)->c_int {
		self.inner.assert_window();
		self.SetItemData(nIndex, pData as DWORD_PTR)
	}

	// pub fn GetLBText(&self,nIndex: c_int, lpszText: LPTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_GETLBTEXT, nIndex, lpszText as LPARAM) as c_int
	// }

// #ifndef _ATL_NO_COM
// 	pub fn GetLBTextBSTR (c_int nIndex, BSTR& bstrText)->BOOL {
// 		USES_CONVERSION;
// 		self.inner.assert_window();
// 		ATLASSERT(bstrText == NULL);

// 		c_int nLen = GetLBTextLen(nIndex);
// 		if(nLen == CB_ERR)
// 			return FALSE;

// 		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 		LPTSTR lpstrText = buff.Allocate(nLen + 1);
// 		if(lpstrText == NULL)
// 			return FALSE;

// 		if(GetLBText(nIndex, lpstrText) == CB_ERR)
// 			return FALSE;

// 		bstrText = ::SysAllocString(T2OLE(lpstrText));
// 		return (bstrText != NULL) ? TRUE : FALSE;
// 	}
// #endif // !_ATL_NO_COM

// #if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	pub fn GetLBText (c_int nIndex, _CSTRING_NS::CString& strText)->c_int {
// 		self.inner.assert_window();
// 		c_int cchLen = GetLBTextLen(nIndex);
// 		if(cchLen == CB_ERR)
// 			return CB_ERR;
// 		c_int nRet = CB_ERR;
// 		LPTSTR lpstr = strText.GetBufferSetLength(cchLen);
// 		if(lpstr != NULL)
// 		{
// 			nRet = GetLBText(nIndex, lpstr);
// 			strText.ReleaseBuffer();
// 		}
// 		return nRet;
// 	}
// #endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn GetLBTextLen(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETLBTEXTLEN, nIndex as WPARAM, 0) as c_int
	}

	pub fn GetItemHeight(&self,nIndex: c_int)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETITEMHEIGHT, nIndex as WPARAM, 0) as c_int
	}

	pub fn SetItemHeight(&self,nIndex: c_int, cyItemHeight: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETITEMHEIGHT, nIndex as WPARAM, MAKELONG(cyItemHeight as WORD, 0)  as LPARAM) as c_int
	}

	pub fn GetExtendedUI(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETEXTENDEDUI, 0, 0) as BOOL
	}

	pub fn SetExtendedUI (&self,bExtended: BOOL)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETEXTENDEDUI, bExtended as WPARAM, 0) as c_int
	}

	pub fn GetDroppedControlRect(&self,lprect: LPRECT) {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETDROPPEDCONTROLRECT, 0, lprect as LPARAM);
	}

	pub fn GetDroppedState(&self)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETDROPPEDSTATE, 0, 0) as BOOL
	}

//#if (_WIN32_WINNT >= 0x0501)
	pub fn GetMinVisible(&self)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETMINVISIBLE, 0, 0) as c_int
	}

	pub fn SetMinVisible(&self,nMinVisible: c_int)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETMINVISIBLE, nMinVisible as WPARAM, 0) as BOOL
	}

	// Vista only
	pub fn GetCueBannerText(&self,lpwText: LPWSTR, cchText: c_int)->BOOL {
//#ifndef CB_GETCUEBANNER
		const CB_GETCUEBANNER:UINT = CBM_FIRST + 4;
//#endif
		self.inner.assert_window();
		self.inner.SendMessage( CB_GETCUEBANNER, lpwText as WPARAM, cchText as LPARAM) as BOOL
	}

	// Vista only
	pub fn SetCueBannerText(&self,lpcwText: LPCWSTR)->BOOL {
//#ifndef CB_SETCUEBANNER
		const CB_SETCUEBANNER:UINT = CBM_FIRST + 3;
//#endif
		self.inner.assert_window();
		self.inner.SendMessage( CB_SETCUEBANNER, 0, lpcwText as LPARAM) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0501)

// Operations
	pub fn InitStorage(&self,nItems: c_int, nBytes: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_INITSTORAGE, nItems as WPARAM, nBytes as LPARAM) as c_int
	}

	pub fn ResetContent(&self) {
		self.inner.assert_window();
		self.inner.SendMessage( CB_RESETCONTENT, 0, 0);
	}

	// for edit control
	pub fn LimitText(&self,nMaxChars: c_int)->BOOL {
		self.inner.assert_window();
		self.inner.SendMessage( CB_LIMITTEXT, nMaxChars as WPARAM, 0) as BOOL
	}

	// for drop-down combo boxes
	pub fn ShowDropDown (&self, bShowIt: BOOL) {
		self.inner.assert_window();
		self.inner.SendMessage( CB_SHOWDROPDOWN, bShowIt as WPARAM, 0);
	}

	// manipulating listbox items
	// pub fn AddString(&self,lpszString: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_ADDSTRING, 0, lpszString as LPARAM) as c_int
	// }

	pub fn DeleteString(&self,nIndex: UINT)->c_int {
		self.inner.assert_window();
		self.inner.SendMessage( CB_DELETESTRING, nIndex as WPARAM, 0) as c_int
	}

	// pub fn InsertString(&self,nIndex: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_INSERTSTRING, nIndex, lpszString as LPARAM) as c_int
	// }

//#ifndef _WIN32_WCE
	// pub fn Dir(&self,attr: UINT, lpszWildCard: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_DIR, attr, lpszWildCard as LPARAM) as c_int
	// }
//#endif // !_WIN32_WCE

	// selection helpers
	// pub fn FindString(&self,nStartAfter: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_FINDSTRING, nStartAfter, lpszString as LPARAM) as c_int
	// }

	// pub fn FindStringExact(&self,nIndexStart: c_int, lpszFind: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_FINDSTRINGEXACT, nIndexStart, lpszFind as LPARAM) as c_int
	// }

	// pub fn SelectString(&self,nStartAfter: c_int, lpszString: LPCTSTR)->c_int {
	// 	self.inner.assert_window();
	// 	self.inner.SendMessage( CB_SELECTSTRING, nStartAfter, lpszString as LPARAM) as c_int
	// }

	// Clipboard operations
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
}

//typedef CComboBoxT<ATL::CWindow>   CComboBox;


/////////////////////////////////////////////////////////
// expose all inner methods
// impl CComboBox {
// 	expose_cwindow!();
// }
