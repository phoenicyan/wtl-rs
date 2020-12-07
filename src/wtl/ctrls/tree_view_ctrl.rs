#![allow(non_snake_case, dead_code)]
use atl::CWindow;

use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::ntdef::*;
use winapi::um::commctrl::*;

use std::mem;
use wtl::ToCU16Str;
use std::ops::{Deref,DerefMut};

#[derive(Debug)]
pub struct CTreeViewCtrl {
    inner: CWindow,
}

// template <class TBase>
// class CTreeViewCtrlT : public TBase
// {
// public:
// // Constructors
// 	pub fn TBase(@hWnd,)->CTreeViewCtrlT(HWND hWnd = NULL) : { }

// 	CTreeViewCtrlT< TBase >& operator =(HWND hWnd)
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
// 	pub fn GetWndClassName(&self,)->static LPCTSTR {
// 		return WC_TREEVIEW;
// 	}

impl Deref for CTreeViewCtrl {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl DerefMut for CTreeViewCtrl {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl CTreeViewCtrl {
	#[inline(always)]
    pub fn cwin(&self)->&CWindow {
		&self.inner
	}

	// pub fn cwin_mut(&mut self)->&mut CWindow {
	// 	&mut self.inner
	// }
    // pub fn Attach(&mut self,h: HWND) {
    //     self.inner.Attach(h)
    // }
    
    // pub fn Detach(&mut self)->HWND {
    //     self.inner.Detach()
    // }

	pub fn new()->CTreeViewCtrl{
		CTreeViewCtrl{
			inner: CWindow::new(0 as HWND),
		}
	}
	pub fn GetCount(&self,)->UINT {
		self.inner.SendMessage(TVM_GETCOUNT, 0, 0) as UINT
	}

	pub fn GetIndent(&self,)->UINT {
		self.inner.SendMessage(TVM_GETINDENT, 0, 0) as UINT
	}

	pub fn SetIndent(&self,nIndent: UINT,) {
		self.inner.SendMessage(TVM_SETINDENT, nIndent as WPARAM, 0);
	}

	// pub fn GetImageList(nImageListType: Option<c_int> /*= TVSIL_NORMAL*/,)->CImageList {
	// 	self.inner.assert_window();
	// 	CImageList((HIMAGELIST)self.inner.SendMessage(TVM_GETIMAGELIST, (WPARAM)nImageListType, 0))
	// }

	// pub fn SetImageList(&self,hImageList: HIMAGELIST, nImageListType: Option<c_int> /*= TVSIL_NORMAL*/,)->CImageList {
	// 	self.inner.assert_window();
	// 	CImageList((HIMAGELIST)self.inner.SendMessage(TVM_SETIMAGELIST, (WPARAM)nImageListType, (LPARAM)hImageList))
	// }

	pub fn GetItem(&self,pItem: LPTVITEMW,)->BOOL {
		self.inner.SendMessage(TVM_GETITEMW, 0, pItem as LPARAM) as BOOL
	}

	pub fn SetItem2(&self,pItem: LPTVITEMW,)->BOOL {
		self.inner.SendMessage(TVM_SETITEMW, 0, pItem as LPARAM) as BOOL
	}

	pub fn SetItem(&self,hItem: HTREEITEM, nMask: UINT, lpszItem: Option<&str>, nImage: c_int, nSelectedImage: c_int, nState: UINT, nStateMask: UINT, lParam: LPARAM,)->BOOL {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.hItem = hItem;
		item.mask = nMask;
		//hold the vec for lifetime reason
		let mut tmp: Vec<u16>;
		if let Some(s) = lpszItem {
			tmp = s.to_c_u16();
			item.pszText = tmp.as_mut_ptr();	//mut_ptr: windows will store data to this buffer??
		}else{
			item.pszText = 0 as LPWSTR;
		}
		item.iImage = nImage;
		item.iSelectedImage = nSelectedImage;
		item.state = nState;
		item.stateMask = nStateMask;
		item.lParam = lParam;
		self.inner.SendMessage(TVM_SETITEMW, 0, &item as *const _ as LPARAM) as BOOL
	}

	pub fn GetItemText(&self,hItem: HTREEITEM)->String {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.hItem = hItem;
		item.mask = TVIF_TEXT;

		let mut nLen = 256usize;
		loop{
			let mut v: Vec<u16> = Vec::with_capacity(nLen);
			unsafe{v.set_len(nLen)}
			//v = unsafe{mem::zeroed()};
			v[0] = 0;
			v[nLen - 2] = 0;
			item.pszText = v.as_mut_ptr();
			item.cchTextMax = nLen as c_int;

			let nRet = self.inner.SendMessage(TVM_GETITEMW, 0, &item as *const _ as LPARAM) as BOOL;
			// at least an unused space
			if nRet > 0 || v[nLen - 2] == 0 {
				let mut pos = 0;
				for d in &v {
					if *d == 0 {
						break;
					}
					pos += 1;
				}
				//unsafe{v.set_len(pos+1)};
				return String::from_utf16_lossy(&v[..pos].as_ref());
			}
			nLen *= 2;
		}
	}

	/*
//#ifndef _ATL_NO_COM
	pub fn GetItemText(&self,hItem: HTREEITEM, bstrText: &BSTR,)->BOOL {
		USES_CONVERSION;
		self.inner.assert_window();
		ATLASSERT(bstrText == NULL);
		TVITEM item = { 0 };
		item.hItem = hItem;
		item.mask = TVIF_TEXT;

		LPTSTR lpstrText = NULL;
		BOOL bRet = FALSE;
		for(c_int nLen = 256; ; nLen *= 2)
		{
			ATLTRY(lpstrText = new TCHAR[nLen]);
			if(lpstrText == NULL)
				break;
			lpstrText[0] = NULL;
			item.pszText = lpstrText;
			item.cchTextMax = nLen;
			bRet = (BOOL)self.inner.SendMessage(TVM_GETITEM, 0, (LPARAM)&item);
			if(!bRet || (lstrlen(item.pszText) < nLen - 1))
				break;
			delete [] lpstrText;
			lpstrText = NULL;
		}

		if(lpstrText != NULL)
		{
			if(bRet)
				bstrText = ::SysAllocString(T2OLE(lpstrText));
			delete [] lpstrText;
		}

		return (bstrText != NULL) ? TRUE : FALSE;
	}
//#endif // !_ATL_NO_COM

//#if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
	pub fn GetItemText(&self,hItem: HTREEITEM, @ _CSTRING_NS::CString& strText,)->BOOL {
		self.inner.assert_window();
		TVITEM item = { 0 };
		item.hItem = hItem;
		item.mask = TVIF_TEXT;

		strText.Empty();
		BOOL bRet = FALSE;
		for(c_int nLen = 256; ; nLen *= 2)
		{
			item.pszText = strText.GetBufferSetLength(nLen);
			if(item.pszText == NULL)
			{
				bRet = FALSE;
				break;
			}
			item.cchTextMax = nLen;
			bRet = (BOOL)self.inner.SendMessage(TVM_GETITEM, 0, (LPARAM)&item);
			if(!bRet || (lstrlen(item.pszText) < nLen - 1))
				break;
		}
		strText.ReleaseBuffer();
		bRet
	}
	*/
//#endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn SetItemText(&self,hItem: HTREEITEM, lpszItem: &str,)->BOOL {
		self.SetItem(hItem, TVIF_TEXT, Some(lpszItem), 0, 0, 0, 0, 0 as LPARAM)
	}

	pub fn GetItemImage(&self,hItem: HTREEITEM, nImage: &mut c_int,nSelectedImage: &mut c_int,)->BOOL {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.hItem = hItem;
		item.mask = TVIF_IMAGE|TVIF_SELECTEDIMAGE;
		let bRes = self.inner.SendMessage(TVM_GETITEMW, 0, &item as *const _ as LPARAM) as BOOL;
		if bRes > 0	{
			*nImage = item.iImage;
			*nSelectedImage = item.iSelectedImage;
		}
		bRes
	}

	pub fn SetItemImage(&self,hItem: HTREEITEM, nImage: c_int, nSelectedImage: c_int,)->BOOL {
		self.SetItem(hItem, TVIF_IMAGE|TVIF_SELECTEDIMAGE, None, nImage, nSelectedImage, 0, 0, 0 as LPARAM)
	}

	pub fn GetItemState(&self,hItem: HTREEITEM, nStateMask: UINT,)->UINT {
//#if (_WIN32_IE >= 0x0500) && !defined(_WIN32_WCE)
		(self.inner.SendMessage(TVM_GETITEMSTATE, hItem as WPARAM, nStateMask as LPARAM) as UINT) & nStateMask
//#else // !((_WIN32_IE >= 0x0500) && !defined(_WIN32_WCE))
		// let mut item: TVITEMW = mem::uninitialized();
		// item.hItem = hItem;
		// item.mask = TVIF_STATE;
		// item.state = 0;
		// item.stateMask = nStateMask;
		// self.inner.SendMessage(TVM_GETITEM, 0, (LPARAM)&item);
		// (item.state & nStateMask)
//#endif // !((_WIN32_IE >= 0x0500) && !defined(_WIN32_WCE))
	}

	pub fn SetItemState(&self,hItem: HTREEITEM, nState: UINT, nStateMask: UINT,)->BOOL {
		self.SetItem(hItem, TVIF_STATE, None, 0, 0, nState, nStateMask, 0 as LPARAM)
	}

	pub fn GetItemData(&self,hItem: HTREEITEM,)->DWORD_PTR {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.hItem = hItem;
		item.mask = TVIF_PARAM;
		let bRet = self.inner.SendMessage(TVM_GETITEMW, 0, &item as *const _ as LPARAM) as BOOL;
		//return (DWORD_PTR)(bRet ?  : NULL);
		if bRet > 0 {
			item.lParam as DWORD_PTR
		}else{
			0
		}
	}

	pub fn SetItemData(&self,hItem: HTREEITEM, dwData: DWORD_PTR,)->BOOL {
		self.SetItem(hItem, TVIF_PARAM, None, 0, 0, 0, 0, dwData as LPARAM)
	}

	// pub fn GetEditControl(&self,)->CEdit {
	// 	CEdit(self.inner.SendMessage(TVM_GETEDITCONTROL, 0, 0) as HWND)
	// }

	pub fn GetVisibleCount(&self,)->UINT {
		self.inner.SendMessage(TVM_GETVISIBLECOUNT, 0, 0) as UINT
	}

	pub fn GetItemRect(&self,hItem: HTREEITEM, lpRect: LPRECT, bTextOnly: BOOL,)->BOOL {
		unsafe{*(lpRect as *mut HTREEITEM) = hItem};
		//*(HTREEITEM*)lpRect = hItem;
		self.inner.SendMessage(TVM_GETITEMRECT, bTextOnly as WPARAM, lpRect as LPARAM) as BOOL
	}

	pub fn ItemHasChildren(&self,hItem: HTREEITEM,)->BOOL {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.hItem = hItem;
		item.mask = TVIF_CHILDREN;
		self.inner.SendMessage(TVM_GETITEMW, 0, &item as *const _ as LPARAM);
		item.cChildren
	}

//#ifndef _WIN32_WCE
	// pub fn GetToolTips(&self,)->CToolTipCtrl {
	// 	CToolTipCtrl((HWND)self.inner.SendMessage(TVM_GETTOOLTIPS, 0, 0))
	// }

	// pub fn SetToolTips(&self,hWndTT: HWND,)->CToolTipCtrl {
	// 	CToolTipCtrl((HWND)self.inner.SendMessage(TVM_SETTOOLTIPS, hWndTT as WPARAM, 0))
	// }
//#endif // !_WIN32_WCE

	// pub fn GetISearchString(&self,lpstr: LPTSTR,)->c_int {
	// 	self.inner.SendMessage(TVM_GETISEARCHSTRING, 0, lpstr as LPARAM) as c_int
	// }

	// checkboxes only
	pub fn GetCheckState(&self,hItem: HTREEITEM,)->BOOL {
		debug_assert!((self.inner.GetStyle() & TVS_CHECKBOXES) != 0);
		let uRet = self.GetItemState(hItem, TVIS_STATEIMAGEMASK) as UINT;
		((uRet >> 12) - 1) as BOOL
	}

	pub fn SetCheckState(&self,hItem: HTREEITEM, bCheck: BOOL,)->BOOL {
		let nCheck = if bCheck > 0 { 2 } else { 1 };   // one based index
		self.SetItemState(hItem, INDEXTOSTATEIMAGEMASK(nCheck), TVIS_STATEIMAGEMASK)
	}

//#if (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)
	pub fn GetBkColor(&self,)->COLORREF {
		self.inner.SendMessage(TVM_GETBKCOLOR, 0, 0) as COLORREF
	}

	pub fn SetBkColor(&self,clr: COLORREF,)->COLORREF {
		self.inner.SendMessage(TVM_SETBKCOLOR, 0, clr as LPARAM) as COLORREF
	}

	pub fn GetInsertMarkColor(&self,)->COLORREF {
		self.inner.SendMessage(TVM_GETINSERTMARKCOLOR, 0, 0) as COLORREF
	}

	pub fn SetInsertMarkColor(&self,clr: COLORREF,)->COLORREF {
		self.inner.SendMessage(TVM_SETINSERTMARKCOLOR, 0, clr as LPARAM) as COLORREF
	}

	pub fn GetItemHeight(&self,)->c_int {
		self.inner.SendMessage(TVM_GETITEMHEIGHT, 0, 0) as c_int
	}

	pub fn SetItemHeight(&self,cyHeight: c_int,)->c_int {
		self.inner.SendMessage(TVM_SETITEMHEIGHT, cyHeight as WPARAM, 0) as c_int
	}

	pub fn GetScrollTime(&self,)->c_int {
		self.inner.SendMessage(TVM_GETSCROLLTIME, 0, 0) as c_int
	}

	pub fn SetScrollTime(&self,nScrollTime: c_int,)->c_int {
		self.inner.SendMessage(TVM_SETSCROLLTIME, nScrollTime as WPARAM, 0) as c_int
	}

	pub fn GetTextColor(&self,)->COLORREF {
		self.inner.SendMessage(TVM_GETTEXTCOLOR, 0, 0) as COLORREF
	}

	pub fn SetTextColor(&self,clr: COLORREF,)->COLORREF {
		self.inner.SendMessage(TVM_SETTEXTCOLOR, 0, clr as LPARAM) as COLORREF
	}

	pub fn GetUnicodeFormat(&self,)->BOOL {
		self.inner.SendMessage(TVM_GETUNICODEFORMAT, 0, 0) as BOOL
	}

	pub fn SetUnicodeFormat(&self,bUnicode: Option<BOOL> /*= TRUE*/,)->BOOL {
		let b = if let Some(u) = bUnicode {u}else{winapi::shared::minwindef::TRUE};
		self.inner.SendMessage(TVM_SETUNICODEFORMAT, b as WPARAM, 0) as BOOL
	}
//#endif // (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)

//#if (_WIN32_IE >= 0x0500) && !defined(_WIN32_WCE)
	pub fn GetLineColor(&self,)->COLORREF {
		self.inner.SendMessage(TVM_GETLINECOLOR, 0, 0) as COLORREF
	}

	pub fn SetLineColor(&self, clrNew: COLORREF /*= CLR_DEFAULT*/,)->COLORREF {
		self.inner.SendMessage(TVM_SETLINECOLOR, 0, clrNew as LPARAM) as COLORREF
	}
//#endif // (_WIN32_IE >= 0x0500) && !defined(_WIN32_WCE)

//#if (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)
	pub fn GetItemEx(&self,pItem: LPTVITEMEXW,)->BOOL {
		self.inner.SendMessage(TVM_GETITEMW, 0, pItem as LPARAM) as BOOL
	}

	pub fn SetItemEx(&self,pItem: LPTVITEMEXW,)->BOOL {
		self.inner.SendMessage(TVM_SETITEMW, 0, pItem as LPARAM) as BOOL
	}
//#endif // (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)

	pub fn GetExtendedStyle(&self,)->DWORD {
//#ifndef TVM_GETEXTENDEDSTYLE
		const TVM_GETEXTENDEDSTYLE: UINT = TV_FIRST + 45;
//#endif
		self.inner.SendMessage(TVM_GETEXTENDEDSTYLE, 0, 0) as DWORD
	}

	pub fn SetExtendedStyle(&self,dwStyle: DWORD, dwMask: DWORD,)->DWORD {
//#ifndef TVM_SETEXTENDEDSTYLE
		const TVM_SETEXTENDEDSTYLE: UINT = TV_FIRST + 44;
//#endif
		self.inner.SendMessage(TVM_SETEXTENDEDSTYLE, dwMask as WPARAM, dwStyle  as LPARAM) as DWORD
	}

//#if (_WIN32_WINNT >= 0x0600)
	pub fn SetAutoScrollInfo(&self,uPixPerSec: UINT, uUpdateTime: UINT,)->BOOL {
		self.inner.SendMessage(TVM_SETAUTOSCROLLINFO, uPixPerSec as WPARAM, uUpdateTime as LPARAM) as BOOL
	}

	pub fn GetSelectedCount(&self,)->DWORD {
		self.inner.SendMessage(TVM_GETSELECTEDCOUNT, 0, 0) as DWORD
	}

	pub fn GetItemPartRect(&self,hItem: HTREEITEM, partID: TVITEMPART, lpRect: LPRECT,)->BOOL {
		let gipri = TVGETITEMPARTRECTINFO{ hti: hItem, prc: lpRect, partID: partID };
		self.inner.SendMessage(TVM_GETITEMPARTRECT, 0, &gipri as *const _ as LPARAM) as BOOL
	}
//#endif // (_WIN32_WINNT >= 0x0600)

// Operations
	pub fn InsertItem2(&self,lpInsertStruct: LPTVINSERTSTRUCTW,)->HTREEITEM {
		self.inner.SendMessage(TVM_INSERTITEMW, 0, lpInsertStruct as LPARAM) as HTREEITEM
	}

	pub fn InsertItem3(&self,lpszItem: &str, nImage: c_int, nSelectedImage: c_int, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->HTREEITEM {
		self.InsertItem(TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE, Some(lpszItem), nImage, nSelectedImage, 0, 0, 0, hParent, hInsertAfter) 
	}

	pub fn InsertItem4(&self,lpszItem: &str, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->HTREEITEM {
		self.InsertItem(TVIF_TEXT, Some(lpszItem), 0, 0, 0, 0, 0, hParent, hInsertAfter)
	}

	pub fn InsertItem(&self,nMask: UINT, lpszItem: Option<&str>, nImage: c_int, nSelectedImage: c_int, nState: UINT, nStateMask: UINT, lParam: LPARAM, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->HTREEITEM {
		let mut tvis: TVINSERTSTRUCTW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		tvis.hParent = hParent;
		tvis.hInsertAfter = hInsertAfter;
		unsafe {
			tvis.u.itemex_mut().mask = nMask;

			let mut tmp: Vec<u16>;
			if let Some(s) = lpszItem {
				tmp = s.to_c_u16();
				tvis.u.itemex_mut().pszText = tmp.as_mut_ptr();
			} else {
				tvis.u.itemex_mut().pszText = 0 as LPWSTR;
			}
			tvis.u.itemex_mut().iImage = nImage;
			tvis.u.itemex_mut().iSelectedImage = nSelectedImage;
			tvis.u.itemex_mut().state = nState;
			tvis.u.itemex_mut().stateMask = nStateMask;
			tvis.u.itemex_mut().lParam = lParam;
		}
		self.inner.SendMessage(TVM_INSERTITEMW, 0, &tvis as *const _ as LPARAM) as HTREEITEM
	}

	pub fn DeleteItem(&self,hItem: HTREEITEM,)->BOOL {
		self.inner.SendMessage(TVM_DELETEITEM, 0, hItem as LPARAM) as BOOL
	}

	pub fn DeleteAllItems(&self,)->BOOL {
		self.inner.SendMessage(TVM_DELETEITEM, 0, TVI_ROOT as LPARAM) as BOOL
	}

	pub fn Expand(&self,hItem: HTREEITEM, nCode: Option<UINT> /*= TVE_EXPAND*/,)->BOOL {
		let n = if let Some(n1) = nCode{n1} else{TVE_EXPAND as UINT};
		self.inner.SendMessage(TVM_EXPAND, n as WPARAM, hItem as LPARAM) as BOOL
	}

	pub fn GetNextItem(&self,hItem: HTREEITEM, nCode: UINT,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, nCode as WPARAM, hItem as LPARAM) as HTREEITEM
	}

	pub fn GetChildItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_CHILD, hItem as LPARAM) as HTREEITEM
	}

	pub fn GetNextSiblingItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_NEXT, hItem as LPARAM) as HTREEITEM 
	}

	pub fn GetPrevSiblingItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_PREVIOUS, hItem as LPARAM) as HTREEITEM
	}

	pub fn GetParentItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_PARENT, hItem as LPARAM) as HTREEITEM 
	}

	pub fn GetFirstVisibleItem(&self,)->HTREEITEM {
		 
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_FIRSTVISIBLE, 0) as HTREEITEM
	}

	pub fn GetNextVisibleItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_NEXTVISIBLE, hItem as LPARAM) as HTREEITEM
	}

	pub fn GetPrevVisibleItem(&self,hItem: HTREEITEM,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_PREVIOUSVISIBLE, hItem as LPARAM) as HTREEITEM
	}

	pub fn GetSelectedItem(&self,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_CARET, 0) as HTREEITEM
	}

	pub fn GetDropHilightItem(&self,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_DROPHILITE, 0) as HTREEITEM
	}

	pub fn GetRootItem(&self,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_ROOT, 0) as HTREEITEM
	}

//#if !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)
	pub fn GetLastVisibleItem(&self,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_LASTVISIBLE, 0) as HTREEITEM
	}
//#endif // !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)

//#if (_WIN32_IE >= 0x0600)
	pub fn GetNextSelectedItem(&self,)->HTREEITEM {
		self.inner.SendMessage(TVM_GETNEXTITEM, TVGN_NEXTSELECTED, 0) as HTREEITEM
	}
//#endif // (_WIN32_IE >= 0x0600)

	pub fn Select(&self,hItem: HTREEITEM, nCode: UINT,)->BOOL {
		self.inner.SendMessage(TVM_SELECTITEM, nCode as WPARAM, hItem as LPARAM) as BOOL
	}

	pub fn SelectItem(&self,hItem: HTREEITEM,)->BOOL {
		self.inner.SendMessage(TVM_SELECTITEM, TVGN_CARET, hItem as LPARAM) as BOOL
	}

	pub fn SelectDropTarget(&self,hItem: HTREEITEM,)->BOOL {
		self.inner.SendMessage(TVM_SELECTITEM, TVGN_DROPHILITE, hItem as LPARAM) as BOOL
	}

	pub fn SelectSetFirstVisible(&self,hItem: HTREEITEM,)->BOOL {
		self.inner.SendMessage(TVM_SELECTITEM, TVGN_FIRSTVISIBLE, hItem as LPARAM) as BOOL
	}

	// pub fn EditLabel(&self,hItem: HTREEITEM,)->CEdit {
	// 	CEdit(self.inner.SendMessage(TVM_EDITLABEL, 0, hItem as LPARAM) as HWND)
	// }

	pub fn EndEditLabelNow(&self,bCancel: BOOL,)->BOOL {
		self.inner.SendMessage(TVM_ENDEDITLABELNOW, bCancel as WPARAM, 0) as BOOL
	}

	pub fn HitTest(&self,pHitTestInfo: *mut TVHITTESTINFO,)->HTREEITEM {
		self.inner.SendMessage(TVM_HITTEST, 0, pHitTestInfo as LPARAM) as HTREEITEM
	}

	pub fn HitTest_point(&self,pt: POINT, pFlags: Option<&mut UINT>,)->HTREEITEM {
		let mut hti: TVHITTESTINFO = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		hti.pt = pt;
		let hTreeItem = self.inner.SendMessage(TVM_HITTEST, 0, &hti as *const _ as LPARAM) as HTREEITEM;
		if let Some(p) = pFlags {
			*p = hti.flags;
		}
		hTreeItem
	}

	pub fn SortChildren(&self,hItem: HTREEITEM, bRecurse: Option<BOOL> /*= FALSE*/,)->BOOL {
		let b = if let Some(b1) = bRecurse {b1} else{ winapi::shared::minwindef::FALSE };
		self.inner.SendMessage(TVM_SORTCHILDREN, b as WPARAM, hItem as LPARAM) as BOOL
	}

	pub fn EnsureVisible(&self,hItem: HTREEITEM,)->BOOL {
		self.inner.SendMessage(TVM_ENSUREVISIBLE, 0, hItem as LPARAM) as BOOL
	}

	pub fn SortChildrenCB(&self,pSort: LPTVSORTCB, bRecurse: Option<BOOL> /*= FALSE*/,)->BOOL {
		let b = if let Some(b1) = bRecurse {b1} else{ winapi::shared::minwindef::FALSE };
		self.inner.SendMessage(TVM_SORTCHILDRENCB, b as WPARAM, pSort as LPARAM) as BOOL
	}

	// pub fn RemoveImageList(&self,nImageList: c_int,)->CImageList {
	// 	CImageList(self.inner.SendMessage(TVM_SETIMAGELIST, nImageList as WPARAM, NULL) as HIMAGELIST)
	// }

	// pub fn CreateDragImage(&self,hItem: HTREEITEM,)->CImageList {
	// 	CImageList(self.inner.SendMessage(TVM_CREATEDRAGIMAGE, 0, hItem as LPARAM) as HIMAGELIST)
	// }

//#if (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)
	pub fn SetInsertMark(&self,hTreeItem: HTREEITEM, bAfter: BOOL,)->BOOL {
		self.inner.SendMessage(TVM_SETINSERTMARK, bAfter as WPARAM, hTreeItem as LPARAM) as BOOL
	}

	pub fn RemoveInsertMark(&self,)->BOOL {
		self.inner.SendMessage(TVM_SETINSERTMARK, 0, 0) as BOOL
	}
//#endif // (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)

//#if (_WIN32_WINNT >= 0x0501)
	pub fn MapAccIDToHTREEITEM(&self,uID: UINT,)->HTREEITEM {
		self.inner.SendMessage(TVM_MAPACCIDTOHTREEITEM, uID as WPARAM, 0) as HTREEITEM
	}

	pub fn MapHTREEITEMToAccID(&self,hTreeItem: HTREEITEM,)->UINT {
		self.inner.SendMessage(TVM_MAPHTREEITEMTOACCID, hTreeItem as WPARAM, 0) as UINT
	}
//#endif // (_WIN32_WINNT >= 0x0501)

//#if (_WIN32_WINNT >= 0x0600)
	pub fn ShowInfoTip(&self,hItem: HTREEITEM,) {
		self.inner.SendMessage(TVM_SHOWINFOTIP, 0, hItem as LPARAM);
	}
//#endif // (_WIN32_WINNT >= 0x0600)
}

//typedef CTreeViewCtrlT<ATL::CWindow>   CTreeViewCtrl;



//typedef CTreeViewCtrlExT<ATL::CWindow>   CTreeViewCtrlEx;


