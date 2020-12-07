#![allow(non_snake_case,dead_code,unused_variables,unused_assignments)]
use std::ops::{Deref,DerefMut};
use std::{self, ptr};
use winapi::*;
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::fmt;

use atl::consts::*;
use atl::{thunk,Event,Handler,HandleKey};
use atl::cwindow::*;
use super::DlgMsg;

use wtl::ctrls::BtnMsg;

type DLGPROC2 = unsafe extern "system" fn(HWND, u32, u64, i64) -> i64;

pub struct Dialog<T>{
    inner: CWindow, // basic operations for objects that have HWND
    thk: &'static mut thunk::Thunk, // thunk that convert static function call to
    idd: WORD, // resource id of the dlg
    state: DWORD, // destroy or not
    modal: bool, // is modal dialog

    // T can't be mut borrowed more than once,so dialog save raw pointer,
    // when all gui run in one thread this is safe
    root:*mut T, //raw pointer to the Root Dialogs
    //messages
    bin_search_cnt:u32,         //used for combine search,search step cnt for bin search
    pub handlers: Vec<Handler<T>>,
}

impl<T> fmt::Display for Dialog<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hwnd:0x{:x}", self.inner.GetHwnd() as usize)
    }
}

fn MAKEINTRESOURCEW(id: WORD) -> LPCWSTR {
    id as usize as LPCWSTR
}

//This trait allow you to call functions in CWindow
//compiler looks for a function in Dialog first,if nothing found it will add a "*" to check if the function can be found
//if functions in both Dialog and CWindow,the functions in Dialogs is choose by the compile
//so Dialog.cwin() is useful when you want to call functions in CWindow
impl<T> Deref for Dialog<T> {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl<T> DerefMut for Dialog<T> {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

//frequently used
impl<T> Dialog<T> {
    fn InitThunk(&mut self, h: HWND) -> DLGPROC {
        let pself = self as *mut _ as *mut c_void;
        self.thk.init(Self::DialogProc as DWORD_PTR, pself);
        self.inner.Attach(h);
        let p = self.thk.GetCodeAddress();
        unsafe {
            std::mem::transmute(p)
        }
    }

    //user can pass a dlg_proc to override the default DLGPROC of CDialogImpl,and take over every msg your self
    pub fn new(idd: WORD) -> Dialog<T> {
        Dialog {
            inner: CWindow::new(NULL_HWND),
            thk: thunk::get_thunk(),
            idd: idd,
            state: 0,
            modal: false,
            root: 0 as *mut T,
            bin_search_cnt:0,
            handlers:vec![Handler::new(0xFFFF,0xFFFF, 0xFFFF , 0xFFFF, |_,_|{})],   //put a sentinel in the vec
        }
    }

    pub fn cwin(&self)->&CWindow {
        &self.inner
    }
}

//CDialogImplBaseT
impl<T> Dialog<T> {
    unsafe extern "system" fn StartDialogProc(hWnd: HWND,
                                              uMsg: UINT,
                                              wParam: WPARAM,
                                              lParam: LPARAM)
                                              -> INT_PTR {
        let p_this = thunk::get_this();
        //println!("4. get this:{:p}", p_this);
        let pself = p_this as *mut Self;
        //println!("5. start dialog proc,addr:0x{:x},DialogProc:0x{:x}",Self::StartDialogProc as usize,Self::DialogProc as usize);
        //println!("6. proc_msg before init thunk:0x{:x}", Self::DialogProc as usize);
        let dlg_proc_thunk = Self::InitThunk(&mut *pself, hWnd);

        let pthunk = dlg_proc_thunk.unwrap();

        //println!("7. start proc,thunk addr:0x{:x}", proc_msg as usize);
        // handler must be sorted here:before any message been processed
        Self::sort_handlers(&mut *pself);


        //DWLP_DLGPROC = sizeof(LRESULT) + DWLP_MSGRESULT
        #[cfg(target_pointer_width = "32")]
            {
                um::winuser::SetWindowLongPtrW(hWnd,
                                               (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int,
                                               pthunk as LONG);
            }
        #[cfg(target_pointer_width = "64")]
            {
                um::winuser::SetWindowLongPtrW(hWnd,
                                               (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int,
                                               pthunk as LONG_PTR);
            }

        //it is actually the entry of the thunk
        pthunk(hWnd, uMsg, wParam, lParam)
    }

    //if bHandled return TRUE
    unsafe extern "system" fn DialogProc(hWnd: HWND,
                                         uMsg: UINT,
                                         wParam: WPARAM,
                                         lParam: LPARAM)
                                         -> INT_PTR {
        
        let mut_self = &mut *(hWnd as *mut Self);

        let mut lRes: LRESULT = 0;
        let h = mut_self.inner.GetHwnd();
        let mut bRet = Self::ProcessWindowMessage(mut_self,h,uMsg,wParam,lParam,&mut lRes,0);//unsafe{};

        if bRet == winapi::shared::minwindef::TRUE {
            match uMsg {
                WM_COMPAREITEM |
                WM_VKEYTOITEM |
                WM_CHARTOITEM |
                WM_INITDIALOG |
                WM_QUERYDRAGICON |
                WM_CTLCOLORMSGBOX |
                WM_CTLCOLOREDIT |
                WM_CTLCOLORLISTBOX |
                WM_CTLCOLORBTN |
                WM_CTLCOLORDLG |
                WM_CTLCOLORSCROLLBAR |
                WM_CTLCOLORSTATIC => {
                    if lRes > 0 {
                        bRet = winapi::shared::minwindef::TRUE;
                    }
                }
                // return in DWL_MSGRESULT
                //Make sure the window was not destroyed before setting attributes.
                _ => {
                    #[cfg(target_pointer_width = "32")]
                    if mut_self.state & WINSTATE_DESTROYED == 0
                    {
                        um::winuser::SetWindowLongPtrW(mut_self.inner.GetHwnd(),DWLP_MSGRESULT as c_int,lRes as LONG);
                    }
                    #[cfg(target_pointer_width = "64")]
                    if mut_self.state & WINSTATE_DESTROYED == 0
                    {
                        um::winuser::SetWindowLongPtrW(mut_self.inner.GetHwnd(),DWLP_MSGRESULT as c_int,lRes);
                    }
                }
            }
        } else if uMsg == WM_NCDESTROY {
            mut_self.state |= WINSTATE_DESTROYED;
        }

        if mut_self.state & WINSTATE_DESTROYED != 0 {
            let hWndThis = mut_self.inner.Detach();
            mut_self.state &= !WINSTATE_DESTROYED;
            // clean up after dialog is destroyed
            //mut_self->OnFinalMessage(hWndThis);
        }
        bRet as INT_PTR
        //0
    }
}

impl<T> Dialog<T> {
    fn sort_handlers(&mut self) {
        //sort handlers for search algorithm
        self.handlers.sort_by(|f1,f2|{
            f1.cmp(&f2)
        });

        //calculate how many steps can bin search do
        self.bin_search_cnt = (self.handlers.len() as f32).log2() as u32;

        // bin_search_cnt - 4 will be a very big u32 value(equals max_u32 - bin_search_cnt) when bin_search_cnt < 4
        if self.bin_search_cnt > 4 {
            self.bin_search_cnt -= 4;
        }else{
            self.bin_search_cnt = 0;
        }

        // priority only used in sort algorithm
        for h in &mut self.handlers {
            h.priority = 0;
        }

        // for h in &self.handlers{
        //     println!("{}", h);
        // }
    }

    //messages
    pub fn ProcessWindowMessage(&mut self,hWnd:HWND,uMsg:UINT,wParam:WPARAM,lParam:LPARAM,lResult:&mut LRESULT,dwMsgMapID:DWORD ) -> BOOL {
        let mut e = Event::new(hWnd,uMsg,wParam,lParam,lResult);
        
        // this should be called before TranslateMessage and DispathMessage
        // let mut i = 0;
        // // check if "PreTranslateMessage" exist
        // while self.handlers[i].key() == 0 {
        //     // call PreTranslateMessage
        //     unsafe{
        //         (self.handlers[i].call)(e,&mut *self.root);
        //     }

        //     // stop call PreTranslateMessage that has low priority,or the real message handlers
        //     if *lResult > 0{
        //         return winapi::shared::minwindef::TRUE;
        //     }

        //     i+=1;
        // }

        let k:HandleKey;
        match uMsg {
            WM_COMMAND=>{
                k = HandleKey::new(uMsg, LOWORD(wParam as DWORD), HIWORD(wParam as DWORD));
            },
            WM_NOTIFY=>{
                let p = unsafe{&*(lParam as LPNMHDR)};
                //id == ((LPNMHDR)lParam)->idFrom && cd == ((LPNMHDR)lParam)->code)
                // TODO:check if idFrom and code range is u16??
                k = HandleKey::new(uMsg, p.idFrom as u16,p.code as u16);
            },
            _=>{
                k = HandleKey::new_msg(uMsg);
            }
        }
        self.combine_search(k.key(),&mut e)
    }


    // https://en.wikipedia.org/wiki/Binary_search_algorithm
    // https://schani.wordpress.com/2010/04/30/linear-vs-binary-search/
    // according to the bench,the compiler already cmov optimized
    // return TRUE if message handled
    fn combine_search(&self,key:u64,e:&mut Event) -> BOOL {
        // bin search
        let mut left = 0;
        let mut right = self.handlers.len() - 1;
        let mut mid = 0;
        for i in 0..self.bin_search_cnt {
            mid = (left + right) >> 1;
            debug_assert!(mid < right);
            if self.handlers[mid].key() < key {
                left = mid + 1;
            }else{
                right = mid;
            }
        }

        // linear search ,we must put a sentinel at end
        let mut i = left;

        // find smallest key
        loop{
            if self.handlers[i].key() >= key {
                break;
            }
            i+=1;
        }

        let mut bFind = winapi::shared::minwindef::FALSE;
        // call
        while self.handlers[i].key() == key {
            unsafe{
                (self.handlers[i].call)(e,&mut *self.root);
            }
            bFind = winapi::shared::minwindef::TRUE;
            i+=1;
        }
        bFind
    }
}

//CDialogImpl
impl<T> Dialog<T> {
    pub fn DoModal2(&mut self,r:*mut T) {
        let hWndParent = unsafe {
            um::winuser::GetActiveWindow()
        };
        self.DoModal(hWndParent, NULL_LPARAM,r);
    }

    pub fn DoModal(&mut self,hWndParent: HWND,dwInitParam: LPARAM,r:*mut T) -> INT_PTR {
        //ATLASSUME(m_hWnd == NULL);
        self.root = r;
        self.modal = true;
        //self.puser = puser;
        thunk::set_this(self as *mut Self as *mut c_void);

        unsafe {
            let hInst = um::libloaderapi::GetModuleHandleW(ptr::null()) as HINSTANCE;
            let r = um::winuser::DialogBoxParamW(hInst,
                                            MAKEINTRESOURCEW(self.idd),
                                            hWndParent,
                                            Some(Self::StartDialogProc),
                                            dwInitParam);
            //let e = um::errhandlingapi::GetLastError();
            //println!("err:{}", e);
            r
        }
    }

    pub fn EndDialog(&self, nRetCode: c_int) -> BOOL {
        self.inner.assert_window();
        assert!(self.modal);
        unsafe {
            um::winuser::EndDialog(self.inner.GetHwnd(), nRetCode as INT_PTR)
        }
    }

    // modeless dialogs
    pub fn Create3(&mut self,r:*mut T) ->HWND {
        self.Create(NULL_HWND, NULL_LPARAM,r)
    }

    pub fn Create2(&mut self, hWndParent: HWND,r:*mut T) -> HWND {
        self.Create(hWndParent, NULL_LPARAM,r)
    }

    pub fn Create(&mut self, hWndParent: HWND, dwInitParam: LPARAM,r:*mut T) -> HWND {
        //ATLASSUME(m_hWnd == NULL);
        self.root = r;
        thunk::set_this(self as *mut Self as *mut c_void);
        self.modal = false;

        unsafe {
            let hWnd = um::winuser::CreateDialogParamW(0 as HINSTANCE,
                                                  MAKEINTRESOURCEW(self.idd),
                                                  hWndParent,
                                                  Some(Self::StartDialogProc),
                                                  dwInitParam);
            let e = winapi::um::errhandlingapi::GetLastError();
            println!("err:{}  idd:{}", e, self.idd);
            //self.inner.Attach(hWnd);
            um::winuser::ShowWindow(hWnd, SW_SHOW);
            //self.inner.ShowWindow(SW_SHOW);
            //ATLASSUME(m_hWnd == hWnd);
            hWnd
        }
    }

    //duplicate with cwindow
    // pub fn DestroyWindow(&mut self) -> BOOL {
    //     self.inner.DestroyWindow()
    //     // self.inner.assert_window();
    //     // assert!(self.modal == false);
    //     // unsafe {
    //     //     if um::winuser::DestroyWindow(self.inner.GetHwnd()) == winapi::shared::minwindef::FALSE {
    //     //         return winapi::shared::minwindef::FALSE;
    //     //     }
    //     // }
    //     // return TRUE;
    // }
}

// all handlers
impl<T> Dialog<T> {
    pub fn msg_handler(&mut self)->DlgMsg<T>{
        DlgMsg::new(&mut self.handlers)
    }

    pub fn btn_handler(&mut self,id:WORD)->BtnMsg<T>{
        BtnMsg::new(id,&mut self.handlers)   
    }
}

//expose all functions in cwindow
// impl<T> Dialog<T> {
//     expose_cwindow!();
// }