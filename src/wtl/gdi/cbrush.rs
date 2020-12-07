

class CBrushT
{
public:
// Data members
	HBRUSH m_hBrush;

// Constructor/destructor/operators
	CBrushT(HBRUSH hBrush = NULL) : m_hBrush(hBrush)
	{ }

	~CBrushT()
	{
		if(t_bManaged && m_hBrush != NULL)
			DeleteObject();
	}

	CBrushT<t_bManaged>& operator =(HBRUSH hBrush)
	{
		Attach(hBrush);
		return *this;
	}

	void Attach(HBRUSH hBrush)
	{
		if(t_bManaged && m_hBrush != NULL && m_hBrush != hBrush)
			::DeleteObject(m_hBrush);
		m_hBrush = hBrush;
	}

	HBRUSH Detach()
	{
		HBRUSH hBrush = m_hBrush;
		m_hBrush = NULL;
		return hBrush;
	}

	operator HBRUSH() const { return m_hBrush; }

	bool IsNull() const { return (m_hBrush == NULL); }

// Create methods
	HBRUSH CreateSolidBrush(COLORREF crColor)
	{
		ATLASSERT(m_hBrush == NULL);
		m_hBrush = ::CreateSolidBrush(crColor);
		return m_hBrush;
	}

#ifndef _WIN32_WCE
	HBRUSH CreateHatchBrush(int nIndex, COLORREF crColor)
	{
		ATLASSERT(m_hBrush == NULL);
		m_hBrush = ::CreateHatchBrush(nIndex, crColor);
		return m_hBrush;
	}
#endif // !_WIN32_WCE

#if !defined(_WIN32_WCE) || (_ATL_VER >= 0x0800)
	HBRUSH CreateBrushIndirect(const LOGBRUSH* lpLogBrush)
	{
		ATLASSERT(m_hBrush == NULL);
#ifndef _WIN32_WCE
		m_hBrush = ::CreateBrushIndirect(lpLogBrush);
#else // CE specific
		m_hBrush = ATL::CreateBrushIndirect(lpLogBrush);
#endif // _WIN32_WCE
		return m_hBrush;
	}
#endif // !defined(_WIN32_WCE) || (_ATL_VER >= 0x0800)

	HBRUSH CreatePatternBrush(HBITMAP hBitmap)
	{
		ATLASSERT(m_hBrush == NULL);
		m_hBrush = ::CreatePatternBrush(hBitmap);
		return m_hBrush;
	}

	HBRUSH CreateDIBPatternBrush(HGLOBAL hPackedDIB, UINT nUsage)
	{
		ATLASSERT(hPackedDIB != NULL);
		const void* lpPackedDIB = GlobalLock(hPackedDIB);
		ATLASSERT(lpPackedDIB != NULL);
		m_hBrush = ::CreateDIBPatternBrushPt(lpPackedDIB, nUsage);
		GlobalUnlock(hPackedDIB);
		return m_hBrush;
	}

	HBRUSH CreateDIBPatternBrush(const void* lpPackedDIB, UINT nUsage)
	{
		ATLASSERT(m_hBrush == NULL);
		m_hBrush = ::CreateDIBPatternBrushPt(lpPackedDIB, nUsage);
		return m_hBrush;
	}

	HBRUSH CreateSysColorBrush(int nIndex)
	{
		ATLASSERT(m_hBrush == NULL);
		m_hBrush = ::GetSysColorBrush(nIndex);
		return m_hBrush;
	}

	BOOL DeleteObject()
	{
		ATLASSERT(m_hBrush != NULL);
		BOOL bRet = ::DeleteObject(m_hBrush);
		if(bRet)
			m_hBrush = NULL;
		return bRet;
	}

// Attributes
	int GetLogBrush(LOGBRUSH* pLogBrush) const
	{
		ATLASSERT(m_hBrush != NULL);
		return ::GetObject(m_hBrush, sizeof(LOGBRUSH), pLogBrush);
	}

	bool GetLogBrush(LOGBRUSH& LogBrush) const
	{
		ATLASSERT(m_hBrush != NULL);
		return (::GetObject(m_hBrush, sizeof(LOGBRUSH), &LogBrush) == sizeof(LOGBRUSH));
	}
};

typedef CBrushT<false>   CBrushHandle;
typedef CBrushT<true>    CBrush;