#![allow(non_snake_case,dead_code,unused_variables,unused_assignments)]

use winapi::*;
use winapi::ctypes::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std;
use atl::consts::*;
use atl::cwindow::*;
use atl::{thunk,Handler,Event,HandleKey};
use std::ops::{Deref,DerefMut};

pub struct CWindowImpl<T> {
	inner: CWindow,
	thk: &'static mut thunk::Thunk,
	state: DWORD, // destroy or not
    super_win_proc: WNDPROC,
    root:*mut T, //raw pointer to the Root Dialogs
    bin_search_cnt:u32,         //used for combine search,search step cnt for bin search
    pub handlers: Vec<Handler<T>>,
}

impl<T> Deref for CWindowImpl<T> {
    type Target = CWindow;
    fn deref<'a>(&'a self)->&'a CWindow {
        &self.inner
    }
}

//impl this for Attach and Detach
impl<T> DerefMut for CWindowImpl<T> {
    //type Target = CWindow;
    fn deref_mut<'a>(&'a mut self)->&'a mut CWindow{
        &mut self.inner
    }
}

impl<T> CWindowImpl<T> {
    pub fn new()->CWindowImpl<T> {
    	CWindowImpl {
    		super_win_proc: Some(um::winuser::DefWindowProcW),
    		inner: CWindow::new(NULL_HWND),
            thk: thunk::get_thunk(),
            //idd: idd,
            state: 0,
            //modal: false,
            root: 0 as *mut T,
            bin_search_cnt:0,
            handlers:vec![Handler::new(0xFFFF,0xFFFF, 0xFFFF , 0xFFFF, |_,_|{})],   //put a sentinel in the vec
    	}
    }

    // pub fn Attach(&mut self,h: HWND) {
    //     self.inner.Attach(h)
    // }
    
    // pub fn Detach(&mut self)->HWND {
    //     self.inner.Detach()
    // }
}

impl<T> CWindowImpl<T> {
    fn InitThunk(&mut self, h: HWND) -> WNDPROC {
        let pself = self as *mut _ as *mut c_void;
        self.thk.init(Self::WindowProc as DWORD_PTR, pself);
        self.inner.Attach(h);
        let p = self.thk.GetCodeAddress();
        unsafe {
            std::mem::transmute(p)
        }
        
    }
}

impl<T> CWindowImpl<T> {
 //    fn DefWindowProc2(&self)->LRESULT	{
	// 	//const _ATL_MSG* pMsg = m_pCurrentMsg;
	// 	//let mut lRes: LRESULT = 0;
	// 	//if (pMsg != NULL)
	// 	return DefWindowProc(pMsg->message, pMsg->wParam, pMsg->lParam);
	// 	//return lRes;
	// }

	fn DefWindowProc(&self,uMsg: UINT,wParam: WPARAM,lParam: LPARAM)->LRESULT {
		unsafe {
			um::winuser::CallWindowProcW(self.super_win_proc, self.inner.GetHwnd(), uMsg, wParam, lParam)
		}
		//return ::CallWindowProc((FARPROC)m_pfnSuperWindowProc, m_hWnd, uMsg, wParam, lParam);
	}

	//virtual function
	fn OnFinalMessage(&self,hWnd: HWND)	{
		// override to do something, if needed
	}
}
// CWindowImplBase
impl<T> CWindowImpl<T> {
    unsafe extern "system" fn StartWindowProc(hWnd: HWND,uMsg: UINT,wParam: WPARAM,lParam: LPARAM)->LRESULT {
    	0
    }
	/*
	CWindowImplBaseT< TBase, TWinTraits >* pThis = (CWindowImplBaseT< TBase, TWinTraits >*)hWnd;
	// set a ptr to this message and save the old value
	_ATL_MSG msg(pThis->m_hWnd, uMsg, wParam, lParam);
	const _ATL_MSG* pOldMsg = pThis->m_pCurrentMsg;
	pThis->m_pCurrentMsg = &msg;
	// pass to the message map to process
	LRESULT lRes = 0;
	BOOL bRet = pThis->ProcessWindowMessage(pThis->m_hWnd, uMsg, wParam, lParam, lRes, 0);
	// restore saved value for the current message
	ATLASSERT(pThis->m_pCurrentMsg == &msg);

	// do the default processing if message was not handled
	if(!bRet)
	{
		if(uMsg != WM_NCDESTROY)
			lRes = pThis->DefWindowProc(uMsg, wParam, lParam);
		else
		{
			// unsubclass, if needed
			LONG_PTR pfnWndProc = ::GetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC);
			lRes = pThis->DefWindowProc(uMsg, wParam, lParam);
			if(pThis->m_pfnSuperWindowProc != ::DefWindowProc && ::GetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC) == pfnWndProc)
				::SetWindowLongPtr(pThis->m_hWnd, GWLP_WNDPROC, (LONG_PTR)pThis->m_pfnSuperWindowProc);
			// mark window as destryed
			pThis->m_dwState |= WINSTATE_DESTROYED;
		}
	}
	if((pThis->m_dwState & WINSTATE_DESTROYED) && pOldMsg== NULL)
	{
		// clear out window handle
		HWND hWndThis = pThis->m_hWnd;
		pThis->m_hWnd = NULL;
		pThis->m_dwState &= ~WINSTATE_DESTROYED;
		// clean up after window is destroyed
		pThis->m_pCurrentMsg = pOldMsg;
		pThis->OnFinalMessage(hWndThis);
	}else {
		pThis->m_pCurrentMsg = pOldMsg;
	}
	return lRes;
	*/
    // only CMDIFrameWindowImpl in wtl use virtual function GetWindowProc to override this,so skip GetWindowProc first
    unsafe extern "system" fn WindowProc(hWnd: HWND,uMsg: UINT,wParam: WPARAM,lParam: LPARAM)->LRESULT {
    	let mut_self = &mut *(hWnd as *mut Self);

        let mut lRes: LRESULT = 0;
        let h = mut_self.inner.GetHwnd();


		//CWindowImplBaseT< TBase, TWinTraits >* pThis = (CWindowImplBaseT< TBase, TWinTraits >*)hWnd;
		// set a ptr to this message and save the old value
		//_ATL_MSG msg(pThis->m_hWnd, uMsg, wParam, lParam);
		//const _ATL_MSG* pOldMsg = pThis->m_pCurrentMsg;
		//pThis->m_pCurrentMsg = &msg;
		// pass to the message map to process
		//LRESULT lRes = 0;
		let bRet = Self::ProcessWindowMessage(mut_self,h , uMsg, wParam, lParam, &mut lRes, 0);
		// restore saved value for the current message
		//ATLASSERT(pThis->m_pCurrentMsg == &msg);

		// do the default processing if message was not handled
		//if(!bRet)
		if bRet == winapi::shared::minwindef::FALSE {
			if uMsg != WM_NCDESTROY {
				lRes = Self::DefWindowProc(mut_self,uMsg, wParam, lParam);
			} else {
				// unsubclass, if needed
				// let pfnWndProc = um::winuser::GetWindowLongPtrW(h, GWLP_WNDPROC);
				// lRes = Self::DefWindowProc(mut_self,uMsg, wParam, lParam);
				// if let Some(p) = mut_self.super_win_proc {
				// 	if p != um::winuser::DefWindowProcW && um::winuser::GetWindowLongPtrW(h, GWLP_WNDPROC) == pfnWndProc {
				// 		um::winuser::SetWindowLongPtrW(h, GWLP_WNDPROC, mut_self.super_win_proc as LONG_PTR);
				// 	}
				// }
				// mark window as destryed
				mut_self.state |= WINSTATE_DESTROYED;
			}
		}

		// if (mut_self.m_dwState & WINSTATE_DESTROYED) && pOldMsg== NULL {
		// 	// clear out window handle
		// 	HWND hWndThis = pThis->m_hWnd;
		// 	pThis->m_hWnd = NULL;
		// 	pThis->m_dwState &= ~WINSTATE_DESTROYED;
		// 	// clean up after window is destroyed
		// 	pThis->m_pCurrentMsg = pOldMsg;
		// 	pThis->OnFinalMessage(hWndThis);
		// }else {
		// 	pThis->m_pCurrentMsg = pOldMsg;
		// }
		return lRes;
    }

    pub fn SubclassWindow(&mut self,hWnd: HWND)->BOOL {
    	debug_assert!(self.inner.GetHwnd() == 0 as HWND);
    	debug_assert!(unsafe{ um::winuser::IsWindow(hWnd) } == winapi::shared::minwindef::TRUE);
		//ATLASSUME(m_hWnd == NULL);
		//ATLASSERT(::IsWindow(hWnd));

		// Allocate the thunk structure here, where we can fail gracefully.

		let proc_thk = self.InitThunk(hWnd);
		let pthunk = proc_thk.unwrap();

		self.sort_handlers();
		//Self::sort_handlers(&mut *pself);
		// BOOL result = m_thunk.Init(GetWindowProc(), this);
		// if (result == winapi::shared::minwindef::FALSE)
		// {
		// 	return winapi::shared::minwindef::FALSE;
		// }
		//WNDPROC pProc = m_thunk.GetWNDPROC();
		//WNDPROC pfnWndProc = (WNDPROC)::SetWindowLongPtr(hWnd, GWLP_WNDPROC, (LONG_PTR)pProc);
        #[cfg(target_pointer_width = "32")]
            {
                let pfnWndProc =
              unsafe { um::winuser::SetWindowLongPtrW(hWnd,
                                           (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int,
                                           pthunk as i32)};

                if pfnWndProc == 0 {
                    return winapi::shared::minwindef::FALSE;
                }
                self.super_win_proc = unsafe{ std::mem::transmute(pfnWndProc) };
                //m_hWnd = hWnd;
                self.inner.Attach(hWnd);
            }
        #[cfg(target_pointer_width = "64")]
            {
                let pfnWndProc =
              unsafe { um::winuser::SetWindowLongPtrW(hWnd,
                                           (std::mem::size_of::<LRESULT>() + DWLP_MSGRESULT as usize) as c_int,
                                           pthunk as LONG_PTR)};

                if pfnWndProc == 0 {
                    return winapi::shared::minwindef::FALSE;
                }
                self.super_win_proc = unsafe{ std::mem::transmute(pfnWndProc) };
                //m_hWnd = hWnd;
                self.inner.Attach(hWnd);
            }
		return winapi::shared::minwindef::TRUE;
    }

    pub fn UnsubclassWindow(&mut self,bForce: Option<BOOL>)->HWND {
		//ATLASSUME(m_hWnd != NULL);
		// debug_assert!(self.inner.GetHwnd() != NULL_HWND);

		// WNDPROC pOurProc = m_thunk.GetWNDPROC();
		// WNDPROC pActiveProc = (WNDPROC)::GetWindowLongPtr(m_hWnd, GWLP_WNDPROC);

		// HWND hWnd = NULL;
		// if (bForce || pOurProc == pActiveProc)
		// {
		// 	if(!::SetWindowLongPtr(m_hWnd, GWLP_WNDPROC, (LONG_PTR)m_pfnSuperWindowProc))
		// 		return NULL;

		// 	m_pfnSuperWindowProc = ::DefWindowProc;
		// 	hWnd = m_hWnd;
		// 	m_hWnd = NULL;
		// }
		// return hWnd;
		NULL_HWND
    }
}


impl<T> CWindowImpl<T> {
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


// impl<T> CWindowImpl<T>{
// 	expose_cwindow!();
// }