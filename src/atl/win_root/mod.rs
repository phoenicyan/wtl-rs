#![allow(non_snake_case)]
// CWindowImplRoot
use winapi::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use atl::consts::*;
use atl::cwindow::*;

pub fn ReflectNotifications(cwin: &CWindow,uMsg: UINT,wParam: WPARAM,lParam: LPARAM,bHandled: &mut BOOL)-> LRESULT {
    let mut hWndChild = NULL_HWND;
    unsafe {
        match uMsg {
            WM_COMMAND => {
                if lParam != NULL_LPARAM {
                    hWndChild = lParam as HWND;
                }
            }
            WM_NOTIFY => {
                hWndChild = (*(lParam as LPNMHDR)).hwndFrom;
            }
            WM_PARENTNOTIFY => {
                match LOWORD(wParam as DWORD) as DWORD {
                    WM_CREATE | WM_DESTROY => hWndChild = lParam as HWND,
                    _ => hWndChild =
                             cwin.GetDlgItem2(HIWORD(wParam as DWORD) as WORD).GetHwnd(),
                }
            }
            WM_DRAWITEM => {
                if wParam != 0 {
                    hWndChild = (*(lParam as LPDRAWITEMSTRUCT)).hwndItem;
                }
            }
            WM_MEASUREITEM => {
                if wParam != 0 {
                    let id = (*(lParam as LPMEASUREITEMSTRUCT)).CtlID;
                    hWndChild = cwin.GetDlgItem2(id as WORD).GetHwnd();
                }
            }
            WM_COMPAREITEM => {
                if wParam != 0 {
                    hWndChild = (*(lParam as LPCOMPAREITEMSTRUCT)).hwndItem;
                }
            }
            WM_DELETEITEM => {
                if wParam != 0 {
                    hWndChild = (*(lParam as LPDELETEITEMSTRUCT)).hwndItem;
                }
            }
            WM_VKEYTOITEM | WM_CHARTOITEM | WM_HSCROLL | WM_VSCROLL => hWndChild =
                                                                           lParam as HWND,
            WM_CTLCOLORBTN |
            WM_CTLCOLORDLG |
            WM_CTLCOLOREDIT |
            WM_CTLCOLORLISTBOX |
            WM_CTLCOLORMSGBOX |
            WM_CTLCOLORSCROLLBAR |
            WM_CTLCOLORSTATIC => hWndChild = lParam as HWND,
            _ => (),
        }

        if hWndChild == NULL_HWND {
            *bHandled = FALSE;
            return 1;
        }

        //ATLASSERT(::IsWindow(hWndChild));
        assert!(um::winuser::IsWindow(hWndChild) == TRUE);
        um::winuser::SendMessageW(hWndChild, OCM__BASE + uMsg, wParam, lParam)
    }
}


pub fn ForwardNotifications(cwin: &CWindow,uMsg: UINT,wParam: WPARAM,lParam: LPARAM,bHandled: &mut BOOL)-> LRESULT {
    let mut lResult: LRESULT = 0;
    match uMsg {
        WM_COMMAND |
        WM_NOTIFY |
        WM_PARENTNOTIFY |
        WM_DRAWITEM |
        WM_MEASUREITEM |
        WM_COMPAREITEM |
        WM_DELETEITEM |
        WM_VKEYTOITEM |
        WM_CHARTOITEM |
        WM_HSCROLL |
        WM_VSCROLL |
        WM_CTLCOLORBTN |
        WM_CTLCOLORDLG |
        WM_CTLCOLOREDIT |
        WM_CTLCOLORLISTBOX |
        WM_CTLCOLORMSGBOX |
        WM_CTLCOLORSCROLLBAR |
        WM_CTLCOLORSTATIC => {
            lResult = cwin.GetParent2().SendMessage(uMsg, wParam, lParam);
        }
        _ => *bHandled = FALSE,
    }
    lResult
}

    

//static function
pub fn DefaultReflectionHandler(hWnd: HWND,uMsg: UINT,wParam: WPARAM,lParam: LPARAM,lResult: &mut LRESULT)-> BOOL {
    match uMsg {
        OCM_COMMAND |
        OCM_NOTIFY |
        OCM_PARENTNOTIFY |
        OCM_DRAWITEM |
        OCM_MEASUREITEM |
        OCM_COMPAREITEM |
        OCM_DELETEITEM |
        OCM_VKEYTOITEM |
        OCM_CHARTOITEM |
        OCM_HSCROLL |
        OCM_VSCROLL |
        OCM_CTLCOLORBTN |
        OCM_CTLCOLORDLG |
        OCM_CTLCOLOREDIT |
        OCM_CTLCOLORLISTBOX |
        OCM_CTLCOLORMSGBOX |
        OCM_CTLCOLORSCROLLBAR |
        OCM_CTLCOLORSTATIC => {
            unsafe {
                *lResult = um::winuser::DefWindowProcW(hWnd, uMsg - OCM__BASE, wParam, lParam);
            }
            return TRUE;
        }
        _ => (),
    }
    FALSE
}