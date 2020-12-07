class CPaletteT
{
public:
// Data members
	HPALETTE m_hPalette;

// Constructor/destructor/operators
	CPaletteT(HPALETTE hPalette = NULL) : m_hPalette(hPalette)
	{ }

	~CPaletteT()
	{
		if(t_bManaged && m_hPalette != NULL)
			DeleteObject();
	}

	CPaletteT<t_bManaged>& operator =(HPALETTE hPalette)
	{
		Attach(hPalette);
		return *this;
	}

	void Attach(HPALETTE hPalette)
	{
		if(t_bManaged && m_hPalette != NULL && m_hPalette != hPalette)
			::DeleteObject(m_hPalette);
		m_hPalette = hPalette;
	}

	HPALETTE Detach()
	{
		HPALETTE hPalette = m_hPalette;
		m_hPalette = NULL;
		return hPalette;
	}

	operator HPALETTE() const { return m_hPalette; }

	bool IsNull() const { return (m_hPalette == NULL); }

// Create methods
	HPALETTE CreatePalette(LPLOGPALETTE lpLogPalette)
	{
		ATLASSERT(m_hPalette == NULL);
		m_hPalette = ::CreatePalette(lpLogPalette);
		return m_hPalette;
	}

#ifndef _WIN32_WCE
	HPALETTE CreateHalftonePalette(HDC hDC)
	{
		ATLASSERT(m_hPalette == NULL);
		ATLASSERT(hDC != NULL);
		m_hPalette = ::CreateHalftonePalette(hDC);
		return m_hPalette;
	}
#endif // !_WIN32_WCE

	BOOL DeleteObject()
	{
		ATLASSERT(m_hPalette != NULL);
		BOOL bRet = ::DeleteObject(m_hPalette);
		if(bRet)
			m_hPalette = NULL;
		return bRet;
	}

// Attributes
	int GetEntryCount() const
	{
		ATLASSERT(m_hPalette != NULL);
		WORD nEntries = 0;
		::GetObject(m_hPalette, sizeof(WORD), &nEntries);
		return (int)nEntries;
	}

	UINT GetPaletteEntries(UINT nStartIndex, UINT nNumEntries, LPPALETTEENTRY lpPaletteColors) const
	{
		ATLASSERT(m_hPalette != NULL);
		return ::GetPaletteEntries(m_hPalette, nStartIndex, nNumEntries, lpPaletteColors);
	}

	UINT SetPaletteEntries(UINT nStartIndex, UINT nNumEntries, LPPALETTEENTRY lpPaletteColors)
	{
		ATLASSERT(m_hPalette != NULL);
		return ::SetPaletteEntries(m_hPalette, nStartIndex, nNumEntries, lpPaletteColors);
	}

// Operations
#ifndef _WIN32_WCE
	void AnimatePalette(UINT nStartIndex, UINT nNumEntries, LPPALETTEENTRY lpPaletteColors)
	{
		ATLASSERT(m_hPalette != NULL);
		::AnimatePalette(m_hPalette, nStartIndex, nNumEntries, lpPaletteColors);
	}

	BOOL ResizePalette(UINT nNumEntries)
	{
		ATLASSERT(m_hPalette != NULL);
		return ::ResizePalette(m_hPalette, nNumEntries);
	}
#endif // !_WIN32_WCE

	UINT GetNearestPaletteIndex(COLORREF crColor) const
	{
		ATLASSERT(m_hPalette != NULL);
		return ::GetNearestPaletteIndex(m_hPalette, crColor);
	}
};

typedef CPaletteT<false>   CPaletteHandle;
typedef CPaletteT<true>    CPalette;
