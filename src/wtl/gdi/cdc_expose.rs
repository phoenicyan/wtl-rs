//! racer doesn't support macro yet,so expose all functions in Inner by hand
#![allow(non_snake_case)]

use winapi::ctypes::*;
use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use super::{CDC,CDCHandle,CPaintDC,CClientDC,CWindowDC};


impl CDC {
	//  pub fn Attach(&self,hDC: HDC) {

	#[inline(always)]
	pub fn Detach(&mut self) -> HDC {
		self.inner.Detach()
	}

	#[inline(always)]
	pub fn assert_dc(&self) {
		self.inner.assert_dc()
	}

	#[inline(always)]
	pub fn assert_null_dc(&self) {
		self.inner.assert_null_dc()
	}
	//pub fn HDC (&self)->operator { return self.inner.hdc; }

	#[inline(always)]
	pub fn IsNull(&self) -> bool {
		self.inner.IsNull()
	}

	#[inline(always)]
	pub fn WindowFromDC(&self) -> HWND {
		self.inner.WindowFromDC()
	}
	// pub fn GetCurrentPen (&self)->CPenHandle {
	// pub fn GetCurrentBrush (&self)->CBrushHandle {
	// pub fn GetCurrentPalette (&self)->CPaletteHandle {
	// pub fn GetCurrentFont (&self)->CFontHandle {
	// pub fn GetCurrentBitmap (&self)->CBitmapHandle {
	//  pub fn CreateDC(&self,lpszDriverName: LPCTSTR,lpszDeviceName: LPCTSTR,lpszOutput: LPCTSTR,lpInitData: &DEVMODE)->HDC {

	#[inline(always)]
	pub fn CreateCompatibleDC(&mut self, hDC: Option<HDC>) -> HDC {
		self.inner.CreateCompatibleDC(hDC)
	}

	#[inline(always)]
	pub fn DeleteDC(&mut self) -> BOOL {
		self.inner.DeleteDC()
	}

	#[inline(always)]
	pub fn SaveDC(&self) -> c_int {
		self.inner.SaveDC()
	}

	#[inline(always)]
	pub fn RestoreDC(&self,nSavedDC: c_int) -> BOOL {
		self.inner.RestoreDC(nSavedDC)
	}

	#[inline(always)]
	pub fn GetDeviceCaps(&self,nIndex: c_int) -> c_int {
		self.inner.GetDeviceCaps(nIndex)
	}

	#[inline(always)]
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT) -> UINT {
		self.inner.SetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT) -> UINT {
		self.inner.GetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn ResetDC(&self,lpDevMode: &DEVMODEW) -> BOOL {
		self.inner.ResetDC(lpDevMode)
	}

	#[inline(always)]
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetBrushOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg_point(point,lpPointRet)
	}
	// 	 pub fn EnumObjects(&self,nObjectType: c_int,@ c_int (CALLBACK* lpfn)(LPVOID,@ LPARAM),lpData: LPARAM)->c_int {

	#[inline(always)]
	pub fn SelectPen(&self,hPen: HPEN) -> HPEN {
		self.inner.SelectPen(hPen)
	}

	#[inline(always)]
	pub fn SelectBrush(&self,hBrush: HBRUSH) -> HBRUSH {
		self.inner.SelectBrush(hBrush)
	}

	#[inline(always)]
	pub fn SelectFont(&self,hFont: HFONT) -> HFONT {
		self.inner.SelectFont(hFont)
	}

	#[inline(always)]
	pub fn SelectBitmap(&self,hBitmap: HBITMAP) -> HBITMAP {
		self.inner.SelectBitmap(hBitmap)
	}

	#[inline(always)]
	pub fn SelectRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectRgn(hRgn)
	}

	#[inline(always)]
	pub fn SelectStockPen(&self,nPen: c_int) -> HPEN {
		self.inner.SelectStockPen((nPen as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockBrush(&self,nBrush: c_int) -> HBRUSH {
		self.inner.SelectStockBrush((nBrush as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockFont(&self,nFont: c_int) -> HFONT {
		self.inner.SelectStockFont((nFont as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectStockPalette(nPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn GetNearestColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.GetNearestColor(crColor)
	}

	#[inline(always)]
	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectPalette(hPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn RealizePalette(&self) -> UINT {
		self.inner.RealizePalette()
	}
		pub fn UpdateColors(&self) {
		self.inner.UpdateColors()
	}

	#[inline(always)]
	pub fn GetBkColor(&self) -> COLORREF {
		self.inner.GetBkColor()
	}

	#[inline(always)]
	pub fn GetBkMode(&self) -> c_int {
		self.inner.GetBkMode()
	}

	#[inline(always)]
	pub fn GetPolyFillMode(&self) -> c_int {
		self.inner.GetPolyFillMode()
	}

	#[inline(always)]
	pub fn GetROP2(&self) -> c_int {
		self.inner.GetROP2()
	}

	#[inline(always)]
	pub fn GetStretchBltMode(&self) -> c_int {
		self.inner.GetStretchBltMode()
	}

	#[inline(always)]
	pub fn GetTextColor(&self) -> COLORREF {
		self.inner.GetTextColor()
	}

	#[inline(always)]
	pub fn SetBkColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetBkColor(crColor)
	}

	#[inline(always)]
	pub fn SetBkMode(&self,nBkMode: c_int) -> c_int {
		self.inner.SetBkMode(nBkMode)
	}

	#[inline(always)]
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int) -> c_int {
		self.inner.SetPolyFillMode(nPolyFillMode)
	}

	#[inline(always)]
	pub fn SetROP2(&self,nDrawMode: c_int) -> c_int {
		self.inner.SetROP2(nDrawMode)
	}

	#[inline(always)]
	pub fn SetStretchBltMode(&self,nStretchMode: c_int) -> c_int {
		self.inner.SetStretchBltMode(nStretchMode)
	}

	#[inline(always)]
	pub fn SetTextColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetTextColor(crColor)
	}

	#[inline(always)]
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT) -> BOOL {
		self.inner.GetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT) -> BOOL {
		self.inner.SetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn GetMapMode(&self) -> c_int {
		self.inner.GetMapMode()
	}

	#[inline(always)]
	pub fn GetViewportOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetViewportOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetMapMode(&self,nMapMode: c_int) -> c_int {
		self.inner.SetMapMode(nMapMode)
	}

	#[inline(always)]
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetViewportOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetViewportExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetViewportExt(lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleViewportExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetWindowOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetWindowOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetWindowExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetWindowExt(lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleWindowExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.DPtoLP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.DPtoLP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.DPtoLP_Size(lpSize)
	}

	#[inline(always)]
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.LPtoDP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.LPtoDP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.LPtoDP_Size(lpSize)
	}

	#[inline(always)]
	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.DPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoDP(lpSize)
	}

	#[inline(always)]
	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.LPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoLP(lpSize)
	}

	#[inline(always)]
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRgn(hRgn,hBrush)
	}

	#[inline(always)]
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.FrameRgn(hRgn,hBrush,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn InvertRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.InvertRgn(hRgn)
	}

	#[inline(always)]
	pub fn PaintRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.PaintRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetClipBox(&self,lpRect: LPRECT) -> c_int {
		self.inner.GetClipBox(lpRect)
	}
	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {

	#[inline(always)]
	pub fn PtVisible(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.PtVisible(x,y)
	}

	#[inline(always)]
	pub fn PtVisible_point(&self,point: POINT) -> BOOL {
		self.inner.PtVisible_point(point)
	}

	#[inline(always)]
	pub fn RectVisible(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.RectVisible(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.ExcludeClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.ExcludeClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND) -> c_int {
		self.inner.ExcludeUpdateRgn(hWnd)
	}

	#[inline(always)]
	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.IntersectClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.IntersectClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int) -> c_int {
		self.inner.OffsetClipRgn(x,y)
	}

	#[inline(always)]
	pub fn OffsetClipRgn_size(&self,size: SIZE) -> c_int {
		self.inner.OffsetClipRgn_size(size)
	}

	#[inline(always)]
	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int) -> c_int {
		self.inner.SelectClipRgn_mode(hRgn,nMode)
	}

	#[inline(always)]
	pub fn SelectClipRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectClipRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetCurrentPosition(lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn LineTo(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.LineTo(x,y)
	}

	#[inline(always)]
	pub fn LineTo_point(&self,point: POINT) -> BOOL {
		self.inner.LineTo_point(point)
	}

	#[inline(always)]
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Arc(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Arc_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polyline(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT) -> BOOL {
		self.inner.AngleArc(x,y,nRadius,fStartAngle,fSweepAngle)
	}

	#[inline(always)]
	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.ArcTo(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.ArcTo_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn GetArcDirection(&self) -> c_int {
		self.inner.GetArcDirection()
	}

	#[inline(always)]
	pub fn SetArcDirection(&self,nArcDirection: c_int) -> c_int {
		self.inner.SetArcDirection(nArcDirection)
	}

	#[inline(always)]
	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int) -> BOOL {
		self.inner.PolyDraw(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolylineTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int) -> BOOL {
		self.inner.PolyPolyline(lpPoints,lpPolyPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezier(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezierTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int) -> BOOL {
		self.inner.FillRect_index(lpRect,nColorIndex)
	}

	#[inline(always)]
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FrameRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn InvertRect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.InvertRect(lpRect)
	}

	#[inline(always)]
	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon(x,y,hIcon)
	}

	#[inline(always)]
	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon_point(point,hIcon)
	}

	#[inline(always)]
	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx(x,y,hIcon,cxWidth,cyWidth,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx_point(point,hIcon,size,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_bitmap(pt,size,hBitmap,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState(pt,size,hIcon,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str, nFlags: UINT, bPrefixText: Option<BOOL>, nTextLen: Option<c_int>, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_text(pt,size,lpszText, nFlags, bPrefixText, nTextLen, hBrush)
	}

	#[inline(always)]
	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_proc(pt,size,lpDrawProc,lData,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Chord(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Chord_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.inner.DrawFocusRect(lpRect)
	}

	#[inline(always)]
	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Ellipse(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Ellipse_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Ellipse_rect(lpRect)
	}

	#[inline(always)]
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Pie(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Pie_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polygon(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int) -> BOOL {
		self.inner.PolyPolygon(lpPoints,lpPolyCounts,nCount)
	}

	#[inline(always)]
	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Rectangle(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Rectangle_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Rectangle_rect(lpRect)
	}

	#[inline(always)]
	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int) -> BOOL {
		self.inner.RoundRect(x1,y1,x2,y2,x3,y3)
	}

	#[inline(always)]
	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT) -> BOOL {
		self.inner.RoundRect_rect(lpRect,point)
	}

	#[inline(always)]
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.PatBlt(x,y,nWidth,nHeight,dwRop)
	}

	#[inline(always)]
	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD) -> BOOL {
		self.inner.BitBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,dwRop)
	}

	#[inline(always)]
	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.StretchBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,dwRop)
	}

	#[inline(always)]
	pub fn GetPixel(&self,x: c_int,y: c_int) -> COLORREF {
		self.inner.GetPixel(x,y)
	}

	#[inline(always)]
	pub fn GetPixel_point(&self,point: POINT) -> COLORREF {
		self.inner.GetPixel_point(point)
	}

	#[inline(always)]
	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel_point(point,crColor)
	}

	#[inline(always)]
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.FloodFill(x,y,crColor)
	}

	#[inline(always)]
	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT) -> BOOL {
		self.inner.ExtFloodFill(x,y,crColor,nFillType)
	}

	#[inline(always)]
	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD) -> BOOL {
		self.inner.MaskBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,hMaskBitmap,xMask,yMask,dwRop)
	}

	#[inline(always)]
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int) -> BOOL {
		self.inner.PlgBlt(lpPoint,hSrcDC,xSrc,ySrc,nWidth,nHeight,hMaskBitmap,xMask,yMask)
	}

	#[inline(always)]
	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV_point(point,crColor)
	}

	#[inline(always)]
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT) -> BOOL {
		self.inner.TransparentBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,crTransparent)
	}
	//  pub fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {

	#[inline(always)]
	pub fn GradientFill(&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD) -> BOOL {
		self.inner.GradientFill(pVertices,nVertices,pMeshElements,nMeshElements,dwMode)
	}

	#[inline(always)]
	pub fn GradientFillRect(&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool) -> BOOL {
		self.inner.GradientFillRect(rect,clr1,clr2,bHorizontal)
	}

	#[inline(always)]
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION) -> BOOL {
		self.inner.AlphaBlend(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,bf)
	}
	//pub fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>)->BOOL {

	#[inline(always)]
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>) -> BOOL {
		self.inner.TextOut(x,y,lpszString,nCount)
	}
	//pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT>,mut lpDxWidths: Option<LPINT>)->BOOL {

	#[inline(always)]
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT>, lpDxWidths: Option<LPINT>) -> BOOL {
		self.inner.ExtTextOut(x,y,nOptions,lpRect,lpszString,nCount,lpDxWidths)
	}
	//pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>,mut nTabPositions: Option<c_int>,mut lpnTabStopPositions: Option<LPINT>,mut nTabOrigin: Option<c_int>)->SIZE {

	#[inline(always)]
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>, nTabOrigin: Option<c_int>) -> SIZE {
		self.inner.TabbedTextOut(x,y,lpszString,nCount,nTabPositions,lpnTabStopPositions,nTabOrigin)
	}
	//pub fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {

	#[inline(always)]
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT) -> c_int {
		self.inner.DrawText(lpstrText,cchText,lpRect,uFormat)
	}
	//  pub fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	//pub fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS>)->c_int {

	#[inline(always)]
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS>) -> c_int {
		self.inner.DrawTextEx(lpstrText,cchText,lpRect,uFormat,lpDTParams)
	}

	#[inline(always)]
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int) -> c_int {
		self.inner.DrawShadowText(lpstrText,cchText,lpRect,dwFlags,clrText,clrShadow,xOffset,yOffset)
	}

	#[inline(always)]
	pub fn GetTextExtent(&self,lpszString: &str, nCount: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtent(lpszString, nCount, lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT>, alpDx: Option<LPINT>) -> BOOL {
		self.inner.GetTextExtentExPoint(lpszString,cchString,lpSize,nMaxExtent,lpnFit,alpDx)
	}

	#[inline(always)]
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>) -> DWORD {
		self.inner.GetTabbedTextExtent(lpszString,nCount,nTabPositions,lpnTabStopPositions)
	}

	#[inline(always)]
	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.GrayString(hBrush,lpfnOutput,lpData,nCount,x,y,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn GetTextAlign(&self) -> UINT {
		self.inner.GetTextAlign()
	}

	#[inline(always)]
	pub fn SetTextAlign(&self,nFlags: UINT) -> UINT {
		self.inner.SetTextAlign(nFlags)
	}
	// pub fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {

	#[inline(always)]
	pub fn GetTextFaceLen(&self) -> c_int {
		self.inner.GetTextFaceLen()
	}
	// pub fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// pub fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	#[inline(always)]
	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW) -> BOOL {
		self.inner.GetTextMetrics(lpMetrics)
	}

	#[inline(always)]
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int) -> c_int {
		self.inner.SetTextJustification(nBreakExtra,nBreakCount)
	}

	#[inline(always)]
	pub fn GetTextCharacterExtra(&self) -> c_int {
		self.inner.GetTextCharacterExtra()
	}

	#[inline(always)]
	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int) -> c_int {
		self.inner.SetTextCharacterExtra(nCharExtra)
	}

	#[inline(always)]
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT) -> BOOL {
		self.inner.DrawEdge(lpRect,nEdge,nFlags)
	}

	#[inline(always)]
	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT) -> BOOL {
		self.inner.DrawFrameControl(lpRect,nType,nState)
	}

	#[inline(always)]
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT) -> BOOL {
		self.inner.ScrollDC(dx,dy,lpRectScroll,lpRectClip,hRgnUpdate,lpRectUpdate)
	}

	#[inline(always)]
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT) -> BOOL {
		self.inner.GetCharWidth_float(nFirstChar,nLastChar,lpFloatBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth32(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn SetMapperFlags(&self,dwFlag: DWORD) -> DWORD {
		self.inner.SetMapperFlags(dwFlag)
	}

	#[inline(always)]
	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetAspectRatioFilter(lpSize)
	}

	#[inline(always)]
	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidths(nFirstChar,nLastChar,lpabc)
	}

	#[inline(always)]
	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD) -> DWORD {
		self.inner.GetFontData(dwTable,dwOffset,lpData,cbData)
	}

	#[inline(always)]
	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR) -> c_int {
		self.inner.GetKerningPairs(nPairs,lpkrnpair)
	}

	#[inline(always)]
	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW) -> UINT {
		self.inner.GetOutlineTextMetrics(cbData,lpotm)
	}

	#[inline(always)]
	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2) -> DWORD {
		self.inner.GetGlyphOutline(nChar,nFormat,lpgm,cbBuffer,lpBuffer,lpmat2)
	}

	#[inline(always)]
	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT) -> BOOL {
		self.inner.GetCharABCWidths_float(nFirstChar,nLastChar,lpABCF)
	}

	#[inline(always)]
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID) -> c_int {
		self.inner.Escape(nEscape,nCount,lpszInData,lpOutData)
	}

	#[inline(always)]
	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR) -> c_int {
		self.inner.Escape_ext(nEscape,nInputSize,lpszInputData,nOutputSize,lpszOutputData)
	}

	#[inline(always)]
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR) -> c_int {
		self.inner.DrawEscape(nEscape,nInputSize,lpszInputData)
	}

	#[inline(always)]
	pub fn StartDoc_name(&self,lpszDocName: &str) -> c_int {
		self.inner.StartDoc_name(lpszDocName)
	}

	#[inline(always)]
	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW) -> c_int {
		self.inner.StartDoc(lpDocInfo)
	}

	#[inline(always)]
	pub fn StartPage(&self) -> c_int {
		self.inner.StartPage()
	}

	#[inline(always)]
	pub fn EndPage(&self) -> c_int {
		self.inner.EndPage()
	}

	#[inline(always)]
	pub fn SetAbortProc(&self,lpfn: ABORTPROC ) -> c_int {
		self.inner.SetAbortProc(lpfn)
	}

	#[inline(always)]
	pub fn AbortDoc(&self) -> c_int {
		self.inner.AbortDoc()
	}

	#[inline(always)]
	pub fn EndDoc(&self) -> c_int {
		self.inner.EndDoc()
	}
	
	#[inline(always)]
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.inner.PlayMetaFile(hMF)
	}

	#[inline(always)]
	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT) -> BOOL {
		self.inner.PlayMetaFile_enh(hEnhMetaFile,lpBounds)
	}

	#[inline(always)]
	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE) -> BOOL {
		self.inner.AddMetaFileComment(nDataSize,pCommentData)
	}

	#[inline(always)]
	pub fn AbortPath(&self) -> BOOL {
		self.inner.AbortPath()
	}

	#[inline(always)]
	pub fn BeginPath(&self) -> BOOL {
		self.inner.BeginPath()
	}

	#[inline(always)]
	pub fn CloseFigure(&self) -> BOOL {
		self.inner.CloseFigure()
	}

	#[inline(always)]
	pub fn EndPath(&self) -> BOOL {
		self.inner.EndPath()
	}

	#[inline(always)]
	pub fn FillPath(&self) -> BOOL {
		self.inner.FillPath()
	}

	#[inline(always)]
	pub fn FlattenPath(&self) -> BOOL {
		self.inner.FlattenPath()
	}

	#[inline(always)]
	pub fn StrokeAndFillPath(&self) -> BOOL {
		self.inner.StrokeAndFillPath()
	}

	#[inline(always)]
	pub fn StrokePath(&self) -> BOOL {
		self.inner.StrokePath()
	}

	#[inline(always)]
	pub fn WidenPath(&self) -> BOOL {
		self.inner.WidenPath()
	}

	#[inline(always)]
	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT) -> BOOL {
		self.inner.GetMiterLimit(pfMiterLimit)
	}

	#[inline(always)]
	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT) -> BOOL {
		self.inner.SetMiterLimit(fMiterLimit)
	}

	#[inline(always)]
	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int) -> c_int {
		self.inner.GetPath(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn SelectClipPath(&self,nMode: c_int) -> BOOL {
		self.inner.SelectClipPath(nMode)
	}
	// pub fn GetHalftoneBrush()->CBrushHandle {

	// #[inline(always)]
	// pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH>,mut hBrushLast: Option<HBRUSH>) {
	// 	self.inner.DrawDragRect(lpRect,size,lpRectLast,sizeLast, hBrush, hBrushLast)
	// }

	#[inline(always)]
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.inner.FillSolidRect_rect(lpRect,clr)
	}

	#[inline(always)]
	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.inner.FillSolidRect(x,y,cx,cy,clr)
	}

	#[inline(always)]
	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect_rect(lpRect,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect(x,y,cx,cy,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT) -> c_int {
		self.inner.SetDIBitsToDevice(x,y,dwWidth,dwHeight,xSrc,ySrc,uStartScan,cScanLines,lpvBits,lpbmi,uColorUse)
	}

	#[inline(always)]
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD) -> c_int {
		self.inner.StretchDIBits(x,y,nWidth,nHeight,xSrc,ySrc,nSrcWidth,nSrcHeight,lpvBits,lpbmi,uColorUse,dwRop)
	}

	#[inline(always)]
	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD) -> UINT {
		self.inner.GetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD) -> UINT {
		self.inner.SetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.ChoosePixelFormat(ppfd)
	}

	#[inline(always)]
	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.DescribePixelFormat(iPixelFormat,nBytes,ppfd)
	}

	#[inline(always)]
	pub fn GetPixelFormat(&self) -> c_int {
		self.inner.GetPixelFormat()
	}

	#[inline(always)]
	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR) -> BOOL {
		self.inner.SetPixelFormat(iPixelFormat,ppfd)
	}

	#[inline(always)]
	pub fn SwapBuffers(&self) -> BOOL {
		self.inner.SwapBuffers()
	}

	#[inline(always)]
	pub fn wglCreateContext(&self) -> HGLRC {
		self.inner.wglCreateContext()
	}

	#[inline(always)]
	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int) -> HGLRC {
		self.inner.wglCreateLayerContext(iLayerPlane)
	}

	#[inline(always)]
	pub fn wglMakeCurrent(&self,hglrc: HGLRC) -> BOOL {
		self.inner.wglMakeCurrent(hglrc)
	}

	#[inline(always)]
	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD) -> BOOL {
		self.inner.wglUseFontBitmaps(dwFirst,dwCount,listBase)
	}

	#[inline(always)]
	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
		self.inner.wglUseFontOutlines(dwFirst,dwCount,listBase,deviation,extrusion,format,lpgmf)
	}

	#[inline(always)]
	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR) -> BOOL {
		self.inner.wglDescribeLayerPlane(iPixelFormat,iLayerPlane,nBytes,plpd)
	}

	#[inline(always)]
	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF) -> c_int {
		self.inner.wglSetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF) -> c_int {
		self.inner.wglGetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL) -> BOOL {
		self.inner.wglRealizeLayerPalette(iLayerPlane,bRealize)
	}

	#[inline(always)]
	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT) -> BOOL {
		self.inner.wglSwapLayerBuffers(uPlanes)
	}

	#[inline(always)]
	pub fn GetDCPenColor(&self) -> COLORREF {
		self.inner.GetDCPenColor()
	}

	#[inline(always)]
	pub fn SetDCPenColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCPenColor(clr)
	}

	#[inline(always)]
	pub fn GetDCBrushColor(&self) -> COLORREF {
		self.inner.GetDCBrushColor()
	}

	#[inline(always)]
	pub fn SetDCBrushColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCBrushColor(clr)
	}

	#[inline(always)]
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET) -> DWORD {
		self.inner.GetFontUnicodeRanges(lpgs)
	}

	#[inline(always)]
	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD) -> DWORD {
		self.inner.GetGlyphIndices(lpstr,cch,pgi,dwFlags)
	}

	#[inline(always)]
	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentPointI(pgiIn,cgi,lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentExPointI(pgiIn,cgi,nMaxExtent,lpnFit,alpDx,lpSize)
	}

	#[inline(always)]
	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidthI(giFirst,cgi,pgi,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidthsI(giFirst,cgi,pgi,lpabc)
	}

	#[inline(always)]
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD) -> BOOL {
		self.inner.ColorCorrectPalette(hPalette,dwFirstEntry,dwNumOfEntries)
	}
}

////////////////////////////////////////////////////
impl CDCHandle {
	//  pub fn Attach(&self,hDC: HDC) {

	#[inline(always)]
	pub fn Detach(&mut self) -> HDC {
		self.inner.Detach()
	}

	#[inline(always)]
	pub fn assert_dc(&self) {
		self.inner.assert_dc()
	}

	#[inline(always)]
	pub fn assert_null_dc(&self) {
		self.inner.assert_null_dc()
	}
	//pub fn HDC (&self)->operator { return self.inner.hdc; }

	#[inline(always)]
	pub fn IsNull(&self) -> bool {
		self.inner.IsNull()
	}

	#[inline(always)]
	pub fn WindowFromDC(&self) -> HWND {
		self.inner.WindowFromDC()
	}
	// pub fn GetCurrentPen (&self)->CPenHandle {
	// pub fn GetCurrentBrush (&self)->CBrushHandle {
	// pub fn GetCurrentPalette (&self)->CPaletteHandle {
	// pub fn GetCurrentFont (&self)->CFontHandle {
	// pub fn GetCurrentBitmap (&self)->CBitmapHandle {
	//  pub fn CreateDC(&self,lpszDriverName: LPCTSTR,lpszDeviceName: LPCTSTR,lpszOutput: LPCTSTR,lpInitData: &DEVMODE)->HDC {

	#[inline(always)]
	pub fn CreateCompatibleDC(&mut self, hDC: Option<HDC>) -> HDC {
		self.inner.CreateCompatibleDC(hDC)
	}

	#[inline(always)]
	pub fn DeleteDC(&mut self) -> BOOL {
		self.inner.DeleteDC()
	}

	#[inline(always)]
	pub fn SaveDC(&self) -> c_int {
		self.inner.SaveDC()
	}

	#[inline(always)]
	pub fn RestoreDC(&self,nSavedDC: c_int) -> BOOL {
		self.inner.RestoreDC(nSavedDC)
	}

	#[inline(always)]
	pub fn GetDeviceCaps(&self,nIndex: c_int) -> c_int {
		self.inner.GetDeviceCaps(nIndex)
	}

	#[inline(always)]
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT) -> UINT {
		self.inner.SetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT) -> UINT {
		self.inner.GetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn ResetDC(&self,lpDevMode: &DEVMODEW) -> BOOL {
		self.inner.ResetDC(lpDevMode)
	}

	#[inline(always)]
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetBrushOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg_point(point,lpPointRet)
	}
	// 	 pub fn EnumObjects(&self,nObjectType: c_int,@ c_int (CALLBACK* lpfn)(LPVOID,@ LPARAM),lpData: LPARAM)->c_int {

	#[inline(always)]
	pub fn SelectPen(&self,hPen: HPEN) -> HPEN {
		self.inner.SelectPen(hPen)
	}

	#[inline(always)]
	pub fn SelectBrush(&self,hBrush: HBRUSH) -> HBRUSH {
		self.inner.SelectBrush(hBrush)
	}

	#[inline(always)]
	pub fn SelectFont(&self,hFont: HFONT) -> HFONT {
		self.inner.SelectFont(hFont)
	}

	#[inline(always)]
	pub fn SelectBitmap(&self,hBitmap: HBITMAP) -> HBITMAP {
		self.inner.SelectBitmap(hBitmap)
	}

	#[inline(always)]
	pub fn SelectRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectRgn(hRgn)
	}

	#[inline(always)]
	pub fn SelectStockPen(&self,nPen: c_int) -> HPEN {
		self.inner.SelectStockPen((nPen as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockBrush(&self,nBrush: c_int) -> HBRUSH {
		self.inner.SelectStockBrush((nBrush as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockFont(&self,nFont: c_int) -> HFONT {
		self.inner.SelectStockFont((nFont as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectStockPalette(nPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn GetNearestColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.GetNearestColor(crColor)
	}

	#[inline(always)]
	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectPalette(hPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn RealizePalette(&self) -> UINT {
		self.inner.RealizePalette()
	}
		pub fn UpdateColors(&self) {
		self.inner.UpdateColors()
	}

	#[inline(always)]
	pub fn GetBkColor(&self) -> COLORREF {
		self.inner.GetBkColor()
	}

	#[inline(always)]
	pub fn GetBkMode(&self) -> c_int {
		self.inner.GetBkMode()
	}

	#[inline(always)]
	pub fn GetPolyFillMode(&self) -> c_int {
		self.inner.GetPolyFillMode()
	}

	#[inline(always)]
	pub fn GetROP2(&self) -> c_int {
		self.inner.GetROP2()
	}

	#[inline(always)]
	pub fn GetStretchBltMode(&self) -> c_int {
		self.inner.GetStretchBltMode()
	}

	#[inline(always)]
	pub fn GetTextColor(&self) -> COLORREF {
		self.inner.GetTextColor()
	}

	#[inline(always)]
	pub fn SetBkColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetBkColor(crColor)
	}

	#[inline(always)]
	pub fn SetBkMode(&self,nBkMode: c_int) -> c_int {
		self.inner.SetBkMode(nBkMode)
	}

	#[inline(always)]
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int) -> c_int {
		self.inner.SetPolyFillMode(nPolyFillMode)
	}

	#[inline(always)]
	pub fn SetROP2(&self,nDrawMode: c_int) -> c_int {
		self.inner.SetROP2(nDrawMode)
	}

	#[inline(always)]
	pub fn SetStretchBltMode(&self,nStretchMode: c_int) -> c_int {
		self.inner.SetStretchBltMode(nStretchMode)
	}

	#[inline(always)]
	pub fn SetTextColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetTextColor(crColor)
	}

	#[inline(always)]
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT) -> BOOL {
		self.inner.GetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT) -> BOOL {
		self.inner.SetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn GetMapMode(&self) -> c_int {
		self.inner.GetMapMode()
	}

	#[inline(always)]
	pub fn GetViewportOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetViewportOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetMapMode(&self,nMapMode: c_int) -> c_int {
		self.inner.SetMapMode(nMapMode)
	}

	#[inline(always)]
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetViewportOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetViewportExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetViewportExt(lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleViewportExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetWindowOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetWindowOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetWindowExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetWindowExt(lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleWindowExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.DPtoLP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.DPtoLP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.DPtoLP_Size(lpSize)
	}

	#[inline(always)]
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.LPtoDP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.LPtoDP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.LPtoDP_Size(lpSize)
	}

	#[inline(always)]
	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.DPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoDP(lpSize)
	}

	#[inline(always)]
	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.LPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoLP(lpSize)
	}

	#[inline(always)]
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRgn(hRgn,hBrush)
	}

	#[inline(always)]
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.FrameRgn(hRgn,hBrush,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn InvertRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.InvertRgn(hRgn)
	}

	#[inline(always)]
	pub fn PaintRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.PaintRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetClipBox(&self,lpRect: LPRECT) -> c_int {
		self.inner.GetClipBox(lpRect)
	}
	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {

	#[inline(always)]
	pub fn PtVisible(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.PtVisible(x,y)
	}

	#[inline(always)]
	pub fn PtVisible_point(&self,point: POINT) -> BOOL {
		self.inner.PtVisible_point(point)
	}

	#[inline(always)]
	pub fn RectVisible(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.RectVisible(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.ExcludeClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.ExcludeClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND) -> c_int {
		self.inner.ExcludeUpdateRgn(hWnd)
	}

	#[inline(always)]
	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.IntersectClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.IntersectClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int) -> c_int {
		self.inner.OffsetClipRgn(x,y)
	}

	#[inline(always)]
	pub fn OffsetClipRgn_size(&self,size: SIZE) -> c_int {
		self.inner.OffsetClipRgn_size(size)
	}

	#[inline(always)]
	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int) -> c_int {
		self.inner.SelectClipRgn_mode(hRgn,nMode)
	}

	#[inline(always)]
	pub fn SelectClipRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectClipRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetCurrentPosition(lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn LineTo(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.LineTo(x,y)
	}

	#[inline(always)]
	pub fn LineTo_point(&self,point: POINT) -> BOOL {
		self.inner.LineTo_point(point)
	}

	#[inline(always)]
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Arc(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Arc_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polyline(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT) -> BOOL {
		self.inner.AngleArc(x,y,nRadius,fStartAngle,fSweepAngle)
	}

	#[inline(always)]
	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.ArcTo(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.ArcTo_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn GetArcDirection(&self) -> c_int {
		self.inner.GetArcDirection()
	}

	#[inline(always)]
	pub fn SetArcDirection(&self,nArcDirection: c_int) -> c_int {
		self.inner.SetArcDirection(nArcDirection)
	}

	#[inline(always)]
	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int) -> BOOL {
		self.inner.PolyDraw(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolylineTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int) -> BOOL {
		self.inner.PolyPolyline(lpPoints,lpPolyPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezier(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezierTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int) -> BOOL {
		self.inner.FillRect_index(lpRect,nColorIndex)
	}

	#[inline(always)]
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FrameRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn InvertRect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.InvertRect(lpRect)
	}

	#[inline(always)]
	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon(x,y,hIcon)
	}

	#[inline(always)]
	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon_point(point,hIcon)
	}

	#[inline(always)]
	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx(x,y,hIcon,cxWidth,cyWidth,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx_point(point,hIcon,size,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_bitmap(pt,size,hBitmap,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState(pt,size,hIcon,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str, nFlags: UINT, bPrefixText: Option<BOOL>, nTextLen: Option<c_int>, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_text(pt,size,lpszText, nFlags, bPrefixText, nTextLen, hBrush)
	}

	#[inline(always)]
	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_proc(pt,size,lpDrawProc,lData,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Chord(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Chord_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.inner.DrawFocusRect(lpRect)
	}

	#[inline(always)]
	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Ellipse(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Ellipse_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Ellipse_rect(lpRect)
	}

	#[inline(always)]
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Pie(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Pie_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polygon(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int) -> BOOL {
		self.inner.PolyPolygon(lpPoints,lpPolyCounts,nCount)
	}

	#[inline(always)]
	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Rectangle(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Rectangle_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Rectangle_rect(lpRect)
	}

	#[inline(always)]
	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int) -> BOOL {
		self.inner.RoundRect(x1,y1,x2,y2,x3,y3)
	}

	#[inline(always)]
	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT) -> BOOL {
		self.inner.RoundRect_rect(lpRect,point)
	}

	#[inline(always)]
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.PatBlt(x,y,nWidth,nHeight,dwRop)
	}

	#[inline(always)]
	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD) -> BOOL {
		self.inner.BitBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,dwRop)
	}

	#[inline(always)]
	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.StretchBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,dwRop)
	}

	#[inline(always)]
	pub fn GetPixel(&self,x: c_int,y: c_int) -> COLORREF {
		self.inner.GetPixel(x,y)
	}

	#[inline(always)]
	pub fn GetPixel_point(&self,point: POINT) -> COLORREF {
		self.inner.GetPixel_point(point)
	}

	#[inline(always)]
	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel_point(point,crColor)
	}

	#[inline(always)]
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.FloodFill(x,y,crColor)
	}

	#[inline(always)]
	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT) -> BOOL {
		self.inner.ExtFloodFill(x,y,crColor,nFillType)
	}

	#[inline(always)]
	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD) -> BOOL {
		self.inner.MaskBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,hMaskBitmap,xMask,yMask,dwRop)
	}

	#[inline(always)]
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int) -> BOOL {
		self.inner.PlgBlt(lpPoint,hSrcDC,xSrc,ySrc,nWidth,nHeight,hMaskBitmap,xMask,yMask)
	}

	#[inline(always)]
	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV_point(point,crColor)
	}

	#[inline(always)]
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT) -> BOOL {
		self.inner.TransparentBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,crTransparent)
	}
	//  pub fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {

	#[inline(always)]
	pub fn GradientFill(&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD) -> BOOL {
		self.inner.GradientFill(pVertices,nVertices,pMeshElements,nMeshElements,dwMode)
	}

	#[inline(always)]
	pub fn GradientFillRect(&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool) -> BOOL {
		self.inner.GradientFillRect(rect,clr1,clr2,bHorizontal)
	}

	#[inline(always)]
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION) -> BOOL {
		self.inner.AlphaBlend(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,bf)
	}
	//pub fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>)->BOOL {

	#[inline(always)]
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>) -> BOOL {
		self.inner.TextOut(x,y,lpszString,nCount)
	}
	//pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT>,mut lpDxWidths: Option<LPINT>)->BOOL {

	#[inline(always)]
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT>, lpDxWidths: Option<LPINT>) -> BOOL {
		self.inner.ExtTextOut(x,y,nOptions,lpRect,lpszString,nCount,lpDxWidths)
	}
	//pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>,mut nTabPositions: Option<c_int>,mut lpnTabStopPositions: Option<LPINT>,mut nTabOrigin: Option<c_int>)->SIZE {

	#[inline(always)]
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>, nTabOrigin: Option<c_int>) -> SIZE {
		self.inner.TabbedTextOut(x,y,lpszString,nCount,nTabPositions,lpnTabStopPositions,nTabOrigin)
	}
	//pub fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {

	#[inline(always)]
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT) -> c_int {
		self.inner.DrawText(lpstrText,cchText,lpRect,uFormat)
	}
	//  pub fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	//pub fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS>)->c_int {

	#[inline(always)]
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS>) -> c_int {
		self.inner.DrawTextEx(lpstrText,cchText,lpRect,uFormat,lpDTParams)
	}

	#[inline(always)]
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int) -> c_int {
		self.inner.DrawShadowText(lpstrText,cchText,lpRect,dwFlags,clrText,clrShadow,xOffset,yOffset)
	}

	#[inline(always)]
	pub fn GetTextExtent(&self,lpszString: &str, nCount: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtent(lpszString, nCount, lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT>, alpDx: Option<LPINT>) -> BOOL {
		self.inner.GetTextExtentExPoint(lpszString,cchString,lpSize,nMaxExtent,lpnFit,alpDx)
	}

	#[inline(always)]
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>) -> DWORD {
		self.inner.GetTabbedTextExtent(lpszString,nCount,nTabPositions,lpnTabStopPositions)
	}

	#[inline(always)]
	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.GrayString(hBrush,lpfnOutput,lpData,nCount,x,y,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn GetTextAlign(&self) -> UINT {
		self.inner.GetTextAlign()
	}

	#[inline(always)]
	pub fn SetTextAlign(&self,nFlags: UINT) -> UINT {
		self.inner.SetTextAlign(nFlags)
	}
	// pub fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {

	#[inline(always)]
	pub fn GetTextFaceLen(&self) -> c_int {
		self.inner.GetTextFaceLen()
	}
	// pub fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// pub fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	#[inline(always)]
	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW) -> BOOL {
		self.inner.GetTextMetrics(lpMetrics)
	}

	#[inline(always)]
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int) -> c_int {
		self.inner.SetTextJustification(nBreakExtra,nBreakCount)
	}

	#[inline(always)]
	pub fn GetTextCharacterExtra(&self) -> c_int {
		self.inner.GetTextCharacterExtra()
	}

	#[inline(always)]
	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int) -> c_int {
		self.inner.SetTextCharacterExtra(nCharExtra)
	}

	#[inline(always)]
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT) -> BOOL {
		self.inner.DrawEdge(lpRect,nEdge,nFlags)
	}

	#[inline(always)]
	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT) -> BOOL {
		self.inner.DrawFrameControl(lpRect,nType,nState)
	}

	#[inline(always)]
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT) -> BOOL {
		self.inner.ScrollDC(dx,dy,lpRectScroll,lpRectClip,hRgnUpdate,lpRectUpdate)
	}

	#[inline(always)]
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT) -> BOOL {
		self.inner.GetCharWidth_float(nFirstChar,nLastChar,lpFloatBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth32(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn SetMapperFlags(&self,dwFlag: DWORD) -> DWORD {
		self.inner.SetMapperFlags(dwFlag)
	}

	#[inline(always)]
	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetAspectRatioFilter(lpSize)
	}

	#[inline(always)]
	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidths(nFirstChar,nLastChar,lpabc)
	}

	#[inline(always)]
	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD) -> DWORD {
		self.inner.GetFontData(dwTable,dwOffset,lpData,cbData)
	}

	#[inline(always)]
	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR) -> c_int {
		self.inner.GetKerningPairs(nPairs,lpkrnpair)
	}

	#[inline(always)]
	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW) -> UINT {
		self.inner.GetOutlineTextMetrics(cbData,lpotm)
	}

	#[inline(always)]
	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2) -> DWORD {
		self.inner.GetGlyphOutline(nChar,nFormat,lpgm,cbBuffer,lpBuffer,lpmat2)
	}

	#[inline(always)]
	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT) -> BOOL {
		self.inner.GetCharABCWidths_float(nFirstChar,nLastChar,lpABCF)
	}

	#[inline(always)]
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID) -> c_int {
		self.inner.Escape(nEscape,nCount,lpszInData,lpOutData)
	}

	#[inline(always)]
	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR) -> c_int {
		self.inner.Escape_ext(nEscape,nInputSize,lpszInputData,nOutputSize,lpszOutputData)
	}

	#[inline(always)]
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR) -> c_int {
		self.inner.DrawEscape(nEscape,nInputSize,lpszInputData)
	}

	#[inline(always)]
	pub fn StartDoc_name(&self,lpszDocName: &str) -> c_int {
		self.inner.StartDoc_name(lpszDocName)
	}

	#[inline(always)]
	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW) -> c_int {
		self.inner.StartDoc(lpDocInfo)
	}

	#[inline(always)]
	pub fn StartPage(&self) -> c_int {
		self.inner.StartPage()
	}

	#[inline(always)]
	pub fn EndPage(&self) -> c_int {
		self.inner.EndPage()
	}

	#[inline(always)]
	pub fn SetAbortProc(&self,lpfn: ABORTPROC ) -> c_int {
		self.inner.SetAbortProc(lpfn)
	}

	#[inline(always)]
	pub fn AbortDoc(&self) -> c_int {
		self.inner.AbortDoc()
	}

	#[inline(always)]
	pub fn EndDoc(&self) -> c_int {
		self.inner.EndDoc()
	}
		
	#[inline(always)]
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.inner.PlayMetaFile(hMF)
	}

	#[inline(always)]
	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT) -> BOOL {
		self.inner.PlayMetaFile_enh(hEnhMetaFile,lpBounds)
	}

	#[inline(always)]
	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE) -> BOOL {
		self.inner.AddMetaFileComment(nDataSize,pCommentData)
	}

	#[inline(always)]
	pub fn AbortPath(&self) -> BOOL {
		self.inner.AbortPath()
	}

	#[inline(always)]
	pub fn BeginPath(&self) -> BOOL {
		self.inner.BeginPath()
	}

	#[inline(always)]
	pub fn CloseFigure(&self) -> BOOL {
		self.inner.CloseFigure()
	}

	#[inline(always)]
	pub fn EndPath(&self) -> BOOL {
		self.inner.EndPath()
	}

	#[inline(always)]
	pub fn FillPath(&self) -> BOOL {
		self.inner.FillPath()
	}

	#[inline(always)]
	pub fn FlattenPath(&self) -> BOOL {
		self.inner.FlattenPath()
	}

	#[inline(always)]
	pub fn StrokeAndFillPath(&self) -> BOOL {
		self.inner.StrokeAndFillPath()
	}

	#[inline(always)]
	pub fn StrokePath(&self) -> BOOL {
		self.inner.StrokePath()
	}

	#[inline(always)]
	pub fn WidenPath(&self) -> BOOL {
		self.inner.WidenPath()
	}

	#[inline(always)]
	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT) -> BOOL {
		self.inner.GetMiterLimit(pfMiterLimit)
	}

	#[inline(always)]
	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT) -> BOOL {
		self.inner.SetMiterLimit(fMiterLimit)
	}

	#[inline(always)]
	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int) -> c_int {
		self.inner.GetPath(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn SelectClipPath(&self,nMode: c_int) -> BOOL {
		self.inner.SelectClipPath(nMode)
	}
	// pub fn GetHalftoneBrush()->CBrushHandle {

	// #[inline(always)]
	// pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH>,mut hBrushLast: Option<HBRUSH>) {
	// 	self.inner.DrawDragRect(lpRect,size,lpRectLast,sizeLast, hBrush, hBrushLast)
	// }

	#[inline(always)]
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.inner.FillSolidRect_rect(lpRect,clr)
	}

	#[inline(always)]
	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.inner.FillSolidRect(x,y,cx,cy,clr)
	}

	#[inline(always)]
	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect_rect(lpRect,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect(x,y,cx,cy,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT) -> c_int {
		self.inner.SetDIBitsToDevice(x,y,dwWidth,dwHeight,xSrc,ySrc,uStartScan,cScanLines,lpvBits,lpbmi,uColorUse)
	}

	#[inline(always)]
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD) -> c_int {
		self.inner.StretchDIBits(x,y,nWidth,nHeight,xSrc,ySrc,nSrcWidth,nSrcHeight,lpvBits,lpbmi,uColorUse,dwRop)
	}

	#[inline(always)]
	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD) -> UINT {
		self.inner.GetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD) -> UINT {
		self.inner.SetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.ChoosePixelFormat(ppfd)
	}

	#[inline(always)]
	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.DescribePixelFormat(iPixelFormat,nBytes,ppfd)
	}

	#[inline(always)]
	pub fn GetPixelFormat(&self) -> c_int {
		self.inner.GetPixelFormat()
	}

	#[inline(always)]
	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR) -> BOOL {
		self.inner.SetPixelFormat(iPixelFormat,ppfd)
	}

	#[inline(always)]
	pub fn SwapBuffers(&self) -> BOOL {
		self.inner.SwapBuffers()
	}

	#[inline(always)]
	pub fn wglCreateContext(&self) -> HGLRC {
		self.inner.wglCreateContext()
	}

	#[inline(always)]
	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int) -> HGLRC {
		self.inner.wglCreateLayerContext(iLayerPlane)
	}

	#[inline(always)]
	pub fn wglMakeCurrent(&self,hglrc: HGLRC) -> BOOL {
		self.inner.wglMakeCurrent(hglrc)
	}

	#[inline(always)]
	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD) -> BOOL {
		self.inner.wglUseFontBitmaps(dwFirst,dwCount,listBase)
	}

	#[inline(always)]
	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
		self.inner.wglUseFontOutlines(dwFirst,dwCount,listBase,deviation,extrusion,format,lpgmf)
	}

	#[inline(always)]
	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR) -> BOOL {
		self.inner.wglDescribeLayerPlane(iPixelFormat,iLayerPlane,nBytes,plpd)
	}

	#[inline(always)]
	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF) -> c_int {
		self.inner.wglSetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF) -> c_int {
		self.inner.wglGetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL) -> BOOL {
		self.inner.wglRealizeLayerPalette(iLayerPlane,bRealize)
	}

	#[inline(always)]
	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT) -> BOOL {
		self.inner.wglSwapLayerBuffers(uPlanes)
	}

	#[inline(always)]
	pub fn GetDCPenColor(&self) -> COLORREF {
		self.inner.GetDCPenColor()
	}

	#[inline(always)]
	pub fn SetDCPenColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCPenColor(clr)
	}

	#[inline(always)]
	pub fn GetDCBrushColor(&self) -> COLORREF {
		self.inner.GetDCBrushColor()
	}

	#[inline(always)]
	pub fn SetDCBrushColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCBrushColor(clr)
	}

	#[inline(always)]
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET) -> DWORD {
		self.inner.GetFontUnicodeRanges(lpgs)
	}

	#[inline(always)]
	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD) -> DWORD {
		self.inner.GetGlyphIndices(lpstr,cch,pgi,dwFlags)
	}

	#[inline(always)]
	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentPointI(pgiIn,cgi,lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentExPointI(pgiIn,cgi,nMaxExtent,lpnFit,alpDx,lpSize)
	}

	#[inline(always)]
	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidthI(giFirst,cgi,pgi,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidthsI(giFirst,cgi,pgi,lpabc)
	}

	#[inline(always)]
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD) -> BOOL {
		self.inner.ColorCorrectPalette(hPalette,dwFirstEntry,dwNumOfEntries)
	}
}

////////////////////////////////////////////////////
impl CPaintDC {
	//  pub fn Attach(&self,hDC: HDC) {

	#[inline(always)]
	pub fn Detach(&mut self) -> HDC {
		self.inner.Detach()
	}

	#[inline(always)]
	pub fn assert_dc(&self) {
		self.inner.assert_dc()
	}

	#[inline(always)]
	pub fn assert_null_dc(&self) {
		self.inner.assert_null_dc()
	}
	//pub fn HDC (&self)->operator { return self.inner.hdc; }

	#[inline(always)]
	pub fn IsNull(&self) -> bool {
		self.inner.IsNull()
	}

	#[inline(always)]
	pub fn WindowFromDC(&self) -> HWND {
		self.inner.WindowFromDC()
	}
	// pub fn GetCurrentPen (&self)->CPenHandle {
	// pub fn GetCurrentBrush (&self)->CBrushHandle {
	// pub fn GetCurrentPalette (&self)->CPaletteHandle {
	// pub fn GetCurrentFont (&self)->CFontHandle {
	// pub fn GetCurrentBitmap (&self)->CBitmapHandle {
	//  pub fn CreateDC(&self,lpszDriverName: LPCTSTR,lpszDeviceName: LPCTSTR,lpszOutput: LPCTSTR,lpInitData: &DEVMODE)->HDC {

	#[inline(always)]
	pub fn CreateCompatibleDC(&mut self, hDC: Option<HDC>) -> HDC {
		self.inner.CreateCompatibleDC(hDC)
	}

	#[inline(always)]
	pub fn DeleteDC(&mut self) -> BOOL {
		self.inner.DeleteDC()
	}

	#[inline(always)]
	pub fn SaveDC(&self) -> c_int {
		self.inner.SaveDC()
	}

	#[inline(always)]
	pub fn RestoreDC(&self,nSavedDC: c_int) -> BOOL {
		self.inner.RestoreDC(nSavedDC)
	}

	#[inline(always)]
	pub fn GetDeviceCaps(&self,nIndex: c_int) -> c_int {
		self.inner.GetDeviceCaps(nIndex)
	}

	#[inline(always)]
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT) -> UINT {
		self.inner.SetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT) -> UINT {
		self.inner.GetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn ResetDC(&self,lpDevMode: &DEVMODEW) -> BOOL {
		self.inner.ResetDC(lpDevMode)
	}

	#[inline(always)]
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetBrushOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg_point(point,lpPointRet)
	}
	// 	 pub fn EnumObjects(&self,nObjectType: c_int,@ c_int (CALLBACK* lpfn)(LPVOID,@ LPARAM),lpData: LPARAM)->c_int {

	#[inline(always)]
	pub fn SelectPen(&self,hPen: HPEN) -> HPEN {
		self.inner.SelectPen(hPen)
	}

	#[inline(always)]
	pub fn SelectBrush(&self,hBrush: HBRUSH) -> HBRUSH {
		self.inner.SelectBrush(hBrush)
	}

	#[inline(always)]
	pub fn SelectFont(&self,hFont: HFONT) -> HFONT {
		self.inner.SelectFont(hFont)
	}

	#[inline(always)]
	pub fn SelectBitmap(&self,hBitmap: HBITMAP) -> HBITMAP {
		self.inner.SelectBitmap(hBitmap)
	}

	#[inline(always)]
	pub fn SelectRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectRgn(hRgn)
	}

	#[inline(always)]
	pub fn SelectStockPen(&self,nPen: c_int) -> HPEN {
		self.inner.SelectStockPen((nPen as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockBrush(&self,nBrush: c_int) -> HBRUSH {
		self.inner.SelectStockBrush((nBrush as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockFont(&self,nFont: c_int) -> HFONT {
		self.inner.SelectStockFont((nFont as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectStockPalette(nPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn GetNearestColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.GetNearestColor(crColor)
	}

	#[inline(always)]
	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectPalette(hPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn RealizePalette(&self) -> UINT {
		self.inner.RealizePalette()
	}
		pub fn UpdateColors(&self) {
		self.inner.UpdateColors()
	}

	#[inline(always)]
	pub fn GetBkColor(&self) -> COLORREF {
		self.inner.GetBkColor()
	}

	#[inline(always)]
	pub fn GetBkMode(&self) -> c_int {
		self.inner.GetBkMode()
	}

	#[inline(always)]
	pub fn GetPolyFillMode(&self) -> c_int {
		self.inner.GetPolyFillMode()
	}

	#[inline(always)]
	pub fn GetROP2(&self) -> c_int {
		self.inner.GetROP2()
	}

	#[inline(always)]
	pub fn GetStretchBltMode(&self) -> c_int {
		self.inner.GetStretchBltMode()
	}

	#[inline(always)]
	pub fn GetTextColor(&self) -> COLORREF {
		self.inner.GetTextColor()
	}

	#[inline(always)]
	pub fn SetBkColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetBkColor(crColor)
	}

	#[inline(always)]
	pub fn SetBkMode(&self,nBkMode: c_int) -> c_int {
		self.inner.SetBkMode(nBkMode)
	}

	#[inline(always)]
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int) -> c_int {
		self.inner.SetPolyFillMode(nPolyFillMode)
	}

	#[inline(always)]
	pub fn SetROP2(&self,nDrawMode: c_int) -> c_int {
		self.inner.SetROP2(nDrawMode)
	}

	#[inline(always)]
	pub fn SetStretchBltMode(&self,nStretchMode: c_int) -> c_int {
		self.inner.SetStretchBltMode(nStretchMode)
	}

	#[inline(always)]
	pub fn SetTextColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetTextColor(crColor)
	}

	#[inline(always)]
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT) -> BOOL {
		self.inner.GetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT) -> BOOL {
		self.inner.SetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn GetMapMode(&self) -> c_int {
		self.inner.GetMapMode()
	}

	#[inline(always)]
	pub fn GetViewportOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetViewportOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetMapMode(&self,nMapMode: c_int) -> c_int {
		self.inner.SetMapMode(nMapMode)
	}

	#[inline(always)]
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetViewportOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetViewportExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetViewportExt(lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleViewportExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetWindowOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetWindowOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetWindowExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetWindowExt(lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleWindowExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.DPtoLP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.DPtoLP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.DPtoLP_Size(lpSize)
	}

	#[inline(always)]
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.LPtoDP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.LPtoDP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.LPtoDP_Size(lpSize)
	}

	#[inline(always)]
	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.DPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoDP(lpSize)
	}

	#[inline(always)]
	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.LPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoLP(lpSize)
	}

	#[inline(always)]
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRgn(hRgn,hBrush)
	}

	#[inline(always)]
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.FrameRgn(hRgn,hBrush,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn InvertRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.InvertRgn(hRgn)
	}

	#[inline(always)]
	pub fn PaintRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.PaintRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetClipBox(&self,lpRect: LPRECT) -> c_int {
		self.inner.GetClipBox(lpRect)
	}
	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {

	#[inline(always)]
	pub fn PtVisible(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.PtVisible(x,y)
	}

	#[inline(always)]
	pub fn PtVisible_point(&self,point: POINT) -> BOOL {
		self.inner.PtVisible_point(point)
	}

	#[inline(always)]
	pub fn RectVisible(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.RectVisible(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.ExcludeClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.ExcludeClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND) -> c_int {
		self.inner.ExcludeUpdateRgn(hWnd)
	}

	#[inline(always)]
	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.IntersectClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.IntersectClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int) -> c_int {
		self.inner.OffsetClipRgn(x,y)
	}

	#[inline(always)]
	pub fn OffsetClipRgn_size(&self,size: SIZE) -> c_int {
		self.inner.OffsetClipRgn_size(size)
	}

	#[inline(always)]
	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int) -> c_int {
		self.inner.SelectClipRgn_mode(hRgn,nMode)
	}

	#[inline(always)]
	pub fn SelectClipRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectClipRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetCurrentPosition(lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn LineTo(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.LineTo(x,y)
	}

	#[inline(always)]
	pub fn LineTo_point(&self,point: POINT) -> BOOL {
		self.inner.LineTo_point(point)
	}

	#[inline(always)]
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Arc(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Arc_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polyline(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT) -> BOOL {
		self.inner.AngleArc(x,y,nRadius,fStartAngle,fSweepAngle)
	}

	#[inline(always)]
	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.ArcTo(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.ArcTo_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn GetArcDirection(&self) -> c_int {
		self.inner.GetArcDirection()
	}

	#[inline(always)]
	pub fn SetArcDirection(&self,nArcDirection: c_int) -> c_int {
		self.inner.SetArcDirection(nArcDirection)
	}

	#[inline(always)]
	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int) -> BOOL {
		self.inner.PolyDraw(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolylineTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int) -> BOOL {
		self.inner.PolyPolyline(lpPoints,lpPolyPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezier(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezierTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int) -> BOOL {
		self.inner.FillRect_index(lpRect,nColorIndex)
	}

	#[inline(always)]
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FrameRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn InvertRect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.InvertRect(lpRect)
	}

	#[inline(always)]
	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon(x,y,hIcon)
	}

	#[inline(always)]
	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon_point(point,hIcon)
	}

	#[inline(always)]
	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx(x,y,hIcon,cxWidth,cyWidth,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx_point(point,hIcon,size,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_bitmap(pt,size,hBitmap,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState(pt,size,hIcon,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str, nFlags: UINT, bPrefixText: Option<BOOL>, nTextLen: Option<c_int>, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_text(pt,size,lpszText, nFlags, bPrefixText, nTextLen, hBrush)
	}

	#[inline(always)]
	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_proc(pt,size,lpDrawProc,lData,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Chord(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Chord_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.inner.DrawFocusRect(lpRect)
	}

	#[inline(always)]
	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Ellipse(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Ellipse_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Ellipse_rect(lpRect)
	}

	#[inline(always)]
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Pie(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Pie_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polygon(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int) -> BOOL {
		self.inner.PolyPolygon(lpPoints,lpPolyCounts,nCount)
	}

	#[inline(always)]
	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Rectangle(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Rectangle_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Rectangle_rect(lpRect)
	}

	#[inline(always)]
	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int) -> BOOL {
		self.inner.RoundRect(x1,y1,x2,y2,x3,y3)
	}

	#[inline(always)]
	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT) -> BOOL {
		self.inner.RoundRect_rect(lpRect,point)
	}

	#[inline(always)]
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.PatBlt(x,y,nWidth,nHeight,dwRop)
	}

	#[inline(always)]
	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD) -> BOOL {
		self.inner.BitBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,dwRop)
	}

	#[inline(always)]
	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.StretchBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,dwRop)
	}

	#[inline(always)]
	pub fn GetPixel(&self,x: c_int,y: c_int) -> COLORREF {
		self.inner.GetPixel(x,y)
	}

	#[inline(always)]
	pub fn GetPixel_point(&self,point: POINT) -> COLORREF {
		self.inner.GetPixel_point(point)
	}

	#[inline(always)]
	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel_point(point,crColor)
	}

	#[inline(always)]
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.FloodFill(x,y,crColor)
	}

	#[inline(always)]
	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT) -> BOOL {
		self.inner.ExtFloodFill(x,y,crColor,nFillType)
	}

	#[inline(always)]
	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD) -> BOOL {
		self.inner.MaskBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,hMaskBitmap,xMask,yMask,dwRop)
	}

	#[inline(always)]
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int) -> BOOL {
		self.inner.PlgBlt(lpPoint,hSrcDC,xSrc,ySrc,nWidth,nHeight,hMaskBitmap,xMask,yMask)
	}

	#[inline(always)]
	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV_point(point,crColor)
	}

	#[inline(always)]
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT) -> BOOL {
		self.inner.TransparentBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,crTransparent)
	}
	//  pub fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {

	#[inline(always)]
	pub fn GradientFill(&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD) -> BOOL {
		self.inner.GradientFill(pVertices,nVertices,pMeshElements,nMeshElements,dwMode)
	}

	#[inline(always)]
	pub fn GradientFillRect(&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool) -> BOOL {
		self.inner.GradientFillRect(rect,clr1,clr2,bHorizontal)
	}

	#[inline(always)]
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION) -> BOOL {
		self.inner.AlphaBlend(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,bf)
	}
	//pub fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>)->BOOL {

	#[inline(always)]
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>) -> BOOL {
		self.inner.TextOut(x,y,lpszString,nCount)
	}
	//pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT>,mut lpDxWidths: Option<LPINT>)->BOOL {

	#[inline(always)]
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT>, lpDxWidths: Option<LPINT>) -> BOOL {
		self.inner.ExtTextOut(x,y,nOptions,lpRect,lpszString,nCount,lpDxWidths)
	}
	//pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>,mut nTabPositions: Option<c_int>,mut lpnTabStopPositions: Option<LPINT>,mut nTabOrigin: Option<c_int>)->SIZE {

	#[inline(always)]
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>, nTabOrigin: Option<c_int>) -> SIZE {
		self.inner.TabbedTextOut(x,y,lpszString,nCount,nTabPositions,lpnTabStopPositions,nTabOrigin)
	}
	//pub fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {

	#[inline(always)]
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT) -> c_int {
		self.inner.DrawText(lpstrText,cchText,lpRect,uFormat)
	}
	//  pub fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	//pub fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS>)->c_int {

	#[inline(always)]
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS>) -> c_int {
		self.inner.DrawTextEx(lpstrText,cchText,lpRect,uFormat,lpDTParams)
	}

	#[inline(always)]
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int) -> c_int {
		self.inner.DrawShadowText(lpstrText,cchText,lpRect,dwFlags,clrText,clrShadow,xOffset,yOffset)
	}

	#[inline(always)]
	pub fn GetTextExtent(&self,lpszString: &str, nCount: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtent(lpszString, nCount, lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT>, alpDx: Option<LPINT>) -> BOOL {
		self.inner.GetTextExtentExPoint(lpszString,cchString,lpSize,nMaxExtent,lpnFit,alpDx)
	}

	#[inline(always)]
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>) -> DWORD {
		self.inner.GetTabbedTextExtent(lpszString,nCount,nTabPositions,lpnTabStopPositions)
	}

	#[inline(always)]
	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.GrayString(hBrush,lpfnOutput,lpData,nCount,x,y,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn GetTextAlign(&self) -> UINT {
		self.inner.GetTextAlign()
	}

	#[inline(always)]
	pub fn SetTextAlign(&self,nFlags: UINT) -> UINT {
		self.inner.SetTextAlign(nFlags)
	}
	// pub fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {

	#[inline(always)]
	pub fn GetTextFaceLen(&self) -> c_int {
		self.inner.GetTextFaceLen()
	}
	// pub fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// pub fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	#[inline(always)]
	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW) -> BOOL {
		self.inner.GetTextMetrics(lpMetrics)
	}

	#[inline(always)]
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int) -> c_int {
		self.inner.SetTextJustification(nBreakExtra,nBreakCount)
	}

	#[inline(always)]
	pub fn GetTextCharacterExtra(&self) -> c_int {
		self.inner.GetTextCharacterExtra()
	}

	#[inline(always)]
	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int) -> c_int {
		self.inner.SetTextCharacterExtra(nCharExtra)
	}

	#[inline(always)]
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT) -> BOOL {
		self.inner.DrawEdge(lpRect,nEdge,nFlags)
	}

	#[inline(always)]
	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT) -> BOOL {
		self.inner.DrawFrameControl(lpRect,nType,nState)
	}

	#[inline(always)]
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT) -> BOOL {
		self.inner.ScrollDC(dx,dy,lpRectScroll,lpRectClip,hRgnUpdate,lpRectUpdate)
	}

	#[inline(always)]
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT) -> BOOL {
		self.inner.GetCharWidth_float(nFirstChar,nLastChar,lpFloatBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth32(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn SetMapperFlags(&self,dwFlag: DWORD) -> DWORD {
		self.inner.SetMapperFlags(dwFlag)
	}

	#[inline(always)]
	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetAspectRatioFilter(lpSize)
	}

	#[inline(always)]
	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidths(nFirstChar,nLastChar,lpabc)
	}

	#[inline(always)]
	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD) -> DWORD {
		self.inner.GetFontData(dwTable,dwOffset,lpData,cbData)
	}

	#[inline(always)]
	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR) -> c_int {
		self.inner.GetKerningPairs(nPairs,lpkrnpair)
	}

	#[inline(always)]
	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW) -> UINT {
		self.inner.GetOutlineTextMetrics(cbData,lpotm)
	}

	#[inline(always)]
	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2) -> DWORD {
		self.inner.GetGlyphOutline(nChar,nFormat,lpgm,cbBuffer,lpBuffer,lpmat2)
	}

	#[inline(always)]
	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT) -> BOOL {
		self.inner.GetCharABCWidths_float(nFirstChar,nLastChar,lpABCF)
	}

	#[inline(always)]
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID) -> c_int {
		self.inner.Escape(nEscape,nCount,lpszInData,lpOutData)
	}

	#[inline(always)]
	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR) -> c_int {
		self.inner.Escape_ext(nEscape,nInputSize,lpszInputData,nOutputSize,lpszOutputData)
	}

	#[inline(always)]
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR) -> c_int {
		self.inner.DrawEscape(nEscape,nInputSize,lpszInputData)
	}

	#[inline(always)]
	pub fn StartDoc_name(&self,lpszDocName: &str) -> c_int {
		self.inner.StartDoc_name(lpszDocName)
	}

	#[inline(always)]
	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW) -> c_int {
		self.inner.StartDoc(lpDocInfo)
	}

	#[inline(always)]
	pub fn StartPage(&self) -> c_int {
		self.inner.StartPage()
	}

	#[inline(always)]
	pub fn EndPage(&self) -> c_int {
		self.inner.EndPage()
	}

	#[inline(always)]
	pub fn SetAbortProc(&self,lpfn: ABORTPROC ) -> c_int {
		self.inner.SetAbortProc(lpfn)
	}

	#[inline(always)]
	pub fn AbortDoc(&self) -> c_int {
		self.inner.AbortDoc()
	}

	#[inline(always)]
	pub fn EndDoc(&self) -> c_int {
		self.inner.EndDoc()
	}
	
	#[inline(always)]
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.inner.PlayMetaFile(hMF)
	}

	#[inline(always)]
	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT) -> BOOL {
		self.inner.PlayMetaFile_enh(hEnhMetaFile,lpBounds)
	}

	#[inline(always)]
	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE) -> BOOL {
		self.inner.AddMetaFileComment(nDataSize,pCommentData)
	}

	#[inline(always)]
	pub fn AbortPath(&self) -> BOOL {
		self.inner.AbortPath()
	}

	#[inline(always)]
	pub fn BeginPath(&self) -> BOOL {
		self.inner.BeginPath()
	}

	#[inline(always)]
	pub fn CloseFigure(&self) -> BOOL {
		self.inner.CloseFigure()
	}

	#[inline(always)]
	pub fn EndPath(&self) -> BOOL {
		self.inner.EndPath()
	}

	#[inline(always)]
	pub fn FillPath(&self) -> BOOL {
		self.inner.FillPath()
	}

	#[inline(always)]
	pub fn FlattenPath(&self) -> BOOL {
		self.inner.FlattenPath()
	}

	#[inline(always)]
	pub fn StrokeAndFillPath(&self) -> BOOL {
		self.inner.StrokeAndFillPath()
	}

	#[inline(always)]
	pub fn StrokePath(&self) -> BOOL {
		self.inner.StrokePath()
	}

	#[inline(always)]
	pub fn WidenPath(&self) -> BOOL {
		self.inner.WidenPath()
	}

	#[inline(always)]
	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT) -> BOOL {
		self.inner.GetMiterLimit(pfMiterLimit)
	}

	#[inline(always)]
	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT) -> BOOL {
		self.inner.SetMiterLimit(fMiterLimit)
	}

	#[inline(always)]
	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int) -> c_int {
		self.inner.GetPath(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn SelectClipPath(&self,nMode: c_int) -> BOOL {
		self.inner.SelectClipPath(nMode)
	}
	// pub fn GetHalftoneBrush()->CBrushHandle {

	// #[inline(always)]
	// pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH>,mut hBrushLast: Option<HBRUSH>) {
	// 	self.inner.DrawDragRect(lpRect,size,lpRectLast,sizeLast, hBrush, hBrushLast)
	// }

	#[inline(always)]
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.inner.FillSolidRect_rect(lpRect,clr)
	}

	#[inline(always)]
	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.inner.FillSolidRect(x,y,cx,cy,clr)
	}

	#[inline(always)]
	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect_rect(lpRect,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect(x,y,cx,cy,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT) -> c_int {
		self.inner.SetDIBitsToDevice(x,y,dwWidth,dwHeight,xSrc,ySrc,uStartScan,cScanLines,lpvBits,lpbmi,uColorUse)
	}

	#[inline(always)]
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD) -> c_int {
		self.inner.StretchDIBits(x,y,nWidth,nHeight,xSrc,ySrc,nSrcWidth,nSrcHeight,lpvBits,lpbmi,uColorUse,dwRop)
	}

	#[inline(always)]
	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD) -> UINT {
		self.inner.GetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD) -> UINT {
		self.inner.SetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.ChoosePixelFormat(ppfd)
	}

	#[inline(always)]
	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.DescribePixelFormat(iPixelFormat,nBytes,ppfd)
	}

	#[inline(always)]
	pub fn GetPixelFormat(&self) -> c_int {
		self.inner.GetPixelFormat()
	}

	#[inline(always)]
	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR) -> BOOL {
		self.inner.SetPixelFormat(iPixelFormat,ppfd)
	}

	#[inline(always)]
	pub fn SwapBuffers(&self) -> BOOL {
		self.inner.SwapBuffers()
	}

	#[inline(always)]
	pub fn wglCreateContext(&self) -> HGLRC {
		self.inner.wglCreateContext()
	}

	#[inline(always)]
	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int) -> HGLRC {
		self.inner.wglCreateLayerContext(iLayerPlane)
	}

	#[inline(always)]
	pub fn wglMakeCurrent(&self,hglrc: HGLRC) -> BOOL {
		self.inner.wglMakeCurrent(hglrc)
	}

	#[inline(always)]
	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD) -> BOOL {
		self.inner.wglUseFontBitmaps(dwFirst,dwCount,listBase)
	}

	#[inline(always)]
	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
		self.inner.wglUseFontOutlines(dwFirst,dwCount,listBase,deviation,extrusion,format,lpgmf)
	}

	#[inline(always)]
	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR) -> BOOL {
		self.inner.wglDescribeLayerPlane(iPixelFormat,iLayerPlane,nBytes,plpd)
	}

	#[inline(always)]
	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF) -> c_int {
		self.inner.wglSetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF) -> c_int {
		self.inner.wglGetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL) -> BOOL {
		self.inner.wglRealizeLayerPalette(iLayerPlane,bRealize)
	}

	#[inline(always)]
	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT) -> BOOL {
		self.inner.wglSwapLayerBuffers(uPlanes)
	}

	#[inline(always)]
	pub fn GetDCPenColor(&self) -> COLORREF {
		self.inner.GetDCPenColor()
	}

	#[inline(always)]
	pub fn SetDCPenColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCPenColor(clr)
	}

	#[inline(always)]
	pub fn GetDCBrushColor(&self) -> COLORREF {
		self.inner.GetDCBrushColor()
	}

	#[inline(always)]
	pub fn SetDCBrushColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCBrushColor(clr)
	}

	#[inline(always)]
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET) -> DWORD {
		self.inner.GetFontUnicodeRanges(lpgs)
	}

	#[inline(always)]
	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD) -> DWORD {
		self.inner.GetGlyphIndices(lpstr,cch,pgi,dwFlags)
	}

	#[inline(always)]
	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentPointI(pgiIn,cgi,lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentExPointI(pgiIn,cgi,nMaxExtent,lpnFit,alpDx,lpSize)
	}

	#[inline(always)]
	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidthI(giFirst,cgi,pgi,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidthsI(giFirst,cgi,pgi,lpabc)
	}

	#[inline(always)]
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD) -> BOOL {
		self.inner.ColorCorrectPalette(hPalette,dwFirstEntry,dwNumOfEntries)
	}
}

////////////////////////////////////////////////////
impl CClientDC {
	//  pub fn Attach(&self,hDC: HDC) {

	#[inline(always)]
	pub fn Detach(&mut self) -> HDC {
		self.inner.Detach()
	}

	#[inline(always)]
	pub fn assert_dc(&self) {
		self.inner.assert_dc()
	}

	#[inline(always)]
	pub fn assert_null_dc(&self) {
		self.inner.assert_null_dc()
	}
	//pub fn HDC (&self)->operator { return self.inner.hdc; }

	#[inline(always)]
	pub fn IsNull(&self) -> bool {
		self.inner.IsNull()
	}

	#[inline(always)]
	pub fn WindowFromDC(&self) -> HWND {
		self.inner.WindowFromDC()
	}
	// pub fn GetCurrentPen (&self)->CPenHandle {
	// pub fn GetCurrentBrush (&self)->CBrushHandle {
	// pub fn GetCurrentPalette (&self)->CPaletteHandle {
	// pub fn GetCurrentFont (&self)->CFontHandle {
	// pub fn GetCurrentBitmap (&self)->CBitmapHandle {
	//  pub fn CreateDC(&self,lpszDriverName: LPCTSTR,lpszDeviceName: LPCTSTR,lpszOutput: LPCTSTR,lpInitData: &DEVMODE)->HDC {

	#[inline(always)]
	pub fn CreateCompatibleDC(&mut self, hDC: Option<HDC>) -> HDC {
		self.inner.CreateCompatibleDC(hDC)
	}

	#[inline(always)]
	pub fn DeleteDC(&mut self) -> BOOL {
		self.inner.DeleteDC()
	}

	#[inline(always)]
	pub fn SaveDC(&self) -> c_int {
		self.inner.SaveDC()
	}

	#[inline(always)]
	pub fn RestoreDC(&self,nSavedDC: c_int) -> BOOL {
		self.inner.RestoreDC(nSavedDC)
	}

	#[inline(always)]
	pub fn GetDeviceCaps(&self,nIndex: c_int) -> c_int {
		self.inner.GetDeviceCaps(nIndex)
	}

	#[inline(always)]
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT) -> UINT {
		self.inner.SetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT) -> UINT {
		self.inner.GetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn ResetDC(&self,lpDevMode: &DEVMODEW) -> BOOL {
		self.inner.ResetDC(lpDevMode)
	}

	#[inline(always)]
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetBrushOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg_point(point,lpPointRet)
	}
	// 	 pub fn EnumObjects(&self,nObjectType: c_int,@ c_int (CALLBACK* lpfn)(LPVOID,@ LPARAM),lpData: LPARAM)->c_int {

	#[inline(always)]
	pub fn SelectPen(&self,hPen: HPEN) -> HPEN {
		self.inner.SelectPen(hPen)
	}

	#[inline(always)]
	pub fn SelectBrush(&self,hBrush: HBRUSH) -> HBRUSH {
		self.inner.SelectBrush(hBrush)
	}

	#[inline(always)]
	pub fn SelectFont(&self,hFont: HFONT) -> HFONT {
		self.inner.SelectFont(hFont)
	}

	#[inline(always)]
	pub fn SelectBitmap(&self,hBitmap: HBITMAP) -> HBITMAP {
		self.inner.SelectBitmap(hBitmap)
	}

	#[inline(always)]
	pub fn SelectRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectRgn(hRgn)
	}

	#[inline(always)]
	pub fn SelectStockPen(&self,nPen: c_int) -> HPEN {
		self.inner.SelectStockPen((nPen as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockBrush(&self,nBrush: c_int) -> HBRUSH {
		self.inner.SelectStockBrush((nBrush as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockFont(&self,nFont: c_int) -> HFONT {
		self.inner.SelectStockFont((nFont as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectStockPalette(nPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn GetNearestColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.GetNearestColor(crColor)
	}

	#[inline(always)]
	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectPalette(hPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn RealizePalette(&self) -> UINT {
		self.inner.RealizePalette()
	}
		pub fn UpdateColors(&self) {
		self.inner.UpdateColors()
	}

	#[inline(always)]
	pub fn GetBkColor(&self) -> COLORREF {
		self.inner.GetBkColor()
	}

	#[inline(always)]
	pub fn GetBkMode(&self) -> c_int {
		self.inner.GetBkMode()
	}

	#[inline(always)]
	pub fn GetPolyFillMode(&self) -> c_int {
		self.inner.GetPolyFillMode()
	}

	#[inline(always)]
	pub fn GetROP2(&self) -> c_int {
		self.inner.GetROP2()
	}

	#[inline(always)]
	pub fn GetStretchBltMode(&self) -> c_int {
		self.inner.GetStretchBltMode()
	}

	#[inline(always)]
	pub fn GetTextColor(&self) -> COLORREF {
		self.inner.GetTextColor()
	}

	#[inline(always)]
	pub fn SetBkColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetBkColor(crColor)
	}

	#[inline(always)]
	pub fn SetBkMode(&self,nBkMode: c_int) -> c_int {
		self.inner.SetBkMode(nBkMode)
	}

	#[inline(always)]
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int) -> c_int {
		self.inner.SetPolyFillMode(nPolyFillMode)
	}

	#[inline(always)]
	pub fn SetROP2(&self,nDrawMode: c_int) -> c_int {
		self.inner.SetROP2(nDrawMode)
	}

	#[inline(always)]
	pub fn SetStretchBltMode(&self,nStretchMode: c_int) -> c_int {
		self.inner.SetStretchBltMode(nStretchMode)
	}

	#[inline(always)]
	pub fn SetTextColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetTextColor(crColor)
	}

	#[inline(always)]
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT) -> BOOL {
		self.inner.GetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT) -> BOOL {
		self.inner.SetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn GetMapMode(&self) -> c_int {
		self.inner.GetMapMode()
	}

	#[inline(always)]
	pub fn GetViewportOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetViewportOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetMapMode(&self,nMapMode: c_int) -> c_int {
		self.inner.SetMapMode(nMapMode)
	}

	#[inline(always)]
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetViewportOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetViewportExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetViewportExt(lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleViewportExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetWindowOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetWindowOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetWindowExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetWindowExt(lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleWindowExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.DPtoLP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.DPtoLP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.DPtoLP_Size(lpSize)
	}

	#[inline(always)]
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.LPtoDP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.LPtoDP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.LPtoDP_Size(lpSize)
	}

	#[inline(always)]
	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.DPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoDP(lpSize)
	}

	#[inline(always)]
	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.LPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoLP(lpSize)
	}

	#[inline(always)]
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRgn(hRgn,hBrush)
	}

	#[inline(always)]
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.FrameRgn(hRgn,hBrush,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn InvertRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.InvertRgn(hRgn)
	}

	#[inline(always)]
	pub fn PaintRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.PaintRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetClipBox(&self,lpRect: LPRECT) -> c_int {
		self.inner.GetClipBox(lpRect)
	}
	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {

	#[inline(always)]
	pub fn PtVisible(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.PtVisible(x,y)
	}

	#[inline(always)]
	pub fn PtVisible_point(&self,point: POINT) -> BOOL {
		self.inner.PtVisible_point(point)
	}

	#[inline(always)]
	pub fn RectVisible(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.RectVisible(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.ExcludeClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.ExcludeClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND) -> c_int {
		self.inner.ExcludeUpdateRgn(hWnd)
	}

	#[inline(always)]
	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.IntersectClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.IntersectClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int) -> c_int {
		self.inner.OffsetClipRgn(x,y)
	}

	#[inline(always)]
	pub fn OffsetClipRgn_size(&self,size: SIZE) -> c_int {
		self.inner.OffsetClipRgn_size(size)
	}

	#[inline(always)]
	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int) -> c_int {
		self.inner.SelectClipRgn_mode(hRgn,nMode)
	}

	#[inline(always)]
	pub fn SelectClipRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectClipRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetCurrentPosition(lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn LineTo(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.LineTo(x,y)
	}

	#[inline(always)]
	pub fn LineTo_point(&self,point: POINT) -> BOOL {
		self.inner.LineTo_point(point)
	}

	#[inline(always)]
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Arc(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Arc_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polyline(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT) -> BOOL {
		self.inner.AngleArc(x,y,nRadius,fStartAngle,fSweepAngle)
	}

	#[inline(always)]
	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.ArcTo(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.ArcTo_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn GetArcDirection(&self) -> c_int {
		self.inner.GetArcDirection()
	}

	#[inline(always)]
	pub fn SetArcDirection(&self,nArcDirection: c_int) -> c_int {
		self.inner.SetArcDirection(nArcDirection)
	}

	#[inline(always)]
	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int) -> BOOL {
		self.inner.PolyDraw(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolylineTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int) -> BOOL {
		self.inner.PolyPolyline(lpPoints,lpPolyPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezier(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezierTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int) -> BOOL {
		self.inner.FillRect_index(lpRect,nColorIndex)
	}

	#[inline(always)]
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FrameRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn InvertRect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.InvertRect(lpRect)
	}

	#[inline(always)]
	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon(x,y,hIcon)
	}

	#[inline(always)]
	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon_point(point,hIcon)
	}

	#[inline(always)]
	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx(x,y,hIcon,cxWidth,cyWidth,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx_point(point,hIcon,size,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_bitmap(pt,size,hBitmap,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState(pt,size,hIcon,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str, nFlags: UINT, bPrefixText: Option<BOOL>, nTextLen: Option<c_int>, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_text(pt,size,lpszText, nFlags, bPrefixText, nTextLen, hBrush)
	}

	#[inline(always)]
	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_proc(pt,size,lpDrawProc,lData,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Chord(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Chord_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.inner.DrawFocusRect(lpRect)
	}

	#[inline(always)]
	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Ellipse(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Ellipse_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Ellipse_rect(lpRect)
	}

	#[inline(always)]
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Pie(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Pie_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polygon(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int) -> BOOL {
		self.inner.PolyPolygon(lpPoints,lpPolyCounts,nCount)
	}

	#[inline(always)]
	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Rectangle(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Rectangle_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Rectangle_rect(lpRect)
	}

	#[inline(always)]
	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int) -> BOOL {
		self.inner.RoundRect(x1,y1,x2,y2,x3,y3)
	}

	#[inline(always)]
	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT) -> BOOL {
		self.inner.RoundRect_rect(lpRect,point)
	}

	#[inline(always)]
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.PatBlt(x,y,nWidth,nHeight,dwRop)
	}

	#[inline(always)]
	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD) -> BOOL {
		self.inner.BitBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,dwRop)
	}

	#[inline(always)]
	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.StretchBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,dwRop)
	}

	#[inline(always)]
	pub fn GetPixel(&self,x: c_int,y: c_int) -> COLORREF {
		self.inner.GetPixel(x,y)
	}

	#[inline(always)]
	pub fn GetPixel_point(&self,point: POINT) -> COLORREF {
		self.inner.GetPixel_point(point)
	}

	#[inline(always)]
	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel_point(point,crColor)
	}

	#[inline(always)]
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.FloodFill(x,y,crColor)
	}

	#[inline(always)]
	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT) -> BOOL {
		self.inner.ExtFloodFill(x,y,crColor,nFillType)
	}

	#[inline(always)]
	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD) -> BOOL {
		self.inner.MaskBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,hMaskBitmap,xMask,yMask,dwRop)
	}

	#[inline(always)]
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int) -> BOOL {
		self.inner.PlgBlt(lpPoint,hSrcDC,xSrc,ySrc,nWidth,nHeight,hMaskBitmap,xMask,yMask)
	}

	#[inline(always)]
	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV_point(point,crColor)
	}

	#[inline(always)]
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT) -> BOOL {
		self.inner.TransparentBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,crTransparent)
	}
	//  pub fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {

	#[inline(always)]
	pub fn GradientFill(&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD) -> BOOL {
		self.inner.GradientFill(pVertices,nVertices,pMeshElements,nMeshElements,dwMode)
	}

	#[inline(always)]
	pub fn GradientFillRect(&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool) -> BOOL {
		self.inner.GradientFillRect(rect,clr1,clr2,bHorizontal)
	}

	#[inline(always)]
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION) -> BOOL {
		self.inner.AlphaBlend(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,bf)
	}
	//pub fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>)->BOOL {

	#[inline(always)]
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>) -> BOOL {
		self.inner.TextOut(x,y,lpszString,nCount)
	}
	//pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT>,mut lpDxWidths: Option<LPINT>)->BOOL {

	#[inline(always)]
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT>, lpDxWidths: Option<LPINT>) -> BOOL {
		self.inner.ExtTextOut(x,y,nOptions,lpRect,lpszString,nCount,lpDxWidths)
	}
	//pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>,mut nTabPositions: Option<c_int>,mut lpnTabStopPositions: Option<LPINT>,mut nTabOrigin: Option<c_int>)->SIZE {

	#[inline(always)]
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>, nTabOrigin: Option<c_int>) -> SIZE {
		self.inner.TabbedTextOut(x,y,lpszString,nCount,nTabPositions,lpnTabStopPositions,nTabOrigin)
	}
	//pub fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {

	#[inline(always)]
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT) -> c_int {
		self.inner.DrawText(lpstrText,cchText,lpRect,uFormat)
	}
	//  pub fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	//pub fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS>)->c_int {

	#[inline(always)]
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS>) -> c_int {
		self.inner.DrawTextEx(lpstrText,cchText,lpRect,uFormat,lpDTParams)
	}

	#[inline(always)]
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int) -> c_int {
		self.inner.DrawShadowText(lpstrText,cchText,lpRect,dwFlags,clrText,clrShadow,xOffset,yOffset)
	}

	#[inline(always)]
	pub fn GetTextExtent(&self,lpszString: &str, nCount: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtent(lpszString, nCount, lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT>, alpDx: Option<LPINT>) -> BOOL {
		self.inner.GetTextExtentExPoint(lpszString,cchString,lpSize,nMaxExtent,lpnFit,alpDx)
	}

	#[inline(always)]
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>) -> DWORD {
		self.inner.GetTabbedTextExtent(lpszString,nCount,nTabPositions,lpnTabStopPositions)
	}

	#[inline(always)]
	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.GrayString(hBrush,lpfnOutput,lpData,nCount,x,y,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn GetTextAlign(&self) -> UINT {
		self.inner.GetTextAlign()
	}

	#[inline(always)]
	pub fn SetTextAlign(&self,nFlags: UINT) -> UINT {
		self.inner.SetTextAlign(nFlags)
	}
	// pub fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {

	#[inline(always)]
	pub fn GetTextFaceLen(&self) -> c_int {
		self.inner.GetTextFaceLen()
	}
	// pub fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// pub fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	#[inline(always)]
	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW) -> BOOL {
		self.inner.GetTextMetrics(lpMetrics)
	}

	#[inline(always)]
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int) -> c_int {
		self.inner.SetTextJustification(nBreakExtra,nBreakCount)
	}

	#[inline(always)]
	pub fn GetTextCharacterExtra(&self) -> c_int {
		self.inner.GetTextCharacterExtra()
	}

	#[inline(always)]
	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int) -> c_int {
		self.inner.SetTextCharacterExtra(nCharExtra)
	}

	#[inline(always)]
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT) -> BOOL {
		self.inner.DrawEdge(lpRect,nEdge,nFlags)
	}

	#[inline(always)]
	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT) -> BOOL {
		self.inner.DrawFrameControl(lpRect,nType,nState)
	}

	#[inline(always)]
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT) -> BOOL {
		self.inner.ScrollDC(dx,dy,lpRectScroll,lpRectClip,hRgnUpdate,lpRectUpdate)
	}

	#[inline(always)]
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT) -> BOOL {
		self.inner.GetCharWidth_float(nFirstChar,nLastChar,lpFloatBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth32(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn SetMapperFlags(&self,dwFlag: DWORD) -> DWORD {
		self.inner.SetMapperFlags(dwFlag)
	}

	#[inline(always)]
	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetAspectRatioFilter(lpSize)
	}

	#[inline(always)]
	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidths(nFirstChar,nLastChar,lpabc)
	}

	#[inline(always)]
	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD) -> DWORD {
		self.inner.GetFontData(dwTable,dwOffset,lpData,cbData)
	}

	#[inline(always)]
	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR) -> c_int {
		self.inner.GetKerningPairs(nPairs,lpkrnpair)
	}

	#[inline(always)]
	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW) -> UINT {
		self.inner.GetOutlineTextMetrics(cbData,lpotm)
	}

	#[inline(always)]
	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2) -> DWORD {
		self.inner.GetGlyphOutline(nChar,nFormat,lpgm,cbBuffer,lpBuffer,lpmat2)
	}

	#[inline(always)]
	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT) -> BOOL {
		self.inner.GetCharABCWidths_float(nFirstChar,nLastChar,lpABCF)
	}

	#[inline(always)]
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID) -> c_int {
		self.inner.Escape(nEscape,nCount,lpszInData,lpOutData)
	}

	#[inline(always)]
	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR) -> c_int {
		self.inner.Escape_ext(nEscape,nInputSize,lpszInputData,nOutputSize,lpszOutputData)
	}

	#[inline(always)]
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR) -> c_int {
		self.inner.DrawEscape(nEscape,nInputSize,lpszInputData)
	}

	#[inline(always)]
	pub fn StartDoc_name(&self,lpszDocName: &str) -> c_int {
		self.inner.StartDoc_name(lpszDocName)
	}

	#[inline(always)]
	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW) -> c_int {
		self.inner.StartDoc(lpDocInfo)
	}

	#[inline(always)]
	pub fn StartPage(&self) -> c_int {
		self.inner.StartPage()
	}

	#[inline(always)]
	pub fn EndPage(&self) -> c_int {
		self.inner.EndPage()
	}

	#[inline(always)]
	pub fn SetAbortProc(&self,lpfn: ABORTPROC ) -> c_int {
		self.inner.SetAbortProc(lpfn)
	}

	#[inline(always)]
	pub fn AbortDoc(&self) -> c_int {
		self.inner.AbortDoc()
	}

	#[inline(always)]
	pub fn EndDoc(&self) -> c_int {
		self.inner.EndDoc()
	}

	#[inline(always)]
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.inner.PlayMetaFile(hMF)
	}

	#[inline(always)]
	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT) -> BOOL {
		self.inner.PlayMetaFile_enh(hEnhMetaFile,lpBounds)
	}

	#[inline(always)]
	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE) -> BOOL {
		self.inner.AddMetaFileComment(nDataSize,pCommentData)
	}

	#[inline(always)]
	pub fn AbortPath(&self) -> BOOL {
		self.inner.AbortPath()
	}

	#[inline(always)]
	pub fn BeginPath(&self) -> BOOL {
		self.inner.BeginPath()
	}

	#[inline(always)]
	pub fn CloseFigure(&self) -> BOOL {
		self.inner.CloseFigure()
	}

	#[inline(always)]
	pub fn EndPath(&self) -> BOOL {
		self.inner.EndPath()
	}

	#[inline(always)]
	pub fn FillPath(&self) -> BOOL {
		self.inner.FillPath()
	}

	#[inline(always)]
	pub fn FlattenPath(&self) -> BOOL {
		self.inner.FlattenPath()
	}

	#[inline(always)]
	pub fn StrokeAndFillPath(&self) -> BOOL {
		self.inner.StrokeAndFillPath()
	}

	#[inline(always)]
	pub fn StrokePath(&self) -> BOOL {
		self.inner.StrokePath()
	}

	#[inline(always)]
	pub fn WidenPath(&self) -> BOOL {
		self.inner.WidenPath()
	}

	#[inline(always)]
	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT) -> BOOL {
		self.inner.GetMiterLimit(pfMiterLimit)
	}

	#[inline(always)]
	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT) -> BOOL {
		self.inner.SetMiterLimit(fMiterLimit)
	}

	#[inline(always)]
	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int) -> c_int {
		self.inner.GetPath(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn SelectClipPath(&self,nMode: c_int) -> BOOL {
		self.inner.SelectClipPath(nMode)
	}
	// pub fn GetHalftoneBrush()->CBrushHandle {

	// #[inline(always)]
	// pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH>,mut hBrushLast: Option<HBRUSH>) {
	// 	self.inner.DrawDragRect(lpRect,size,lpRectLast,sizeLast, hBrush, hBrushLast)
	// }

	#[inline(always)]
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.inner.FillSolidRect_rect(lpRect,clr)
	}

	#[inline(always)]
	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.inner.FillSolidRect(x,y,cx,cy,clr)
	}

	#[inline(always)]
	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect_rect(lpRect,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect(x,y,cx,cy,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT) -> c_int {
		self.inner.SetDIBitsToDevice(x,y,dwWidth,dwHeight,xSrc,ySrc,uStartScan,cScanLines,lpvBits,lpbmi,uColorUse)
	}

	#[inline(always)]
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD) -> c_int {
		self.inner.StretchDIBits(x,y,nWidth,nHeight,xSrc,ySrc,nSrcWidth,nSrcHeight,lpvBits,lpbmi,uColorUse,dwRop)
	}

	#[inline(always)]
	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD) -> UINT {
		self.inner.GetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD) -> UINT {
		self.inner.SetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.ChoosePixelFormat(ppfd)
	}

	#[inline(always)]
	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.DescribePixelFormat(iPixelFormat,nBytes,ppfd)
	}

	#[inline(always)]
	pub fn GetPixelFormat(&self) -> c_int {
		self.inner.GetPixelFormat()
	}

	#[inline(always)]
	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR) -> BOOL {
		self.inner.SetPixelFormat(iPixelFormat,ppfd)
	}

	#[inline(always)]
	pub fn SwapBuffers(&self) -> BOOL {
		self.inner.SwapBuffers()
	}

	#[inline(always)]
	pub fn wglCreateContext(&self) -> HGLRC {
		self.inner.wglCreateContext()
	}

	#[inline(always)]
	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int) -> HGLRC {
		self.inner.wglCreateLayerContext(iLayerPlane)
	}

	#[inline(always)]
	pub fn wglMakeCurrent(&self,hglrc: HGLRC) -> BOOL {
		self.inner.wglMakeCurrent(hglrc)
	}

	#[inline(always)]
	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD) -> BOOL {
		self.inner.wglUseFontBitmaps(dwFirst,dwCount,listBase)
	}

	#[inline(always)]
	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
		self.inner.wglUseFontOutlines(dwFirst,dwCount,listBase,deviation,extrusion,format,lpgmf)
	}

	#[inline(always)]
	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR) -> BOOL {
		self.inner.wglDescribeLayerPlane(iPixelFormat,iLayerPlane,nBytes,plpd)
	}

	#[inline(always)]
	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF) -> c_int {
		self.inner.wglSetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF) -> c_int {
		self.inner.wglGetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL) -> BOOL {
		self.inner.wglRealizeLayerPalette(iLayerPlane,bRealize)
	}

	#[inline(always)]
	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT) -> BOOL {
		self.inner.wglSwapLayerBuffers(uPlanes)
	}

	#[inline(always)]
	pub fn GetDCPenColor(&self) -> COLORREF {
		self.inner.GetDCPenColor()
	}

	#[inline(always)]
	pub fn SetDCPenColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCPenColor(clr)
	}

	#[inline(always)]
	pub fn GetDCBrushColor(&self) -> COLORREF {
		self.inner.GetDCBrushColor()
	}

	#[inline(always)]
	pub fn SetDCBrushColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCBrushColor(clr)
	}

	#[inline(always)]
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET) -> DWORD {
		self.inner.GetFontUnicodeRanges(lpgs)
	}

	#[inline(always)]
	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD) -> DWORD {
		self.inner.GetGlyphIndices(lpstr,cch,pgi,dwFlags)
	}

	#[inline(always)]
	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentPointI(pgiIn,cgi,lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentExPointI(pgiIn,cgi,nMaxExtent,lpnFit,alpDx,lpSize)
	}

	#[inline(always)]
	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidthI(giFirst,cgi,pgi,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidthsI(giFirst,cgi,pgi,lpabc)
	}

	#[inline(always)]
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD) -> BOOL {
		self.inner.ColorCorrectPalette(hPalette,dwFirstEntry,dwNumOfEntries)
	}
}

////////////////////////////////////////////////////
impl CWindowDC {
	//  pub fn Attach(&self,hDC: HDC) {

	#[inline(always)]
	pub fn Detach(&mut self) -> HDC {
		self.inner.Detach()
	}

	#[inline(always)]
	pub fn assert_dc(&self) {
		self.inner.assert_dc()
	}

	#[inline(always)]
	pub fn assert_null_dc(&self) {
		self.inner.assert_null_dc()
	}
	//pub fn HDC (&self)->operator { return self.inner.hdc; }

	#[inline(always)]
	pub fn IsNull(&self) -> bool {
		self.inner.IsNull()
	}

	#[inline(always)]
	pub fn WindowFromDC(&self) -> HWND {
		self.inner.WindowFromDC()
	}
	// pub fn GetCurrentPen (&self)->CPenHandle {
	// pub fn GetCurrentBrush (&self)->CBrushHandle {
	// pub fn GetCurrentPalette (&self)->CPaletteHandle {
	// pub fn GetCurrentFont (&self)->CFontHandle {
	// pub fn GetCurrentBitmap (&self)->CBitmapHandle {
	//  pub fn CreateDC(&self,lpszDriverName: LPCTSTR,lpszDeviceName: LPCTSTR,lpszOutput: LPCTSTR,lpInitData: &DEVMODE)->HDC {

	#[inline(always)]
	pub fn CreateCompatibleDC(&mut self, hDC: Option<HDC>) -> HDC {
		self.inner.CreateCompatibleDC(hDC)
	}

	#[inline(always)]
	pub fn DeleteDC(&mut self) -> BOOL {
		self.inner.DeleteDC()
	}

	#[inline(always)]
	pub fn SaveDC(&self) -> c_int {
		self.inner.SaveDC()
	}

	#[inline(always)]
	pub fn RestoreDC(&self,nSavedDC: c_int) -> BOOL {
		self.inner.RestoreDC(nSavedDC)
	}

	#[inline(always)]
	pub fn GetDeviceCaps(&self,nIndex: c_int) -> c_int {
		self.inner.GetDeviceCaps(nIndex)
	}

	#[inline(always)]
	pub fn SetBoundsRect(&self,lpRectBounds: LPCRECT,flags: UINT) -> UINT {
		self.inner.SetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn GetBoundsRect(&self,lpRectBounds: LPRECT,flags: UINT) -> UINT {
		self.inner.GetBoundsRect(lpRectBounds,flags)
	}

	#[inline(always)]
	pub fn ResetDC(&self,lpDevMode: &DEVMODEW) -> BOOL {
		self.inner.ResetDC(lpDevMode)
	}

	#[inline(always)]
	pub fn GetBrushOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetBrushOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetBrushOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetBrushOrg_point(point,lpPointRet)
	}
	// 	 pub fn EnumObjects(&self,nObjectType: c_int,@ c_int (CALLBACK* lpfn)(LPVOID,@ LPARAM),lpData: LPARAM)->c_int {

	#[inline(always)]
	pub fn SelectPen(&self,hPen: HPEN) -> HPEN {
		self.inner.SelectPen(hPen)
	}

	#[inline(always)]
	pub fn SelectBrush(&self,hBrush: HBRUSH) -> HBRUSH {
		self.inner.SelectBrush(hBrush)
	}

	#[inline(always)]
	pub fn SelectFont(&self,hFont: HFONT) -> HFONT {
		self.inner.SelectFont(hFont)
	}

	#[inline(always)]
	pub fn SelectBitmap(&self,hBitmap: HBITMAP) -> HBITMAP {
		self.inner.SelectBitmap(hBitmap)
	}

	#[inline(always)]
	pub fn SelectRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectRgn(hRgn)
	}

	#[inline(always)]
	pub fn SelectStockPen(&self,nPen: c_int) -> HPEN {
		self.inner.SelectStockPen((nPen as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockBrush(&self,nBrush: c_int) -> HBRUSH {
		self.inner.SelectStockBrush((nBrush as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockFont(&self,nFont: c_int) -> HFONT {
		self.inner.SelectStockFont((nFont as usize) as c_uint)
	}

	#[inline(always)]
	pub fn SelectStockPalette(&self,nPalette: c_int,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectStockPalette(nPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn GetNearestColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.GetNearestColor(crColor)
	}

	#[inline(always)]
	pub fn SelectPalette(&self,hPalette: HPALETTE,bForceBackground: BOOL) -> HPALETTE {
		self.inner.SelectPalette(hPalette,bForceBackground)
	}

	#[inline(always)]
	pub fn RealizePalette(&self) -> UINT {
		self.inner.RealizePalette()
	}
		pub fn UpdateColors(&self) {
		self.inner.UpdateColors()
	}

	#[inline(always)]
	pub fn GetBkColor(&self) -> COLORREF {
		self.inner.GetBkColor()
	}

	#[inline(always)]
	pub fn GetBkMode(&self) -> c_int {
		self.inner.GetBkMode()
	}

	#[inline(always)]
	pub fn GetPolyFillMode(&self) -> c_int {
		self.inner.GetPolyFillMode()
	}

	#[inline(always)]
	pub fn GetROP2(&self) -> c_int {
		self.inner.GetROP2()
	}

	#[inline(always)]
	pub fn GetStretchBltMode(&self) -> c_int {
		self.inner.GetStretchBltMode()
	}

	#[inline(always)]
	pub fn GetTextColor(&self) -> COLORREF {
		self.inner.GetTextColor()
	}

	#[inline(always)]
	pub fn SetBkColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetBkColor(crColor)
	}

	#[inline(always)]
	pub fn SetBkMode(&self,nBkMode: c_int) -> c_int {
		self.inner.SetBkMode(nBkMode)
	}

	#[inline(always)]
	pub fn SetPolyFillMode(&self,nPolyFillMode: c_int) -> c_int {
		self.inner.SetPolyFillMode(nPolyFillMode)
	}

	#[inline(always)]
	pub fn SetROP2(&self,nDrawMode: c_int) -> c_int {
		self.inner.SetROP2(nDrawMode)
	}

	#[inline(always)]
	pub fn SetStretchBltMode(&self,nStretchMode: c_int) -> c_int {
		self.inner.SetStretchBltMode(nStretchMode)
	}

	#[inline(always)]
	pub fn SetTextColor(&self,crColor: COLORREF) -> COLORREF {
		self.inner.SetTextColor(crColor)
	}

	#[inline(always)]
	pub fn GetColorAdjustment(&self,lpColorAdjust: LPCOLORADJUSTMENT) -> BOOL {
		self.inner.GetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn SetColorAdjustment(&self,lpColorAdjust: &COLORADJUSTMENT) -> BOOL {
		self.inner.SetColorAdjustment(lpColorAdjust)
	}

	#[inline(always)]
	pub fn GetMapMode(&self) -> c_int {
		self.inner.GetMapMode()
	}

	#[inline(always)]
	pub fn GetViewportOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetViewportOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetMapMode(&self,nMapMode: c_int) -> c_int {
		self.inner.SetMapMode(nMapMode)
	}

	#[inline(always)]
	pub fn SetViewportOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetViewportOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetViewportOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetViewportOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetViewportOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetViewportExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetViewportExt(lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetViewportExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetViewportExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleViewportExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleViewportExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn GetWindowOrg(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetWindowOrg(lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn SetWindowOrg_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.SetWindowOrg_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn OffsetWindowOrg(&self,nWidth: c_int,nHeight: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.OffsetWindowOrg(nWidth,nHeight,lpPoint)
	}

	#[inline(always)]
	pub fn GetWindowExt(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetWindowExt(lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt(&self,x: c_int,y: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt(x,y,lpSize)
	}

	#[inline(always)]
	pub fn SetWindowExt_size(&self,size: SIZE, lpSizeRet: Option<LPSIZE>) -> BOOL {
		self.inner.SetWindowExt_size(size,lpSizeRet)
	}

	#[inline(always)]
	pub fn ScaleWindowExt(&self,xNum: c_int,xDenom: c_int,yNum: c_int,yDenom: c_int, lpSize: Option<LPSIZE>) -> BOOL {
		self.inner.ScaleWindowExt(xNum,xDenom,yNum,yDenom,lpSize)
	}

	#[inline(always)]
	pub fn DPtoLP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.DPtoLP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn DPtoLP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.DPtoLP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn DPtoLP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.DPtoLP_Size(lpSize)
	}

	#[inline(always)]
	pub fn LPtoDP(&self,lpPoints: LPPOINT, nCount: Option<c_int>) -> BOOL {
		self.inner.LPtoDP(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn LPtoDP_Rect(&self,lpRect: LPRECT) -> BOOL {
		self.inner.LPtoDP_Rect(lpRect)
	}

	#[inline(always)]
	pub fn LPtoDP_Size(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.LPtoDP_Size(lpSize)
	}

	#[inline(always)]
	pub fn DPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.DPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoDP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoDP(lpSize)
	}

	#[inline(always)]
	pub fn LPtoHIMETRIC(&self,lpSize: LPSIZE) {
		self.inner.LPtoHIMETRIC(lpSize)
	}

	#[inline(always)]
	pub fn HIMETRICtoLP(&self,lpSize: LPSIZE) {
		self.inner.HIMETRICtoLP(lpSize)
	}

	#[inline(always)]
	pub fn FillRgn(&self,hRgn: HRGN,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRgn(hRgn,hBrush)
	}

	#[inline(always)]
	pub fn FrameRgn(&self,hRgn: HRGN,hBrush: HBRUSH,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.FrameRgn(hRgn,hBrush,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn InvertRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.InvertRgn(hRgn)
	}

	#[inline(always)]
	pub fn PaintRgn(&self,hRgn: HRGN) -> BOOL {
		self.inner.PaintRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetClipBox(&self,lpRect: LPRECT) -> c_int {
		self.inner.GetClipBox(lpRect)
	}
	// pub fn GetClipRgn (&self,region: &mut CRgn)->c_int {

	#[inline(always)]
	pub fn PtVisible(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.PtVisible(x,y)
	}

	#[inline(always)]
	pub fn PtVisible_point(&self,point: POINT) -> BOOL {
		self.inner.PtVisible_point(point)
	}

	#[inline(always)]
	pub fn RectVisible(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.RectVisible(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.ExcludeClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn ExcludeClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.ExcludeClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn ExcludeUpdateRgn(&self,hWnd: HWND) -> c_int {
		self.inner.ExcludeUpdateRgn(hWnd)
	}

	#[inline(always)]
	pub fn IntersectClipRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> c_int {
		self.inner.IntersectClipRect(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn IntersectClipRect_rect(&self,lpRect: LPCRECT) -> c_int {
		self.inner.IntersectClipRect_rect(lpRect)
	}

	#[inline(always)]
	pub fn OffsetClipRgn(&self,x: c_int,y: c_int) -> c_int {
		self.inner.OffsetClipRgn(x,y)
	}

	#[inline(always)]
	pub fn OffsetClipRgn_size(&self,size: SIZE) -> c_int {
		self.inner.OffsetClipRgn_size(size)
	}

	#[inline(always)]
	pub fn SelectClipRgn_mode(&self,hRgn: HRGN,nMode: c_int) -> c_int {
		self.inner.SelectClipRgn_mode(hRgn,nMode)
	}

	#[inline(always)]
	pub fn SelectClipRgn(&self,hRgn: HRGN) -> c_int {
		self.inner.SelectClipRgn(hRgn)
	}

	#[inline(always)]
	pub fn GetCurrentPosition(&self,lpPoint: LPPOINT) -> BOOL {
		self.inner.GetCurrentPosition(lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo(&self,x: c_int,y: c_int, lpPoint: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo(x,y,lpPoint)
	}

	#[inline(always)]
	pub fn MoveTo_point(&self,point: POINT, lpPointRet: Option<LPPOINT>) -> BOOL {
		self.inner.MoveTo_point(point,lpPointRet)
	}

	#[inline(always)]
	pub fn LineTo(&self,x: c_int,y: c_int) -> BOOL {
		self.inner.LineTo(x,y)
	}

	#[inline(always)]
	pub fn LineTo_point(&self,point: POINT) -> BOOL {
		self.inner.LineTo_point(point)
	}

	#[inline(always)]
	pub fn Arc(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Arc(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Arc_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Arc_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polyline(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polyline(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn AngleArc(&self,x: c_int,y: c_int,nRadius: c_int,fStartAngle: FLOAT,fSweepAngle: FLOAT) -> BOOL {
		self.inner.AngleArc(x,y,nRadius,fStartAngle,fSweepAngle)
	}

	#[inline(always)]
	pub fn ArcTo(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.ArcTo(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn ArcTo_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.ArcTo_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn GetArcDirection(&self) -> c_int {
		self.inner.GetArcDirection()
	}

	#[inline(always)]
	pub fn SetArcDirection(&self,nArcDirection: c_int) -> c_int {
		self.inner.SetArcDirection(nArcDirection)
	}

	#[inline(always)]
	pub fn PolyDraw(&self,lpPoints: &POINT,lpTypes: &BYTE,nCount: c_int) -> BOOL {
		self.inner.PolyDraw(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn PolylineTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolylineTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolyline(&self, lpPoints: &POINT, lpPolyPoints: &DWORD, nCount: c_int) -> BOOL {
		self.inner.PolyPolyline(lpPoints,lpPolyPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezier(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezier(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyBezierTo(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.PolyBezierTo(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn FillRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FillRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn FillRect_index(&self,lpRect: LPCRECT,nColorIndex: c_int) -> BOOL {
		self.inner.FillRect_index(lpRect,nColorIndex)
	}

	#[inline(always)]
	pub fn FrameRect(&self,lpRect: LPCRECT,hBrush: HBRUSH) -> BOOL {
		self.inner.FrameRect(lpRect,hBrush)
	}

	#[inline(always)]
	pub fn InvertRect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.InvertRect(lpRect)
	}

	#[inline(always)]
	pub fn DrawIcon(&self,x: c_int,y: c_int,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon(x,y,hIcon)
	}

	#[inline(always)]
	pub fn DrawIcon_point(&self,point: POINT,hIcon: HICON) -> BOOL {
		self.inner.DrawIcon_point(point,hIcon)
	}

	#[inline(always)]
	pub fn DrawIconEx(&self,x: c_int,y: c_int,hIcon: HICON,cxWidth: c_int,cyWidth: c_int, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx(x,y,hIcon,cxWidth,cyWidth,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawIconEx_point(&self,point: POINT,hIcon: HICON,size: SIZE, uStepIfAniCur: Option<UINT>, hbrFlickerFreeDraw: Option<HBRUSH>, uFlags: Option<UINT>) -> BOOL {
		self.inner.DrawIconEx_point(point,hIcon,size,uStepIfAniCur,hbrFlickerFreeDraw,uFlags)
	}

	#[inline(always)]
	pub fn DrawState_bitmap(&self,pt: POINT,size: SIZE,hBitmap: HBITMAP,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_bitmap(pt,size,hBitmap,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState(&self,pt: POINT,size: SIZE,hIcon: HICON,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState(pt,size,hIcon,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn DrawState_text(&self,pt: POINT,size: SIZE,lpszText: &str, nFlags: UINT, bPrefixText: Option<BOOL>, nTextLen: Option<c_int>, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_text(pt,size,lpszText, nFlags, bPrefixText, nTextLen, hBrush)
	}

	#[inline(always)]
	pub fn DrawState_proc(&self,pt: POINT,size: SIZE,lpDrawProc: DRAWSTATEPROC,lData: LPARAM,nFlags: UINT, hBrush: Option<HBRUSH>) -> BOOL {
		self.inner.DrawState_proc(pt,size,lpDrawProc,lData,nFlags,hBrush)
	}

	#[inline(always)]
	pub fn Chord(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Chord(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Chord_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Chord_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn DrawFocusRect(&self,lpRect: LPCRECT) {
		self.inner.DrawFocusRect(lpRect)
	}

	#[inline(always)]
	pub fn Ellipse(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Ellipse(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Ellipse_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Ellipse_rect(lpRect)
	}

	#[inline(always)]
	pub fn Pie(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int,x4: c_int,y4: c_int) -> BOOL {
		self.inner.Pie(x1,y1,x2,y2,x3,y3,x4,y4)
	}

	#[inline(always)]
	pub fn Pie_rect(&self,lpRect: LPCRECT,ptStart: POINT,ptEnd: POINT) -> BOOL {
		self.inner.Pie_rect(lpRect,ptStart,ptEnd)
	}

	#[inline(always)]
	pub fn Polygon(&self,lpPoints: &POINT,nCount: c_int) -> BOOL {
		self.inner.Polygon(lpPoints,nCount)
	}

	#[inline(always)]
	pub fn PolyPolygon(&self,lpPoints: &POINT,lpPolyCounts: &c_int,nCount: c_int) -> BOOL {
		self.inner.PolyPolygon(lpPoints,lpPolyCounts,nCount)
	}

	#[inline(always)]
	pub fn Rectangle(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int) -> BOOL {
		self.inner.Rectangle(x1,y1,x2,y2)
	}

	#[inline(always)]
	pub fn Rectangle_rect(&self,lpRect: LPCRECT) -> BOOL {
		self.inner.Rectangle_rect(lpRect)
	}

	#[inline(always)]
	pub fn RoundRect(&self,x1: c_int,y1: c_int,x2: c_int,y2: c_int,x3: c_int,y3: c_int) -> BOOL {
		self.inner.RoundRect(x1,y1,x2,y2,x3,y3)
	}

	#[inline(always)]
	pub fn RoundRect_rect(&self,lpRect: LPCRECT,point: POINT) -> BOOL {
		self.inner.RoundRect_rect(lpRect,point)
	}

	#[inline(always)]
	pub fn PatBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.PatBlt(x,y,nWidth,nHeight,dwRop)
	}

	#[inline(always)]
	pub fn BitBlt(&self,x: c_int, y: c_int, nWidth: c_int, nHeight: c_int, hSrcDC: HDC, xSrc: c_int, ySrc: c_int, dwRop: DWORD) -> BOOL {
		self.inner.BitBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,dwRop)
	}

	#[inline(always)]
	pub fn StretchBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,dwRop: DWORD) -> BOOL {
		self.inner.StretchBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,dwRop)
	}

	#[inline(always)]
	pub fn GetPixel(&self,x: c_int,y: c_int) -> COLORREF {
		self.inner.GetPixel(x,y)
	}

	#[inline(always)]
	pub fn GetPixel_point(&self,point: POINT) -> COLORREF {
		self.inner.GetPixel_point(point)
	}

	#[inline(always)]
	pub fn SetPixel(&self,x: c_int,y: c_int,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixel_point(&self,point: POINT,crColor: COLORREF) -> COLORREF {
		self.inner.SetPixel_point(point,crColor)
	}

	#[inline(always)]
	pub fn FloodFill(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.FloodFill(x,y,crColor)
	}

	#[inline(always)]
	pub fn ExtFloodFill(&self,x: c_int,y: c_int,crColor: COLORREF,nFillType: UINT) -> BOOL {
		self.inner.ExtFloodFill(x,y,crColor,nFillType)
	}

	#[inline(always)]
	pub fn MaskBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int,dwRop: DWORD) -> BOOL {
		self.inner.MaskBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,hMaskBitmap,xMask,yMask,dwRop)
	}

	#[inline(always)]
	pub fn PlgBlt(&self,lpPoint: LPPOINT,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nWidth: c_int,nHeight: c_int,hMaskBitmap: HBITMAP,xMask: c_int,yMask: c_int) -> BOOL {
		self.inner.PlgBlt(lpPoint,hSrcDC,xSrc,ySrc,nWidth,nHeight,hMaskBitmap,xMask,yMask)
	}

	#[inline(always)]
	pub fn SetPixelV(&self,x: c_int,y: c_int,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV(x,y,crColor)
	}

	#[inline(always)]
	pub fn SetPixelV_point(&self,point: POINT,crColor: COLORREF) -> BOOL {
		self.inner.SetPixelV_point(point,crColor)
	}

	#[inline(always)]
	pub fn TransparentBlt(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT) -> BOOL {
		self.inner.TransparentBlt(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,crTransparent)
	}
	//  pub fn TransparentImage(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,crTransparent: UINT)->BOOL {

	#[inline(always)]
	pub fn GradientFill(&self,pVertices: &mut TRIVERTEX,nVertices: DWORD,pMeshElements: LPVOID,nMeshElements: DWORD,dwMode: DWORD) -> BOOL {
		self.inner.GradientFill(pVertices,nVertices,pMeshElements,nMeshElements,dwMode)
	}

	#[inline(always)]
	pub fn GradientFillRect(&self, rect: &RECT, clr1: COLORREF, clr2: COLORREF,  bHorizontal: bool) -> BOOL {
		self.inner.GradientFillRect(rect,clr1,clr2,bHorizontal)
	}

	#[inline(always)]
	pub fn AlphaBlend(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,hSrcDC: HDC,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,bf: BLENDFUNCTION) -> BOOL {
		self.inner.AlphaBlend(x,y,nWidth,nHeight,hSrcDC,xSrc,ySrc,nSrcWidth,nSrcHeight,bf)
	}
	//pub fn TextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>)->BOOL {

	#[inline(always)]
	pub fn TextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>) -> BOOL {
		self.inner.TextOut(x,y,lpszString,nCount)
	}
	//pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: LPCTSTR,mut nCount: Option<UINT>,mut lpDxWidths: Option<LPINT>)->BOOL {

	#[inline(always)]
	pub fn ExtTextOut(&self,x: c_int,y: c_int,nOptions: UINT,lpRect: LPCRECT,lpszString: &str, nCount: Option<UINT>, lpDxWidths: Option<LPINT>) -> BOOL {
		self.inner.ExtTextOut(x,y,nOptions,lpRect,lpszString,nCount,lpDxWidths)
	}
	//pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: LPCTSTR,mut nCount: Option<c_int>,mut nTabPositions: Option<c_int>,mut lpnTabStopPositions: Option<LPINT>,mut nTabOrigin: Option<c_int>)->SIZE {

	#[inline(always)]
	pub fn TabbedTextOut(&self,x: c_int,y: c_int,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>, nTabOrigin: Option<c_int>) -> SIZE {
		self.inner.TabbedTextOut(x,y,lpszString,nCount,nTabPositions,lpnTabStopPositions,nTabOrigin)
	}
	//pub fn DrawText(&self,lpstrText: LPCTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {

	#[inline(always)]
	pub fn DrawText(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT) -> c_int {
		self.inner.DrawText(lpstrText,cchText,lpRect,uFormat)
	}
	//  pub fn DrawText(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT)->c_int {
	//pub fn DrawTextEx(&self,lpstrText: LPTSTR,cchText: c_int,lpRect: LPRECT,uFormat: UINT,mut lpDTParams: Option<LPDRAWTEXTPARAMS>)->c_int {

	#[inline(always)]
	pub fn DrawTextEx(&self,lpstrText: &str,cchText: c_int,lpRect: LPRECT,uFormat: UINT, lpDTParams: Option<LPDRAWTEXTPARAMS>) -> c_int {
		self.inner.DrawTextEx(lpstrText,cchText,lpRect,uFormat,lpDTParams)
	}

	#[inline(always)]
	pub fn DrawShadowText(&self,lpstrText: LPCWSTR,cchText: c_int,lpRect: LPRECT,dwFlags: DWORD,clrText: COLORREF,clrShadow: COLORREF,xOffset: c_int,yOffset: c_int) -> c_int {
		self.inner.DrawShadowText(lpstrText,cchText,lpRect,dwFlags,clrText,clrShadow,xOffset,yOffset)
	}

	#[inline(always)]
	pub fn GetTextExtent(&self,lpszString: &str, nCount: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtent(lpszString, nCount, lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPoint(&self,lpszString: &str,cchString: c_int,lpSize: LPSIZE,nMaxExtent: c_int,lpnFit: Option<LPINT>, alpDx: Option<LPINT>) -> BOOL {
		self.inner.GetTextExtentExPoint(lpszString,cchString,lpSize,nMaxExtent,lpnFit,alpDx)
	}

	#[inline(always)]
	pub fn GetTabbedTextExtent(&self,lpszString: &str, nCount: Option<c_int>, nTabPositions: Option<c_int>, lpnTabStopPositions: Option<LPINT>) -> DWORD {
		self.inner.GetTabbedTextExtent(lpszString,nCount,nTabPositions,lpnTabStopPositions)
	}

	#[inline(always)]
	pub fn GrayString(&self,hBrush: HBRUSH,lpfnOutput: GRAYSTRINGPROC,lpData: LPARAM,nCount: c_int,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int) -> BOOL {
		self.inner.GrayString(hBrush,lpfnOutput,lpData,nCount,x,y,nWidth,nHeight)
	}

	#[inline(always)]
	pub fn GetTextAlign(&self) -> UINT {
		self.inner.GetTextAlign()
	}

	#[inline(always)]
	pub fn SetTextAlign(&self,nFlags: UINT) -> UINT {
		self.inner.SetTextAlign(nFlags)
	}
	// pub fn GetTextFace(&self,lpszFacename: &String,nCount: c_int) -> c_int {

	#[inline(always)]
	pub fn GetTextFaceLen(&self) -> c_int {
		self.inner.GetTextFaceLen()
	}
	// pub fn GetTextFace (@BSTR& bstrFace)->BOOL {
	// pub fn GetTextFace (@_CSTRING_NS::CString& strFace)->c_int {

	#[inline(always)]
	pub fn GetTextMetrics(&self,lpMetrics: LPTEXTMETRICW) -> BOOL {
		self.inner.GetTextMetrics(lpMetrics)
	}

	#[inline(always)]
	pub fn SetTextJustification(&self,nBreakExtra: c_int,nBreakCount: c_int) -> c_int {
		self.inner.SetTextJustification(nBreakExtra,nBreakCount)
	}

	#[inline(always)]
	pub fn GetTextCharacterExtra(&self) -> c_int {
		self.inner.GetTextCharacterExtra()
	}

	#[inline(always)]
	pub fn SetTextCharacterExtra(&self,nCharExtra: c_int) -> c_int {
		self.inner.SetTextCharacterExtra(nCharExtra)
	}

	#[inline(always)]
	pub fn DrawEdge(&self,lpRect: LPRECT,nEdge: UINT,nFlags: UINT) -> BOOL {
		self.inner.DrawEdge(lpRect,nEdge,nFlags)
	}

	#[inline(always)]
	pub fn DrawFrameControl(&self,lpRect: LPRECT,nType: UINT,nState: UINT) -> BOOL {
		self.inner.DrawFrameControl(lpRect,nType,nState)
	}

	#[inline(always)]
	pub fn ScrollDC(&self,dx: c_int,dy: c_int,lpRectScroll: LPCRECT,lpRectClip: LPCRECT,hRgnUpdate: HRGN,lpRectUpdate: LPRECT) -> BOOL {
		self.inner.ScrollDC(dx,dy,lpRectScroll,lpRectClip,hRgnUpdate,lpRectUpdate)
	}

	#[inline(always)]
	pub fn GetCharWidth(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth_float(&self,nFirstChar: UINT,nLastChar: UINT,lpFloatBuffer: &mut FLOAT) -> BOOL {
		self.inner.GetCharWidth_float(nFirstChar,nLastChar,lpFloatBuffer)
	}

	#[inline(always)]
	pub fn GetCharWidth32(&self,nFirstChar: UINT,nLastChar: UINT,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidth32(nFirstChar,nLastChar,lpBuffer)
	}

	#[inline(always)]
	pub fn SetMapperFlags(&self,dwFlag: DWORD) -> DWORD {
		self.inner.SetMapperFlags(dwFlag)
	}

	#[inline(always)]
	pub fn GetAspectRatioFilter(&self,lpSize: LPSIZE) -> BOOL {
		self.inner.GetAspectRatioFilter(lpSize)
	}

	#[inline(always)]
	pub fn GetCharABCWidths(&self,nFirstChar: UINT,nLastChar: UINT,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidths(nFirstChar,nLastChar,lpabc)
	}

	#[inline(always)]
	pub fn GetFontData(&self,dwTable: DWORD,dwOffset: DWORD,lpData: LPVOID,cbData: DWORD) -> DWORD {
		self.inner.GetFontData(dwTable,dwOffset,lpData,cbData)
	}

	#[inline(always)]
	pub fn GetKerningPairs(&self,nPairs: c_int,lpkrnpair: LPKERNINGPAIR) -> c_int {
		self.inner.GetKerningPairs(nPairs,lpkrnpair)
	}

	#[inline(always)]
	pub fn GetOutlineTextMetrics(&self,cbData: UINT,lpotm: LPOUTLINETEXTMETRICW) -> UINT {
		self.inner.GetOutlineTextMetrics(cbData,lpotm)
	}

	#[inline(always)]
	pub fn GetGlyphOutline(&self,nChar: UINT,nFormat: UINT,lpgm: LPGLYPHMETRICS,cbBuffer: DWORD,lpBuffer: LPVOID,lpmat2: &MAT2) -> DWORD {
		self.inner.GetGlyphOutline(nChar,nFormat,lpgm,cbBuffer,lpBuffer,lpmat2)
	}

	#[inline(always)]
	pub fn GetCharABCWidths_float(&self,nFirstChar: UINT,nLastChar: UINT,lpABCF: LPABCFLOAT) -> BOOL {
		self.inner.GetCharABCWidths_float(nFirstChar,nLastChar,lpABCF)
	}

	#[inline(always)]
	pub fn Escape(&self,nEscape: c_int,nCount: c_int,lpszInData: LPCSTR,lpOutData: LPVOID) -> c_int {
		self.inner.Escape(nEscape,nCount,lpszInData,lpOutData)
	}

	#[inline(always)]
	pub fn Escape_ext(&self, nEscape: c_int , nInputSize: c_int, lpszInputData: LPCSTR, nOutputSize: c_int, lpszOutputData: LPSTR) -> c_int {
		self.inner.Escape_ext(nEscape,nInputSize,lpszInputData,nOutputSize,lpszOutputData)
	}

	#[inline(always)]
	pub fn DrawEscape(&self,nEscape: c_int,nInputSize: c_int,lpszInputData: LPCSTR) -> c_int {
		self.inner.DrawEscape(nEscape,nInputSize,lpszInputData)
	}

	#[inline(always)]
	pub fn StartDoc_name(&self,lpszDocName: &str) -> c_int {
		self.inner.StartDoc_name(lpszDocName)
	}

	#[inline(always)]
	pub fn StartDoc(&self,lpDocInfo: LPDOCINFOW) -> c_int {
		self.inner.StartDoc(lpDocInfo)
	}

	#[inline(always)]
	pub fn StartPage(&self) -> c_int {
		self.inner.StartPage()
	}

	#[inline(always)]
	pub fn EndPage(&self) -> c_int {
		self.inner.EndPage()
	}

	#[inline(always)]
	pub fn SetAbortProc(&self,lpfn: ABORTPROC ) -> c_int {
		self.inner.SetAbortProc(lpfn)
	}

	#[inline(always)]
	pub fn AbortDoc(&self) -> c_int {
		self.inner.AbortDoc()
	}

	#[inline(always)]
	pub fn EndDoc(&self) -> c_int {
		self.inner.EndDoc()
	}
	
	#[inline(always)]
	pub fn PlayMetaFile(&self,hMF: HMETAFILE)->BOOL {
		self.inner.PlayMetaFile(hMF)
	}

	#[inline(always)]
	pub fn PlayMetaFile_enh(&self,hEnhMetaFile: HENHMETAFILE,lpBounds: LPCRECT) -> BOOL {
		self.inner.PlayMetaFile_enh(hEnhMetaFile,lpBounds)
	}

	#[inline(always)]
	pub fn AddMetaFileComment(&self,nDataSize: UINT,pCommentData: &BYTE) -> BOOL {
		self.inner.AddMetaFileComment(nDataSize,pCommentData)
	}

	#[inline(always)]
	pub fn AbortPath(&self) -> BOOL {
		self.inner.AbortPath()
	}

	#[inline(always)]
	pub fn BeginPath(&self) -> BOOL {
		self.inner.BeginPath()
	}

	#[inline(always)]
	pub fn CloseFigure(&self) -> BOOL {
		self.inner.CloseFigure()
	}

	#[inline(always)]
	pub fn EndPath(&self) -> BOOL {
		self.inner.EndPath()
	}

	#[inline(always)]
	pub fn FillPath(&self) -> BOOL {
		self.inner.FillPath()
	}

	#[inline(always)]
	pub fn FlattenPath(&self) -> BOOL {
		self.inner.FlattenPath()
	}

	#[inline(always)]
	pub fn StrokeAndFillPath(&self) -> BOOL {
		self.inner.StrokeAndFillPath()
	}

	#[inline(always)]
	pub fn StrokePath(&self) -> BOOL {
		self.inner.StrokePath()
	}

	#[inline(always)]
	pub fn WidenPath(&self) -> BOOL {
		self.inner.WidenPath()
	}

	#[inline(always)]
	pub fn GetMiterLimit(&self,pfMiterLimit: PFLOAT) -> BOOL {
		self.inner.GetMiterLimit(pfMiterLimit)
	}

	#[inline(always)]
	pub fn SetMiterLimit(&self,fMiterLimit: FLOAT) -> BOOL {
		self.inner.SetMiterLimit(fMiterLimit)
	}

	#[inline(always)]
	pub fn GetPath(&self,lpPoints: LPPOINT,lpTypes: LPBYTE,nCount: c_int) -> c_int {
		self.inner.GetPath(lpPoints,lpTypes,nCount)
	}

	#[inline(always)]
	pub fn SelectClipPath(&self,nMode: c_int) -> BOOL {
		self.inner.SelectClipPath(nMode)
	}
	// pub fn GetHalftoneBrush()->CBrushHandle {

	// #[inline(always)]
	// pub fn DrawDragRect(&self,lpRect: LPCRECT,size: SIZE,lpRectLast: LPCRECT,sizeLast: SIZE,mut hBrush: Option<HBRUSH>,mut hBrushLast: Option<HBRUSH>) {
	// 	self.inner.DrawDragRect(lpRect,size,lpRectLast,sizeLast, hBrush, hBrushLast)
	// }

	#[inline(always)]
	pub fn FillSolidRect_rect(&self,lpRect: LPCRECT,clr: COLORREF) {
		self.inner.FillSolidRect_rect(lpRect,clr)
	}

	#[inline(always)]
	pub fn FillSolidRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clr: COLORREF) {
		self.inner.FillSolidRect(x,y,cx,cy,clr)
	}

	#[inline(always)]
	pub fn Draw3dRect_rect(&self,lpRect: LPCRECT,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect_rect(lpRect,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn Draw3dRect(&self,x: c_int,y: c_int,cx: c_int,cy: c_int,clrTopLeft: COLORREF,clrBottomRight: COLORREF) {
		self.inner.Draw3dRect(x,y,cx,cy,clrTopLeft,clrBottomRight)
	}

	#[inline(always)]
	pub fn SetDIBitsToDevice(&self,x: c_int,y: c_int,dwWidth: DWORD,dwHeight: DWORD,xSrc: c_int,ySrc: c_int,uStartScan: UINT,cScanLines: UINT,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT) -> c_int {
		self.inner.SetDIBitsToDevice(x,y,dwWidth,dwHeight,xSrc,ySrc,uStartScan,cScanLines,lpvBits,lpbmi,uColorUse)
	}

	#[inline(always)]
	pub fn StretchDIBits(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,xSrc: c_int,ySrc: c_int,nSrcWidth: c_int,nSrcHeight: c_int,lpvBits: &VOID,lpbmi: &BITMAPINFO,uColorUse: UINT,dwRop: DWORD) -> c_int {
		self.inner.StretchDIBits(x,y,nWidth,nHeight,xSrc,ySrc,nSrcWidth,nSrcHeight,lpvBits,lpbmi,uColorUse,dwRop)
	}

	#[inline(always)]
	pub fn GetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT, pColors: &mut RGBQUAD) -> UINT {
		self.inner.GetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn SetDIBColorTable(&self,uStartIndex: UINT,cEntries: UINT,pColors: &RGBQUAD) -> UINT {
		self.inner.SetDIBColorTable(uStartIndex,cEntries,pColors)
	}

	#[inline(always)]
	pub fn ChoosePixelFormat(&self,ppfd: &PIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.ChoosePixelFormat(ppfd)
	}

	#[inline(always)]
	pub fn DescribePixelFormat(&self,iPixelFormat: c_int,nBytes: UINT,ppfd: LPPIXELFORMATDESCRIPTOR) -> c_int {
		self.inner.DescribePixelFormat(iPixelFormat,nBytes,ppfd)
	}

	#[inline(always)]
	pub fn GetPixelFormat(&self) -> c_int {
		self.inner.GetPixelFormat()
	}

	#[inline(always)]
	pub fn SetPixelFormat(&self,iPixelFormat: c_int,ppfd: &PIXELFORMATDESCRIPTOR) -> BOOL {
		self.inner.SetPixelFormat(iPixelFormat,ppfd)
	}

	#[inline(always)]
	pub fn SwapBuffers(&self) -> BOOL {
		self.inner.SwapBuffers()
	}

	#[inline(always)]
	pub fn wglCreateContext(&self) -> HGLRC {
		self.inner.wglCreateContext()
	}

	#[inline(always)]
	pub fn wglCreateLayerContext(&self,iLayerPlane: c_int) -> HGLRC {
		self.inner.wglCreateLayerContext(iLayerPlane)
	}

	#[inline(always)]
	pub fn wglMakeCurrent(&self,hglrc: HGLRC) -> BOOL {
		self.inner.wglMakeCurrent(hglrc)
	}

	#[inline(always)]
	pub fn wglUseFontBitmaps(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD) -> BOOL {
		self.inner.wglUseFontBitmaps(dwFirst,dwCount,listBase)
	}

	#[inline(always)]
	pub fn wglUseFontOutlines(&self,dwFirst: DWORD,dwCount: DWORD,listBase: DWORD,deviation: FLOAT,extrusion: FLOAT,format: c_int,lpgmf: LPGLYPHMETRICSFLOAT) -> BOOL {
		self.inner.wglUseFontOutlines(dwFirst,dwCount,listBase,deviation,extrusion,format,lpgmf)
	}

	#[inline(always)]
	pub fn wglDescribeLayerPlane(&self,iPixelFormat: c_int,iLayerPlane: c_int,nBytes: UINT,plpd: LPLAYERPLANEDESCRIPTOR) -> BOOL {
		self.inner.wglDescribeLayerPlane(iPixelFormat,iLayerPlane,nBytes,plpd)
	}

	#[inline(always)]
	pub fn wglSetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int,pclr: &COLORREF) -> c_int {
		self.inner.wglSetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglGetLayerPaletteEntries(&self,iLayerPlane: c_int,iStart: c_int,cEntries: c_int, pclr: &mut COLORREF) -> c_int {
		self.inner.wglGetLayerPaletteEntries(iLayerPlane,iStart,cEntries,pclr)
	}

	#[inline(always)]
	pub fn wglRealizeLayerPalette(&self,iLayerPlane: c_int,bRealize: BOOL) -> BOOL {
		self.inner.wglRealizeLayerPalette(iLayerPlane,bRealize)
	}

	#[inline(always)]
	pub fn wglSwapLayerBuffers(&self,uPlanes: UINT) -> BOOL {
		self.inner.wglSwapLayerBuffers(uPlanes)
	}

	#[inline(always)]
	pub fn GetDCPenColor(&self) -> COLORREF {
		self.inner.GetDCPenColor()
	}

	#[inline(always)]
	pub fn SetDCPenColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCPenColor(clr)
	}

	#[inline(always)]
	pub fn GetDCBrushColor(&self) -> COLORREF {
		self.inner.GetDCBrushColor()
	}

	#[inline(always)]
	pub fn SetDCBrushColor(&self,clr: COLORREF) -> COLORREF {
		self.inner.SetDCBrushColor(clr)
	}

	#[inline(always)]
	pub fn GetFontUnicodeRanges(&self,lpgs: LPGLYPHSET) -> DWORD {
		self.inner.GetFontUnicodeRanges(lpgs)
	}

	#[inline(always)]
	pub fn GetGlyphIndices(&self,lpstr: &str,cch: c_int,pgi: LPWORD,dwFlags: DWORD) -> DWORD {
		self.inner.GetGlyphIndices(lpstr,cch,pgi,dwFlags)
	}

	#[inline(always)]
	pub fn GetTextExtentPointI(&self,pgiIn: LPWORD,cgi: c_int,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentPointI(pgiIn,cgi,lpSize)
	}

	#[inline(always)]
	pub fn GetTextExtentExPointI(&self,pgiIn: LPWORD,cgi: c_int,nMaxExtent: c_int,lpnFit: LPINT,alpDx: LPINT,lpSize: LPSIZE) -> BOOL {
		self.inner.GetTextExtentExPointI(pgiIn,cgi,nMaxExtent,lpnFit,alpDx,lpSize)
	}

	#[inline(always)]
	pub fn GetCharWidthI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpBuffer: LPINT) -> BOOL {
		self.inner.GetCharWidthI(giFirst,cgi,pgi,lpBuffer)
	}

	#[inline(always)]
	pub fn GetCharABCWidthsI(&self,giFirst: UINT,cgi: UINT,pgi: LPWORD,lpabc: LPABC) -> BOOL {
		self.inner.GetCharABCWidthsI(giFirst,cgi,pgi,lpabc)
	}

	#[inline(always)]
	pub fn ColorCorrectPalette(&self,hPalette: HPALETTE,dwFirstEntry: DWORD,dwNumOfEntries: DWORD) -> BOOL {
		self.inner.ColorCorrectPalette(hPalette,dwFirstEntry,dwNumOfEntries)
	}
}

////////////////////////////////////////////////////