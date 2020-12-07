

pub struct Inner {
    bitmap: HBITMAP,
}

// class CBitmapT
// {
// public:
// // Data members
// 	HBITMAP m_hBitmap;

// // Constructor/destructor/operators
// 	CBitmapT(HBITMAP hBitmap = NULL) : m_hBitmap(hBitmap)
// 	{ }

// 	~CBitmapT()
// 	{
// 		if(t_bManaged && m_hBitmap != NULL)
// 			DeleteObject();
// 	}

// 	CBitmapT<t_bManaged>& operator =(HBITMAP hBitmap)
// 	{
// 		Attach(hBitmap);
// 		return *this;
// 	}
impl Inner {
	
	void Attach(HBITMAP hBitmap)
	{
		if(t_bManaged && m_hBitmap != NULL&& m_hBitmap != hBitmap)
			::DeleteObject(m_hBitmap);
		m_hBitmap = hBitmap;
	}

	HBITMAP Detach()
	{
		HBITMAP hBitmap = m_hBitmap;
		m_hBitmap = NULL;
		return hBitmap;
	}

	operator HBITMAP() const { return m_hBitmap; }

	bool IsNull() const { return (m_hBitmap == NULL); }

// Create and load methods
	HBITMAP LoadBitmap(ATL::_U_STRINGorID bitmap)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::LoadBitmap(ModuleHelper::GetResourceInstance(), bitmap.m_lpstr);
		return m_hBitmap;
	}

	HBITMAP LoadOEMBitmap(UINT nIDBitmap) // for OBM_/OCR_/OIC_
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::LoadBitmap(NULL, MAKEINTRESOURCE(nIDBitmap));
		return m_hBitmap;
	}

#ifndef _WIN32_WCE
	HBITMAP LoadMappedBitmap(UINT nIDBitmap, UINT nFlags = 0, LPCOLORMAP lpColorMap = NULL, int nMapSize = 0)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateMappedBitmap(ModuleHelper::GetResourceInstance(), nIDBitmap, (WORD)nFlags, lpColorMap, nMapSize);
		return m_hBitmap;
	}
#endif // !_WIN32_WCE

	HBITMAP CreateBitmap(int nWidth, int nHeight, UINT nPlanes, UINT nBitsPerPixel, const void* lpBits)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateBitmap(nWidth, nHeight, nPlanes, nBitsPerPixel, lpBits);
		return m_hBitmap;
	}

#ifndef _WIN32_WCE
	HBITMAP CreateBitmapIndirect(LPBITMAP lpBitmap)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateBitmapIndirect(lpBitmap);
		return m_hBitmap;
	}
#endif // !_WIN32_WCE

	HBITMAP CreateCompatibleBitmap(HDC hDC, int nWidth, int nHeight)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateCompatibleBitmap(hDC, nWidth, nHeight);
		return m_hBitmap;
	}

#ifndef _WIN32_WCE
	HBITMAP CreateDiscardableBitmap(HDC hDC, int nWidth, int nHeight)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateDiscardableBitmap(hDC, nWidth, nHeight);
		return m_hBitmap;
	}
#endif // !_WIN32_WCE

	BOOL DeleteObject()
	{
		ATLASSERT(m_hBitmap != NULL);
		BOOL bRet = ::DeleteObject(m_hBitmap);
		if(bRet)
			m_hBitmap = NULL;
		return bRet;
	}

// Attributes
	int GetBitmap(BITMAP* pBitMap) const
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::GetObject(m_hBitmap, sizeof(BITMAP), pBitMap);
	}

	bool GetBitmap(BITMAP& bm) const
	{
		ATLASSERT(m_hBitmap != NULL);
		return (::GetObject(m_hBitmap, sizeof(BITMAP), &bm) == sizeof(BITMAP));
	}

	bool GetSize(SIZE& size) const
	{
		ATLASSERT(m_hBitmap != NULL);
		BITMAP bm = { 0 };
		if(!GetBitmap(&bm))
			return false;
		size.cx = bm.bmWidth;
		size.cy = bm.bmHeight;
		return true;
	}

#ifndef _WIN32_WCE
	DWORD GetBitmapBits(DWORD dwCount, LPVOID lpBits) const
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::GetBitmapBits(m_hBitmap, dwCount, lpBits);
	}
#endif // !_WIN32_WCE

#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)
	DWORD SetBitmapBits(DWORD dwCount, const void* lpBits)
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::SetBitmapBits(m_hBitmap, dwCount, lpBits);
	}
#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)

#ifndef _WIN32_WCE
	BOOL GetBitmapDimension(LPSIZE lpSize) const
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::GetBitmapDimensionEx(m_hBitmap, lpSize);
	}

	BOOL SetBitmapDimension(int nWidth, int nHeight, LPSIZE lpSize = NULL)
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::SetBitmapDimensionEx(m_hBitmap, nWidth, nHeight, lpSize);
	}

// DIB support
	HBITMAP CreateDIBitmap(HDC hDC, CONST BITMAPINFOHEADER* lpbmih, DWORD dwInit, CONST VOID* lpbInit, CONST BITMAPINFO* lpbmi, UINT uColorUse)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateDIBitmap(hDC, lpbmih, dwInit, lpbInit, lpbmi, uColorUse);
		return m_hBitmap;
	}
#endif // !_WIN32_WCE

	HBITMAP CreateDIBSection(HDC hDC, CONST BITMAPINFO* lpbmi, UINT uColorUse, VOID** ppvBits, HANDLE hSection, DWORD dwOffset)
	{
		ATLASSERT(m_hBitmap == NULL);
		m_hBitmap = ::CreateDIBSection(hDC, lpbmi, uColorUse, ppvBits, hSection, dwOffset);
		return m_hBitmap;
	}

#ifndef _WIN32_WCE
	int GetDIBits(HDC hDC, UINT uStartScan, UINT cScanLines,  LPVOID lpvBits, LPBITMAPINFO lpbmi, UINT uColorUse) const
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::GetDIBits(hDC, m_hBitmap, uStartScan, cScanLines,  lpvBits, lpbmi, uColorUse);
	}

	int SetDIBits(HDC hDC, UINT uStartScan, UINT cScanLines, CONST VOID* lpvBits, CONST BITMAPINFO* lpbmi, UINT uColorUse)
	{
		ATLASSERT(m_hBitmap != NULL);
		return ::SetDIBits(hDC, m_hBitmap, uStartScan, cScanLines, lpvBits, lpbmi, uColorUse);
	}
#endif // !_WIN32_WCE
};

typedef CBitmapT<false>   CBitmapHandle;
typedef CBitmapT<true>    CBitmap;