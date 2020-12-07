#![allow(dead_code)]

use super::Thunk;

use winapi::*;
use winapi::um::winnt::{MEM_COMMIT, PAGE_EXECUTE_READWRITE, MEM_RELEASE};
use winapi::shared::minwindef::{LPVOID};
use winapi::shared::basetsd::{SIZE_T};
use std;
use std::cell::RefCell;

//thread local thunk pool
//we need mutable borrow of TP,so RefCell is uesd
thread_local!(static TP: RefCell<ThunkPool> = RefCell::new(ThunkPool::new()));

//pub function
pub fn get_thunk() -> &'static mut Thunk {
    TP.with(|tp|{
		let mut tp_mut = tp.borrow_mut();
		tp_mut.get_thunk()
	})
}

pub fn put_back(t: &mut Thunk) {
    TP.with(|tp|{
		let mut tp_mut = tp.borrow_mut();
		tp_mut.put_back(t);
	});
}

//a global stores the obj that created a window
pub fn set_this(p_this: LPVOID) {
    TP.with(|tp|{
		let mut tp_mut = tp.borrow_mut();
		tp_mut.set_this(p_this);
	})
}

pub fn get_this() -> LPVOID {
    TP.with(|tp|{
		let mut tp_mut = tp.borrow_mut();
		tp_mut.get_this()
	})
}

pub fn drop_pool() {
    TP.with(|tp|{
		let mut tp_mut = tp.borrow_mut();
		tp_mut.drop();
	})
}

//inner
struct ThunkPool {
    pages: Vec<LPVOID>,
    free_thunks: Vec<*mut Thunk>, // store addrs of all free thunks
    last_this: LPVOID,
}

const PAGE_SIZE: SIZE_T = 4096;
impl ThunkPool {
    fn new() -> ThunkPool {
        ThunkPool { pages: Vec::new(), free_thunks: Vec::new(), last_this: 0 as LPVOID }
    }

    fn alloc(&mut self) {
        unsafe {
            let p = um::memoryapi::VirtualAlloc(0 as LPVOID,
                                           PAGE_SIZE,
                                           MEM_COMMIT,
                                           PAGE_EXECUTE_READWRITE);
			//save VirtualAlloc mem pointer
            self.pages.push(p);

            let thunk_cnt = PAGE_SIZE / (std::mem::size_of::<Thunk>() as SIZE_T);

            let mut p_thunk: *mut Thunk = p as *mut Thunk;

			//split into thunk and push all the pointers to free_thunks
            for _i in 0..thunk_cnt {
                self.free_thunks.push(p_thunk);
                p_thunk = p_thunk.offset(1);
            }
        }
    }

	//Drop trait can't free thread_local automatically
	//so wo free resources manually at the end of main(after all windows has been closed)
    fn drop(&mut self) {
        unsafe {
            for page in &self.pages {
                um::memoryapi::VirtualFree(*page, 0, MEM_RELEASE);
            }
        }
    }

    fn get_thunk(&mut self) -> &'static mut Thunk {
        unsafe {
			//alloc if no free thunks
            if self.free_thunks.len() == 0 {
                self.alloc();
            }

            if let Some(t) = self.free_thunks.pop() {
                &mut (*t) as &'static mut Thunk
            } else {
                panic!("should not panic");
            }
        }
    }

    fn put_back(&mut self, t: &mut Thunk) {
        self.free_thunks.push(t);
    }

    fn set_this(&mut self, p_this: LPVOID) {
        self.last_this = p_this;
    }

    fn get_this(&mut self) -> LPVOID {
        let t = self.last_this;
        self.last_this = 0 as LPVOID;
        t
    }
}
