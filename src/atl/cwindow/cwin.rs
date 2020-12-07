#![allow(dead_code,unused_assignments,non_snake_case)]
use std;
use winapi::*;
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use wtl::ToCU16Str;
use std::mem;

pub const NULL_HWND  : HWND  = 0 as HWND;
pub const NULL_LPARAM:LPARAM = 0 as LPARAM;

// pub trait WindowHandler {
//     fn FromHwnd(h: HWND) -> Self;
// }

fn GetModuleInstance() -> HINSTANCE {
    0 as HINSTANCE
}

#[derive(Debug)]
pub struct CWindow(HWND);

impl CWindow {
    pub fn new(h: HWND) -> CWindow {
        CWindow(h)
    }
}
///////////////////////////////////////

static RC_DEFAULT: RECT = RECT{left: CW_USEDEFAULT, right: CW_USEDEFAULT, top: 0, bottom: 0};

//this is different from CWinTrait,and it was introdced since wtl-rs
impl CWindow {
    pub fn GetHwnd(&self) -> HWND {
        self.0
    }

    pub fn Detach(&mut self) -> HWND { //only set m_hWnd = 0,this prevent most write ability from a hwndTrait
        let hWnd = self.0;
        self.0 = NULL_HWND;
        hWnd
    }

    pub fn Attach(&mut self, hWndNew: HWND) {
        debug_assert!(self.0 == NULL_HWND);
        debug_assert!(hWndNew != NULL_HWND);
        unsafe {
            debug_assert!(um::winuser::IsWindow(hWndNew) == winapi::shared::minwindef::TRUE);
        }
        self.0 = hWndNew;
    }

    #[inline]
    pub fn assert_window(&self) {
        debug_assert!(self.IsWindow());
    }

	//all get functions
	//output type depends on the inference of compiler
	// pub fn GetParent<T:WindowHandler> (&self) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{um::winuser::GetParent(self.0)})
	// }

	// pub fn SetParent<T:WindowHandler> (&self,hWndNewParent:HWND) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{um::winuser::SetParent(self.0, hWndNewParent)})
	// }

	// pub fn GetDlgItem<T:WindowHandler> (&self,nID:c_int) -> T {
	// 	self.assert_window();
	// 	T::FromHwnd(unsafe{um::winuser::GetDlgItem(self.0, nID)})
	// }
    pub fn GetParent(&self) -> HWND {
        self.assert_window();
        unsafe{um::winuser::GetParent(self.0)}
    }

    pub fn SetParent(&self,hWndNewParent:HWND) -> HWND {
        self.assert_window();
        unsafe{um::winuser::SetParent(self.0, hWndNewParent)}
    }

    pub fn GetDlgItem(&self,nID:WORD) -> HWND {
        self.assert_window();
        unsafe{um::winuser::GetDlgItem(self.0, nID as c_int)}
    }

	//add rewritted functions of above that use cwindow as output,sometimes very convenient
    pub fn GetParent2(&self) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                um::winuser::GetParent(self.0)
            })
    }

    pub fn SetParent2(&self, hWndNewParent: HWND) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                um::winuser::SetParent(self.0, hWndNewParent)
            })
    }

    pub fn GetDlgItem2(&self, nID: WORD) -> CWindow {
        self.assert_window();
        CWindow::new(unsafe {
                um::winuser::GetDlgItem(self.0, nID as c_int)
            })
    }

	//return cwindow
    pub fn GetTopWindow(&self) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::GetTopWindow(self.0))
        }
    }

    pub fn GetWindow(&self, nCmd: UINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::GetWindow(self.0, nCmd))
        }
    }

    pub fn GetLastActivePopup(&self) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::GetLastActivePopup(self.0))
        }
    }

	//https://msdn.microsoft.com/en-us/library/windows/desktop/ms632676(v=vs.85).aspx
	//I don't know what will get,so the return must be cwindow
    pub fn ChildWindowFromPoint(&self, point: POINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::ChildWindowFromPoint(self.0, point))
        }
    }

    pub fn ChildWindowFromPointEx(&self, point: POINT, uFlags: UINT) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::ChildWindowFromPointEx(self.0, point, uFlags))
        }
    }

    pub fn GetNextDlgGroupItem(&self, hWndCtl: HWND, bPrevious: BOOL) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::GetNextDlgGroupItem(self.0, hWndCtl, bPrevious))
        }
    }

    pub fn GetNextDlgTabItem(&self, hWndCtl: HWND, bPrevious: BOOL) -> CWindow {
        self.assert_window();
        unsafe {
            CWindow::new(um::winuser::GetNextDlgTabItem(self.0, hWndCtl, bPrevious))
        }
    }

    pub fn GetTopLevelParent(&self) -> CWindow {
        self.assert_window();

        let mut hWndParent: HWND = self.0;
        unsafe {
            let mut hWndTmp: HWND;
            loop {
                hWndTmp = um::winuser::GetParent(hWndParent);
                if hWndTmp == NULL_HWND {
                    break;
                }
                hWndParent = hWndTmp;
            }
            CWindow::new(hWndParent)
        }
    }

    pub fn GetTopLevelWindow(&self) -> CWindow {
        self.assert_window();

        let mut hWndParent: HWND;
        let mut hWndTmp: HWND = self.0;

        unsafe {
            loop {
                hWndParent = hWndTmp;
                hWndTmp = if (um::winuser::GetWindowLongW(hWndParent, GWL_STYLE) as DWORD) &
                             WS_CHILD != 0 {
                    um::winuser::GetParent(hWndParent)
                } else {
                    um::winuser::GetWindow(hWndParent, GW_OWNER)
                };

                if hWndTmp == NULL_HWND {
                    break;
                }
            }
        }

        CWindow::new(hWndParent)
    }

    pub fn GetDescendantWindow(&self, nID: c_int) -> CWindow {
        self.assert_window();
        let mut hWndTmp: HWND;
        unsafe {
            let mut hWndChild = um::winuser::GetDlgItem(self.0, nID);
            if hWndChild != NULL_HWND {
                if um::winuser::GetTopWindow(hWndChild) != NULL_HWND {
                    let wnd = CWindow::new(hWndChild);
                    hWndTmp = wnd.GetDescendantWindow(nID).GetHwnd();
                    if hWndTmp != NULL_HWND {
                        return CWindow::new(hWndTmp);
                    }
                }
                return CWindow::new(hWndChild);
            }

            loop {
                hWndChild = um::winuser::GetTopWindow(self.0);
                if hWndChild == NULL_HWND {
                    break;
                }
				//#define GetNextWindow(hWnd, wCmd) GetWindow(hWnd, wCmd)
                hWndChild = um::winuser::GetWindow(hWndChild, GW_HWNDNEXT);
                let wnd = CWindow::new(hWndChild);
                hWndTmp = wnd.GetDescendantWindow(nID).GetHwnd();
                if hWndTmp != NULL_HWND {
                    return CWindow::new(hWndTmp);
                }
            }

            CWindow::new(NULL_HWND)
        }
    }

    pub fn Create(&mut self,lpstrWndClass: &str,
                  hWndParent: HWND,
                  // _U_RECT rect = NULL,
                  rect: Option<&RECT>,
                  szWindowName: &str,
                  dwStyle: DWORD,
                  dwExStyle: DWORD,
                  // _U_MENUorID MenuOrID = 0U,
                  hMenu: HMENU,
                  lpCreateParam: LPVOID)
                  -> HWND {
		//ATLASSUME(self.0 == NULL);
        debug_assert!(self.0 == NULL_HWND);
		//assert!(self.0 == (0 as HWND));
		//if(rect.m_lpRect == NULL)
		//	rect.m_lpRect = &RC_DEFAULT;
        let r = if let Some(r1) = rect {
            r1
        }else{
            &RC_DEFAULT
        };
        let c = lpstrWndClass.to_c_u16();
        let n = szWindowName.to_c_u16();
        unsafe {
            self.0 = um::winuser::CreateWindowExW(dwExStyle,
                                    c.as_ptr(),
                                    n.as_ptr(),
                                    dwStyle,
                                    r.left,
                                    r.top,
                                    r.right - r.left,
                                    r.bottom - r.top,
                                    hWndParent,
                                    hMenu,
                                    // _AtlBaseModule.GetModuleInstance(), lpCreateParam);
                                    GetModuleInstance(),
                                    lpCreateParam);
            self.0
        }
    }

	// //create dialog controls
	// pub fn Create2(lpstrWndClass:LPCTSTR,hWndParent:HWND,rect:&RECT) -> HWND {

	// }

    pub fn DestroyWindow(&mut self) -> BOOL {
        self.assert_window();
        unsafe{
            if um::winuser::DestroyWindow(self.0) == winapi::shared::minwindef::TRUE {
                self.Detach();
                return winapi::shared::minwindef::TRUE;
            }
            winapi::shared::minwindef::FALSE
        }
    }

    pub fn GetStyle(&self) -> DWORD {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowLongW(self.0, GWL_STYLE) as DWORD
        }
    }

    pub fn GetExStyle(&self) -> DWORD {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowLongW(self.0, GWL_EXSTYLE) as DWORD
        }
    }

    pub fn GetWindowLong(&self, nIndex: c_int) -> LONG {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowLongW(self.0, nIndex)
        }
    }

    pub fn GetWindowLongPtr(&self, nIndex: c_int) -> LONG_PTR {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowLongPtrW(self.0, nIndex) as LONG_PTR
        }
    }

    pub fn SetWindowLong(&self, nIndex: c_int, dwNewLong: LONG) -> LONG {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowLongW(self.0, nIndex, dwNewLong)
        }
    }

    pub fn SetWindowLongPtr(&self, nIndex: c_int, dwNewLong: LONG_PTR) -> LONG_PTR {
        self.assert_window();
        unsafe {
            #[cfg(target_pointer_width = "32")]
                {
                    um::winuser::SetWindowLongPtrW(self.0, nIndex, dwNewLong as i32) as isize
                }
            #[cfg(target_pointer_width = "64")]
                {
                    um::winuser::SetWindowLongPtrW(self.0, nIndex, dwNewLong)
                }
        }
    }

    pub fn GetWindowWord(&self, nIndex: c_int) -> WORD {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowWord(self.0, nIndex)
        }
    }

    pub fn SetWindowWord(&self, nIndex: c_int, wNewWord: WORD) -> WORD {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowWord(self.0, nIndex, wNewWord)
        }
    }

    pub fn SendMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, message, wParam, lParam)
        }
    }

    pub fn PostMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::PostMessageW(self.0, message, wParam, lParam) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SendNotifyMessage(&self, message: UINT, wParam: WPARAM, lParam: LPARAM) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SendNotifyMessageW(self.0, message, wParam, lParam) == winapi::shared::minwindef::TRUE
        }
    }

	//static function
	// pub fn SendMessage (hWnd:HWND,message:UINT,wParam:WPARAM,lParam:LPARAM) -> LRESULT {
	// 	//ATLASSERT(::IsWindow(hWnd));
	// 	assert!(um::winuser::IsWindow(hWnd) == TRUE);
	// 	um::winuser::SendMessage(hWnd, message, wParam, lParam)
	// }

	pub fn SetWindowText (&self, lpszString: &str) -> bool {
		self.assert_window();
        let s = lpszString.to_c_u16();
		unsafe{um::winuser::SetWindowTextW(self.0, s.as_ptr())  == winapi::shared::minwindef::TRUE}
	}

	pub fn GetWindowText (&self) -> String {
		self.assert_window();
        self.get_text(self.0)
	}

    #[inline]
    fn get_text(&self,h: HWND) -> String {
        let nLength = unsafe{um::winuser::GetWindowTextLengthW(h) + 1};
        if nLength < 128 {
            let mut pszText = [0u16; 128];
            let nRead = unsafe{um::winuser::GetWindowTextW(h, pszText.as_mut_ptr(), nLength)};
            debug_assert!(nRead == nLength - 1);
            String::from_utf16_lossy(&pszText[..nRead as usize].as_ref())
        }else{
            let mut pszText: Vec<u16> = Vec::with_capacity(nLength as usize);
            unsafe{pszText.set_len(nLength as usize)};
            let nRead = unsafe{um::winuser::GetWindowTextW(h, pszText.as_mut_ptr(), nLength)};
            //pszText[nRead as usize - 1] = 0;
            debug_assert!(nRead == nLength - 1);
            String::from_utf16_lossy(&pszText[..nRead as usize].as_ref())
        }
    }

	// c_int GetWindowText( CSimpleString& strText) const

    pub fn GetWindowTextLength(&self) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowTextLengthW(self.0)
        }
    }

	// MAKELPARAM is a macro in user32.h
	// #define MAKELPARAM(l, h)      (LPARAM)MAKELONG(l, h)
	// MAKELONG is a macro in common.h:
	// #define MAKELONG(low, high)   ((DWORD)(((WORD)(low)) | (((DWORD)((WORD)(high))) << 16)))

    pub fn SetFont(&self, hFont: HFONT, bRedraw: BOOL) {
        self.assert_window();
		//um::winuser::SendMessage(self.0, WM_SETFONT, hFont as WPARAM, MAKELPARAM(bRedraw, 0));
        unsafe {
            um::winuser::SendMessageW(self.0, WM_SETFONT, hFont as WPARAM, (bRedraw & 0xFFFF) as LPARAM);
        }
    }

    pub fn GetFont(&self) -> HFONT {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_GETFONT, 0, 0)  as HFONT
        }
    }

    pub fn GetMenu(&self) -> HMENU {
        self.assert_window();
        unsafe {
            um::winuser::GetMenu(self.0) as HMENU
        }
    }

    pub fn SetMenu(&self, hMenu: HMENU) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetMenu(self.0, hMenu) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn DrawMenuBar(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::DrawMenuBar(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetSystemMenu(&self, bRevert: BOOL) -> HMENU {
        self.assert_window();
        unsafe {
            um::winuser::GetSystemMenu(self.0, bRevert) as HMENU
        }
    }

    pub fn HiliteMenuItem(&self, hMenu: HMENU, uItemHilite: UINT, uHilite: UINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::HiliteMenuItem(self.0, hMenu, uItemHilite, uHilite) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn IsIconic(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsIconic(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn IsZoomed(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsZoomed(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn MoveWindow(&self,x: c_int,y: c_int,nWidth: c_int,nHeight: c_int,bRepaint: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::MoveWindow(self.0, x, y, nWidth, nHeight, bRepaint) == winapi::shared::minwindef::TRUE
        }
    }

	//pub fn MoveWindow2 (&self,lpRect:LPCRECT,bRepaint:BOOL) -> bool {
    pub fn MoveWindow2(&self, lpRect: &RECT, bRepaint: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::MoveWindow(self.0,
                               lpRect.left,
                               lpRect.top,
                               lpRect.right - lpRect.left,
                               lpRect.bottom - lpRect.top,
                               bRepaint) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetWindowPos(&self,
                        hWndInsertAfter: HWND,
                        x: c_int,
                        y: c_int,
                        cx: c_int,
                        cy: c_int,
                        nFlags: UINT)
                        -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowPos(self.0, hWndInsertAfter, x, y, cx, cy, nFlags) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetWindowPos2(&self, hWndInsertAfter: HWND, lpRect: &RECT, nFlags: UINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowPos(self.0,
                                 hWndInsertAfter,
                                 lpRect.left,
                                 lpRect.top,
                                 lpRect.right - lpRect.left,
                                 lpRect.bottom - lpRect.top,
                                 nFlags) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ArrangeIconicWindows(&self) -> UINT {
        self.assert_window();
        unsafe {
            um::winuser::ArrangeIconicWindows(self.0)
        }
    }

    pub fn BringWindowToTop(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::BringWindowToTop(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetWindowRect(&self, lpRect: LPRECT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowRect(self.0, lpRect) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetClientRect(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p = lpRect as LPRECT;
        unsafe {
            um::winuser::GetClientRect(self.0, p) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetWindowPlacement(&self, lpwndpl: &mut WINDOWPLACEMENT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowPlacement(self.0, lpwndpl) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetWindowPlacement(&self, lpwndpl: &WINDOWPLACEMENT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowPlacement(self.0, lpwndpl) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ClientToScreen(&self, lpPoint: LPPOINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ClientToScreen(self.0, lpPoint) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ClientToScreen2(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p1 = lpRect as LPRECT;
        let p2 = p1 as LPPOINT;
        if unsafe {
            um::winuser::ClientToScreen(self.0, p2)
        } == winapi::shared::minwindef::FALSE {
            return false;
        }
        unsafe {
            um::winuser::ClientToScreen(self.0, p2.offset(1)) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ScreenToClient(&self, lpPoint: LPPOINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ScreenToClient(self.0, lpPoint) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ScreenToClient2(&self, lpRect: &mut RECT) -> bool {
        self.assert_window();
        let p1 = lpRect as LPRECT;
        let p2 = p1 as LPPOINT;

        if unsafe {
            um::winuser::ScreenToClient(self.0, p2)
        } == winapi::shared::minwindef::FALSE {
            return false;
        }
		//um::winuser::ScreenToClient(self.0, ((LPPOINT)lpRect)+1) == winapi::shared::minwindef::TRUE
        unsafe {
            um::winuser::ScreenToClient(self.0, p2.offset(1)) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn MapWindowPoints(&self, hWndTo: HWND, lpPoint: LPPOINT, nCount: UINT) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::MapWindowPoints(self.0, hWndTo, lpPoint, nCount)
        }
    }

    pub fn MapWindowPoints2(&self, hWndTo: HWND, lpRect: LPRECT) -> c_int {
        self.assert_window();
		//um::winuser::MapWindowPoints(self.0, hWndTo, (LPPOINT)lpRect, 2)
        unsafe {
            um::winuser::MapWindowPoints(self.0, hWndTo, lpRect as LPPOINT, 2)
        }
    }

    pub fn BeginPaint(&self, lpPaint: LPPAINTSTRUCT) -> HDC {
        self.assert_window();
        unsafe {
            um::winuser::BeginPaint(self.0, lpPaint)
        }
    }

    pub fn EndPaint(&self, lpPaint: LPPAINTSTRUCT) {
        self.assert_window();
        unsafe {
            um::winuser::EndPaint(self.0, lpPaint);
        }
    }

    pub fn GetDC(&self) -> HDC {
        self.assert_window();
        unsafe {
            um::winuser::GetDC(self.0)
        }
    }

    pub fn GetWindowDC(&self) -> HDC {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowDC(self.0)
        }
    }

    pub fn ReleaseDC(&self, hDC: HDC) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::ReleaseDC(self.0, hDC)
        }
    }

    pub fn Print(&self, hDC: HDC, dwFlags: DWORD) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_PRINT, hDC as WPARAM, dwFlags as LPARAM);
        }
    }

    pub fn PrintClient(&self, hDC: HDC, dwFlags: DWORD) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_PRINTCLIENT, hDC as WPARAM, dwFlags as LPARAM);
        }
    }

    pub fn UpdateWindow(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::UpdateWindow(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetRedraw(&self, bRedraw: BOOL) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_SETREDRAW, bRedraw as WPARAM, NULL_LPARAM);
        }
    }

    pub fn GetUpdateRect(&self, lpRect: LPRECT, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::GetUpdateRect(self.0, lpRect, bErase) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetUpdateRgn(&self, hRgn: HRGN, bErase: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::GetUpdateRgn(self.0, hRgn, bErase)
        }
    }

    pub fn Invalidate(&self, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::InvalidateRect(self.0, 0 as LPRECT, bErase) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn Invalidate2(&self, lpRect: LPCRECT, bErase: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::InvalidateRect(self.0, lpRect, bErase) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ValidateRect(&self, lpRect: LPCRECT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ValidateRect(self.0, lpRect) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn InvalidateRgn(&self, hRgn: HRGN, bErase: BOOL) {
        self.assert_window();
        unsafe {
            um::winuser::InvalidateRgn(self.0, hRgn, bErase);
        }
    }

    pub fn ValidateRgn(&self, hRgn: HRGN) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ValidateRgn(self.0, hRgn) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ShowWindow(&self, nCmdShow: c_int) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ShowWindow(self.0, nCmdShow) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn IsWindowVisible(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsWindowVisible(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ShowOwnedPopups(&self, bShow: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ShowOwnedPopups(self.0, bShow) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetDCEx(&self, hRgnClip: HRGN, flags: DWORD) -> HDC {
        self.assert_window();
        unsafe {
            um::winuser::GetDCEx(self.0, hRgnClip, flags)
        }
    }

    pub fn LockWindowUpdate(&self, bLock: bool) -> bool {
        self.assert_window();
        if bLock {
            unsafe {
                um::winuser::LockWindowUpdate(self.0) == winapi::shared::minwindef::TRUE
            }
        } else {
            unsafe {
                um::winuser::LockWindowUpdate(NULL_HWND) == winapi::shared::minwindef::TRUE
            }
        }
    }

    pub fn RedrawWindow2(&self) -> bool {
        self.RedrawWindow(0 as LPCRECT,0 as HRGN,RDW_INVALIDATE | RDW_UPDATENOW | RDW_ERASE)
    }

    pub fn RedrawWindow(&self, lpRectUpdate: LPCRECT, hRgnUpdate: HRGN, flags: UINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::RedrawWindow(self.0, lpRectUpdate, hRgnUpdate, flags) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetTimer(&self, nIDEvent: UINT_PTR, nElapse: UINT) -> UINT_PTR {
        self.assert_window();
        unsafe {
            um::winuser::SetTimer(self.0, nIDEvent, nElapse, None)
        }
    }

    pub fn SetTimer2(&self, nIDEvent: UINT_PTR, nElapse: UINT, lpfnTimer: TIMERPROC) -> UINT_PTR {
        self.assert_window();
        unsafe {
            um::winuser::SetTimer(self.0, nIDEvent, nElapse, lpfnTimer)
        }
    }

    pub fn KillTimer(&self, nIDEvent: UINT_PTR) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::KillTimer(self.0, nIDEvent) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn IsWindowEnabled(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsWindowEnabled(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn EnableWindow(&self, bEnable: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::EnableWindow(self.0, bEnable) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetActiveWindow(&self) -> HWND {
        self.assert_window();
        unsafe {
            um::winuser::SetActiveWindow(self.0)
        }
    }

    pub fn SetCapture(&self) -> HWND {
        self.assert_window();
        unsafe {
            um::winuser::SetCapture(self.0)
        }
    }

    pub fn SetFocus(&self) -> HWND {
        self.assert_window();
        unsafe {
            um::winuser::SetFocus(self.0)
        }
    }



    pub fn CheckDlgButton(&self, nIDButton: c_int, nCheck: UINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::CheckDlgButton(self.0, nIDButton, nCheck) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn CheckRadioButton(&self,
                            nIDFirstButton: c_int,
                            nIDLastButton: c_int,
                            nIDCheckButton: c_int)
                            -> bool {
        self.assert_window();
        unsafe {
            um::winuser::CheckRadioButton(self.0, nIDFirstButton, nIDLastButton, nIDCheckButton) == winapi::shared::minwindef::TRUE
        }
    }

    /*
	pub fn DlgDirList (&self,lpPathSpec: &str,nIDListBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
		self.assert_window();
        let p  = lpPathSpec.to_c_u16();
        //TCHAR path[MAX_PATH];
        let mut path = [0u16, MAX_PATH];
		unsafe{um::winuser::DlgDirListW(self.0, p.as_ptr(), nIDListBox, nIDStaticPath, nFileType)}
	}

	pub fn DlgDirListComboBox (&self,lpPathSpec: &str,nIDComboBox:c_int,nIDStaticPath:c_int,nFileType:UINT) -> c_int {
		self.assert_window();
        let p = lpPathSpec.to_c_u16();
		unsafe{um::winuser::DlgDirListComboBoxW(self.0, p.as_ptr(), nIDComboBox, nIDStaticPath, nFileType)}
	}

	pub fn DlgDirSelect (&self,lpString: &str,nCount:c_int,nIDListBox:c_int) -> bool {
		self.assert_window();
        let s = lpString.to_c_u16();
		unsafe{um::winuser::DlgDirSelectExW(self.0, s.as_ptr(), nCount, nIDListBox) == winapi::shared::minwindef::TRUE}
	}

	pub fn DlgDirSelectComboBox (&self,lpString:&str,nCount:c_int,nIDComboBox:c_int) -> bool {
		self.assert_window();
        let s = lpString.to_c_u16();
		unsafe{um::winuser::DlgDirSelectComboBoxExW(self.0, s.as_ptr(), nCount, nIDComboBox) == winapi::shared::minwindef::TRUE}
	}
    */
    pub fn GetDlgItemInt(&self, nID: c_int) -> UINT {
        self.assert_window();
        unsafe {
            um::winuser::GetDlgItemInt(self.0, nID, 0 as *mut BOOL, winapi::shared::minwindef::TRUE)
        }
    }

    pub fn GetDlgItemInt2(&self, nID: c_int, lpTrans: &mut BOOL, bSigned: BOOL) -> UINT {
        self.assert_window();
        unsafe {
            um::winuser::GetDlgItemInt(self.0, nID, lpTrans as *mut BOOL, bSigned)
        }
    }

	//pub fn GetDlgItemText (&self,nID:c_int,lpStr:LPTSTR,nMaxCount:c_int) -> UINT {
    pub fn GetDlgItemText (&self, nID: WORD) -> String {
		self.assert_window();
        let hItem = self.GetDlgItem(nID);
        if hItem != NULL_HWND {
            self.get_text(hItem)
        }
        else
        {
            String::new()
        }
	}

	//UINT GetDlgItemText(c_int nID,CSimpleString& strText) const

	//OLE
	//BOOL GetDlgItemText(c_int nID,_Deref_post_opt_z_ BSTR& bstrText)

    pub fn IsDlgButtonChecked(&self, nIDButton: c_int) -> UINT {
        self.assert_window();
        unsafe {
            um::winuser::IsDlgButtonChecked(self.0, nIDButton)
        }
    }

    pub fn SendDlgItemMessage(&self,
                              nID: c_int,
                              message: UINT,
                              wParam: WPARAM,
                              lParam: LPARAM)
                              -> LRESULT {
        self.assert_window();
        unsafe {
            um::winuser::SendDlgItemMessageW(self.0, nID, message, wParam, lParam)
        }
    }

    pub fn SetDlgItemInt(&self, nID: c_int, nValue: UINT, bSigned: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetDlgItemInt(self.0, nID, nValue, bSigned) == winapi::shared::minwindef::TRUE
        }
    }

	pub fn SetDlgItemText (&self,nID:c_int,lpszString: &str) -> bool {
		self.assert_window();
        let s = lpszString.to_c_u16();
		unsafe{um::winuser::SetDlgItemTextW(self.0, nID, s.as_ptr()) == winapi::shared::minwindef::TRUE}
	}

// #ipub fndef _ATL_NO_HOSTING
//ATLPREFAST_SUPPRESS(6387)
	//HRESULT GetDlgControl(c_int nID,REFIID iid,void** ppCtrl)

// ATLPREFAST_SUPPRESS(6387)
	//HRESULT GetDlgHost(c_int nID,REFIID iid,void** ppHost)

// #endif

    pub fn GetScrollPos(&self, nBar: c_int) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::GetScrollPos(self.0, nBar)
        }
    }

    pub fn GetScrollRange(&self, nBar: c_int, lpMinPos: LPINT, lpMaxPos: LPINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::GetScrollRange(self.0, nBar, lpMinPos, lpMaxPos) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ScrollWindow(&self,
                        xAmount: c_int,
                        yAmount: c_int,
                        lpRect: LPCRECT,
                        lpClipRect: LPCRECT)
                        -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ScrollWindow(self.0, xAmount, yAmount, lpRect, lpClipRect) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ScrollWindowEx(&self,
                          dx: c_int,
                          dy: c_int,
                          lpRectScroll: LPCRECT,
                          lpRectClip: LPCRECT,
                          hRgnUpdate: HRGN,
                          lpRectUpdate: LPRECT,
                          uFlags: UINT)
                          -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::ScrollWindowEx(self.0,
                                   dx,
                                   dy,
                                   lpRectScroll,
                                   lpRectClip,
                                   hRgnUpdate,
                                   lpRectUpdate,
                                   uFlags)
        }
    }

    pub fn ScrollWindowExDefault(&self, dx: c_int, dy: c_int, uFlags: UINT) -> c_int {
        self.ScrollWindowEx(dx,dy,0 as LPCRECT,0 as LPCRECT,0 as HRGN,0 as LPRECT,uFlags)
    }

    pub fn SetScrollPos(&self, nBar: c_int, nPos: c_int, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::SetScrollPos(self.0, nBar, nPos, bRedraw)
        }
    }

    pub fn SetScrollRange(&self,
                          nBar: c_int,
                          nMinPos: c_int,
                          nMaxPos: c_int,
                          bRedraw: BOOL)
                          -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetScrollRange(self.0, nBar, nMinPos, nMaxPos, bRedraw) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ShowScrollBar(&self, nBar: c_int, bShow: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ShowScrollBar(self.0, nBar, bShow) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn EnableScrollBar(&self, uSBFlags: UINT, uArrowFlags: UINT) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::EnableScrollBar(self.0, uSBFlags, uArrowFlags) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn IsChild(&self, hWnd: HWND) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsChild(self.0, hWnd) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetDlgCtrlID(&self) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::GetDlgCtrlID(self.0)
        }
    }

    pub fn SetDlgCtrlID(&self, nID: c_int) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowLongW(self.0, GWL_ID, nID)
        }
    }

    pub fn FlashWindow(&self, bInvert: BOOL) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::FlashWindow(self.0, bInvert) == winapi::shared::minwindef::TRUE
        }
    }

	pub fn MessageBox(&self,lpszText:&str ,lpszCaption:&str,nType:UINT) -> c_int {
		self.assert_window();
        let t = lpszText.to_c_u16();
        let c = lpszCaption.to_c_u16();
		unsafe{um::winuser::MessageBoxW(self.0, t.as_ptr(), c.as_ptr(), nType)}
	}

    pub fn ChangeClipboardChain(&self, hWndNewNext: HWND) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ChangeClipboardChain(self.0, hWndNewNext) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn SetClipboardViewer(&self) -> HWND {
        self.assert_window();
        unsafe {
            um::winuser::SetClipboardViewer(self.0)
        }
    }

    pub fn OpenClipboard(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::OpenClipboard(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn CloseClipboard() -> bool {
        unsafe {
            um::winuser::CloseClipboard() == winapi::shared::minwindef::TRUE
        }
    }

    pub fn CreateCaret(&self, hBitmap: HBITMAP) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::CreateCaret(self.0, hBitmap, 0, 0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn CreateSolidCaret(&self, nWidth: c_int, nHeight: c_int) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::CreateCaret(self.0, 0 as HBITMAP, nWidth, nHeight) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn CreateGrayCaret(&self, nWidth: c_int, nHeight: c_int) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::CreateCaret(self.0, 1 as HBITMAP, nWidth, nHeight) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn HideCaret(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::HideCaret(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn ShowCaret(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ShowCaret(self.0) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn DragAcceptFiles(&self, bAccept: BOOL) {
        self.assert_window();
        unsafe {
            winapi::um::shellapi::DragAcceptFiles(self.0, bAccept);
        }
    }

    pub fn SetIcon(&self, hIcon: HICON, bBigIcon: BOOL) -> HICON {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_SETICON, bBigIcon as WPARAM, hIcon as LPARAM) as HICON
        }
    }

    pub fn GetIcon(&self, bBigIcon: BOOL) -> HICON {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_GETICON, bBigIcon as WPARAM, NULL_LPARAM) as HICON
        }
    }

	pub fn WinHelp (&self,lpszHelp:&str,nCmd:UINT,dwData:DWORD) -> bool {
		self.assert_window();
        let h = lpszHelp.to_c_u16();
		unsafe{um::winuser::WinHelpW(self.0, h.as_ptr(), nCmd, dwData as ULONG_PTR) == winapi::shared::minwindef::TRUE}
	}

    pub fn SetWindowContextHelpId(&self, dwContextHelpId: DWORD) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowContextHelpId(self.0, dwContextHelpId) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn GetWindowContextHelpId(&self) -> DWORD {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowContextHelpId(self.0)
        }
    }

    pub fn SetHotKey(&self, wVirtualKeyCode: WORD, wModifiers: WORD) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_SETHOTKEY, MAKEWORD(wVirtualKeyCode as u8, wModifiers as u8) as WPARAM, 0 ) as c_int
        }
    }

    pub fn GetHotKey(&self) -> DWORD {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_GETHOTKEY, 0, 0) as DWORD
        }
    }

    pub fn GetScrollInfo(&self, nBar: c_int, lpScrollInfo: LPSCROLLINFO) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::GetScrollInfo(self.0, nBar, lpScrollInfo) == winapi::shared::minwindef::TRUE
        }
    }
    pub fn SetScrollInfo(&self, nBar: c_int, lpScrollInfo: LPSCROLLINFO, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::SetScrollInfo(self.0, nBar, lpScrollInfo, bRedraw)
        }
    }
    pub fn IsDialogMessage(&self, lpMsg: LPMSG) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsDialogMessageW(self.0, lpMsg) == winapi::shared::minwindef::TRUE
        }
    }

    pub fn NextDlgCtrl(&self) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_NEXTDLGCTL, 0, 0);
        }
    }
    pub fn PrevDlgCtrl(&self) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_NEXTDLGCTL, 1, 0);
        }
    }
    pub fn GotoDlgCtrl(&self, hWndCtrl: HWND) {
        self.assert_window();
        unsafe {
            um::winuser::SendMessageW(self.0, WM_NEXTDLGCTL, hWndCtrl as WPARAM, 1);
        }
    }

    pub fn ResizeClient(&self, nWidth: c_int, nHeight: c_int, bRedraw: BOOL) -> bool {
        self.assert_window();

        let mut rcWnd = RECT { left: 0, right: 0, top: 0, bottom: 0 };
		//if(!GetClientRect(&rcWnd))
		//	return winapi::shared::minwindef::FALSE;
        if self.GetClientRect(&mut rcWnd) == false {
            return false;
        }

        if nWidth != -1 {
            rcWnd.right = nWidth;
        }

        if nHeight != -1 {
            rcWnd.bottom = nHeight;
        }

		//let b1 = !(self.GetStyle() & WS_CHILD) && (self.GetMenu() != 0 as HMENU);
        let b1 = ((self.GetStyle() & WS_CHILD) == 0) && (self.GetMenu() != (0 as HMENU));
        if unsafe {
            um::winuser::AdjustWindowRectEx(&mut rcWnd, self.GetStyle(), b1 as BOOL, self.GetExStyle())
        } == winapi::shared::minwindef::TRUE {
            return false;
        }

        let mut uFlags: UINT = SWP_NOZORDER | SWP_NOMOVE | SWP_NOACTIVATE;
        if bRedraw == winapi::shared::minwindef::FALSE {
            uFlags |= SWP_NOREDRAW;
        }

        self.SetWindowPos(NULL_HWND, 0, 0, rcWnd.right - rcWnd.left, rcWnd.bottom - rcWnd.top, uFlags)
    }

    pub fn GetWindowRgn(&self, hRgn: HRGN) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowRgn(self.0, hRgn)
        }
    }
    pub fn SetWindowRgn(&self, hRgn: HRGN, bRedraw: BOOL) -> c_int {
        self.assert_window();
        unsafe {
            um::winuser::SetWindowRgn(self.0, hRgn, bRedraw)
        }
    }

    pub fn DeferWindowPos(&self,
                          hWinPosInfo: HDWP,
                          hWndInsertAfter: HWND,
                          x: c_int,
                          y: c_int,
                          cx: c_int,
                          cy: c_int,
                          uFlags: UINT)
                          -> HDWP {
        self.assert_window();
        unsafe {
            um::winuser::DeferWindowPos(hWinPosInfo, self.0, hWndInsertAfter, x, y, cx, cy, uFlags)
        }
    }

    pub fn GetWindowThreadID(&self) -> DWORD {
        self.assert_window();
        unsafe {
            um::winuser::GetWindowThreadProcessId(self.0, 0 as LPDWORD)
        }
    }

    pub fn GetWindowProcessID(&self) -> DWORD {
        self.assert_window();
        let mut dwProcessID: DWORD = 0;
        unsafe {
            um::winuser::GetWindowThreadProcessId(self.0, &mut dwProcessID);
        }
        dwProcessID
    }

    pub fn IsWindow(&self) -> bool {
        unsafe {
            um::winuser::IsWindow(self.0) == winapi::shared::minwindef::TRUE
        }
    }
    pub fn IsWindowUnicode(&self) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::IsWindowUnicode(self.0) == winapi::shared::minwindef::TRUE
        }
    }
	// pub fn IsParentDialog (&self) -> bool {
	// 	self.assert_window();
	// 	TCHAR szBuf[8];
	// 	if (GetClassName(GetParent(), szBuf, sizeof(szBuf)/sizeof(szBuf[0])) == 0)
	// 		return winapi::shared::minwindef::FALSE;
	// 	return lstrcmp(szBuf, _T("#32770")) == 0;
	// }
    pub fn ShowWindowAsync(&self, nCmdShow: c_int) -> bool {
        self.assert_window();
        unsafe {
            um::winuser::ShowWindowAsync(self.0, nCmdShow) == winapi::shared::minwindef::TRUE
        }
    }

	pub fn SendMessageToDescendants (&self,message:UINT,wParam:WPARAM,lParam:LPARAM,bDeep:BOOL)  {
        unsafe{
            let mut hWndChild = um::winuser::GetTopWindow(self.0);
            while hWndChild != NULL_HWND {
                um::winuser::SendMessageW(hWndChild, message, wParam, lParam);

                if bDeep > 0 && um::winuser::GetTopWindow(hWndChild) != NULL_HWND {

                    //CWindow wnd(hWndChild);
                    let wnd = CWindow::new(hWndChild);
                    wnd.SendMessageToDescendants(message, wParam, lParam, bDeep);
                }
                hWndChild = um::winuser::GetWindow(hWndChild, GW_HWNDNEXT);
            }
        }
	}

    pub fn CenterWindow(&self, hCenter: HWND) -> BOOL {
        self.assert_window();
        let mut hWndCenter = hCenter;
        unsafe {
            let dwStyle = self.GetStyle();
            if hWndCenter == NULL_HWND {
                if dwStyle & WS_CHILD != 0 {
                    hWndCenter = self.GetParent2().GetHwnd();//::GetParent(self.0);
                } else {
                    hWndCenter = um::winuser::GetWindow(self.0, GW_OWNER);
                }
            }

            let mut rcDlg: RECT = mem::zeroed();//Default::default();
            um::winuser::GetWindowRect(self.0, &mut rcDlg);
            let mut rcArea: RECT = mem::zeroed();//Default::default();
            let mut rcCenter: RECT = mem::zeroed();//Default::default();
            let hWndParent: HWND;
            if dwStyle & WS_CHILD == 0 {

                if hWndCenter != NULL_HWND {
                    let dwStyleCenter = um::winuser::GetWindowLongW(hWndCenter, GWL_STYLE) as DWORD;
                    if (dwStyleCenter & WS_VISIBLE) == 0 || (dwStyleCenter & WS_MINIMIZE) != 0 {
                        hWndCenter = NULL_HWND;
                    }
                }

				//>=win2k
                let mut hMonitor: HMONITOR = 0 as HMONITOR;
                if hWndCenter != NULL_HWND {
                    hMonitor = um::winuser::MonitorFromWindow(hWndCenter, MONITOR_DEFAULTTONEAREST);
                } else {
                    hMonitor = um::winuser::MonitorFromWindow(self.0, MONITOR_DEFAULTTONEAREST);
                }
				//ATLENSURE_RETURN_VAL(hMonitor != NULL, winapi::shared::minwindef::FALSE);
                let mut minfo: MONITORINFO = mem::zeroed();//Default::default();
                minfo.cbSize = std::mem::size_of::<MONITORINFO>() as DWORD;
                //let bResult: BOOL = um::winuser::GetMonitorInfoW(hMonitor, &mut minfo);
                um::winuser::GetMonitorInfoW(hMonitor, &mut minfo);
				//ATLENSURE_RETURN_VAL(bResult, winapi::shared::minwindef::FALSE);

                rcArea = minfo.rcWork;

                if hWndCenter == NULL_HWND {
                    rcCenter = rcArea;
                } else {
                    um::winuser::GetWindowRect(hWndCenter, &mut rcCenter);
                }
            } else {

                hWndParent = um::winuser::GetParent(self.0);
				//ATLASSERT(::IsWindow(hWndParent));

                um::winuser::GetClientRect(hWndParent, &mut rcArea);
				//ATLASSERT(::IsWindow(hWndCenter));
                um::winuser::GetClientRect(hWndCenter, &mut rcCenter);
                um::winuser::MapWindowPoints(hWndCenter,
                                        hWndParent,
                                        &mut rcCenter as *mut RECT as *mut POINT,
                                        2);
            }

            let DlgWidth: c_int = rcDlg.right - rcDlg.left;
            let DlgHeight: c_int = rcDlg.bottom - rcDlg.top;


            let mut xLeft: c_int = (rcCenter.left + rcCenter.right) / 2 - DlgWidth / 2;
            let mut yTop: c_int = (rcCenter.top + rcCenter.bottom) / 2 - DlgHeight / 2;


            if xLeft + DlgWidth > rcArea.right {
                xLeft = rcArea.right - DlgWidth;
            }

            if xLeft < rcArea.left {
                xLeft = rcArea.left;
            }

            if yTop + DlgHeight > rcArea.bottom {
                yTop = rcArea.bottom - DlgHeight;
            }

            if yTop < rcArea.top {
                yTop = rcArea.top;
            }

            um::winuser::SetWindowPos(self.0,
                                 NULL_HWND,
                                 xLeft,
                                 yTop,
                                 -1,
                                 -1,
                                 SWP_NOSIZE | SWP_NOZORDER | SWP_NOACTIVATE)
        }
    }

    pub fn ModifyStyle(&self, dwRemove: DWORD, dwAdd: DWORD, nFlags: UINT) -> bool {
        self.assert_window();

        let dwStyle: DWORD = unsafe {
            um::winuser::GetWindowLongW(self.0, GWL_STYLE) as DWORD
        };
        let dwNewStyle = (dwStyle & !dwRemove) | dwAdd;
        if dwStyle == dwNewStyle {
            return false;
        }

        unsafe {
            um::winuser::SetWindowLongW(self.0, GWL_STYLE, dwNewStyle as LONG);
        }
        if nFlags != 0 {
            unsafe {
                um::winuser::SetWindowPos(self.0,
                                     NULL_HWND,
                                     0,
                                     0,
                                     0,
                                     0,
                                     SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE |
                                     nFlags)
            };
        }

        true
    }

    pub fn ModifyStyleEx(&self, dwRemove: DWORD, dwAdd: DWORD, nFlags: UINT) -> bool {
        self.assert_window();

        let dwStyle: DWORD = unsafe {
            um::winuser::GetWindowLongW(self.0, GWL_EXSTYLE) as DWORD
        };
        let dwNewStyle: DWORD = (dwStyle & !dwRemove) | dwAdd;
        if dwStyle == dwNewStyle {
            return false;
        }

        unsafe {
            um::winuser::SetWindowLongW(self.0, GWL_EXSTYLE, dwNewStyle as LONG);
        }
        if nFlags != 0 {
            unsafe {
                um::winuser::SetWindowPos(self.0,
                                     NULL_HWND,
                                     0,
                                     0,
                                     0,
                                     0,
                                     SWP_NOSIZE | SWP_NOMOVE | SWP_NOZORDER | SWP_NOACTIVATE |
                                     nFlags)
            };
        }
        true
    }

	// BOOL GetWindowText( _Deref_post_opt_z_ BSTR* pbstrText)
	// BOOL GetWindowText( BSTR& bstrText)
}


const MONITOR_DEFAULTTONULL: DWORD = 0x00000000;
const MONITOR_DEFAULTTOPRIMARY: DWORD = 0x00000001;
const MONITOR_DEFAULTTONEAREST: DWORD = 0x00000002;