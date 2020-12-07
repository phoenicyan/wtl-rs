class CRgnT
{
public:
// Data members
	HRGN m_hRgn;

// Constructor/destructor/operators
	CRgnT(HRGN hRgn = NULL) : m_hRgn(hRgn)
	{ }

	~CRgnT()
	{
		if(t_bManaged && m_hRgn != NULL)
			DeleteObject();
	}

	CRgnT<t_bManaged>& operator =(HRGN hRgn)
	{
		Attach(hRgn);
		return *this;
	}

	void Attach(HRGN hRgn)
	{
		if(t_bManaged && m_hRgn != NULL && m_hRgn != hRgn)
			::DeleteObject(m_hRgn);
		m_hRgn = hRgn;
	}

	HRGN Detach()
	{
		HRGN hRgn = m_hRgn;
		m_hRgn = NULL;
		return hRgn;
	}

	operator HRGN() const { return m_hRgn; }

	bool IsNull() const { return (m_hRgn == NULL); }

// Create methods
	HRGN CreateRectRgn(int x1, int y1, int x2, int y2)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreateRectRgn(x1, y1, x2, y2);
		return m_hRgn;
	}

	HRGN CreateRectRgnIndirect(LPCRECT lpRect)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreateRectRgnIndirect(lpRect);
		return m_hRgn;
	}

#ifndef _WIN32_WCE
	HRGN CreateEllipticRgn(int x1, int y1, int x2, int y2)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreateEllipticRgn(x1, y1, x2, y2);
		return m_hRgn;
	}

	HRGN CreateEllipticRgnIndirect(LPCRECT lpRect)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreateEllipticRgnIndirect(lpRect);
		return m_hRgn;
	}

	HRGN CreatePolygonRgn(LPPOINT lpPoints, int nCount, int nMode)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreatePolygonRgn(lpPoints, nCount, nMode);
		return m_hRgn;
	}

	HRGN CreatePolyPolygonRgn(LPPOINT lpPoints, LPINT lpPolyCounts, int nCount, int nPolyFillMode)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreatePolyPolygonRgn(lpPoints, lpPolyCounts, nCount, nPolyFillMode);
		return m_hRgn;
	}

	HRGN CreateRoundRectRgn(int x1, int y1, int x2, int y2, int x3, int y3)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::CreateRoundRectRgn(x1, y1, x2, y2, x3, y3);
		return m_hRgn;
	}

	HRGN CreateFromPath(HDC hDC)
	{
		ATLASSERT(m_hRgn == NULL);
		ATLASSERT(hDC != NULL);
		m_hRgn = ::PathToRegion(hDC);
		return m_hRgn;
	}

	HRGN CreateFromData(const XFORM* lpXForm, int nCount, const RGNDATA* pRgnData)
	{
		ATLASSERT(m_hRgn == NULL);
		m_hRgn = ::ExtCreateRegion(lpXForm, nCount, pRgnData);
		return m_hRgn;
	}
#endif // !_WIN32_WCE

	BOOL DeleteObject()
	{
		ATLASSERT(m_hRgn != NULL);
		BOOL bRet = ::DeleteObject(m_hRgn);
		if(bRet)
			m_hRgn = NULL;
		return bRet;
	}

// Operations
	void SetRectRgn(int x1, int y1, int x2, int y2)
	{
		ATLASSERT(m_hRgn != NULL);
		::SetRectRgn(m_hRgn, x1, y1, x2, y2);
	}

	void SetRectRgn(LPCRECT lpRect)
	{
		ATLASSERT(m_hRgn != NULL);
		::SetRectRgn(m_hRgn, lpRect->left, lpRect->top, lpRect->right, lpRect->bottom);
	}

	int CombineRgn(HRGN hRgnSrc1, HRGN hRgnSrc2, int nCombineMode)
	{
		ATLASSERT(m_hRgn != NULL);
		return ::CombineRgn(m_hRgn, hRgnSrc1, hRgnSrc2, nCombineMode);
	}

	int CombineRgn(HRGN hRgnSrc, int nCombineMode)
	{
		ATLASSERT(m_hRgn != NULL);
		return ::CombineRgn(m_hRgn, m_hRgn, hRgnSrc, nCombineMode);
	}

	int CopyRgn(HRGN hRgnSrc)
	{
		ATLASSERT(m_hRgn != NULL);
		return ::CombineRgn(m_hRgn, hRgnSrc, NULL, RGN_COPY);
	}

	BOOL EqualRgn(HRGN hRgn) const
	{
		ATLASSERT(m_hRgn != NULL);
		return ::EqualRgn(m_hRgn, hRgn);
	}

	int OffsetRgn(int x, int y)
	{
		ATLASSERT(m_hRgn != NULL);
		return ::OffsetRgn(m_hRgn, x, y);
	}

	int OffsetRgn(POINT point)
	{
		ATLASSERT(m_hRgn != NULL);
		return ::OffsetRgn(m_hRgn, point.x, point.y);
	}

	int GetRgnBox(LPRECT lpRect) const
	{
		ATLASSERT(m_hRgn != NULL);
		return ::GetRgnBox(m_hRgn, lpRect);
	}

	BOOL PtInRegion(int x, int y) const
	{
		ATLASSERT(m_hRgn != NULL);
		return ::PtInRegion(m_hRgn, x, y);
	}

	BOOL PtInRegion(POINT point) const
	{
		ATLASSERT(m_hRgn != NULL);
		return ::PtInRegion(m_hRgn, point.x, point.y);
	}

	BOOL RectInRegion(LPCRECT lpRect) const
	{
		ATLASSERT(m_hRgn != NULL);
		return ::RectInRegion(m_hRgn, lpRect);
	}

	int GetRegionData(LPRGNDATA lpRgnData, int nDataSize) const
	{
		ATLASSERT(m_hRgn != NULL);
		return (int)::GetRegionData(m_hRgn, nDataSize, lpRgnData);
	}
};

typedef CRgnT<false>   CRgnHandle;
typedef CRgnT<true>    CRgn;