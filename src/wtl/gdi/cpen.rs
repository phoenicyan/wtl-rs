#![allow(non_snake_case, dead_code)]
use winapi::*;
use winapi::ctypes::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::ntdef::*;
use winapi::um::wingdi::*;

use std::mem;
use std::ops::Drop;


//////////////////////////////////////////////////////////////////
pub struct CPen {
    inner: Inner,
}

impl CPen {
	pub fn new()->CPen{
		CPen{
			inner: Inner::new(),
		}
	}

	pub fn from_pen(h: HPEN)->CPen{
		CPen{
			inner: Inner::from_pen(h)
		}
	}

	pub fn Attach(&mut self,hpen: HPEN){
		if self.inner.hpen != NULL_PEN2 && self.inner.hpen != hpen {
			unsafe{um::wingdi::DeleteObject(self.inner.hpen as HGDIOBJ)};
		}
		self.inner.hpen = hpen;
	}
}

impl Drop for CPen {
	fn drop(&mut self){
		self.inner.DeleteObject();
	}
}
//////////////////////////////////////////////////////////////////
pub struct CPenHandle {
    inner: Inner,
}

impl CPenHandle {
	pub fn new()->CPenHandle{
		CPenHandle{
			inner: Inner::new(),
		}
	}

	pub fn from_pen(h: HPEN)->CPenHandle{
		CPenHandle{
			inner: Inner::from_pen(h)
		}
	}

	pub fn Attach(&mut self,hpen: HPEN){
		self.inner.hpen = hpen;
	}
}

//////////////////////////////////////////////////////////////////
struct Inner {
    hpen: HPEN,
}

const NULL_PEN2: HPEN = 0 as HPEN;

impl Inner {
	fn new()->Inner{
		Inner{
			hpen: NULL_PEN2,
		}
	}

	fn from_pen(h: HPEN)->Inner{
		Inner{
			hpen: h,
		}
	}

	fn Detach (&mut self)->HPEN {
		let hPen = self.hpen;
		self.hpen = NULL_PEN2;
		hPen
	}

	//pub fn HPEN (&self)->operator { self.hpen }

	fn IsNull (&self)->bool { self.hpen == NULL_PEN2 }

	fn assert_null(&self){
		debug_assert!(self.hpen == NULL_PEN2);
	}
// Create methods
	fn CreatePen(&mut self,nPenStyle: c_int, nWidth: c_int, crColor: COLORREF)->HPEN {
		self.assert_null();
		self.hpen = unsafe{um::wingdi::CreatePen(nPenStyle, nWidth, crColor)};
		self.hpen
	}

//#ifndef _WIN32_WCE
	fn CreatePen_ext(&mut self,nPenStyle: c_int, nWidth: c_int, pLogBrush: &LOGBRUSH,nStyleCount: Option<c_int> /*= 0*/, lpStyle: Option<*const DWORD> /*= NULL*/)->HPEN {
		self.assert_null();
		let n = extract_opt_by_null!(nStyleCount,c_int) as DWORD;
		let s = extract_opt_by_null!(lpStyle,*const DWORD);
		self.hpen = unsafe{um::wingdi::ExtCreatePen(nPenStyle as DWORD, nWidth as DWORD, pLogBrush, n, s)};
		self.hpen
	}
//#endif // !_WIN32_WCE

	fn CreatePenIndirect(&mut self,lpLogPen: LPLOGPEN)->HPEN {
		self.assert_null();
		self.hpen = unsafe{um::wingdi::CreatePenIndirect(lpLogPen)};
		self.hpen
	}

	fn DeleteObject (&mut self)->BOOL {
		debug_assert!(self.hpen != NULL_PEN2);
		let bRet = unsafe{um::wingdi::DeleteObject(self.hpen as HGDIOBJ) as BOOL};
		if bRet == winapi::shared::minwindef::TRUE {
			self.hpen = NULL_PEN2;
		}
		bRet
	}

// Attributes
	fn GetLogPen (&self,pLogPen: &mut LOGPEN)->c_int {
		debug_assert!(self.hpen != NULL_PEN2);
		unsafe{um::wingdi::GetObjectW(self.hpen as HANDLE, mem::size_of::<LOGPEN>() as c_int, pLogPen as *mut _ as LPVOID)}
	}

	// pub fn GetLogPen (@LOGPEN& LogPen)->bool {
	// 	ATLASSERT(self.hpen != NULL);
	// 	um::wingdi::GetObject(self.hpen, sizeof(LOGPEN), &LogPen) == sizeof(LOGPEN)
	// }

//#ifndef _WIN32_WCE
	fn GetExtLogPen (&self,pLogPen: &mut LOGPEN,nSize: Option<c_int> /*= sizeof(EXTLOGPEN)*/)->c_int {
		debug_assert!(self.hpen != NULL_PEN2);
		let n = extract_opt_by_default!(mem::size_of::<EXTLOGPEN>(),nSize,c_int);
		unsafe{um::wingdi::GetObjectW(self.hpen as HANDLE, n, pLogPen as *mut _ as LPVOID)}
	}

	// pub fn GetExtLogPen (@EXTLOGPEN& ExtLogPen,nSize: Option<c_int> /*= sizeof(EXTLOGPEN)*/)->bool {
	// 	ATLASSERT(self.hpen != NULL);
	// 	let n = extract_opt_by_default!(mem::size_of::<EXTLOGPEN>(),nSize,c_int);
	// 	let nRet = ::GetObject(self.hpen, n, &ExtLogPen) as c_int;
	// 	(nRet > 0) && (nRet <= nSize)
	// }
//#endif // !_WIN32_WCE
}

//////////////////////////////////////////////////////////////////
// expose all functions in Inner
impl CPen {
	pub fn Detach (&mut self)->HPEN {
		self.inner.Detach()
	}
	pub fn IsNull (&self)->bool {
		self.inner.IsNull()
	}

	pub fn CreatePen(&mut self,nPenStyle: c_int, nWidth: c_int, crColor: COLORREF)->HPEN {
		self.inner.CreatePen(nPenStyle, nWidth, crColor)
	}
	pub fn CreatePen_ext(&mut self,nPenStyle: c_int, nWidth: c_int, pLogBrush: &LOGBRUSH,nStyleCount: Option<c_int>, lpStyle: Option<*const DWORD>)->HPEN {
		self.inner.CreatePen_ext(nPenStyle, nWidth, pLogBrush, nStyleCount, lpStyle)
	}
	pub fn CreatePenIndirect(&mut self,lpLogPen: LPLOGPEN)->HPEN {
		self.inner.CreatePenIndirect(lpLogPen)
	}
	pub fn DeleteObject (&mut self)->BOOL {
		self.inner.DeleteObject()
	}
	pub fn GetLogPen (&self,pLogPen: &mut LOGPEN)->c_int {
		self.inner.GetLogPen(pLogPen)
	}
	pub fn GetExtLogPen (&self,pLogPen: &mut LOGPEN,nSize: Option<c_int> /*= sizeof(EXTLOGPEN)*/)->c_int {
		self.inner.GetExtLogPen(pLogPen, nSize)
	}
}

//////////////////////////////////////////////////////////////////
impl CPenHandle {
	pub fn Detach (&mut self)->HPEN {
		self.inner.Detach()
	}
	pub fn IsNull (&self)->bool {
		self.inner.IsNull()
	}

	pub fn CreatePen(&mut self,nPenStyle: c_int, nWidth: c_int, crColor: COLORREF)->HPEN {
		self.inner.CreatePen(nPenStyle, nWidth, crColor)
	}
	pub fn CreatePen_ext(&mut self,nPenStyle: c_int, nWidth: c_int, pLogBrush: &LOGBRUSH,nStyleCount: Option<c_int>, lpStyle: Option<*const DWORD>)->HPEN {
		self.inner.CreatePen_ext(nPenStyle, nWidth, pLogBrush, nStyleCount, lpStyle)
	}
	pub fn CreatePenIndirect(&mut self,lpLogPen: LPLOGPEN)->HPEN {
		self.inner.CreatePenIndirect(lpLogPen)
	}
	pub fn DeleteObject (&mut self)->BOOL {
		self.inner.DeleteObject()
	}
	pub fn GetLogPen (&self,pLogPen: &mut LOGPEN)->c_int {
		self.inner.GetLogPen(pLogPen)
	}
	pub fn GetExtLogPen (&self,pLogPen: &mut LOGPEN,nSize: Option<c_int> /*= sizeof(EXTLOGPEN)*/)->c_int {
		self.inner.GetExtLogPen(pLogPen, nSize)
	}
}