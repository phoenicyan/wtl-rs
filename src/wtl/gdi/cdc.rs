#![allow(dead_code, non_snake_case, non_camel_case_types, unused_variables)]

use std::mem;
use winapi::*;
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::windowsx::*;
use winapi::shared::ntdef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;

use std::ops::Drop;
use atl::NULL_HWND;

use wtl::ToCU16Str;

/* Image type */
pub const DST_COMPLEX: UINT = 0x0000;
pub const DST_TEXT: UINT = 0x0001;
pub const DST_PREFIXTEXT: UINT = 0x0002;
pub const DST_ICON: UINT = 0x0003;
pub const DST_BITMAP: UINT = 0x0004;
pub const DI_MASK: UINT = 0x0001;
pub const DI_IMAGE: UINT = 0x0002;
pub const DI_NORMAL: UINT = 0x0003;
pub const DI_COMPAT: UINT = 0x0004;
pub const DI_DEFAULTSIZE: UINT = 0x0008;

////////////////////////////////////////////
/// only operate on HDC,but not owened
pub struct CDCHandle {
    pub inner: Inner,
}

impl CDCHandle {
	pub fn new()->CDCHandle{
		CDCHandle{
			inner: Inner::new(),
		}
	}

	pub fn from_handle(h: HDC) -> CDCHandle {
		CDCHandle{
			inner: Inner::from_handle(h),
		}
	}

	pub fn Attach(&mut self, hDC: HDC) {
		self.inner.hdc = hDC;
	}
}

////////////////////////////////////////////
/// the HDC is owened by CDC
pub struct CDC {
    pub inner: Inner,
}

impl CDC {
	pub fn new()->CDC{
		CDC{
			inner: Inner::new(),
		}
	}

	pub fn from_handle(h: HDC) -> CDC {
		CDC{
			inner: Inner::from_handle(h),
		}
	}

	pub fn Attach(&mut self, hDC: HDC) {
		if self.inner.hdc != NULL_HDC && self.inner.hdc != hDC {
			unsafe{um::wingdi::DeleteDC(self.inner.hdc)};
		}
		self.inner.hdc = hDC;
	}
}

impl Drop for CDC {
	fn drop(&mut self){
		if self.inner.hdc != NULL_HDC {
			unsafe{um::wingdi::DeleteDC(self.inner.Detach())};
		}
	}
}

////////////////////////////////////////////
///
pub struct CPaintDC {
    pub inner: Inner,
    hwnd: HWND,
    ps: PAINTSTRUCT,
}

impl CPaintDC {
	pub fn new(hwnd: HWND) -> CPaintDC {
		debug_assert!(unsafe{um::winuser::IsWindow(hwnd)} == winapi::shared::minwindef::TRUE);
		let mut ps:PAINTSTRUCT = unsafe{mem::zeroed()};
		let hdc = unsafe{um::winuser::BeginPaint(hwnd, &mut ps)};
		CPaintDC{
			inner: Inner::from_handle(hdc),
			hwnd : hwnd,
			ps: ps,
		}
	}
}

impl Drop for CPaintDC {
	fn drop(&mut self){
		debug_assert!(self.inner.hdc != NULL_HDC);
		debug_assert!(unsafe{um::winuser::IsWindow(self.hwnd)} == winapi::shared::minwindef::TRUE);
		unsafe{um::winuser::EndPaint(self.hwnd, &self.ps)};
		self.inner.Detach();
	}
}
// class CPaintDC : public CDC
// {
// public:
// // Data members
// 	HWND m_hWnd;
// 	PAINTSTRUCT m_ps;

// // Constructor/destructor
// 	CPaintDC(HWND hWnd)
// 	{
// 		ATLASSERT(::IsWindow(hWnd));
// 		m_hWnd = hWnd;
// 		m_hDC = ::BeginPaint(hWnd, &m_ps);
// 	}

// 	~CPaintDC()
// 	{
// 		ATLASSERT(m_hDC != NULL);
// 		ATLASSERT(::IsWindow(m_hWnd));
// 		::EndPaint(m_hWnd, &m_ps);
// 		Detach();
// 	}
// };

////////////////////////////////////////////
pub struct CClientDC {
    pub inner: Inner,
    hwnd : HWND,
}

impl CClientDC {
	pub fn new(hwnd: HWND) -> CClientDC {
		debug_assert!(hwnd != NULL_HWND || unsafe{um::winuser::IsWindow(hwnd)}  == winapi::shared::minwindef::TRUE);
		let hdc = unsafe{um::winuser::GetDC(hwnd)};
		CClientDC{
			inner: Inner::from_handle(hdc),
			hwnd : hwnd,
		}
	}
}

impl Drop for CClientDC {
	fn drop(&mut self){
		debug_assert!(self.inner.hdc != NULL_HDC);
		unsafe{um::winuser::ReleaseDC(self.hwnd, self.inner.Detach())};
	}
}

// class CClientDC : public CDC
// {
// public:
// // Data members
// 	HWND m_hWnd;

// // Constructor/destructor
// 	CClientDC(HWND hWnd)
// 	{
// 		ATLASSERT(hWnd == NULL || ::IsWindow(hWnd));
// 		m_hWnd = hWnd;
// 		m_hDC = ::GetDC(hWnd);
// 	}

// 	~CClientDC()
// 	{
// 		ATLASSERT(m_hDC != NULL);
// 		::ReleaseDC(m_hWnd, Detach());
// 	}
// };

////////////////////////////////////////////
pub struct CWindowDC {
    pub inner: Inner,
    hwnd : HWND,
}

impl CWindowDC {
	pub fn new(hwnd: HWND) -> CWindowDC {
		debug_assert!(hwnd == NULL_HWND || unsafe{um::winuser::IsWindow(hwnd)} == winapi::shared::minwindef::TRUE);
		let hdc = unsafe{um::winuser::GetWindowDC(hwnd)};
		CWindowDC{
			inner: Inner::from_handle(hdc),
			hwnd : hwnd,
		}
	}
}
impl Drop for CWindowDC {
	fn drop(&mut self){
		debug_assert!(self.inner.hdc != NULL_HDC);
		unsafe{um::winuser::ReleaseDC(self.hwnd, self.inner.Detach())};
	}
}

// class CWindowDC : public CDC
// {
// public:
// // Data members
// 	HWND m_hWnd;

// // Constructor/destructor
// 	CWindowDC(HWND hWnd)
// 	{
// 		ATLASSERT(hWnd == NULL || ::IsWindow(hWnd));
// 		m_hWnd = hWnd;
// 		m_hDC = ::GetWindowDC(hWnd);
// 	}

// 	~CWindowDC()
// 	{
// 		ATLASSERT(m_hDC != NULL);
// 		::ReleaseDC(m_hWnd, Detach());
// 	}
// };

////////////////////////////////////////////
// pub struct CMemoryDC {
//     inner: Inner,
//     hdc_org: HDC,
//     rc_paint: RECT,
//     bmp: CBitmap,
//     bmp_old: HBITMAP,
// }

// impl CMemoryDC {
// 	pub fn new(hdc: HDC, rc: &RECT) -> CMemoryDC {

// 	}
// }
// class CMemoryDC : public CDC
// {
// public:
// // Data members
// 	HDC m_hDCOriginal;
// 	RECT m_rcPaint;
// 	CBitmap m_bmp;
// 	HBITMAP m_hBmpOld;

// // Constructor/destructor
// 	CMemoryDC(HDC hDC, const RECT& rcPaint) : m_hDCOriginal(hDC), m_hBmpOld(NULL)
// 	{
// 		m_rcPaint = rcPaint;
// 		CreateCompatibleDC(m_hDCOriginal);
// 		ATLASSERT(m_hDC != NULL);
// 		m_bmp.CreateCompatibleBitmap(m_hDCOriginal, m_rcPaint.right - m_rcPaint.left, m_rcPaint.bottom - m_rcPaint.top);
// 		ATLASSERT(m_bmp.m_hBitmap != NULL);
// 		m_hBmpOld = SelectBitmap(m_bmp);
// 		SetViewportOrg(-m_rcPaint.left, -m_rcPaint.top);
// 	}

// 	~CMemoryDC()
// 	{
// 		::BitBlt(m_hDCOriginal, m_rcPaint.left, m_rcPaint.top, m_rcPaint.right - m_rcPaint.left, m_rcPaint.bottom - m_rcPaint.top, m_hDC, m_rcPaint.left, m_rcPaint.top, SRCCOPY);
// 		SelectBitmap(m_hBmpOld);
// 	}
// };

////////////////////////////////////////////
// inner of cdc,cdchandle,cpaintdc,cclientdc...
pub struct Inner {
    hdc: HDC,
}

impl Inner {
	pub fn new() -> Inner {
		Inner{
			hdc: NULL_HDC,
		}
	}

	pub fn from_handle(h: HDC) -> Inner {
		Inner{
			hdc: h,
		}
	}

	pub fn Detach (&mut self)->HDC {
		let hDC = self.hdc;
		self.hdc = NULL_HDC;
		hDC
	}

	pub fn assert_dc(&self){
		debug_assert!(self.hdc != NULL_HDC);
	}

	pub fn assert_null_dc(&self){
		debug_assert!(self.hdc == NULL_HDC);
	}
	//fn HDC (&self)->operator { return self.hdc; }

	pub fn IsNull (&self)->bool { self.hdc == NULL_HDC }

// Operations
//#ifndef _WIN32_WCE
	pub fn WindowFromDC (&self)->HWND {
		self.assert_dc();
		unsafe{um::winuser::WindowFromDC(self.hdc)}
	}
//#endif // !_WIN32_WCE

	// fn GetCurrentPen (&self)->CPenHandle {
	// 	self.assert_dc();
	// 	CPenHandle(um::wingdi::GetCurrentObject(self.hdc, OBJ_PEN) as HPEN)
	// }

	// fn GetCurrentBrush (&self)->CBrushHandle {
	// 	self.assert_dc();
	// 	CBrushHandle(um::wingdi::GetCurrentObject(self.hdc, OBJ_BRUSH) as HBRUSH)
	// }

	// fn GetCurrentPalette (&self)->CPaletteHandle {
	// 	self.assert_dc();
	// 	CPaletteHandle(um::wingdi::GetCurrentObject(self.hdc, OBJ_PAL) as HPALETTE)
	// }

	// fn GetCurrentFont (&self)->CFontHandle {
	// 	self.assert_dc();
	// 	CFontHandle(um::wingdi::GetCurrentObject(self.hdc, OBJ_FONT) as HFONT)
	// }

	// fn GetCurrentBitmap (&self)->CBitmapHandle {
	// 	self.assert_dc();
	// 	CBitmapHandle(um::wingdi::GetCurrentObject(self.hdc, OBJ_BITMAP) as HBITMAP)
	// }

	pub fn CreateDC(&mut self,lpszDriverName: Option<&str>,lpszDeviceName: Option<&str>,lpszOutput: Option<&str>,lpInitData: &DEVMODEW)->HDC {
		debug_assert!(self.hdc == NULL_HDC);
		//hold vec returned by to_c_u16 for lifetime reason
		let tmp1: Vec<u16>;
		let driver_name = if let Some(p1) = lpszDriverName{
			tmp1 = p1.to_c_u16();
			tmp1.as_ptr()
		}else{
			0 as *const u16
		};
		//hold vec returned by to_c_u16 for lifetime reason
		let tmp2: Vec<u16>;
		let device_name = if let Some(p2) = lpszDeviceName{
			tmp2 = p2.to_c_u16();
			tmp2.as_ptr()
		}else{
			0 as *const u16
		};

		//hold vec returned by to_c_u16 for lifetime reason
		let tmp3: Vec<u16>;
		let out = if let Some(p3) = lpszOutput{
			tmp3 = p3.to_c_u16();
			tmp3.as_ptr()
		}else{
			0 as *const u16
		};

		self.hdc = unsafe{um::wingdi::CreateDCW(driver_name, device_name, out, lpInitData)};
		self.hdc
	}

	pub fn CreateCompatibleDC (&mut self, hDC: Option<HDC> /*NULL*/)->HDC {
		debug_assert!(self.hdc == NULL_HDC);
		let h = extract_opt_by_null!(hDC,HDC);
		self.hdc = unsafe{um::wingdi::CreateCompatibleDC(h)};
		self.hdc
	}

	pub fn DeleteDC (&mut self)->BOOL {
		if self.hdc == NULL_HDC {
			return winapi::shared::minwindef::FALSE;
		}
		let bRet = unsafe{um::wingdi::DeleteDC(self.hdc) as BOOL};
		if bRet == winapi::shared::minwindef::TRUE {
			self.hdc = NULL_HDC;
		}
		bRet
	}

// Device-Context Functions
	pub fn SaveDC (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SaveDC(self.hdc)}
	}

	pub fn RestoreDC(&self,nSavedDC: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::RestoreDC(self.hdc, nSavedDC)}
	}

	pub fn GetDeviceCaps(&self,nIndex: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetDeviceCaps(self.hdc, nIndex)}
	}

//#ifndef _WIN32_WCE
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::SetBoundsRect(self.hdc, lpRectBounds, flags)}
	}

	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::GetBoundsRect(self.hdc, lpRectBounds, flags)}
	}

	pub fn ResetDC(&self,lpDevMode: &DEVMODEW)->BOOL {
		self.assert_dc();
		if unsafe{ um::wingdi::ResetDCW(self.hdc, lpDevMode) } != NULL_HDC {
			winapi::shared::minwindef::TRUE
		}else{
			winapi::shared::minwindef::FALSE
		}
	}

// Drawing-Tool Functions
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetBrushOrgEx(self.hdc, lpPoint)}
	}
//#endif // !_WIN32_WCE

	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		// let mut l:LPPOINT = 0 as LPPOINT;
		// if let Some(l1) = lpPoint {
		// 	l = l1;
		// }
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::SetBrushOrgEx(self.hdc, x, y, p)}
	}

	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPointRet,LPPOINT);
		unsafe{um::wingdi::SetBrushOrgEx(self.hdc, point.x, point.y, p)}
	}

//#ifndef _WIN32_WCE
	pub fn EnumObjects(&self,nObjectType: c_int, lpfn: GOBJENUMPROC, lpData: LPARAM)->c_int {
		self.assert_dc();
//#ifdef STRICT
		return unsafe{um::wingdi::EnumObjects(self.hdc, nObjectType, lpfn, lpData)};
//#else
//		return ::EnumObjects(self.hdc, nObjectType, lpfn as GOBJENUMPROC, lpData as LPVOID);
//#endif
	}
//#endif // !_WIN32_WCE

// Type-safe selection helpers
	pub fn SelectPen(&self,hPen: HPEN)->HPEN {
		self.assert_dc();
//#ifndef _WIN32_WCE
		unsafe{
			debug_assert!(hPen == 0 as HPEN || um::wingdi::GetObjectType(hPen as HGDIOBJ) == OBJ_PEN || um::wingdi::GetObjectType(hPen as HGDIOBJ) == OBJ_EXTPEN);
		
//#else // CE specific
//		ATLASSERT(hPen == NULL || ::GetObjectType(hPen) == OBJ_PEN);
//#endif // _WIN32_WCE
			um::wingdi::SelectObject(self.hdc, hPen as HGDIOBJ) as HPEN
		}
	}

	pub fn SelectBrush(&self,hBrush: HBRUSH)->HBRUSH {
		self.assert_dc();
		unsafe{
			debug_assert!(hBrush == 0 as HBRUSH || um::wingdi::GetObjectType(hBrush as HGDIOBJ) == OBJ_BRUSH);
			um::wingdi::SelectObject(self.hdc, hBrush as HGDIOBJ) as HBRUSH
		}
	}

	pub fn SelectFont(&self,hFont: HFONT)->HFONT {
		self.assert_dc();
		unsafe{
			debug_assert!(hFont == 0 as HFONT || um::wingdi::GetObjectType(hFont as HGDIOBJ) == OBJ_FONT);
			um::wingdi::SelectObject(self.hdc, hFont as HGDIOBJ) as HFONT
		}
	}

	pub fn SelectBitmap(&self,hBitmap: HBITMAP)->HBITMAP {
		self.assert_dc();
		unsafe{
			debug_assert!(hBitmap == 0 as HBITMAP || um::wingdi::GetObjectType(hBitmap as HGDIOBJ) == OBJ_BITMAP);
			um::wingdi::SelectObject(self.hdc, hBitmap as HGDIOBJ) as HBITMAP
		}
	}

	pub fn SelectRgn(&self,hRgn: HRGN)->c_int {
		self.assert_dc();
		unsafe{
			debug_assert!(hRgn == 0 as HRGN || um::wingdi::GetObjectType(hRgn as HGDIOBJ) == OBJ_REGION);
			PtrToInt(um::wingdi::SelectObject(self.hdc, hRgn as HGDIOBJ))
		}
	}

// Type-safe selection helpers for stock objects
	pub fn SelectStockPen(&self, nPen: c_uint)->HPEN {
		self.assert_dc();
//#if (_WIN32_WINNT >= 0x0500)
		debug_assert!(nPen == WHITE_PEN || nPen == BLACK_PEN || nPen == NULL_PEN || nPen == DC_PEN);
//#else
//		debug_assert!(nPen == WHITE_PEN || nPen == BLACK_PEN || nPen == NULL_PEN);
//#endif // !(_WIN32_WINNT >= 0x0500)
		unsafe{
			self.SelectPen(um::wingdi::GetStockObject(nPen as c_int) as HPEN)
		}
	}

	pub fn SelectStockBrush(&self, nBrush: c_uint)->HBRUSH {
//#if (_WIN32_WINNT >= 0x0500)
		debug_assert!((nBrush >= WHITE_BRUSH && nBrush <= HOLLOW_BRUSH) || nBrush == DC_BRUSH);
//#else
//		ATLASSERT(nBrush >= WHITE_BRUSH && nBrush <= HOLLOW_BRUSH);
//#endif // !(_WIN32_WINNT >= 0x0500)
		unsafe{
			self.SelectBrush(um::wingdi::GetStockObject(nBrush as c_int) as HBRUSH)
		}
	}

	pub fn SelectStockFont(&self, nFont: c_uint)->HFONT {
//#ifndef _WIN32_WCE
		debug_assert!((nFont >= OEM_FIXED_FONT && nFont <= SYSTEM_FIXED_FONT) || nFont == DEFAULT_GUI_FONT);
//#else // CE specific
//		ATLASSERT(nFont == SYSTEM_FONT);
//#endif // _WIN32_WCE
		unsafe{
			self.SelectFont(um::wingdi::GetStockObject(nFont as c_int) as HFONT)
		}
	}

	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL)->HPALETTE {
		debug_assert!(nPalette == (DEFAULT_PALETTE as c_int)); // the only one supported
		unsafe{
			self.SelectPalette(um::wingdi::GetStockObject(nPalette) as HPALETTE, bForceBackground)
		}
	}

// Color and Color Palette Functions
	pub fn GetNearestColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetNearestColor(self.hdc, crColor)}
	}

	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL)->HPALETTE {
		self.assert_dc();
		unsafe{um::wingdi::SelectPalette(self.hdc, hPalette, bForceBackground)}
	}

	pub fn RealizePalette (&self)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::RealizePalette(self.hdc)}
	}

//#ifndef _WIN32_WCE
	pub fn UpdateColors (&self) {
		self.assert_dc();
		unsafe{um::wingdi::UpdateColors(self.hdc)};
	}
//#endif // !_WIN32_WCE

// Drawing-Attribute Functions
	pub fn GetBkColor (&self)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetBkColor(self.hdc)}
	}

	pub fn GetBkMode (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetBkMode(self.hdc)}
	}

//#ifndef _WIN32_WCE
	pub fn GetPolyFillMode (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetPolyFillMode(self.hdc)}
	}

	pub fn GetROP2 (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetROP2(self.hdc)}
	}

	pub fn GetStretchBltMode (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetStretchBltMode(self.hdc)}
	}
//#endif // !_WIN32_WCE

	pub fn GetTextColor (&self)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetTextColor(self.hdc)}
	}

	pub fn SetBkColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetBkColor(self.hdc, crColor)}
	}

	pub fn SetBkMode(&self,nBkMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetBkMode(self.hdc, nBkMode)}
	}

//#ifndef _WIN32_WCE
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetPolyFillMode(self.hdc, nPolyFillMode)}
	}
//#endif // !_WIN32_WCE

	pub fn SetROP2(&self,nDrawMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetROP2(self.hdc, nDrawMode)}
	}

//#ifndef _WIN32_WCE
	pub fn SetStretchBltMode(&self,nStretchMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetStretchBltMode(self.hdc, nStretchMode)}
	}
//#endif // !_WIN32_WCE

	pub fn SetTextColor(&self,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetTextColor(self.hdc, crColor)}
	}

//#ifndef _WIN32_WCE
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetColorAdjustment(self.hdc, lpColorAdjust)}
	}

	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SetColorAdjustment(self.hdc, lpColorAdjust)}
	}

// Mapping Functions
	pub fn GetMapMode (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetMapMode(self.hdc)}
	}

	pub fn GetViewportOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetViewportOrgEx(self.hdc, lpPoint)}
	}

	pub fn SetMapMode(&self,nMapMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetMapMode(self.hdc, nMapMode)}
	}
//#endif // !_WIN32_WCE

	// Viewport Origin
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::SetViewportOrgEx(self.hdc, x, y, p)}
	}

	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		//let p = extract_opt_by_null!(lpPointRet,LPPOINT);
		self.SetViewportOrg(point.x, point.y, lpPointRet)
	}

//#ifndef _WIN32_WCE
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::OffsetViewportOrgEx(self.hdc, nWidth, nHeight, p)}
	}

	// Viewport Extent
	pub fn GetViewportExt(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetViewportExtEx(self.hdc, lpSize)}
	}

	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		let sz = extract_opt_by_null!(lpSize,LPSIZE);
		unsafe{um::wingdi::SetViewportExtEx(self.hdc, x, y, sz)}
	}

	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		//let sz = extract_opt_by_null!(lpSizeRet,LPSIZE);
		self.SetViewportExt(size.cx, size.cy, lpSizeRet)
	}

	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		let sz = extract_opt_by_null!(lpSize,LPSIZE);
		unsafe{um::wingdi::ScaleViewportExtEx(self.hdc, xNum, xDenom, yNum, yDenom, sz)}
	}
//#endif // !_WIN32_WCE

	// Window Origin
//#ifndef _WIN32_WCE
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetWindowOrgEx(self.hdc, lpPoint)}
	}

	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::SetWindowOrgEx(self.hdc, x, y, p)}
	}

	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		//let p = extract_opt_by_null!(lpPointRet,LPPOINT);
		self.SetWindowOrg(point.x, point.y, lpPointRet)
	}

	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::OffsetWindowOrgEx(self.hdc, nWidth, nHeight, p)}
	}

	// Window extent
	pub fn GetWindowExt(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetWindowExtEx(self.hdc, lpSize)}
	}

	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		let sz = extract_opt_by_null!(lpSize,LPSIZE);
		unsafe{um::wingdi::SetWindowExtEx(self.hdc, x, y, sz)}
	}

	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		//let sz = extract_opt_by_null!(lpSizeRet,LPSIZE);
		self.SetWindowExt(size.cx, size.cy, lpSizeRet)
	}

	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE> /*=NULL*/)->BOOL {
		self.assert_dc();
		let sz = extract_opt_by_null!(lpSize,LPSIZE);
		unsafe{um::wingdi::ScaleWindowExtEx(self.hdc, xNum, xDenom, yNum, yDenom, sz)}
	}

// Coordinate Functions
	/// for points,overload function
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int> /*=1*/)->BOOL {
		self.assert_dc();
		let n = extract_opt_by_default!(-1,nCount,c_int);
		unsafe{um::wingdi::DPtoLP(self.hdc, lpPoints, n)}
	}

	/// origin name :DPtoLP,overload function ,used for RECT 
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::DPtoLP(self.hdc, lpRect as LPPOINT, 2)}
	}

	/// origin name :DPtoLP,overload function ,used for SIZE 
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE)->BOOL {
		let mut sizeWinExt = SIZE{cx: 0, cy: 0};
		if self.GetWindowExt(&mut sizeWinExt) == winapi::shared::minwindef::FALSE {
			return winapi::shared::minwindef::FALSE;
		}
		let mut sizeVpExt = SIZE{cx: 0, cy: 0};
		if self.GetViewportExt(&mut sizeVpExt) == winapi::shared::minwindef::FALSE {
			return winapi::shared::minwindef::FALSE;
		}
		unsafe{
			let p = &mut *lpSize;
			p.cx = um::winbase::MulDiv(p.cx, abs(sizeWinExt.cx), abs(sizeVpExt.cx));
			p.cy = um::winbase::MulDiv(p.cy, abs(sizeWinExt.cy), abs(sizeVpExt.cy));
		}
		winapi::shared::minwindef::TRUE
	}

	/// for points,overload function
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int> /*=1*/)->BOOL {
		self.assert_dc();
		let n = extract_opt_by_default!(1,nCount,c_int);
		unsafe{um::wingdi::LPtoDP(self.hdc, lpPoints, n)}
	}

	/// origin name :LPtoDP,overload function ,used for RECT 
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::LPtoDP(self.hdc, lpRect as LPPOINT, 2)}
	}

	/// origin name :LPtoDP,overload function ,used for SIZE 
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE)->BOOL {
		let mut sizeWinExt = SIZE{cx: 0, cy: 0};
		if self.GetWindowExt(&mut sizeWinExt) == winapi::shared::minwindef::FALSE {
			return winapi::shared::minwindef::FALSE;
		}
		let mut sizeVpExt = SIZE{cx: 0, cy: 0};
		if self.GetViewportExt(&mut sizeVpExt) == winapi::shared::minwindef::FALSE {
			return winapi::shared::minwindef::FALSE;
		}
		unsafe{
			let p = &mut *lpSize;
			p.cx = um::winbase::MulDiv(p.cx, abs(sizeVpExt.cx), abs(sizeWinExt.cx));
			p.cy = um::winbase::MulDiv(p.cy, abs(sizeVpExt.cy), abs(sizeWinExt.cy));
		}
		winapi::shared::minwindef::TRUE
	}

// Special Coordinate Functions (useful for dealing with metafiles and OLE)
	//#define HIMETRIC_INCH   2540    // HIMETRIC units per inch

	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.assert_dc();
		let nMapMode = self.GetMapMode() as c_uint;
		if (nMapMode < MM_ISOTROPIC) && (nMapMode != MM_TEXT) {
		//if((nMapMode = GetMapMode()) < MM_ISOTROPIC && nMapMode != MM_TEXT)
			// when using a constrained map mode, map against physical inch
			//((CDCHandle*)this)->SetMapMode(MM_HIMETRIC);
			self.SetMapMode(MM_HIMETRIC as c_int);
			self.DPtoLP_Size(lpSize);
			// why use this convertion below?
			//((CDCHandle*)this)->SetMapMode(nMapMode);
			self.SetMapMode(nMapMode as c_int);
		}
		else {
			// map against logical inch for non-constrained mapping modes
			let cxPerInch = self.GetDeviceCaps(LOGPIXELSX);
			let cyPerInch = self.GetDeviceCaps(LOGPIXELSY);
			debug_assert!(cxPerInch != 0 && cyPerInch != 0);
			unsafe{
				let p = &mut *lpSize;
				p.cx = um::winbase::MulDiv(p.cx, HIMETRIC_INCH, cxPerInch);
				p.cy = um::winbase::MulDiv(p.cy, HIMETRIC_INCH, cyPerInch);
			}
		}
	}

	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.assert_dc();
		let nMapMode = self.GetMapMode();
		if (nMapMode < (MM_ISOTROPIC as c_int)) && (nMapMode != (MM_TEXT as c_int)) {
			// when using a constrained map mode, map against physical inch
			self.SetMapMode(MM_HIMETRIC as c_int);
			//((CDCHandle*)this)->SetMapMode(MM_HIMETRIC);
			self.LPtoDP_Size(lpSize);
			//((CDCHandle*)this)->SetMapMode(nMapMode);
			self.SetMapMode(nMapMode);
		}
		else {
			// map against logical inch for non-constrained mapping modes
			let cxPerInch = self.GetDeviceCaps(LOGPIXELSX);
			let cyPerInch = self.GetDeviceCaps(LOGPIXELSY);
			debug_assert!(cxPerInch != 0 && cyPerInch != 0);
			unsafe{
				let p = &mut *lpSize;
				p.cx = um::winbase::MulDiv(p.cx, cxPerInch, HIMETRIC_INCH);
				p.cy = um::winbase::MulDiv(p.cy, cyPerInch, HIMETRIC_INCH);
			}
		}
	}

	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.LPtoDP_Size(lpSize);
		self.DPtoHIMETRIC(lpSize);
	}

	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.HIMETRICtoDP(lpSize);
		self.DPtoLP_Size(lpSize);
	}
//#endif // !_WIN32_WCE

// Region Functions
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::FillRgn(self.hdc, hRgn, hBrush)}
	}

//#ifndef _WIN32_WCE
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::FrameRgn(self.hdc, hRgn, hBrush, nWidth, nHeight)}
	}

	pub fn InvertRgn(&self,hRgn: HRGN)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::InvertRgn(self.hdc, hRgn)}
	}

	pub fn PaintRgn(&self,hRgn: HRGN)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PaintRgn(self.hdc, hRgn)}
	}
//#endif // !_WIN32_WCE

// Clipping Functions
	pub fn GetClipBox(&self,lpRect: LPRECT)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetClipBox(self.hdc, lpRect)}
	}

	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {
	// 	self.assert_dc();
	// 	if region.IsNull() == TRUE {
	// 		region.CreateRectRgn(0, 0, 0, 0);
	// 	}

	// 	let nRet = um::wingdi::GetClipRgn(self.hdc, region) as c_int;
	// 	if nRet != 1 {
	// 		region.DeleteObject();
	// 	}
	// 	nRet
	// }

//#ifndef _WIN32_WCE
	pub fn PtVisible(&self,x: c_int,y: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PtVisible(self.hdc, x, y)}
	}

	pub fn PtVisible_point(&self,point: POINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PtVisible(self.hdc, point.x, point.y)}
	}
//#endif // !_WIN32_WCE

	pub fn RectVisible(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::RectVisible(self.hdc, lpRect)}
	}

	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::ExcludeClipRect(self.hdc, x1, y1, x2, y2)}
	}

	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT)->c_int {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::ExcludeClipRect(self.hdc, p.left, p.top, p.right, p.bottom)
		}
	}

//#ifndef _WIN32_WCE
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND)->c_int {
		self.assert_dc();
		unsafe{um::winuser::ExcludeUpdateRgn(self.hdc, hWnd)}
	}
//#endif // !_WIN32_WCE

	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::IntersectClipRect(self.hdc, x1, y1, x2, y2)}
	}

	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT)->c_int {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::IntersectClipRect(self.hdc, p.left, p.top, p.right, p.bottom)
		}
	}

//#ifndef _WIN32_WCE
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::OffsetClipRgn(self.hdc, x, y)}
	}

	pub fn OffsetClipRgn_size(&self,size: SIZE)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::OffsetClipRgn(self.hdc, size.cx, size.cy)}
	}

	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::ExtSelectClipRgn(self.hdc, hRgn, nMode)}
	}

	pub fn SelectClipRgn(&self,hRgn: HRGN)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SelectClipRgn(self.hdc, hRgn as HRGN)}
	}
//#endif // !_WIN32_WCE

// Line-Output Functions
//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCurrentPositionEx(self.hdc, lpPoint)}
	}

	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let p = extract_opt_by_null!(lpPoint,LPPOINT);
		unsafe{um::wingdi::MoveToEx(self.hdc, x, y, p)}
	}

	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		//let p = extract_opt_by_null!(lpPointRet,LPPOINT);
		self.MoveTo(point.x, point.y, lpPointRet)
	}

	pub fn LineTo(&self,x: c_int,y: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::LineTo(self.hdc, x, y)}
	}

	pub fn LineTo_point(&self,point: POINT)->BOOL {
		self.assert_dc();
		self.LineTo(point.x, point.y)
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

//#ifndef _WIN32_WCE
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Arc(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4)}
	}

	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::Arc(self.hdc, p.left, p.top,p.right, p.bottom, ptStart.x, ptStart.y,ptEnd.x, ptEnd.y)
		}
	}
//#endif // !_WIN32_WCE

	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Polyline(self.hdc, lpPoints, nCount)}
	}

//#ifndef _WIN32_WCE
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::AngleArc(self.hdc, x, y, nRadius as DWORD, fStartAngle, fSweepAngle)}
	}

	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::ArcTo(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4)}
	}

	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			self.ArcTo(p.left, p.top, p.right,p.bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y)
		}
	}

	pub fn GetArcDirection (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetArcDirection(self.hdc)}
	}

	pub fn SetArcDirection(&self,nArcDirection: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetArcDirection(self.hdc, nArcDirection)}
	}

	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolyDraw(self.hdc, lpPoints, lpTypes, nCount)}
	}

	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolylineTo(self.hdc, lpPoints, nCount as DWORD)}
	}

	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolyPolyline(self.hdc, lpPoints, lpPolyPoints, nCount as DWORD)}
	}

	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolyBezier(self.hdc, lpPoints, nCount as DWORD)}
	}

	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolyBezierTo(self.hdc, lpPoints, nCount as DWORD)}
	}
//#endif // !_WIN32_WCE

// Simple Drawing Functions
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::FillRect(self.hdc, lpRect, hBrush)}
	}

	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		unsafe{um::winuser::FillRect(self.hdc, lpRect, LongToPtr(nColorIndex + 1) as HBRUSH)}
//#else // CE specific
//		return ::FillRect(self.hdc, lpRect, ::GetSysColorBrush(nColorIndex));
//#endif // _WIN32_WCE
	}

//#ifndef _WIN32_WCE
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::FrameRect(self.hdc, lpRect, hBrush)}
	}
//#endif // !_WIN32_WCE

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)
	pub fn InvertRect(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::InvertRect(self.hdc, lpRect)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)

	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		unsafe{um::winuser::DrawIcon(self.hdc, x, y, hIcon)}
//#else // CE specific
//		return ::DrawIconEx(self.hdc, x, y, hIcon, 0, 0, 0, NULL, DI_NORMAL);
//#endif // _WIN32_WCE
	}

	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON)->BOOL {
		self.assert_dc();
//#ifndef _WIN32_WCE
		unsafe{um::winuser::DrawIcon(self.hdc, point.x, point.y, hIcon)}
//#else // CE specific
//		return ::DrawIconEx(self.hdc, point.x, point.y, hIcon, 0, 0, 0, NULL, DI_NORMAL);
//#endif // _WIN32_WCE
	}

	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT> /*=0*/, hbrFlickerFreeDraw: Option<HBRUSH> /*=NULL*/, uFlags: Option<UINT> /*=DI_NORMAL*/)->BOOL {
		self.assert_dc();
		let s = extract_opt_by_null!(uStepIfAniCur,UINT);
		let h = extract_opt_by_null!(hbrFlickerFreeDraw,HBRUSH);
		let f = extract_opt_by_default!(DI_NORMAL,uFlags,UINT);
		unsafe{um::winuser::DrawIconEx(self.hdc, x, y, hIcon, cxWidth, cyWidth, s, h, f)}
	}

	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT> /*=0*/, hbrFlickerFreeDraw: Option<HBRUSH> /*=NULL*/, uFlags: Option<UINT> /*=DI_NORMAL*/)->BOOL {
		self.assert_dc();
		let s = extract_opt_by_null!(uStepIfAniCur,UINT);
		let h = extract_opt_by_null!(hbrFlickerFreeDraw,HBRUSH);
		let f = extract_opt_by_default!(DI_NORMAL,uFlags,UINT);
		unsafe{um::winuser::DrawIconEx(self.hdc, point.x, point.y, hIcon, size.cx, size.cy, s, h, f)}
	}

//#ifndef _WIN32_WCE
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH> /*=NULL*/)->BOOL {
		self.assert_dc();
		let h = extract_opt_by_null!(hBrush,HBRUSH);
		unsafe{um::winuser::DrawStateW(self.hdc, h, None, hBitmap as LPARAM, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_BITMAP)}
	}

	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH> /*=NULL*/)->BOOL {
		self.assert_dc();
		let h = extract_opt_by_null!(hBrush,HBRUSH);
		unsafe{um::winuser::DrawStateW(self.hdc, h, None, hIcon as LPARAM, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_ICON)}
	}

	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str,mut nFlags: UINT, bPrefixText: Option<BOOL> /*=TRUE*/, nTextLen: Option<c_int> /*=0*/, hBrush: Option<HBRUSH> /*=NULL*/)->BOOL {
		self.assert_dc();
		let s = lpszText.to_c_u16();
		let b = extract_opt_by_default!(winapi::shared::minwindef::TRUE,bPrefixText,BOOL);
		if b > 0 {
			nFlags |= DST_PREFIXTEXT;
		}else{
			nFlags |= DST_TEXT;
		}
		
		let n = extract_opt_by_null!(nTextLen,c_int);
		let h = extract_opt_by_null!(hBrush,HBRUSH);
		//nFlags | (bPrefixText ? DST_PREFIXTEXT : DST_TEXT)
		unsafe{um::winuser::DrawStateW(self.hdc, h, None, s.as_ptr() as LPARAM, n as WPARAM, pt.x, pt.y, size.cx, size.cy, nFlags)}
	}

	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH> /*=NULL*/)->BOOL {
		self.assert_dc();
		let h = extract_opt_by_null!(hBrush,HBRUSH);
		unsafe{um::winuser::DrawStateW(self.hdc, h, lpDrawProc, lData, 0, pt.x, pt.y, size.cx, size.cy, nFlags | DST_COMPLEX)}
	}
//#endif // !_WIN32_WCE

// Ellipse and Polygon Functions
//#ifndef _WIN32_WCE
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Chord(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4)}
	}

	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::Chord(self.hdc, p.left, p.top, p.right, p.bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y)
		}
	}
//#endif // !_WIN32_WCE

	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.assert_dc();
		unsafe{um::winuser::DrawFocusRect(self.hdc, lpRect);}
	}

	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Ellipse(self.hdc, x1, y1, x2, y2)}
	}

	pub fn Ellipse_rect(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::Ellipse(self.hdc, p.left, p.top, p.right, p.bottom)
		}
	}

//#ifndef _WIN32_WCE
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Pie(self.hdc, x1, y1, x2, y2, x3, y3, x4, y4)}
	}

	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::Pie(self.hdc, p.left, p.top, p.right, p.bottom, ptStart.x, ptStart.y, ptEnd.x, ptEnd.y)
		}
	}
//#endif // !_WIN32_WCE

	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Polygon(self.hdc, lpPoints, nCount)}
	}

//#ifndef _WIN32_WCE
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PolyPolygon(self.hdc, lpPoints, lpPolyCounts, nCount as DWORD)}
	}
//#endif // !_WIN32_WCE

	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::Rectangle(self.hdc, x1, y1, x2, y2)}
	}

	pub fn Rectangle_rect(&self,lpRect: LPCRECT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::Rectangle(self.hdc, p.left, p.top, p.right, p.bottom)
		}
	}

	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::RoundRect(self.hdc, x1, y1, x2, y2, x3, y3)}
	}

	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT)->BOOL {
		self.assert_dc();
		unsafe{
			let p = &*lpRect;
			um::wingdi::RoundRect(self.hdc, p.left, p.top, p.right, p.bottom, point.x, point.y)
		}
	}

// Bitmap Functions
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PatBlt(self.hdc, x, y, nWidth, nHeight, dwRop)}
	}

	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::BitBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, dwRop)}
	}

	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::StretchBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, dwRop)}
	}

	pub fn GetPixel(&self,x: c_int,y: c_int)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetPixel(self.hdc, x, y)}
	}

	pub fn GetPixel_point(&self,point: POINT)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetPixel(self.hdc, point.x, point.y)}
	}

	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetPixel(self.hdc, x, y, crColor)}
	}

	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetPixel(self.hdc, point.x, point.y, crColor)}
	}

//#ifndef _WIN32_WCE
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::FloodFill(self.hdc, x, y, crColor)}
	}

	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::ExtFloodFill(self.hdc, x, y, crColor, nFillType)}
	}
//#endif // !_WIN32_WCE

	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::MaskBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, hMaskBitmap, xMask, yMask, dwRop)}
	}

//#ifndef _WIN32_WCE
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PlgBlt(self.hdc, lpPoint, hSrcDC, xSrc, ySrc, nWidth, nHeight, hMaskBitmap, xMask, yMask)}
	}

	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SetPixelV(self.hdc, x, y, crColor)}
	}

	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SetPixelV(self.hdc, point.x, point.y, crColor)}
	}
//#endif // !_WIN32_WCE

//#if !defined(_ATL_NO_MSIMG) || defined(_WIN32_WCE)
//#ifndef _WIN32_WCE
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GdiTransparentBlt(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, crTransparent)}
	}
//#else // CE specific
	//  fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {
	// 	self.assert_dc();
	// 	um::wingdi::TransparentImage(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, crTransparent)
	// }
//#endif // _WIN32_WCE

//#if (!defined(_WIN32_WCE) || (_WIN32_WCE >= 420))
	pub fn GradientFill (&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GdiGradientFill(self.hdc, pVertices, nVertices, pMeshElements, nMeshElements, dwMode)}
	}

	pub fn GradientFillRect (&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool)->BOOL {
		self.assert_dc();

		let mut arrTvx:[TRIVERTEX; 2] = unsafe {mem::zeroed()};//[mem::zeroed(); 2];

		arrTvx[0].x = rect.left;
		arrTvx[0].y = rect.top;
		arrTvx[0].Red = MAKEWORD(0, GetRValue(clr1));
		arrTvx[0].Green = MAKEWORD(0, GetGValue(clr1));
		arrTvx[0].Blue = MAKEWORD(0, GetBValue(clr1));
		arrTvx[0].Alpha = 0;

		arrTvx[1].x = rect.right;
		arrTvx[1].y = rect.bottom;
		arrTvx[1].Red = MAKEWORD(0, GetRValue(clr2));
		arrTvx[1].Green = MAKEWORD(0, GetGValue(clr2));
		arrTvx[1].Blue = MAKEWORD(0, GetBValue(clr2));
		arrTvx[1].Alpha = 0;

		let mut gr = GRADIENT_RECT{UpperLeft: 0, LowerRight: 1};

		let h = if bHorizontal {
			GRADIENT_FILL_RECT_H
		}else{
			GRADIENT_FILL_RECT_V
		};
		//bHorizontal ? GRADIENT_FILL_RECT_H : GRADIENT_FILL_RECT_V
		unsafe{um::wingdi::GdiGradientFill(self.hdc, &mut arrTvx[0], 2, &mut gr as *mut _ as PVOID, 1, h)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 420)

//#if !defined(_WIN32_WCE) || (_WIN32_WCE > 0x500)
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GdiAlphaBlend(self.hdc, x, y, nWidth, nHeight, hSrcDC, xSrc, ySrc, nSrcWidth, nSrcHeight, bf)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE > 0x500)
//#endif //  !defined(_ATL_NO_MSIMG) || defined(_WIN32_WCE)

// Extra bitmap functions
	// Helper function for painting a disabled toolbar or menu bitmap
	// This function can take either an HBITMAP (for SS) or a DC with 
	//           the bitmap already painted (for cmdbar)
	/*
	BOOL DitherBlt(c_int x, c_int y, c_int nWidth, c_int nHeight, HDC hSrcDC, HBITMAP hBitmap, c_int xSrc, c_int ySrc,
			HBRUSH hBrushBackground = ::GetSysColorBrush(COLOR_3DFACE),
			HBRUSH hBrush3DEffect = ::GetSysColorBrush(COLOR_3DHILIGHT),
			HBRUSH hBrushDisabledImage = ::GetSysColorBrush(COLOR_3DSHADOW))
	{
		debug_assert!(self.hdc != NULL || hBitmap != NULL);
		debug_assert!(nWidth > 0 && nHeight > 0);
		
		// Create a generic DC for all BitBlts
		CDCHandle dc = (hSrcDC != NULL) ? hSrcDC : ::CreateCompatibleDC(self.hdc);
		debug_assert!(dc.self.hdc != NULL);
		if(dc.self.hdc == NULL)
			return FALSE;
		
		// Create a DC for the monochrome DIB section
		CDC dcBW = ::CreateCompatibleDC(self.hdc);
		debug_assert!(dcBW.self.hdc != NULL);
		if(dcBW.self.hdc == NULL)
		{
			if(hSrcDC == NULL)
				dc.DeleteDC();
			return FALSE;
		}

		// Create the monochrome DIB section with a black and white palette
		struct RGBBWBITMAPINFO
		{
			BITMAPINFOHEADER bmiHeader; 
			RGBQUAD bmiColors[2]; 
		};

		RGBBWBITMAPINFO rgbBWBitmapInfo = 
		{
			{ sizeof(BITMAPINFOHEADER), nWidth, nHeight, 1, 1, BI_RGB, 0, 0, 0, 0, 0 },
			{ { 0x00, 0x00, 0x00, 0x00 }, { 0xFF, 0xFF, 0xFF, 0x00 } }
		};

		VOID* pbitsBW;
		CBitmap bmpBW = ::CreateDIBSection(dcBW, (LPBITMAPINFO)&rgbBWBitmapInfo, DIB_RGB_COLORS, &pbitsBW, NULL, 0);
		debug_assert!(bmpBW.m_hBitmap != NULL);
		if(bmpBW.m_hBitmap == NULL)
		{
			if(hSrcDC == NULL)
				dc.DeleteDC();
			return FALSE;
		}
		
		// Attach the monochrome DIB section and the bitmap to the DCs
		HBITMAP hbmOldBW = dcBW.SelectBitmap(bmpBW);
		HBITMAP hbmOldDC = NULL;
		if(hBitmap != NULL)
			hbmOldDC = dc.SelectBitmap(hBitmap);

		// Block: Dark gray removal: we want (128, 128, 128) pixels to become black and not white
		{
			CDC dcTemp1 = ::CreateCompatibleDC(self.hdc);
			CDC dcTemp2 = ::CreateCompatibleDC(self.hdc);
			CBitmap bmpTemp1;
			bmpTemp1.CreateCompatibleBitmap(dc, nWidth, nHeight);
			CBitmap bmpTemp2;
			bmpTemp2.CreateBitmap(nWidth, nHeight, 1, 1, NULL);
			HBITMAP hOldBmp1 = dcTemp1.SelectBitmap(bmpTemp1);
			HBITMAP hOldBmp2 = dcTemp2.SelectBitmap(bmpTemp2);
			// Let's copy our image, it will be altered
			dcTemp1.BitBlt(0, 0, nWidth, nHeight, dc, xSrc, ySrc, SRCCOPY);

			// All dark gray pixels will become white, the others black
			dcTemp1.SetBkColor(RGB(128, 128, 128));
			dcTemp2.BitBlt(0, 0, nWidth, nHeight, dcTemp1, 0, 0, SRCCOPY);
			// Do an XOR to set to black these white pixels
			dcTemp1.BitBlt(0, 0, nWidth, nHeight, dcTemp2, 0, 0, SRCINVERT);

			// BitBlt the bitmap c_into the monochrome DIB section
			// The DIB section will do a true monochrome conversion
			// The magenta background being closer to white will become white
			dcBW.BitBlt(0, 0, nWidth, nHeight, dcTemp1, 0, 0, SRCCOPY);

			// Cleanup
			dcTemp1.SelectBitmap(hOldBmp1);
			dcTemp2.SelectBitmap(hOldBmp2);
		}
		
		// Paint the destination rectangle using hBrushBackground
		if(hBrushBackground != NULL)
		{
			RECT rc = { x, y, x + nWidth, y + nHeight };
			FillRect(&rc, hBrushBackground);
		}

		// BitBlt the black bits in the monochrome bitmap c_into hBrush3DEffect color in the destination DC
		// The magic ROP comes from the Charles Petzold's book
		HBRUSH hOldBrush = SelectBrush(hBrush3DEffect);
		BitBlt(x + 1, y + 1, nWidth, nHeight, dcBW, 0, 0, 0xB8074A);

		// BitBlt the black bits in the monochrome bitmap c_into hBrushDisabledImage color in the destination DC
		SelectBrush(hBrushDisabledImage);
		BitBlt(x, y, nWidth, nHeight, dcBW, 0, 0, 0xB8074A);

		SelectBrush(hOldBrush);
		dcBW.SelectBitmap(hbmOldBW);
		dc.SelectBitmap(hbmOldDC);

		if(hSrcDC == NULL)
			dc.DeleteDC();

		winapi::shared::minwindef::TRUE
	}
	*/
// Text Functions
//#ifndef _WIN32_WCE
	//fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int> /*= -1*/)->BOOL {
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int> /*= -1*/)->BOOL {
		self.assert_dc();
		let s = lpszString.to_c_u16();
		//let l = s.len();
		let n = extract_opt_by_default!(s.len(),nCount,c_int);
		unsafe{um::wingdi::TextOutW(self.hdc, x, y, s.as_ptr(), n)}
	}
//#endif // !_WIN32_WCE

	//fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT> /*= -1*/,mut lpDxWidths: Option<LPINT> /*=NULL*/)->BOOL {
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT> /*= -1*/, lpDxWidths: Option<LPINT> /*=NULL*/)->BOOL {		
		self.assert_dc();
		let s = lpszString.to_c_u16();

		let n = extract_opt_by_default!(s.len(),nCount,UINT);
		// let n = if let Some(n1) = nCount {
		// 	n1
		// }else{
		// 	s.len() as UINT
		// };

		let w = extract_opt_by_null!(lpDxWidths,LPINT);
		unsafe{um::wingdi::ExtTextOutW(self.hdc, x, y, nOptions, lpRect, s.as_ptr(), n, w)}
	}

//#ifndef _WIN32_WCE
	//fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int> /*= -1*/,mut nTabPositions: Option<c_int> /*=0*/,mut lpnTabStopPositions: Option<LPINT> /*=NULL*/,mut nTabOrigin: Option<c_int> /*=0*/)->SIZE {
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int> /*= -1*/, nTabPositions: Option<c_int> /*=0*/, lpnTabStopPositions: Option<LPINT> /*=NULL*/, nTabOrigin: Option<c_int> /*=0*/)->SIZE {
		self.assert_dc();
		let s = lpszString.to_c_u16();
		// let n = if let Some(n1) = nCount {
		// 	n1
		// }else{
		// 	s.len() as c_int
		// };
		let n = extract_opt_by_default!(s.len(),nCount,c_int);
		let pos = extract_opt_by_null!(nTabPositions,c_int);
		let stop_pos = extract_opt_by_null!(lpnTabStopPositions,LPINT);
		let org = extract_opt_by_null!(nTabOrigin,c_int);
		let lRes = unsafe{um::winuser::TabbedTextOutW(self.hdc, x, y, s.as_ptr(), n, pos, stop_pos, org) as LPARAM};
		SIZE{cx: GET_X_LPARAM(lRes), cy: GET_Y_LPARAM(lRes)}
	}
//#endif // !_WIN32_WCE

	//fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
		self.assert_dc();
		let s = lpstrText.to_c_u16();
//#ifndef _WIN32_WCE
		debug_assert!((uFormat & DT_MODIFYSTRING) == 0);
//#endif // !_WIN32_WCE
		unsafe{um::winuser::DrawTextW(self.hdc, s.as_ptr(), cchText, lpRect, uFormat)}
	}

	//  fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	// 	self.assert_dc();
	// 	um::winuser::DrawTextW(self.hdc, lpstrText, cchText, lpRect, uFormat)
	// }

//#ifndef _WIN32_WCE
	//fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS> /*=NULL*/)->c_int {
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS> /*=NULL*/)->c_int {
		self.assert_dc();
		let s = lpstrText.to_c_u16();

		let p = extract_opt_by_null!(lpDTParams,LPDRAWTEXTPARAMS);
		// let p = if let Some(p1) = lpDTParams {
		// 	p1
		// }else {
		// 	0 as LPDRAWTEXTPARAMS
		// }
		unsafe{um::winuser::DrawTextExW(self.hdc, s.as_ptr(), cchText, lpRect, uFormat, p)}
	}
//#endif // !_WIN32_WCE

//#if (_WIN32_WINNT >= 0x0501)
	
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int)->c_int {
		self.assert_dc();
		// This function is present only if comctl32.dll version 6 is loaded;
		// we use LoadLibrary/GetProcAddress to allow apps compiled with
		// _WIN32_WINNT >= 0x0501 to run on older Windows/CommCtrl
		let mut nRet = 0 as c_int;
		let dll_name = "comctl32.dll".to_c_u16();
		unsafe{
			let hCommCtrlDLL = um::libloaderapi::LoadLibraryW(dll_name.as_ptr()) as HMODULE;
			debug_assert!(hCommCtrlDLL != 0 as HMODULE);
			if hCommCtrlDLL != 0 as HMODULE {
				//typedef c_int (WINAPI *PFN_DrawShadowText)(HDC hDC, LPCWSTR lpstrText, UINT cchText, LPRECT lpRect, DWORD dwFlags, COLORREF clrText, COLORREF clrShadow, c_int xOffset, c_int yOffset);
				type PFN_DrawShadowText = fn(hDC: HDC, lpstrText: LPCWSTR, cchText: UINT, lpRect: LPRECT, dwFlags: DWORD, clrText: COLORREF, clrShadow: COLORREF, xOffset: c_int, yOffset: c_int)->c_int;
				//let pname = "DrawShadowText".bytes().map(|x|x as i8).collect::<Vec<_>>();
				let addr = um::libloaderapi::GetProcAddress(hCommCtrlDLL, "DrawShadowText".as_ptr() as *const _);
				debug_assert!(addr != std::ptr::null_mut());   // this function requires CommCtrl6
				if addr != std::ptr::null_mut() {
					let pfnDrawShadowText: PFN_DrawShadowText = mem::transmute(addr);
					nRet = pfnDrawShadowText(self.hdc, lpstrText, cchText as UINT, lpRect, dwFlags, clrText, clrShadow, xOffset, yOffset);
				}
				um::libloaderapi::FreeLibrary(hCommCtrlDLL);
			}
		}
		nRet
	}
	
//#endif // (_WIN32_WINNT >= 0x0501)

	pub fn GetTextExtent(&self,lpszString: &str,mut nCount: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		if nCount == -1 {
			nCount = lpszString.len() as c_int;
		}
		let s = lpszString.to_c_u16();

		unsafe{um::wingdi::GetTextExtentPoint32W(self.hdc, s.as_ptr(), nCount, lpSize)}
	}

	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT> /*=NULL*/, alpDx: Option<LPINT> /*=NULL*/)->BOOL {
		self.assert_dc();
		let s = lpszString.to_c_u16();
		let n = extract_opt_by_null!(lpnFit,LPINT);
		let a = extract_opt_by_null!(alpDx,LPINT);
		unsafe{um::wingdi::GetTextExtentExPointW(self.hdc, s.as_ptr(), cchString, nMaxExtent, n, a, lpSize)}
	}

//#ifndef _WIN32_WCE
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int> /*= -1*/, nTabPositions: Option<c_int> /*=0*/, lpnTabStopPositions: Option<LPINT> /*=NULL*/)->DWORD {
		self.assert_dc();
		let s = lpszString.to_c_u16();

		let c = extract_opt_by_default!(lpszString.len(),nCount,c_int);
		let t = extract_opt_by_default!(0,nTabPositions,c_int);
		let p = extract_opt_by_null!(lpnTabStopPositions,LPINT);
		unsafe{um::winuser::GetTabbedTextExtentW(self.hdc, s.as_ptr(), c, t, p)}
	}

	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::GrayStringW(self.hdc, hBrush, lpfnOutput, lpData, nCount, x, y, nWidth, nHeight)}
	}
//#endif // !_WIN32_WCE

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn GetTextAlign (&self)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::GetTextAlign(self.hdc)}
	}

	pub fn SetTextAlign(&self,nFlags: UINT)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::SetTextAlign(self.hdc, nFlags)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

	//fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {
	fn GetTextFace(&self) -> String {
		self.assert_dc();
		let l = self.GetTextFaceLen();
		if l == 0 {
			return String::new();
		}
		unsafe{
			if l < 128 {
				let mut v = [0u16; 128];
				let l = um::wingdi::GetTextFaceW(self.hdc, l, v.as_mut_ptr());
				String::from_utf16_lossy(v.as_ref())
			}else{
				let mut v: Vec<u16> = Vec::with_capacity(l as usize);
				let l = um::wingdi::GetTextFaceW(self.hdc, l, v.as_mut_ptr());
				String::from_utf16_lossy(v.as_ref())
			}
		}
	}

	pub fn GetTextFaceLen (&self) -> c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetTextFaceW(self.hdc, 0, 0 as LPWSTR)}
	}

	// fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetTextMetricsW(self.hdc, lpMetrics)}
	}

//#ifndef _WIN32_WCE
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetTextJustification(self.hdc, nBreakExtra, nBreakCount)}
	}

	pub fn GetTextCharacterExtra (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetTextCharacterExtra(self.hdc)}
	}

	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetTextCharacterExtra(self.hdc, nCharExtra)}
	}
//#endif // !_WIN32_WCE

// Advanced Drawing
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::DrawEdge(self.hdc, lpRect, nEdge, nFlags)}
	}

	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::DrawFrameControl(self.hdc, lpRect, nType, nState)}
	}

// Scrolling Functions
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT)->BOOL {
		self.assert_dc();
		unsafe{um::winuser::ScrollDC(self.hdc, dx, dy, lpRectScroll, lpRectClip, hRgnUpdate, lpRectUpdate)}
	}

// Font Functions
//#ifndef _WIN32_WCE
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharWidthW(self.hdc, nFirstChar, nLastChar, lpBuffer)}
	}

	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharWidthFloatW(self.hdc, nFirstChar, nLastChar, lpFloatBuffer)}
	}

	// GetCharWidth32 is not supported under Win9x
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharWidth32W(self.hdc, nFirstChar, nLastChar, lpBuffer)}
	}

	pub fn SetMapperFlags(&self,dwFlag: DWORD)->DWORD {
		self.assert_dc();
		unsafe{um::wingdi::SetMapperFlags(self.hdc, dwFlag)}
	}

	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetAspectRatioFilterEx(self.hdc, lpSize)}
	}

	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharABCWidthsW(self.hdc, nFirstChar, nLastChar, lpabc)}
	}

	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD)->DWORD {
		self.assert_dc();
		unsafe{um::wingdi::GetFontData(self.hdc, dwTable, dwOffset, lpData, cbData)}
	}

	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetKerningPairsW(self.hdc, nPairs as DWORD, lpkrnpair) as c_int}
	}

	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::GetOutlineTextMetricsW(self.hdc, cbData, lpotm)}
	}

	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2)->DWORD {
		self.assert_dc();
		unsafe{um::wingdi::GetGlyphOutlineW(self.hdc, nChar, nFormat, lpgm, cbBuffer, lpBuffer, lpmat2)}
	}

	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharABCWidthsFloatW(self.hdc, nFirstChar, nLastChar, lpABCF)}
	}
//#endif // !_WIN32_WCE

// Printer/Device Escape Functions
//#ifndef _WIN32_WCE
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::Escape(self.hdc, nEscape, nCount, lpszInData, lpOutData)}
	}
//#endif // !_WIN32_WCE

	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR)->c_int	{
		self.assert_dc();
		unsafe{um::wingdi::ExtEscape(self.hdc, nEscape, nInputSize, lpszInputData, nOutputSize, lpszOutputData)}
	}

//#ifndef _WIN32_WCE
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::DrawEscape(self.hdc, nEscape, nInputSize, lpszInputData)}
	}
//#endif // !_WIN32_WCE

	// Escape helpers
//#if !defined(_WIN32_WCE) || ((_WIN32_WCE >= 200) && defined(StartDoc))
	pub fn StartDoc_name(&self,lpszDocName: &str)->c_int {
	 	let s = lpszDocName.to_c_u16();
		let mut di: DOCINFOW = unsafe{mem::zeroed()};//DOCINFOW{cbSize: 0, lpszDocName: 0};
		di.cbSize = ::std::mem::size_of::<DOCINFOW>() as i32;
		di.lpszDocName = s.as_ptr();
		self.StartDoc(&mut di)
	}

	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::StartDocW(self.hdc, lpDocInfo)}
	}

	pub fn StartPage (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::StartPage(self.hdc)}
	}

	pub fn EndPage (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::EndPage(self.hdc)}
	}

	pub fn SetAbortProc (&self,lpfn: ABORTPROC )->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetAbortProc(self.hdc, lpfn)}
	}

	pub fn AbortDoc (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::AbortDoc(self.hdc)}
	}

	pub fn EndDoc (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::EndDoc(self.hdc)}
	}
//#endif // !defined(_WIN32_WCE) || ((_WIN32_WCE >= 200) && defined(StartDoc))

// MetaFile Functions
//#ifndef _WIN32_WCE
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.assert_dc();
		if unsafe{um::wingdi::GetDeviceCaps(self.hdc, TECHNOLOGY)} == (DT_METAFILE as c_int) {
			// playing metafile in metafile, just use core windows API
			return unsafe{um::wingdi::PlayMetaFile(self.hdc, hMF)}
		}

		// for special playback, lParam == pDC
		unsafe{um::wingdi::EnumMetaFile(self.hdc, hMF, Some(Self::EnumMetaFileProc), self as *const _ as LPARAM)}
	}

	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::PlayEnhMetaFile(self.hdc, hEnhMetaFile, lpBounds)}
	}

	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GdiComment(self.hdc, nDataSize, pCommentData)}
	}

	
	// Special handling for metafile playback
	unsafe extern "system" fn EnumMetaFileProc(hDC: HDC, pHandleTable: *mut HANDLETABLE, pMetaRec: *mut METARECORD, nHandles: c_int, lParam: LPARAM) -> c_int {
		let pDC = &mut *(lParam as *mut CDCHandle);
		/*
		

		switch (pMetaRec->rdFunction)
		{
		case META_SETMAPMODE:
			pDC->SetMapMode((c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETWINDOWEXT:
			pDC->SetWindowExt((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETWINDOWORG:
			pDC->SetWindowOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETVIEWPORTEXT:
			pDC->SetViewportExt((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETVIEWPORTORG:
			pDC->SetViewportOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SCALEWINDOWEXT:
			pDC->ScaleWindowExt((c_int)(short)pMetaRec->rdParm[3], (c_int)(short)pMetaRec->rdParm[2], 
				(c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SCALEVIEWPORTEXT:
			pDC->ScaleViewportExt((c_int)(short)pMetaRec->rdParm[3], (c_int)(short)pMetaRec->rdParm[2],
				(c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_OFFSETVIEWPORTORG:
			pDC->OffsetViewportOrg((c_int)(short)pMetaRec->rdParm[1], (c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SAVEDC:
			pDC->SaveDC();
			break;
		case META_RESTOREDC:
			pDC->RestoreDC((c_int)(short)pMetaRec->rdParm[0]);
			break;
		case META_SETBKCOLOR:
			pDC->SetBkColor(*(UNALIGNED COLORREF*)&pMetaRec->rdParm[0]);
			break;
		case META_SETTEXTCOLOR:
			pDC->SetTextColor(*(UNALIGNED COLORREF*)&pMetaRec->rdParm[0]);
			break;

		// need to watch out for SelectObject(HFONT), for custom font mapping
		case META_SELECTOBJECT:
			{
				HGDIOBJ hObject = pHandleTable->objectHandle[pMetaRec->rdParm[0]];
				UINT nObjType = ::GetObjectType(hObject);
				if(nObjType == 0)
				{
					// object type is unknown, determine if it is a font
					HFONT hStockFont = ::GetStockObject(SYSTEM_FONT) as HFONT;
					HFONT hFontOld = ::SelectObject(pDC->self.hdc, hStockFont) as HFONT;
					HGDIOBJ hObjOld = ::SelectObject(pDC->self.hdc, hObject);
					if(hObjOld == hStockFont)
					{
						// got the stock object back, so must be selecting a font
						pDC->SelectFont(hObject as HFONT);
						break;  // don't play the default record
					}
					else
					{
						// didn't get the stock object back, so restore everything
						::SelectObject(pDC->self.hdc, hFontOld);
						::SelectObject(pDC->self.hdc, hObjOld);
					}
					// and fall through to PlayMetaFileRecord...
				}
				else if(nObjType == OBJ_FONT)
				{
					// play back as CDCHandle::SelectFont(HFONT)
					pDC->SelectFont(hObject as HFONT);
					break;  // don't play the default record
				}
			}
			// fall through...

		default:
			::PlayMetaFileRecord(hDC, pHandleTable, pMetaRec, nHandles);
			break;
		}
		*/
		1
	}
	
//#endif // !_WIN32_WCE

// Path Functions
//#ifndef _WIN32_WCE
	pub fn AbortPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::AbortPath(self.hdc)}
	}

	pub fn BeginPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::BeginPath(self.hdc)}
	}

	pub fn CloseFigure (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::CloseFigure(self.hdc)}
	}

	pub fn EndPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::EndPath(self.hdc)}
	}

	pub fn FillPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::FillPath(self.hdc)}
	}

	pub fn FlattenPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::FlattenPath(self.hdc)}
	}

	pub fn StrokeAndFillPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::StrokeAndFillPath(self.hdc)}
	}

	pub fn StrokePath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::StrokePath(self.hdc)}
	}

	pub fn WidenPath (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::WidenPath(self.hdc)}
	}

	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetMiterLimit(self.hdc, pfMiterLimit)}
	}

	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SetMiterLimit(self.hdc, fMiterLimit, 0 as PFLOAT)}
	}

	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetPath(self.hdc, lpPoints, lpTypes, nCount)}
	}

	pub fn SelectClipPath(&self,nMode: c_int)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SelectClipPath(self.hdc, nMode)}
	}
//#endif // !_WIN32_WCE

// Misc Helper Functions
	// how to add PASCAL in declaration?
	// fn GetHalftoneBrush()->CBrushHandle {
	// 	let halftoneBrush = 0 as HBRUSH;
	// 	let grayPattern:[WORD;8] = [0;8];
	// 	//for(c_int i = 0; i < 8; i++)
	// 	for i in 0..8 {
	// 		grayPattern[i] = (0x5555 << (i & 1)) as WORD;
	// 	}

	// 	let grayBitmap = um::wingdi::CreateBitmap(8, 8, 1, 1, &grayPattern);
	// 	if grayBitmap != 0 as HBITMAP {
	// 		halftoneBrush = um::wingdi::CreatePatternBrush(grayBitmap);
	// 		um::wingdi::DeleteObject(grayBitmap);
	// 	}
	// 	CBrushHandle(halftoneBrush)
	// }

/*
	pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH> /*=NULL*/,mut hBrushLast: Option<HBRUSH> /*=NULL*/) {
		// first, determine the update region and select it
		CRgn rgnOutside;
		rgnOutside.CreateRectRgnIndirect(lpRect);
		RECT rect = *lpRect;
		::InflateRect(&rect, -size.cx, -size.cy);
		::c_intersectRect(&rect, &rect, lpRect);
		CRgn rgnInside;
		rgnInside.CreateRectRgnIndirect(&rect);
		CRgn rgnNew;
		rgnNew.CreateRectRgn(0, 0, 0, 0);
		rgnNew.CombineRgn(rgnOutside, rgnInside, RGN_XOR);

		HBRUSH hBrushOld = NULL;
		CBrush brushHalftone;
		if(hBrush == NULL)
			brushHalftone = hBrush = CDCHandle::GetHalftoneBrush();
		if(hBrushLast == NULL)
			hBrushLast = hBrush;

		CRgn rgnLast;
		CRgn rgnUpdate;
		if(lpRectLast != NULL)
		{
			// find difference between new region and old region
			rgnLast.CreateRectRgn(0, 0, 0, 0);
			rgnOutside.SetRectRgn(lpRectLast.left, lpRectLast.top, lpRectLast.right, lpRectLast.bottom);
			rect = *lpRectLast;
			::InflateRect(&rect, -sizeLast.cx, -sizeLast.cy);
			::c_intersectRect(&rect, &rect, lpRectLast);
			rgnInside.SetRectRgn(rect.left, rect.top, rect.right, rect.bottom);
			rgnLast.CombineRgn(rgnOutside, rgnInside, RGN_XOR);

			// only diff them if brushes are the same
			if(hBrush == hBrushLast)
			{
				rgnUpdate.CreateRectRgn(0, 0, 0, 0);
				rgnUpdate.CombineRgn(rgnLast, rgnNew, RGN_XOR);
			}
		}
		if(hBrush != hBrushLast && lpRectLast != NULL)
		{
			// brushes are different -- erase old region first
			SelectClipRgn(rgnLast);
			GetClipBox(&rect);
			hBrushOld = SelectBrush(hBrushLast);
			PatBlt(rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, PATINVERT);
			SelectBrush(hBrushOld);
			hBrushOld = NULL;
		}

		// draw c_into the update/new region
		SelectClipRgn(rgnUpdate.IsNull() ? rgnNew : rgnUpdate);
		GetClipBox(&rect);
		hBrushOld = SelectBrush(hBrush);
		PatBlt(rect.left, rect.top, rect.right - rect.left, rect.bottom - rect.top, PATINVERT);

		// cleanup DC
		if(hBrushOld != NULL)
			SelectBrush(hBrushOld);
		SelectClipRgn(NULL);
	}
*/
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.assert_dc();

		let clrOld = unsafe{um::wingdi::SetBkColor(self.hdc, clr) as COLORREF};
		debug_assert!(clrOld != CLR_INVALID);
		if clrOld != CLR_INVALID {
			unsafe{um::wingdi::ExtTextOutW(self.hdc, 0, 0, ETO_OPAQUE, lpRect, 0 as LPCWSTR, 0, 0 as *const winapi::shared::minwindef::INT)};
			unsafe{um::wingdi::SetBkColor(self.hdc, clrOld)};
		}
	}

	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.assert_dc();

		let rect = RECT{ left: x, top: y, right: x + cx, bottom: y + cy };
		self.FillSolidRect_rect(&rect, clr);
	}

	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		unsafe{
			let p = &*lpRect;
			self.Draw3dRect(p.left, p.top, p.right - p.left, p.bottom - p.top, clrTopLeft, clrBottomRight);
		}
	}

	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.FillSolidRect(x, y, cx - 1, 1, clrTopLeft);
		self.FillSolidRect(x, y, 1, cy - 1, clrTopLeft);
		self.FillSolidRect(x + cx, y, -1, cy, clrBottomRight);
		self.FillSolidRect(x, y + cy, cx, -1, clrBottomRight);
	}

// DIB support
//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::SetDIBitsToDevice(self.hdc, x, y, dwWidth, dwHeight, xSrc, ySrc, uStartScan, cScanLines, lpvBits, lpbmi, uColorUse)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 410)

//#if !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::StretchDIBits(self.hdc, x, y, nWidth, nHeight, xSrc, ySrc, nSrcWidth, nSrcHeight, lpvBits, lpbmi, uColorUse, dwRop)}
	}

	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::GetDIBColorTable(self.hdc, uStartIndex, cEntries, pColors as *mut RGBQUAD)}
	}

	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD)->UINT {
		self.assert_dc();
		unsafe{um::wingdi::SetDIBColorTable(self.hdc, uStartIndex, cEntries, pColors)}
	}
//#endif // !defined(_WIN32_WCE) || (_WIN32_WCE >= 400)

// OpenGL support
//#if !defined(_ATL_NO_OPENGL) && !defined(_WIN32_WCE)
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::ChoosePixelFormat(self.hdc, ppfd)}
	}

	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::DescribePixelFormat(self.hdc, iPixelFormat, nBytes, ppfd)}
	}

	pub fn GetPixelFormat (&self)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::GetPixelFormat(self.hdc)}
	}

	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SetPixelFormat(self.hdc, iPixelFormat, ppfd)}
	}

	pub fn SwapBuffers (&self)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::SwapBuffers(self.hdc)}
	}

	pub fn wglCreateContext (&self)->HGLRC {
		self.assert_dc();
		unsafe{um::wingdi::wglCreateContext(self.hdc)}
	}

	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int)->HGLRC {
		self.assert_dc();
		unsafe{um::wingdi::wglCreateLayerContext(self.hdc, iLayerPlane)}
	}

	pub fn wglMakeCurrent(&self,hglrc: HGLRC)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglMakeCurrent(self.hdc, hglrc)}
	}

	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglUseFontBitmapsW(self.hdc, dwFirst, dwCount, listBase)}
	}

	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglUseFontOutlinesW(self.hdc, dwFirst, dwCount, listBase, deviation, extrusion, format, lpgmf)}
	}

	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglDescribeLayerPlane(self.hdc, iPixelFormat, iLayerPlane, nBytes, plpd)}
	}

	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::wglSetLayerPaletteEntries(self.hdc, iLayerPlane, iStart, cEntries, pclr)}
	}

	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF)->c_int {
		self.assert_dc();
		unsafe{um::wingdi::wglGetLayerPaletteEntries(self.hdc, iLayerPlane, iStart, cEntries, pclr)}
	}

	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglRealizeLayerPalette(self.hdc, iLayerPlane, bRealize)}
	}

	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::wglSwapLayerBuffers(self.hdc, uPlanes)}
	}
//#endif // !defined(_ATL_NO_OPENGL) && !defined(_WIN32_WCE)

// New for Windows 2000 only
//#if (_WIN32_WINNT >= 0x0500)
	pub fn GetDCPenColor (&self)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetDCPenColor(self.hdc)}
	}

	pub fn SetDCPenColor(&self,clr: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetDCPenColor(self.hdc, clr)}
	}

	pub fn GetDCBrushColor (&self)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::GetDCBrushColor(self.hdc)}
	}

	pub fn SetDCBrushColor(&self,clr: COLORREF)->COLORREF {
		self.assert_dc();
		unsafe{um::wingdi::SetDCBrushColor(self.hdc, clr)}
	}

//#ifndef _WIN32_WCE
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET)->DWORD {
		self.assert_dc();
		unsafe{um::wingdi::GetFontUnicodeRanges(self.hdc, lpgs)}
	}
//#endif // !_WIN32_WCE

	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD)->DWORD {
		self.assert_dc();
		let s = lpstr.to_c_u16();
		unsafe{um::wingdi::GetGlyphIndicesW(self.hdc, s.as_ptr(), cch, pgi, dwFlags)}
	}

	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetTextExtentPointI(self.hdc, pgiIn, cgi, lpSize)}
	}

	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetTextExtentExPointI(self.hdc, pgiIn, cgi, nMaxExtent, lpnFit, alpDx, lpSize)}
	}

	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharWidthI(self.hdc, giFirst, cgi, pgi, lpBuffer)}
	}

	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::GetCharABCWidthsI(self.hdc, giFirst, cgi, pgi, lpabc)}
	}
//#endif // (_WIN32_WINNT >= 0x0500)

// New for Windows 2000 and Windows 98
//#if (WINVER >= 0x0500) && !defined(_WIN32_WCE)
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD)->BOOL {
		self.assert_dc();
		unsafe{um::wingdi::ColorCorrectPalette(self.hdc, hPalette, dwFirstEntry, dwNumOfEntries)}
	}
//#endif // (WINVER >= 0x0500) && !defined(_WIN32_WCE)
}

//BaseTsd.h
#[inline]
fn PtrToInt(p: *const c_void) -> c_int {
	p as INT_PTR as c_int
}

#[inline]
fn LongToPtr(l: LONG) -> *const c_void {
    l as LONG_PTR as *const c_void
}

const HIMETRIC_INCH: c_int = 2540;

fn abs(i: c_int) -> c_int {
	if i < 0 {-i} else { i }
}

#[inline]
fn GetRValue(rgb: COLORREF) -> u8 {
	LOBYTE(rgb as WORD)
}

#[inline]
fn GetGValue(rgb: COLORREF) -> u8 {
	LOBYTE((rgb as WORD) >> 8)
}

#[inline]
fn GetBValue(rgb: COLORREF) -> u8 {
	LOBYTE((rgb>>16) as WORD)
}

pub const NULL_HDC: HDC = 0 as HDC;
