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
use super::CTreeViewCtrl;
use std::ops::{Deref,DerefMut};

#[derive(Debug)]
pub struct CTreeViewCtrlEx {
	base: CTreeViewCtrl,
	//pub cwin(): CWindow,
}
// class CTreeViewCtrlExT : public CTreeViewCtrlT< TBase >
// {
// public:
// // Constructors
// 	CTreeViewCtrlExT(HWND hWnd = NULL) : CTreeViewCtrlT< TBase >(hWnd)
// 	{ }

// 	CTreeViewCtrlExT< TBase >& operator =(HWND hWnd)
// 	{
// 		m_hWnd = hWnd;
// 		return *this;
// 	}

impl Deref for CTreeViewCtrlEx {
    type Target = CTreeViewCtrl;
    fn deref<'a>(&'a self)->&'a CTreeViewCtrl {
        &self.base
    }
}

//impl this for Attach and Detach
impl DerefMut for CTreeViewCtrlEx {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CTreeViewCtrl {
        &mut self.base
    }
}

impl CTreeViewCtrlEx {
	pub fn new()->CTreeViewCtrlEx{
		CTreeViewCtrlEx{
			base: CTreeViewCtrl::new(),
		}
	}
	
    pub fn cwin(&self)->&CWindow {
		self.base.cwin()
	}

// Operations (overides that return CTreeItem)
	pub fn InsertItem2(&self,lpInsertStruct: LPTVINSERTSTRUCTW,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_INSERTITEMW, 0, lpInsertStruct as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn InsertItem3(&self,lpszItem: &str, nImage: c_int, nSelectedImage: c_int, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->CTreeItem {
		self.InsertItem(TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE, Some(lpszItem), nImage, nSelectedImage, 0, 0, 0, hParent, hInsertAfter) 
	}

	pub fn InsertItem4(&self,lpszItem: &str, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->CTreeItem {
		self.InsertItem(TVIF_TEXT, Some(lpszItem), 0, 0, 0, 0, 0, hParent, hInsertAfter)
	}

	pub fn GetNextItem(&self,hItem: HTREEITEM, nCode: UINT,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, nCode as WPARAM, hItem as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetChildItem(&self,hItem: HTREEITEM,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_CHILD, hItem as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self) 
	}

	pub fn GetNextSiblingItem(&self,hItem: HTREEITEM,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_NEXT, hItem as LPARAM) as HTREEITEM; 
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetPrevSiblingItem(&self,hItem: HTREEITEM,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_PREVIOUS, hItem as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetParentItem(&self,hItem: HTREEITEM,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_PARENT, hItem as LPARAM) as HTREEITEM; 
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetFirstVisibleItem(&self,)->CTreeItem {
		 
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_FIRSTVISIBLE, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetNextVisibleItem(&self,hItem: HTREEITEM,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_NEXTVISIBLE, hItem as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetPrevVisibleItem(&self,hItem: HTREEITEM,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_PREVIOUSVISIBLE, hItem as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetSelectedItem(&self,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_CARET, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetDropHilightItem(&self,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_DROPHILITE, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn GetRootItem(&self,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_ROOT, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

//#if !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)
	pub fn GetLastVisibleItem(&self,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_LASTVISIBLE, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}
//#endif // !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)

//#if (_WIN32_IE >= 0x0600)
	pub fn GetNextSelectedItem(&self,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_GETNEXTITEM, TVGN_NEXTSELECTED, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}
//#endif // (_WIN32_IE >= 0x0600)

	pub fn HitTest(&self,pHitTestInfo: *mut TVHITTESTINFO,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_HITTEST, 0, pHitTestInfo as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn InsertItem(&self,nMask: UINT, lpszItem: Option<&str>, nImage: c_int, nSelectedImage: c_int, nState: UINT, nStateMask: UINT, lParam: LPARAM, hParent: HTREEITEM, hInsertAfter: HTREEITEM,)->CTreeItem {
		let mut tvis: TVINSERTSTRUCTW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		tvis.hParent = hParent;
		tvis.hInsertAfter = hInsertAfter;
		unsafe {
			tvis.u.itemex_mut().mask = nMask;
			//hold the vec for lifetime reason
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
		let hTreeItem = self.base.cwin().SendMessage(TVM_INSERTITEMW, 0, &tvis as *const _ as LPARAM) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}

	pub fn HitTest_point(&self,pt: POINT, pFlags: Option<&mut UINT>,)->CTreeItem {
		let mut hti: TVHITTESTINFO = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		hti.pt = pt;
		let hTreeItem = self.base.cwin().SendMessage(TVM_HITTEST, 0, &hti as *const _ as LPARAM) as HTREEITEM;
		if let Some(p) = pFlags {
			*p = hti.flags;
		}
		CTreeItem::new(hTreeItem, self)
	}

//#if (_WIN32_WINNT >= 0x0501)
	pub fn MapAccIDToHTREEITEM(&self,uID: UINT,)->CTreeItem {
		let hTreeItem = self.base.cwin().SendMessage(TVM_MAPACCIDTOHTREEITEM, uID as WPARAM, 0) as HTREEITEM;
		CTreeItem::new(hTreeItem, self)
	}


	//expose functions of base
	#[inline(always)]
	pub fn DeleteAllItems(&self) {
		self.base.DeleteAllItems();
	}
//#endif // (_WIN32_WINNT >= 0x0501)
}

// Note: TBase here is for CTreeViewCtrlExT, and not for CTreeItemT itself
#[derive(Debug)]
pub struct CTreeItem<'a> {
    hItem: HTREEITEM,
    pView: &'a CTreeViewCtrlEx,
}

// class CTreeItemT
// {
// public:
// 	HTREEITEM m_hTreeItem;
// 	CTreeViewCtrlExT<TBase>* m_pTreeView;

impl<'a> CTreeItem<'a> {
// Construction
	//pub fn m_hTreeItem(@hTreeItem,),@ m_pTreeView(pTreeView,@)->CTreeItemT(HTREEITEM hTreeItem = NULL,$ CTreeViewCtrlExT<TBase>* pTreeView = NULL) : { }
 
 	pub fn new(h: HTREEITEM, v: &'a CTreeViewCtrlEx) -> CTreeItem<'a> {
 		CTreeItem {
 			hItem: h,
 			pView: v,
 		}
 	}
// 	CTreeItemT(const CTreeItemT<TBase>& posSrc)
// 	{
// 		*this = posSrc;
// 	}

// 	pub fn HTREEITEM(&self,)->operator { return m_hTreeItem; }

// 	CTreeItemT<TBase>& operator =(const CTreeItemT<TBase>& itemSrc)
// 	{
// 		m_hTreeItem = itemSrc.m_hTreeItem;
// 		m_pTreeView = itemSrc.m_pTreeView;
// 		return *this;
// 	}

// // Attributes
// 	pub fn GetTreeView(&self,)->CTreeViewCtrlExT<TBase>* { return m_pTreeView; }

// 	BOOL operator !() { return m_hTreeItem == NULL; }

 	pub fn IsNull(&self)->bool {
 		return self.hItem == 0 as HTREEITEM 
 	}
	



// 	BOOL GetRect(LPRECT lpRect, BOOL bTextOnly);
// 	BOOL GetText(LPTSTR lpstrText, c_int nLen);
// //#ifndef _ATL_NO_COM
// 	BOOL GetText(BSTR& bstrText);
// //#endif // !_ATL_NO_COM
// //#if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	BOOL GetText(_CSTRING_NS::CString& strText);
// //#endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)
// 	BOOL SetText(LPCTSTR lpszItem);
// 	BOOL GetImage(c_int& nImage, c_int& nSelectedImage);
// 	BOOL SetImage(c_int nImage, c_int nSelectedImage);
// 	UINT GetState(UINT nStateMask);
// 	BOOL SetState(UINT nState, UINT nStateMask);
// 	DWORD_PTR GetData();
// 	BOOL SetData(DWORD_PTR dwData);
	//BOOL SetItem(UINT nMask, LPCTSTR lpszItem, c_int nImage, c_int nSelectedImage, UINT nState, UINT nStateMask, LPARAM lParam);

// Operations
	pub fn InsertAfter(&self,lpstrItem: &str, hItemAfter: HTREEITEM, nImageIndex: c_int,)->CTreeItem {
		return self._Insert(lpstrItem, nImageIndex, hItemAfter);
	}

	pub fn AddHead(&self,lpstrItem: &str, nImageIndex: c_int,)->CTreeItem {
		return self._Insert(lpstrItem, nImageIndex, TVI_FIRST);
	}

	pub fn AddTail(&self,lpstrItem: &str, nImageIndex: c_int,)->CTreeItem {
		return self._Insert(lpstrItem, nImageIndex, TVI_LAST);
	}

// 	CTreeItemT<TBase> GetChild();
// 	CTreeItemT<TBase> GetNext(UINT nCode);
// 	CTreeItemT<TBase> GetNextSibling();
// 	CTreeItemT<TBase> GetPrevSibling();
// 	CTreeItemT<TBase> GetParent();
// 	CTreeItemT<TBase> GetFirstVisible();
// 	CTreeItemT<TBase> GetNextVisible();
// 	CTreeItemT<TBase> GetPrevVisible();
// 	CTreeItemT<TBase> GetSelected();
// 	CTreeItemT<TBase> GetDropHilight();
// 	CTreeItemT<TBase> GetRoot();
// //#if !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)
// 	CTreeItemT<TBase> GetLastVisible();
// //#endif // !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)
// //#if (_WIN32_IE >= 0x0600)
// 	CTreeItemT<TBase> GetNextSelected();
// //#endif // (_WIN32_IE >= 0x0600)
// 	BOOL HasChildren();
// 	BOOL Delete();
// 	BOOL Expand(UINT nCode = TVE_EXPAND);
// 	BOOL Select(UINT nCode);
// 	BOOL Select();
// 	BOOL SelectDropTarget();
// 	BOOL SelectSetFirstVisible();
// 	HWND EditLabel();
// 	HIMAGELIST CreateDragImage();
// 	BOOL SortChildren(BOOL bRecurse = FALSE);
// 	BOOL EnsureVisible();
// 	CTreeItemT<TBase> _Insert(LPCTSTR lpstrItem, c_int nImageIndex, HTREEITEM hItemAfter);
// 	c_int GetImageIndex();
// //#if (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)
// 	BOOL SetInsertMark(BOOL bAfter);
// //#endif // (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)
// //#if (_WIN32_WINNT >= 0x0501)
// 	UINT MapHTREEITEMToAccID();
// //#endif // (_WIN32_WINNT >= 0x0501)
// //#if (_WIN32_WINNT >= 0x0600)
// 	void ShowInfoTip();
// 	BOOL GetPartRect(TVITEMPART partID, LPRECT lpRect);
//#endif // (_WIN32_WINNT >= 0x0600)
	pub fn GetRect(&self,lpRect: LPRECT, bTextOnly: BOOL,)->BOOL {
		return self.pView.base.GetItemRect(self.hItem,lpRect,bTextOnly);
	}

	pub fn GetNext(&self,nCode: UINT,)->CTreeItem {
		return self.pView.GetNextItem(self.hItem,nCode);
	}

	pub fn GetChild(&self,)->CTreeItem {
		return self.pView.GetChildItem(self.hItem);
	}

	pub fn GetNextSibling(&self,)->CTreeItem {
		return self.pView.GetNextSiblingItem(self.hItem);
	}

	pub fn GetPrevSibling(&self,)->CTreeItem {
		return self.pView.GetPrevSiblingItem(self.hItem);
	}

	pub fn GetParent(&self,)->CTreeItem {
		return self.pView.GetParentItem(self.hItem);
	}

	pub fn GetFirstVisible(&self,)->CTreeItem {
		return self.pView.GetFirstVisibleItem();
	}

	pub fn GetNextVisible(&self,)->CTreeItem {
		return self.pView.GetNextVisibleItem(self.hItem);
	}

	pub fn GetPrevVisible(&self,)->CTreeItem {
		return self.pView.GetPrevVisibleItem(self.hItem);
	}

	pub fn GetSelected(&self,)->CTreeItem {
		return self.pView.GetSelectedItem();
	}

	pub fn GetDropHilight(&self,)->CTreeItem {
		return self.pView.GetDropHilightItem();
	}

	pub fn GetRoot(&self,)->CTreeItem {
		return self.pView.GetRootItem();
	}

	//#if !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)

	pub fn GetLastVisible(&self,)->CTreeItem {
		return self.pView.GetLastVisibleItem();
	}
	//#endif // !defined(_WIN32_WCE) && (_WIN32_IE >= 0x0400)

	//#if (_WIN32_IE >= 0x0600)

	pub fn GetNextSelected(&self,)->CTreeItem {
		return self.pView.GetNextSelectedItem();
	}
	//#endif // (_WIN32_IE >= 0x0600)

	pub fn GetText(&self)->String {
		return self.pView.base.GetItemText(self.hItem);
	}

	//#ifndef _ATL_NO_COM
	//#ifdef _OLEAUTO_H_

	// pub fn GetText(bstrText: &BSTR,)->BOOL {
		
	// 	return self.m_pTreeView.GetItemText(self.m_hTreeItem, bstrText);
	// }
	//#endif // _OLEAUTO_H_
	//#endif // !_ATL_NO_COM

	//#if defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	// pub fn GetText(@_CSTRING_NS::CString& strText,)->BOOL {
	// 	ATLASSERT(m_pTreeView != NULL);
	// 	return m_pTreeView->GetItemText(m_hTreeItem, strText);
	// }
	//#endif // defined(_WTL_USE_CSTRING) || defined(__ATLSTR_H__)

	pub fn GetImage(&self,nImage: &mut c_int,nSelectedImage: &mut c_int,)->BOOL {		
		return self.pView.base.GetItemImage(self.hItem,nImage,nSelectedImage);
	}

	pub fn GetState(&self,nStateMask: UINT,)->UINT {
		return self.pView.base.GetItemState(self.hItem,nStateMask);
	}

	pub fn GetData(&self,)->DWORD_PTR {
		return self.pView.base.GetItemData(self.hItem);
	}

	pub fn SetItem(&self,nMask: UINT, lpszItem: &str, nImage: c_int, nSelectedImage: c_int, nState: UINT, nStateMask: UINT, lParam: LPARAM,)->BOOL {
		return self.pView.base.SetItem(self.hItem, nMask, Some(lpszItem), nImage, nSelectedImage, nState, nStateMask, lParam);
	}

	pub fn SetText(&self,lpszItem: &str,)->BOOL {
		return self.pView.base.SetItemText(self.hItem,lpszItem);
	}

	pub fn SetImage(&self,nImage: c_int, nSelectedImage: c_int,)->BOOL {
		return self.pView.base.SetItemImage(self.hItem,nImage,nSelectedImage);
	}

	pub fn SetState(&self,nState: UINT, nStateMask: UINT,)->BOOL {
		return self.pView.base.SetItemState(self.hItem,nState,nStateMask);
	}

	pub fn SetData(&self,dwData: DWORD_PTR,)->BOOL {
		return self.pView.base.SetItemData(self.hItem,dwData);
	}

	pub fn HasChildren(&self,)->BOOL {
		return self.pView.base.ItemHasChildren(self.hItem);
	}

	pub fn Delete(&self,)->BOOL {
		return self.pView.base.DeleteItem(self.hItem);
	}

	pub fn Expand(&self,nCode: Option<UINT> /*= TVE_EXPAND*/,)->BOOL {
		return self.pView.base.Expand(self.hItem,nCode);
	}

	pub fn Select_code(&self,nCode: UINT,)->BOOL {
		return self.pView.base.Select(self.hItem,nCode);
	}

	pub fn Select(&self,)->BOOL {
		return self.pView.base.SelectItem(self.hItem);
	}

	pub fn SelectDropTarget(&self,)->BOOL {
		return self.pView.base.SelectDropTarget(self.hItem);
	}

	pub fn SelectSetFirstVisible(&self,)->BOOL {
		return self.pView.base.SelectSetFirstVisible(self.hItem);
	}

	// pub fn EditLabel(&self,)->HWND {
	// 	return self.pView.base.EditLabel(self.hItem);
	// }

	pub fn CreateDragImage(&self,)->HIMAGELIST {
		//return self.pView.base.CreateDragImage(self.hItem);
		self.pView.base.cwin().SendMessage(TVM_CREATEDRAGIMAGE, 0, self.hItem as LPARAM) as HIMAGELIST
	}

	pub fn SortChildren(&self, bRecurse: BOOL /*= FALSE*/,)->BOOL {
		return self.pView.base.SortChildren(self.hItem, Some(bRecurse));
	}

	pub fn EnsureVisible(&self,)->BOOL {
		return self.pView.base.EnsureVisible(self.hItem);
	}

	pub fn _Insert(&self,lpstrItem: &str, nImageIndex: c_int, hItemAfter: HTREEITEM,)->CTreeItem {
		let mut ins: TVINSERTSTRUCTW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		ins.hParent = self.hItem;
		ins.hInsertAfter = hItemAfter;
		unsafe {
			ins.u.itemex_mut().mask = TVIF_TEXT;
			//hold vec returned by to_c_u16 for lifetime reason
			let mut tmp = lpstrItem.to_c_u16();
			ins.u.itemex_mut().pszText = tmp.as_mut_ptr();

			if nImageIndex != -1 {
				ins.u.itemex_mut().mask = ins.u.itemex().mask | (TVIF_IMAGE | TVIF_SELECTEDIMAGE);
				ins.u.itemex_mut().iImage = nImageIndex;
				ins.u.itemex_mut().iSelectedImage = nImageIndex;
			}
			CTreeItem::new(self.pView.InsertItem2(&mut ins).hItem, self.pView)
		}
	}

	pub fn GetImageIndex(&self,)->c_int {
		let mut item: TVITEMW = unsafe{mem::MaybeUninit::zeroed().assume_init()};
		item.mask = TVIF_HANDLE | TVIF_IMAGE;
		item.hItem = self.hItem;
		self.pView.base.GetItem(&mut item);
		return item.iImage;
	}

	//#if (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)

	pub fn SetInsertMark(&self,bAfter: BOOL,)->BOOL {
		return self.pView.base.SetInsertMark(self.hItem, bAfter);
	}
	//#endif // (_WIN32_IE >= 0x0400) && !defined(_WIN32_WCE)

	//#if (_WIN32_WINNT >= 0x0501)

	pub fn MapHTREEITEMToAccID(&self,)->UINT {
		return self.pView.base.MapHTREEITEMToAccID(self.hItem);
	}
	//#endif // (_WIN32_WINNT >= 0x0501)

	//#if (_WIN32_WINNT >= 0x0600)

	pub fn ShowInfoTip(&self,) {
		self.pView.base.ShowInfoTip(self.hItem);
	}

	pub fn GetPartRect(&self,partID: TVITEMPART, lpRect: LPRECT,)->BOOL {
		return self.pView.base.GetItemPartRect(self.hItem, partID, lpRect);
	}
}

//typedef CTreeItemT<ATL::CWindow>   CTreeItem;


// CTreeItem inline methods


//#endif // (_WIN32_WINNT >= 0x0600)