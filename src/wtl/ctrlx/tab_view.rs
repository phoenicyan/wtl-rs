
#![allow(non_snake_case,dead_code,unused_variables)]
///////////////////////////////////////////////////////////////////////////////
// CButton - client side for a Windows BUTTON control
//use std::ops::{Deref,DerefMut};
//use atl::{CWindow,NULL_HWND};
//use atl::cwindow::*;

use winapi::shared::minwindef::*;


///////////////////////////////////////////////////////////////////////////////
// CTabView - implements tab view window

// TabView Notifications
// #define TBVN_PAGEACTIVATED   (0U-741)
// #define TBVN_CONTEXTMENU     (0U-742)

// // Notification data for TBVN_CONTEXTMENU
// struct TBVCONTEXTMENUINFO
// {
// 	NMHDR hdr;
// 	POINT pt;
// };

// typedef TBVCONTEXTMENUINFO* LPTBVCONTEXTMENUINFO;

// TabView must impl this trait
// pub trait TabDerive {	
// 	fn IsValidPageIndex(&self, nPage: int)-> bool;

// 	fn MovePage(&self, nMovePage: int, nInsertBeforePage: int)-> bool;

// // Implementation overrideables
// 	fn CreateTabControl(&self)-> bool;

// 	fn CalcTabHeight(&self)-> int;

// 	fn ShowTabControl(&self, bShow: bool);

// 	fn UpdateLayout(&self);

// 	fn UpdateMenu(&self);

// 	fn UpdateTitleBar(&self);

// 	fn DrawMoveMark(&self, nItem: int);

// 	fn GetMoveMarkRect(&self, rect: &RECT);

// 	fn SetMoveCursor(&self, bCanMove: bool);

// 	fn GenerateDragImage(&self, nItem: int);

// 	//fn ShortenTitle(LPCTSTR lpstrTitle, LPTSTR lpstrShortTitle, int cchShortTitle);
// 	fn ShortenTitle(&self, lpstrTitle: &str)-> String;

// 	fn UpdateTooltipText(&self, pTTDI: &NMTTDISPINFO);

// // Text for menu items and title bar - override to provide different strings
// 	fn GetEmptyListText()->String;

// 	fn GetWindowsMenuItemText()->String;

// 	fn GetTitleDividerText()->String;
// }


pub struct CTabView {
    idd: WORD,
}
//T: ui::Root
//TabTrait: 
// pub struct CTabViewImpl<T,D:TabDerive> {
//     //cwin: CWindow,
//     //tab_ctrl: 
//     derive: *mut D,
//     idd: WORD, // resource id of the dlg
//     root:*mut T, //raw pointer to the Root Dialogs
// }

// impl<T,D:TabDerive> CTabViewImpl<T,D> {
//     pub fn new(idd: WORD)-> CTabViewImpl<T> {
//     	CTabViewImpl{
//     		idd: idd,
//     		cwin: CWindow::new(NULL_HWND),
//     		root: 0 as *mut T,
//     	}
//     }
// }

// impl<T,D:TabDerive> CTabViewImpl<T> {
 
// }	
/*
template <class T, class TBase = ATL::CWindow, class TWinTraits = ATL::CControlWinTraits>
class ATL_NO_VTABLE CTabViewImpl : public ATL::CWindowImpl< T, TBase, TWinTraits >
{
public:
	DECLARE_WND_CLASS_EX(NULL, 0, COLOR_APPWORKSPACE)

// Declarations and enums
	struct TABVIEWPAGE
	{
		HWND hWnd;
		LPTSTR lpstrTitle;
		LPVOID pData;
	};

	struct TCITEMEXTRA
	{
		TCITEMHEADER tciheader;
		TABVIEWPAGE tvpage;

		operator LPTCITEM() { return (LPTCITEM)this; }
	};

	enum
	{
		m_nTabID = 1313,
		m_cxMoveMark = 6,
		m_cyMoveMark = 3,
		m_nMenuItemsMax = (ID_WINDOW_TABLAST - ID_WINDOW_TABFIRST + 1)
	};

// Data members
	ATL::CContainedWindowT<CTabCtrl> m_tab;
	int m_cyTabHeight;

	int m_nActivePage;

	int m_nInsertItem;
	POINT m_ptStartDrag;

	CMenuHandle m_menu;

	int m_cchTabTextLength;

	int m_nMenuItemsCount;

	ATL::CWindow m_wndTitleBar;
	LPTSTR m_lpstrTitleBarBase;
	int m_cchTitleBarLength;

	CImageList m_ilDrag;

	bool m_bDestroyPageOnRemove:1;
	bool m_bDestroyImageList:1;
	bool m_bActivePageMenuItem:1;
	bool m_bActiveAsDefaultMenuItem:1;
	bool m_bEmptyMenuItem:1;
	bool m_bWindowsMenuItem:1;
	bool m_bNoTabDrag:1;
	// internal
	bool m_bTabCapture:1;
	bool m_bTabDrag:1;
	bool m_bInternalFont:1;

// Constructor/destructor
	CTabViewImpl() :
			m_nActivePage(-1), 
			m_cyTabHeight(0), 
			m_tab(this, 1), 
			m_nInsertItem(-1), 
			m_cchTabTextLength(30), 
			m_nMenuItemsCount(10), 
			m_lpstrTitleBarBase(NULL), 
			m_cchTitleBarLength(100), 
			m_bDestroyPageOnRemove(true), 
			m_bDestroyImageList(true), 
			m_bActivePageMenuItem(true), 
			m_bActiveAsDefaultMenuItem(false), 
			m_bEmptyMenuItem(false), 
			m_bWindowsMenuItem(false), 
			m_bNoTabDrag(false), 
			m_bTabCapture(false), 
			m_bTabDrag(false), 
			m_bInternalFont(false)
	{
		m_ptStartDrag.x = 0;
		m_ptStartDrag.y = 0;
	}

	~CTabViewImpl()
	{
		delete [] m_lpstrTitleBarBase;
	}

// Message filter function - to be called from PreTranslateMessage of the main window
	BOOL PreTranslateMessage(MSG* pMsg)
	{
		if(IsWindow() == FALSE)
			return FALSE;

		BOOL bRet = FALSE;

		// Check for TabView built-in accelerators (Ctrl+Tab/Ctrl+Shift+Tab - next/previous page)
		int nCount = GetPageCount();
		if(nCount > 0)
		{
			bool bControl = (::GetKeyState(VK_CONTROL) < 0);
			if((pMsg->message == WM_KEYDOWN) && (pMsg->wParam == VK_TAB) && bControl)
			{
				if(nCount > 1)
				{
					int nPage = m_nActivePage;
					bool bShift = (::GetKeyState(VK_SHIFT) < 0);
					if(bShift)
						nPage = (nPage > 0) ? (nPage - 1) : (nCount - 1);
					else
						nPage = ((nPage >= 0) && (nPage < (nCount - 1))) ? (nPage + 1) : 0;

					SetActivePage(nPage);
					T* pT = static_cast<T*>(this);
					pT->OnPageActivated(m_nActivePage);
				}

				bRet = TRUE;
			}
		}

		// If we are doing drag-drop, check for Escape key that cancels it
		if(bRet == FALSE)
		{
			if(m_bTabCapture && pMsg->message == WM_KEYDOWN && pMsg->wParam == VK_ESCAPE)
			{
				::ReleaseCapture();
				bRet = TRUE;
			}
		}

		// Pass the message to the active page
		if(bRet == FALSE)
		{
			if(m_nActivePage != -1)
				bRet = (BOOL)::SendMessage(GetPageHWND(m_nActivePage), WM_FORWARDMSG, 0, (LPARAM)pMsg);
		}

		return bRet;
	}

// Attributes
	int GetPageCount() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return m_tab.GetItemCount();
	}

	int GetActivePage() const
	{
		return m_nActivePage;
	}

	void SetActivePage(int nPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		SetRedraw(FALSE);

		if(m_nActivePage != -1)
			::ShowWindow(GetPageHWND(m_nActivePage), FALSE);
		m_nActivePage = nPage;
		m_tab.SetCurSel(m_nActivePage);
		::ShowWindow(GetPageHWND(m_nActivePage), TRUE);

		pT->UpdateLayout();

		SetRedraw(TRUE);
		RedrawWindow(NULL, NULL, RDW_FRAME | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);

		if(::GetFocus() != m_tab.m_hWnd)
			::SetFocus(GetPageHWND(m_nActivePage));

		pT->UpdateTitleBar();
		pT->UpdateMenu();
	}

	HIMAGELIST GetImageList() const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return m_tab.GetImageList();
	}

	HIMAGELIST SetImageList(HIMAGELIST hImageList)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		return m_tab.SetImageList(hImageList);
	}

	void SetWindowMenu(HMENU hMenu)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		m_menu = hMenu;

		T* pT = static_cast<T*>(this);
		pT->UpdateMenu();
	}

	void SetTitleBarWindow(HWND hWnd)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		delete [] m_lpstrTitleBarBase;
		m_lpstrTitleBarBase = NULL;

		m_wndTitleBar = hWnd;
		if(hWnd == NULL)
			return;

		int cchLen = m_wndTitleBar.GetWindowTextLength() + 1;
		ATLTRY(m_lpstrTitleBarBase = new TCHAR[cchLen]);
		if(m_lpstrTitleBarBase != NULL)
		{
			m_wndTitleBar.GetWindowText(m_lpstrTitleBarBase, cchLen);
			T* pT = static_cast<T*>(this);
			pT->UpdateTitleBar();
		}
	}

// Page attributes
	HWND GetPageHWND(int nPage) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);

		return tcix.tvpage.hWnd;
	}

	LPCTSTR GetPageTitle(int nPage) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_PARAM;
		if(m_tab.GetItem(nPage, tcix) == FALSE)
			return NULL;

		return tcix.tvpage.lpstrTitle;
	}

	bool SetPageTitle(int nPage, LPCTSTR lpstrTitle)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		int cchBuff = lstrlen(lpstrTitle) + 1;
		LPTSTR lpstrBuff = NULL;
		ATLTRY(lpstrBuff = new TCHAR[cchBuff]);
		if(lpstrBuff == NULL)
			return false;

		SecureHelper::strcpy_x(lpstrBuff, cchBuff, lpstrTitle);
		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_PARAM;
		if(m_tab.GetItem(nPage, tcix) == FALSE)
			return false;

		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
		if(lpstrTabText == NULL)
			return false;

		delete [] tcix.tvpage.lpstrTitle;

		pT->ShortenTitle(lpstrTitle, lpstrTabText, m_cchTabTextLength + 1);

		tcix.tciheader.mask = TCIF_TEXT | TCIF_PARAM;
		tcix.tciheader.pszText = lpstrTabText;
		tcix.tvpage.lpstrTitle = lpstrBuff;
		if(m_tab.SetItem(nPage, tcix) == FALSE)
			return false;

		pT->UpdateTitleBar();
		pT->UpdateMenu();

		return true;
	}

	LPVOID GetPageData(int nPage) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);

		return tcix.tvpage.pData;
	}

	LPVOID SetPageData(int nPage, LPVOID pData)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_PARAM;
		m_tab.GetItem(nPage, tcix);
		LPVOID pDataOld = tcix.tvpage.pData;

		tcix.tvpage.pData = pData;
		m_tab.SetItem(nPage, tcix);

		return pDataOld;
	}

	int GetPageImage(int nPage) const
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_IMAGE;
		m_tab.GetItem(nPage, tcix);

		return tcix.tciheader.iImage;
	}

	int SetPageImage(int nPage, int nImage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_IMAGE;
		m_tab.GetItem(nPage, tcix);
		int nImageOld = tcix.tciheader.iImage;

		tcix.tciheader.iImage = nImage;
		m_tab.SetItem(nPage, tcix);

		return nImageOld;
	}

// Operations
	bool AddPage(HWND hWndView, LPCTSTR lpstrTitle, int nImage = -1, LPVOID pData = NULL)
	{
		return InsertPage(GetPageCount(), hWndView, lpstrTitle, nImage, pData);
	}

	bool InsertPage(int nPage, HWND hWndView, LPCTSTR lpstrTitle, int nImage = -1, LPVOID pData = NULL)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(nPage == GetPageCount() || IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		int cchBuff = lstrlen(lpstrTitle) + 1;
		LPTSTR lpstrBuff = NULL;
		ATLTRY(lpstrBuff = new TCHAR[cchBuff]);
		if(lpstrBuff == NULL)
			return false;

		SecureHelper::strcpy_x(lpstrBuff, cchBuff, lpstrTitle);

		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
		if(lpstrTabText == NULL)
			return false;

		pT->ShortenTitle(lpstrTitle, lpstrTabText, m_cchTabTextLength + 1);

		SetRedraw(FALSE);

		TCITEMEXTRA tcix = { 0 };
		tcix.tciheader.mask = TCIF_TEXT | TCIF_IMAGE | TCIF_PARAM;
		tcix.tciheader.pszText = lpstrTabText;
		tcix.tciheader.iImage = nImage;
		tcix.tvpage.hWnd = hWndView;
		tcix.tvpage.lpstrTitle = lpstrBuff;
		tcix.tvpage.pData = pData;
		int nItem = m_tab.InsertItem(nPage, tcix);
		if(nItem == -1)
		{
			delete [] lpstrBuff;
			SetRedraw(TRUE);
			return false;
		}

		// adjust active page index, if inserted before it
		if(nPage <= m_nActivePage)
			m_nActivePage++;

		SetActivePage(nItem);
		pT->OnPageActivated(m_nActivePage);

		if(GetPageCount() == 1)
			pT->ShowTabControl(true);

		pT->UpdateLayout();

		SetRedraw(TRUE);
		RedrawWindow(NULL, NULL, RDW_FRAME | RDW_INVALIDATE | RDW_UPDATENOW | RDW_ALLCHILDREN);

		return true;
	}

	void RemovePage(int nPage)
	{
		ATLASSERT(::IsWindow(m_hWnd));
		ATLASSERT(IsValidPageIndex(nPage));

		T* pT = static_cast<T*>(this);

		SetRedraw(FALSE);

		if(GetPageCount() == 1)
			pT->ShowTabControl(false);

		if(m_bDestroyPageOnRemove)
			::DestroyWindow(GetPageHWND(nPage));
		else
			::ShowWindow(GetPageHWND(nPage), FALSE);
		LPTSTR lpstrTitle = (LPTSTR)GetPageTitle(nPage);
		delete [] lpstrTitle;

		ATLVERIFY(m_tab.DeleteItem(nPage) != FALSE);

		if(m_nActivePage == nPage)
		{
			m_nActivePage = -1;

			if(nPage > 0)
			{
				SetActivePage(nPage - 1);
			}
			else if(GetPageCount() > 0)
			{
				SetActivePage(nPage);
			}
			else
			{
				SetRedraw(TRUE);
				Invalidate();
				UpdateWindow();
				pT->UpdateTitleBar();
				pT->UpdateMenu();
			}
		}
		else
		{
			nPage = (nPage < m_nActivePage) ? (m_nActivePage - 1) : m_nActivePage;
			m_nActivePage = -1;
			SetActivePage(nPage);
		}

		pT->OnPageActivated(m_nActivePage);
	}

	void RemoveAllPages()
	{
		ATLASSERT(::IsWindow(m_hWnd));

		if(GetPageCount() == 0)
			return;

		T* pT = static_cast<T*>(this);

		SetRedraw(FALSE);

		pT->ShowTabControl(false);

		for(int i = 0; i < GetPageCount(); i++)
		{
			if(m_bDestroyPageOnRemove)
				::DestroyWindow(GetPageHWND(i));
			else
				::ShowWindow(GetPageHWND(i), FALSE);
			LPTSTR lpstrTitle = (LPTSTR)GetPageTitle(i);
			delete [] lpstrTitle;
		}
		m_tab.DeleteAllItems();

		m_nActivePage = -1;
		pT->OnPageActivated(m_nActivePage);

		SetRedraw(TRUE);
		Invalidate();
		UpdateWindow();

		pT->UpdateTitleBar();
		pT->UpdateMenu();
	}

	int PageIndexFromHwnd(HWND hWnd) const
	{
		int nIndex = -1;

		for(int i = 0; i < GetPageCount(); i++)
		{
			if(GetPageHWND(i) == hWnd)
			{
				nIndex = i;
				break;
			}
		}

		return nIndex;
	}

	void BuildWindowMenu(HMENU hMenu, int nMenuItemsCount = 10, bool bEmptyMenuItem = true, bool bWindowsMenuItem = true, bool bActivePageMenuItem = true, bool bActiveAsDefaultMenuItem = false)
	{
		ATLASSERT(::IsWindow(m_hWnd));

		CMenuHandle menu = hMenu;
		T* pT = static_cast<T*>(this);
		pT;   // avoid level 4 warning
		int nFirstPos = 0;

		// Find first menu item in our range
#ifndef _WIN32_WCE
		for(nFirstPos = 0; nFirstPos < menu.GetMenuItemCount(); nFirstPos++)
		{
			UINT nID = menu.GetMenuItemID(nFirstPos);
			if((nID >= ID_WINDOW_TABFIRST && nID <= ID_WINDOW_TABLAST) || nID == ID_WINDOW_SHOWTABLIST)
				break;
		}
#else // CE specific
		for(nFirstPos = 0; ; nFirstPos++)
		{
			CMenuItemInfo mii;
			mii.fMask = MIIM_ID;
			BOOL bRet = menu.GetMenuItemInfo(nFirstPos, TRUE, &mii);
			if(bRet == FALSE)
				break;
			if((mii.wID >= ID_WINDOW_TABFIRST && mii.wID <= ID_WINDOW_TABLAST) || mii.wID == ID_WINDOW_SHOWTABLIST)
				break;
		}
#endif // _WIN32_WCE

		// Remove all menu items for tab pages
		BOOL bRet = TRUE;
		while(bRet != FALSE)
			bRet = menu.DeleteMenu(nFirstPos, MF_BYPOSITION);

		// Add separator if it's not already there
		int nPageCount = GetPageCount();
		if((bWindowsMenuItem || (nPageCount > 0)) && (nFirstPos > 0))
		{
			CMenuItemInfo mii;
			mii.fMask = MIIM_TYPE;
			menu.GetMenuItemInfo(nFirstPos - 1, TRUE, &mii);
			if((nFirstPos <= 0) || ((mii.fType & MFT_SEPARATOR) == 0))
			{
				menu.AppendMenu(MF_SEPARATOR);
				nFirstPos++;
			}
		}

		// Add menu items for all pages
		if(nPageCount > 0)
		{
			// Append menu items for all pages
			const int cchPrefix = 3;   // 2 digits + space
			nMenuItemsCount = __min(__min(nPageCount, nMenuItemsCount), (int)m_nMenuItemsMax);
			ATLASSERT(nMenuItemsCount < 100);   // 2 digits only
			if(nMenuItemsCount >= 100)
				nMenuItemsCount = 99;

			for(int i = 0; i < nMenuItemsCount; i++)
			{
				LPCTSTR lpstrTitle = GetPageTitle(i);
				int nLen = lstrlen(lpstrTitle);
				CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
				LPTSTR lpstrText = buff.Allocate(cchPrefix + nLen + 1);
				ATLASSERT(lpstrText != NULL);
				if(lpstrText != NULL)
				{
					LPCTSTR lpstrFormat = (i < 9) ? _T("&%i %s") : _T("%i %s");
					SecureHelper::wsprintf_x(lpstrText, cchPrefix + nLen + 1, lpstrFormat, i + 1, lpstrTitle);
					menu.AppendMenu(MF_STRING, ID_WINDOW_TABFIRST + i, lpstrText);
				}
			}

			// Mark active page
			if(bActivePageMenuItem && (m_nActivePage != -1))
			{
#ifndef _WIN32_WCE
				if(bActiveAsDefaultMenuItem)
				{
					menu.SetMenuDefaultItem((UINT)-1,  TRUE);
					menu.SetMenuDefaultItem(nFirstPos + m_nActivePage,  TRUE);
				}
				else
#else // CE specific
				bActiveAsDefaultMenuItem;   // avoid level 4 warning
#endif // _WIN32_WCE
				{
					menu.CheckMenuRadioItem(nFirstPos, nFirstPos + nMenuItemsCount, nFirstPos + m_nActivePage, MF_BYPOSITION);
				}
			}
		}
		else
		{
			if(bEmptyMenuItem)
			{
				menu.AppendMenu(MF_BYPOSITION | MF_STRING, ID_WINDOW_TABFIRST, pT->GetEmptyListText());
				menu.EnableMenuItem(ID_WINDOW_TABFIRST, MF_GRAYED);
			}

			// Remove separator if nothing else is there
			if(!bEmptyMenuItem && !bWindowsMenuItem && (nFirstPos > 0))
			{
				CMenuItemInfo mii;
				mii.fMask = MIIM_TYPE;
				menu.GetMenuItemInfo(nFirstPos - 1, TRUE, &mii);
				if((mii.fType & MFT_SEPARATOR) != 0)
					menu.DeleteMenu(nFirstPos - 1, MF_BYPOSITION);
			}
		}

		// Add "Windows..." menu item
		if(bWindowsMenuItem)
			menu.AppendMenu(MF_BYPOSITION | MF_STRING, ID_WINDOW_SHOWTABLIST, pT->GetWindowsMenuItemText());
	}

	BOOL SubclassWindow(HWND hWnd)
	{
#if (_MSC_VER >= 1300)
		BOOL bRet = ATL::CWindowImpl< T, TBase, TWinTraits >::SubclassWindow(hWnd);
#else // !(_MSC_VER >= 1300)
		typedef ATL::CWindowImpl< T, TBase, TWinTraits >   _baseClass;
		BOOL bRet = _baseClass::SubclassWindow(hWnd);
#endif // !(_MSC_VER >= 1300)
		if(bRet != FALSE)
		{
			T* pT = static_cast<T*>(this);
			pT->CreateTabControl();
			pT->UpdateLayout();
		}

		return bRet;
	}

// Message map and handlers
	BEGIN_MSG_MAP(CTabViewImpl)
		MESSAGE_HANDLER(WM_CREATE, OnCreate)
		MESSAGE_HANDLER(WM_DESTROY, OnDestroy)
		MESSAGE_HANDLER(WM_SIZE, OnSize)
		MESSAGE_HANDLER(WM_SETFOCUS, OnSetFocus)
		MESSAGE_HANDLER(WM_GETFONT, OnGetFont)
		MESSAGE_HANDLER(WM_SETFONT, OnSetFont)
		NOTIFY_HANDLER(m_nTabID, TCN_SELCHANGE, OnTabChanged)
		NOTIFY_ID_HANDLER(m_nTabID, OnTabNotification)
#ifndef _WIN32_WCE
		NOTIFY_CODE_HANDLER(TTN_GETDISPINFO, OnTabGetDispInfo)
#endif // !_WIN32_WCE
		FORWARD_NOTIFICATIONS()
	ALT_MSG_MAP(1)   // tab control
		MESSAGE_HANDLER(WM_LBUTTONDOWN, OnTabLButtonDown)
		MESSAGE_HANDLER(WM_LBUTTONUP, OnTabLButtonUp)
		MESSAGE_HANDLER(WM_CAPTURECHANGED, OnTabCaptureChanged)
		MESSAGE_HANDLER(WM_MOUSEMOVE, OnTabMouseMove)
		MESSAGE_HANDLER(WM_RBUTTONUP, OnTabRButtonUp)
		MESSAGE_HANDLER(WM_SYSKEYDOWN, OnTabSysKeyDown)
	END_MSG_MAP()

	LRESULT OnCreate(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->CreateTabControl();

		return 0;
	}

	LRESULT OnDestroy(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		RemoveAllPages();

		if(m_bDestroyImageList)
		{
			CImageList il = m_tab.SetImageList(NULL);
			if(il.m_hImageList != NULL)
				il.Destroy();
		}

		if(m_bInternalFont)
		{
			HFONT hFont = m_tab.GetFont();
			m_tab.SetFont(NULL, FALSE);
			::DeleteObject(hFont);
			m_bInternalFont = false;
		}

		return 0;
	}

	LRESULT OnSize(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		T* pT = static_cast<T*>(this);
		pT->UpdateLayout();
		return 0;
	}

	LRESULT OnSetFocus(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		if(m_nActivePage != -1)
			::SetFocus(GetPageHWND(m_nActivePage));
		return 0;
	}

	LRESULT OnGetFont(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& /*bHandled*/)
	{
		return m_tab.SendMessage(WM_GETFONT);
	}

	LRESULT OnSetFont(UINT /*uMsg*/, WPARAM wParam, LPARAM lParam, BOOL& /*bHandled*/)
	{
		if(m_bInternalFont)
		{
			HFONT hFont = m_tab.GetFont();
			m_tab.SetFont(NULL, FALSE);
			::DeleteObject(hFont);
			m_bInternalFont = false;
		}

		m_tab.SendMessage(WM_SETFONT, wParam, lParam);

		T* pT = static_cast<T*>(this);
		m_cyTabHeight = pT->CalcTabHeight();

		if((BOOL)lParam != FALSE)
			pT->UpdateLayout();

		return 0;
	}

	LRESULT OnTabChanged(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		SetActivePage(m_tab.GetCurSel());
		T* pT = static_cast<T*>(this);
		pT->OnPageActivated(m_nActivePage);

		return 0;
	}

	LRESULT OnTabNotification(int /*idCtrl*/, LPNMHDR /*pnmh*/, BOOL& /*bHandled*/)
	{
		// nothing to do - this just blocks all tab control
		// notifications from being propagated further
		return 0;
	}

#ifndef _WIN32_WCE
	LRESULT OnTabGetDispInfo(int /*idCtrl*/, LPNMHDR pnmh, BOOL& bHandled)
	{
		LPNMTTDISPINFO pTTDI = (LPNMTTDISPINFO)pnmh;
		if(pTTDI->hdr.hwndFrom == m_tab.GetTooltips())
		{
			T* pT = static_cast<T*>(this);
			pT->UpdateTooltipText(pTTDI);
		}
		else
		{
			bHandled = FALSE;
		}

		return 0;
	}
#endif // !_WIN32_WCE

// Tab control message handlers
	LRESULT OnTabLButtonDown(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(!m_bNoTabDrag && (m_tab.GetItemCount() > 1))
		{
			m_bTabCapture = true;
			m_tab.SetCapture();

			m_ptStartDrag.x = GET_X_LPARAM(lParam);
			m_ptStartDrag.y = GET_Y_LPARAM(lParam);
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabLButtonUp(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		if(m_bTabCapture)
		{
			if(m_bTabDrag)
			{
				TCHITTESTINFO hti = { 0 };
				hti.pt.x = GET_X_LPARAM(lParam);
				hti.pt.y = GET_Y_LPARAM(lParam);
				int nItem = m_tab.HitTest(&hti);
				if(nItem != -1)
					MovePage(m_nActivePage, nItem);
			}

			::ReleaseCapture();
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabCaptureChanged(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM /*lParam*/, BOOL& bHandled)
	{
		if(m_bTabCapture)
		{
			m_bTabCapture = false;

			if(m_bTabDrag)
			{
				m_bTabDrag = false;
				T* pT = static_cast<T*>(this);
				pT->DrawMoveMark(-1);

#ifndef _WIN32_WCE
				m_ilDrag.DragLeave(GetDesktopWindow());
#endif // !_WIN32_WCE
				m_ilDrag.EndDrag();

				m_ilDrag.Destroy();
				m_ilDrag.m_hImageList = NULL;
			}
		}

		bHandled = FALSE;
		return 0;
	}

	LRESULT OnTabMouseMove(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& bHandled)
	{
		bHandled = FALSE;

		if(m_bTabCapture)
		{
			POINT pt = { GET_X_LPARAM(lParam), GET_Y_LPARAM(lParam) };

			if(!m_bTabDrag)
			{
#ifndef _WIN32_WCE
				if(abs(m_ptStartDrag.x - GET_X_LPARAM(lParam)) >= ::GetSystemMetrics(SM_CXDRAG) ||
				   abs(m_ptStartDrag.y - GET_Y_LPARAM(lParam)) >= ::GetSystemMetrics(SM_CYDRAG))
#else // CE specific
				if(abs(m_ptStartDrag.x - GET_X_LPARAM(lParam)) >= 4 ||
				   abs(m_ptStartDrag.y - GET_Y_LPARAM(lParam)) >= 4)
#endif // _WIN32_WCE
				{
					T* pT = static_cast<T*>(this);
					pT->GenerateDragImage(m_nActivePage);

					int cxCursor = ::GetSystemMetrics(SM_CXCURSOR);
					int cyCursor = ::GetSystemMetrics(SM_CYCURSOR);
					m_ilDrag.BeginDrag(0, -(cxCursor / 2), -(cyCursor / 2));
#ifndef _WIN32_WCE
					POINT ptEnter = m_ptStartDrag;
					m_tab.ClientToScreen(&ptEnter);
					m_ilDrag.DragEnter(GetDesktopWindow(), ptEnter);
#endif // !_WIN32_WCE

					m_bTabDrag = true;
				}
			}

			if(m_bTabDrag)
			{
				TCHITTESTINFO hti = { 0 };
				hti.pt = pt;
				int nItem = m_tab.HitTest(&hti);

				T* pT = static_cast<T*>(this);
				pT->SetMoveCursor(nItem != -1);

				if(m_nInsertItem != nItem)
					pT->DrawMoveMark(nItem);

				m_ilDrag.DragShowNolock((nItem != -1) ? TRUE : FALSE);
				m_tab.ClientToScreen(&pt);
				m_ilDrag.DragMove(pt);

				bHandled = TRUE;
			}
		}

		return 0;
	}

	LRESULT OnTabRButtonUp(UINT /*uMsg*/, WPARAM /*wParam*/, LPARAM lParam, BOOL& /*bHandled*/)
	{
		TCHITTESTINFO hti = { 0 };
		hti.pt.x = GET_X_LPARAM(lParam);
		hti.pt.y = GET_Y_LPARAM(lParam);
		int nItem = m_tab.HitTest(&hti);
		if(nItem != -1)
		{
			T* pT = static_cast<T*>(this);
			pT->OnContextMenu(nItem, hti.pt);
		}

		return 0;
	}

	LRESULT OnTabSysKeyDown(UINT /*uMsg*/, WPARAM wParam, LPARAM /*lParam*/, BOOL& bHandled)
	{
		bool bShift = (::GetKeyState(VK_SHIFT) < 0);
		if(wParam == VK_F10 && bShift)
		{
			if(m_nActivePage != -1)
			{
				RECT rect = { 0 };
				m_tab.GetItemRect(m_nActivePage, &rect);
				POINT pt = { rect.left, rect.bottom };
				T* pT = static_cast<T*>(this);
				pT->OnContextMenu(m_nActivePage, pt);
			}
		}
		else
		{
			bHandled = FALSE;
		}

		return 0;
	}


};

class CTabView : public CTabViewImpl<CTabView>
{
public:
	DECLARE_WND_CLASS_EX(_T("WTL_TabView"), 0, COLOR_APPWORKSPACE)
};

*/

// default impl
//impl CTabViewImpl {
 // Implementation helpers
// 	bool IsValidPageIndex(int nPage) const
// 	{
// 		return (nPage >= 0 && nPage < GetPageCount());
// 	}

// 	bool MovePage(int nMovePage, int nInsertBeforePage)
// 	{
// 		ATLASSERT(IsValidPageIndex(nMovePage));
// 		ATLASSERT(IsValidPageIndex(nInsertBeforePage));

// 		if(!IsValidPageIndex(nMovePage) || !IsValidPageIndex(nInsertBeforePage))
// 			return false;

// 		if(nMovePage == nInsertBeforePage)
// 			return true;   // nothing to do

// 		CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 		LPTSTR lpstrTabText = buff.Allocate(m_cchTabTextLength + 1);
// 		if(lpstrTabText == NULL)
// 			return false;
// 		TCITEMEXTRA tcix = { 0 };
// 		tcix.tciheader.mask = TCIF_TEXT | TCIF_IMAGE | TCIF_PARAM;
// 		tcix.tciheader.pszText = lpstrTabText;
// 		tcix.tciheader.cchTextMax = m_cchTabTextLength + 1;
// 		BOOL bRet = m_tab.GetItem(nMovePage, tcix);
// 		ATLASSERT(bRet != FALSE);
// 		if(bRet == FALSE)
// 			return false;

// 		int nInsertItem = (nInsertBeforePage > nMovePage) ? nInsertBeforePage + 1 : nInsertBeforePage;
// 		int nNewItem = m_tab.InsertItem(nInsertItem, tcix);
// 		ATLASSERT(nNewItem == nInsertItem);
// 		if(nNewItem != nInsertItem)
// 		{
// 			ATLVERIFY(m_tab.DeleteItem(nNewItem));
// 			return false;
// 		}

// 		if(nMovePage > nInsertBeforePage)
// 			ATLVERIFY(m_tab.DeleteItem(nMovePage + 1) != FALSE);
// 		else if(nMovePage < nInsertBeforePage)
// 			ATLVERIFY(m_tab.DeleteItem(nMovePage) != FALSE);

// 		SetActivePage(nInsertBeforePage);
// 		T* pT = static_cast<T*>(this);
// 		pT->OnPageActivated(m_nActivePage);

// 		return true;
// 	}

// // Implementation overrideables
// 	bool CreateTabControl()
// 	{
// #ifndef _WIN32_WCE
// 		m_tab.Create(m_hWnd, rcDefault, NULL, WS_CHILD | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | TCS_TOOLTIPS, 0, m_nTabID);
// #else // CE specific
// 		m_tab.Create(m_hWnd, rcDefault, NULL, WS_CHILD | WS_CLIPSIBLINGS | WS_CLIPCHILDREN, 0, m_nTabID);
// #endif // _WIN32_WCE
// 		ATLASSERT(m_tab.m_hWnd != NULL);
// 		if(m_tab.m_hWnd == NULL)
// 			return false;

// 		m_tab.SetFont(AtlCreateControlFont());
// 		m_bInternalFont = true;

// 		m_tab.SetItemExtra(sizeof(TABVIEWPAGE));

// 		T* pT = static_cast<T*>(this);
// 		m_cyTabHeight = pT->CalcTabHeight();

// 		return true;
// 	}

// 	int CalcTabHeight()
// 	{
// 		int nCount = m_tab.GetItemCount();
// 		TCITEMEXTRA tcix = { 0 };
// 		tcix.tciheader.mask = TCIF_TEXT;
// 		tcix.tciheader.pszText = _T("NS");
// 		int nIndex = m_tab.InsertItem(nCount, tcix);

// 		RECT rect = { 0, 0, 1000, 1000 };
// 		m_tab.AdjustRect(FALSE, &rect);

// 		RECT rcWnd = { 0, 0, 1000, rect.top };
// 		::AdjustWindowRectEx(&rcWnd, m_tab.GetStyle(), FALSE, m_tab.GetExStyle());

// 		int nHeight = rcWnd.bottom - rcWnd.top;

// 		m_tab.DeleteItem(nIndex);

// 		return nHeight;
// 	}

// 	void ShowTabControl(bool bShow)
// 	{
// 		m_tab.ShowWindow(bShow ? SW_SHOWNOACTIVATE : SW_HIDE);
// 		T* pT = static_cast<T*>(this);
// 		pT->UpdateLayout();
// 	}

// 	void UpdateLayout()
// 	{
// 		RECT rect = { 0 };
// 		GetClientRect(&rect);

// 		int cyOffset = 0;
// 		if(m_tab.IsWindow() && ((m_tab.GetStyle() & WS_VISIBLE) != 0))
// 		{
// 			m_tab.SetWindowPos(NULL, 0, 0, rect.right - rect.left, m_cyTabHeight, SWP_NOZORDER);
// 			cyOffset = m_cyTabHeight;
// 		}

// 		if(m_nActivePage != -1)
// 			::SetWindowPos(GetPageHWND(m_nActivePage), NULL, 0, cyOffset, rect.right - rect.left, rect.bottom - rect.top - cyOffset, SWP_NOZORDER);
// 	}

// 	void UpdateMenu()
// 	{
// 		if(m_menu.m_hMenu != NULL)
// 			BuildWindowMenu(m_menu, m_nMenuItemsCount, m_bEmptyMenuItem, m_bWindowsMenuItem, m_bActivePageMenuItem, m_bActiveAsDefaultMenuItem);
// 	}

// 	void UpdateTitleBar()
// 	{
// 		if(!m_wndTitleBar.IsWindow() || m_lpstrTitleBarBase == NULL)
// 			return;   // nothing to do

// 		if(m_nActivePage != -1)
// 		{
// 			T* pT = static_cast<T*>(this);
// 			LPCTSTR lpstrTitle = pT->GetPageTitle(m_nActivePage);
// 			LPCTSTR lpstrDivider = pT->GetTitleDividerText();
// 			int cchBuffer = m_cchTitleBarLength + lstrlen(lpstrDivider) + lstrlen(m_lpstrTitleBarBase) + 1;
// 			CTempBuffer<TCHAR, _WTL_STACK_ALLOC_THRESHOLD> buff;
// 			LPTSTR lpstrPageTitle = buff.Allocate(cchBuffer);
// 			ATLASSERT(lpstrPageTitle != NULL);
// 			if(lpstrPageTitle != NULL)
// 			{
// 				pT->ShortenTitle(lpstrTitle, lpstrPageTitle, m_cchTitleBarLength + 1);
// 				SecureHelper::strcat_x(lpstrPageTitle, cchBuffer, lpstrDivider);
// 				SecureHelper::strcat_x(lpstrPageTitle, cchBuffer, m_lpstrTitleBarBase);
// 			}
// 			else
// 			{
// 				lpstrPageTitle = m_lpstrTitleBarBase;
// 			}

// 			m_wndTitleBar.SetWindowText(lpstrPageTitle);
// 		}
// 		else
// 		{
// 			m_wndTitleBar.SetWindowText(m_lpstrTitleBarBase);
// 		}
// 	}

// 	void DrawMoveMark(int nItem)
// 	{
// 		T* pT = static_cast<T*>(this);

// 		if(m_nInsertItem != -1)
// 		{
// 			RECT rect = { 0 };
// 			pT->GetMoveMarkRect(rect);
// 			m_tab.InvalidateRect(&rect);
// 		}

// 		m_nInsertItem = nItem;

// 		if(m_nInsertItem != -1)
// 		{
// 			CClientDC dc(m_tab.m_hWnd);

// 			RECT rect = { 0 };
// 			pT->GetMoveMarkRect(rect);

// 			CPen pen;
// 			pen.CreatePen(PS_SOLID, 1, ::GetSysColor(COLOR_WINDOWTEXT));
// 			CBrush brush;
// 			brush.CreateSolidBrush(::GetSysColor(COLOR_WINDOWTEXT));

// 			HPEN hPenOld = dc.SelectPen(pen);
// 			HBRUSH hBrushOld = dc.SelectBrush(brush);

// 			int x = rect.left;
// 			int y = rect.top;
// 			POINT ptsTop[3] = { { x, y }, { x + m_cxMoveMark, y }, { x + (m_cxMoveMark / 2), y + m_cyMoveMark } };
// 			dc.Polygon(ptsTop, 3);

// 			y = rect.bottom - 1;
// 			POINT ptsBottom[3] = { { x, y }, { x + m_cxMoveMark, y }, { x + (m_cxMoveMark / 2), y - m_cyMoveMark } };
// 			dc.Polygon(ptsBottom, 3);

// 			dc.SelectPen(hPenOld);
// 			dc.SelectBrush(hBrushOld);
// 		}
// 	}

// 	void GetMoveMarkRect(RECT& rect) const
// 	{
// 		m_tab.GetClientRect(&rect);

// 		RECT rcItem = { 0 };
// 		m_tab.GetItemRect(m_nInsertItem, &rcItem);

// 		if(m_nInsertItem <= m_nActivePage)
// 		{
// 			rect.left = rcItem.left - m_cxMoveMark / 2 - 1;
// 			rect.right = rcItem.left + m_cxMoveMark / 2;
// 		}
// 		else
// 		{
// 			rect.left = rcItem.right - m_cxMoveMark / 2 - 1;
// 			rect.right = rcItem.right + m_cxMoveMark / 2;
// 		}
// 	}

// 	void SetMoveCursor(bool bCanMove)
// 	{
// 		::SetCursor(::LoadCursor(NULL, bCanMove ? IDC_ARROW : IDC_NO));
// 	}

// 	void GenerateDragImage(int nItem)
// 	{
// 		ATLASSERT(IsValidPageIndex(nItem));

// #ifndef _WIN32_WCE
// 		RECT rcItem = { 0 };
// 		m_tab.GetItemRect(nItem, &rcItem);
// 		::InflateRect(&rcItem, 2, 2);   // make bigger to cover selected item
// #else // CE specific
// 		nItem;   // avoid level 4 warning
// 		RECT rcItem = { 0, 0, 40, 20 };
// #endif // _WIN32_WCE

// 		ATLASSERT(m_ilDrag.m_hImageList == NULL);
// 		m_ilDrag.Create(rcItem.right - rcItem.left, rcItem.bottom - rcItem.top, ILC_COLORDDB | ILC_MASK, 1, 1);

// 		CClientDC dc(m_hWnd);
// 		CDC dcMem;
// 		dcMem.CreateCompatibleDC(dc);
// 		ATLASSERT(dcMem.m_hDC != NULL);
// 		dcMem.SetViewportOrg(-rcItem.left, -rcItem.top);

// 		CBitmap bmp;
// 		bmp.CreateCompatibleBitmap(dc, rcItem.right - rcItem.left, rcItem.bottom - rcItem.top);
// 		ATLASSERT(bmp.m_hBitmap != NULL);

// 		HBITMAP hBmpOld = dcMem.SelectBitmap(bmp);
// #ifndef _WIN32_WCE
// 		m_tab.SendMessage(WM_PRINTCLIENT, (WPARAM)dcMem.m_hDC);
// #else // CE specific
// 		dcMem.Rectangle(&rcItem);
// #endif // _WIN32_WCE
// 		dcMem.SelectBitmap(hBmpOld);

// 		ATLVERIFY(m_ilDrag.Add(bmp.m_hBitmap, RGB(255, 0, 255)) != -1);
// 	}

// 	void ShortenTitle(LPCTSTR lpstrTitle, LPTSTR lpstrShortTitle, int cchShortTitle)
// 	{
// 		if(lstrlen(lpstrTitle) >= cchShortTitle)
// 		{
// 			LPCTSTR lpstrEllipsis = _T("...");
// 			int cchEllipsis = lstrlen(lpstrEllipsis);
// 			SecureHelper::strncpy_x(lpstrShortTitle, cchShortTitle, lpstrTitle, cchShortTitle - cchEllipsis - 1);
// 			SecureHelper::strcat_x(lpstrShortTitle, cchShortTitle, lpstrEllipsis);
// 		}
// 		else
// 		{
// 			SecureHelper::strcpy_x(lpstrShortTitle, cchShortTitle, lpstrTitle);
// 		}
// 	}

// #ifndef _WIN32_WCE
// 	void UpdateTooltipText(LPNMTTDISPINFO pTTDI)
// 	{
// 		ATLASSERT(pTTDI != NULL);
// 		pTTDI->lpszText = (LPTSTR)GetPageTitle((int)pTTDI->hdr.idFrom);
// 	}
// #endif // !_WIN32_WCE

// // Text for menu items and title bar - override to provide different strings
// 	static LPCTSTR GetEmptyListText()
// 	{
// 		return _T("(Empty)");
// 	}

// 	static LPCTSTR GetWindowsMenuItemText()
// 	{
// 		return _T("&Windows...");
// 	}

// 	static LPCTSTR GetTitleDividerText()
// 	{
// 		return _T(" - ");
// 	}

// // Notifications - override to provide different behavior
// 	void OnPageActivated(int nPage)
// 	{
// 		NMHDR nmhdr = { 0 };
// 		nmhdr.hwndFrom = m_hWnd;
// 		nmhdr.idFrom = nPage;
// 		nmhdr.code = TBVN_PAGEACTIVATED;
// 		::SendMessage(GetParent(), WM_NOTIFY, GetDlgCtrlID(), (LPARAM)&nmhdr);
// 	}

// 	void OnContextMenu(int nPage, POINT pt)
// 	{
// 		m_tab.ClientToScreen(&pt);

// 		TBVCONTEXTMENUINFO cmi = { 0 };
// 		cmi.hdr.hwndFrom = m_hWnd;
// 		cmi.hdr.idFrom = nPage;
// 		cmi.hdr.code = TBVN_CONTEXTMENU;
// 		cmi.pt = pt;
// 		::SendMessage(GetParent(), WM_NOTIFY, GetDlgCtrlID(), (LPARAM)&cmi);
// 	}   
//}