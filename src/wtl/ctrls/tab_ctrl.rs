
use atl::CWindow;

pub struct CTabCtrl {
    cwin: CWindow,
}

// template <class TBase>
// class CTabCtrlT : public TBase
// {
// public:
// // Constructors
// 	CTabCtrlT(HWND hWnd = NULL) : TBase(hWnd)
// 	{ }

// 	CTabCtrlT< TBase >& operator =(HWND hWnd)
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
// 		return WC_TABCONTROL;
// 	}

// 	CImageList GetImageList() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return CImageList((HIMAGELIST)::SendMessage(m_hWnd, TCM_GETIMAGELIST, 0, 0L));
// 	}

// 	CImageList SetImageList(HIMAGELIST hImageList)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return CImageList((HIMAGELIST)::SendMessage(m_hWnd, TCM_SETIMAGELIST, 0, (LPARAM)hImageList));
// 	}

// 	int GetItemCount() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_GETITEMCOUNT, 0, 0L);
// 	}

// 	BOOL GetItem(int nItem, LPTCITEM pTabCtrlItem) const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_GETITEM, nItem, (LPARAM)pTabCtrlItem);
// 	}

// 	BOOL SetItem(int nItem, LPTCITEM pTabCtrlItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_SETITEM, nItem, (LPARAM)pTabCtrlItem);
// 	}

// 	int SetItem(int nItem, UINT mask, LPCTSTR lpszItem, DWORD dwState, DWORD dwStateMask, int iImage, LPARAM lParam)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		TCITEM tci = { 0 };
// 		tci.mask = mask;
// 		tci.pszText = (LPTSTR) lpszItem;
// 		tci.dwState = dwState;
// 		tci.dwStateMask = dwStateMask;
// 		tci.iImage = iImage;
// 		tci.lParam = lParam;
// 		return (int)::SendMessage(m_hWnd, TCM_SETITEM, nItem, (LPARAM)&tci);
// 	}

// 	BOOL GetItemRect(int nItem, LPRECT lpRect) const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_GETITEMRECT, nItem, (LPARAM)lpRect);
// 	}

// 	int GetCurSel() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_GETCURSEL, 0, 0L);
// 	}

// 	int SetCurSel(int nItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_SETCURSEL, nItem, 0L);
// 	}

// 	SIZE SetItemSize(SIZE size)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		DWORD dwSize = (DWORD)::SendMessage(m_hWnd, TCM_SETITEMSIZE, 0, MAKELPARAM(size.cx, size.cy));
// 		SIZE sizeRet = { GET_X_LPARAM(dwSize), GET_Y_LPARAM(dwSize) };
// 		return sizeRet;
// 	}

// 	void SetItemSize(int cx, int cy)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_SETITEMSIZE, 0, MAKELPARAM(cx, cy));
// 	}

// 	void SetPadding(SIZE size)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_SETPADDING, 0, MAKELPARAM(size.cx, size.cy));
// 	}

// 	int GetRowCount() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_GETROWCOUNT, 0, 0L);
// 	}

// #ifndef _WIN32_WCE
// 	CToolTipCtrl GetToolTips() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return CToolTipCtrl((HWND)::SendMessage(m_hWnd, TCM_GETTOOLTIPS, 0, 0L));
// 	}

// 	// this method is deprecated, please use GetToolTips
// 	CToolTipCtrl GetTooltips() const { return GetToolTips(); }

// 	void SetToolTips(HWND hWndToolTip)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_SETTOOLTIPS, (WPARAM)hWndToolTip, 0L);
// 	}

// 	// this method is deprecated, please use SetToolTips
// 	void SetTooltips(HWND hWndToolTip) { SetToolTips(hWndToolTip); }

// #endif // !_WIN32_WCE

// 	int GetCurFocus() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_GETCURFOCUS, 0, 0L);
// 	}

// 	void SetCurFocus(int nItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_SETCURFOCUS, nItem, 0L);
// 	}

// 	BOOL SetItemExtra(int cbExtra)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		ATLASSERT(GetItemCount() == 0);   // must be empty
// 		return (BOOL)::SendMessage(m_hWnd, TCM_SETITEMEXTRA, cbExtra, 0L);
// 	}

// 	int SetMinTabWidth(int nWidth = -1)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_SETMINTABWIDTH, 0, nWidth);
// 	}

// #if (_WIN32_IE >= 0x0400)
// 	DWORD GetExtendedStyle() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (DWORD)::SendMessage(m_hWnd, TCM_GETEXTENDEDSTYLE, 0, 0L);
// 	}

// 	DWORD SetExtendedStyle(DWORD dwExMask, DWORD dwExStyle)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (DWORD)::SendMessage(m_hWnd, TCM_SETEXTENDEDSTYLE, dwExMask, dwExStyle);
// 	}

// #ifndef _WIN32_WCE
// 	BOOL GetUnicodeFormat() const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_GETUNICODEFORMAT, 0, 0L);
// 	}

// 	BOOL SetUnicodeFormat(BOOL bUnicode = TRUE)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_SETUNICODEFORMAT, bUnicode, 0L);
// 	}
// #endif // !_WIN32_WCE
// #endif // (_WIN32_IE >= 0x0400)

// // Operations
// 	int InsertItem(int nItem, LPTCITEM pTabCtrlItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_INSERTITEM, nItem, (LPARAM)pTabCtrlItem);
// 	}

// 	int InsertItem(int nItem, UINT mask, LPCTSTR lpszItem, int iImage, LPARAM lParam)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		TCITEM tci = { 0 };
// 		tci.mask = mask;
// 		tci.pszText = (LPTSTR) lpszItem;
// 		tci.iImage = iImage;
// 		tci.lParam = lParam;
// 		return (int)::SendMessage(m_hWnd, TCM_INSERTITEM, nItem, (LPARAM)&tci);
// 	}

// 	int InsertItem(int nItem, LPCTSTR lpszItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		TCITEM tci = { 0 };
// 		tci.mask = TCIF_TEXT;
// 		tci.pszText = (LPTSTR) lpszItem;
// 		return (int)::SendMessage(m_hWnd, TCM_INSERTITEM, nItem, (LPARAM)&tci);
// 	}

// 	int AddItem(LPTCITEM pTabCtrlItem)
// 	{
// 		return InsertItem(GetItemCount(), pTabCtrlItem);
// 	}

// 	int AddItem(UINT mask, LPCTSTR lpszItem, int iImage, LPARAM lParam)
// 	{
// 		return InsertItem(GetItemCount(), mask, lpszItem, iImage, lParam);
// 	}

// 	int AddItem(LPCTSTR lpszItem)
// 	{
// 		return InsertItem(GetItemCount(), lpszItem);
// 	}

// 	BOOL DeleteItem(int nItem)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_DELETEITEM, nItem, 0L);
// 	}

// 	BOOL DeleteAllItems()
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_DELETEALLITEMS, 0, 0L);
// 	}

// 	void AdjustRect(BOOL bLarger, LPRECT lpRect)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_ADJUSTRECT, bLarger, (LPARAM)lpRect);
// 	}

// 	void RemoveImage(int nImage)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_REMOVEIMAGE, nImage, 0L);
// 	}

// 	int HitTest(TC_HITTESTINFO* pHitTestInfo) const
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (int)::SendMessage(m_hWnd, TCM_HITTEST, 0, (LPARAM)pHitTestInfo);
// 	}

// 	void DeselectAll(BOOL bExcludeFocus = TRUE)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::SendMessage(m_hWnd, TCM_DESELECTALL, bExcludeFocus, 0L);
// 	}

// #if (_WIN32_IE >= 0x0400)
// 	BOOL HighlightItem(int nIndex, BOOL bHighlight = TRUE)
// 	{
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		return (BOOL)::SendMessage(m_hWnd, TCM_HIGHLIGHTITEM, nIndex, MAKELPARAM(bHighlight, 0));
// 	}
// #endif // (_WIN32_IE >= 0x0400)
// };

// typedef CTabCtrlT<ATL::CWindow>   CTabCtrl;