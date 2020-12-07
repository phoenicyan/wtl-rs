use winapi::um::winuser::{MSG,GetMessageW,PeekMessageW,TranslateMessage,DispatchMessageW,PM_NOREMOVE,};
use winapi::shared::windef::{HWND,POINT};


// all dialog entry(not include child dialog)
pub struct MessageLoop;

impl MessageLoop {
	pub fn init(hWnd: HWND) {
		let mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};
		unsafe{
			PeekMessageW( &mut msg, hWnd, 0, 0, PM_NOREMOVE);
		}
	}

	pub fn run() {
		let mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};
		unsafe{
			while GetMessageW( &mut msg, 0 as HWND, 0, 0 ) > 0 {
				TranslateMessage(&msg);
		        DispatchMessageW(&msg); 
		    }
		}
	}
}