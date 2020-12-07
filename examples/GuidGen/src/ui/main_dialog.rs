#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use uuid::Uuid;
use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::{GetModuleHandleW,LoadStringW,};
use winapi::um::winuser::*;
use wtl::atl::{Dialog,DlgMsg};
use wtl::ctrls::{CButton,BtnMsg};

use super::AboutDialog;

pub struct MainDialog<T> {
    pub this: Dialog<T>,
    pub about_dialog: AboutDialog<T>,
	pub m_nGuidType: u32,
	pub m_guid: Uuid,
}

impl<T> MainDialog<T> {
	pub fn new()->MainDialog<T>{
		MainDialog{
			this: Dialog::new(IDD_GUIDGEN_DIALOG as u16),
			about_dialog: AboutDialog::new(),
			m_nGuidType: 0,
			m_guid: Uuid::nil(),
		}
	}

	pub fn create(&mut self,t:*mut T){
		let h = self.this.Create3(t);
		self.about_dialog.this.Create2(h,t);
		self.about_dialog.this.ShowWindow(SW_HIDE);
	}

	pub fn GetFormattedGuid(&self) -> String {
		let guid = self.m_guid.as_fields();
		let defaultStr = format!("{{{:08X}-{:04X}-{:04x}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}}}\r\n",
								 guid.0,    guid.1,    guid.2,
								 guid.3[0], guid.3[1], guid.3[2], guid.3[3],
								 guid.3[4], guid.3[5], guid.3[6], guid.3[7]);

		match self.m_nGuidType {
			0 => format!("// {}IMPLEMENT_OLECREATE(<<class>>, <<external_name>>, \r\n0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x});\r\n",
						 defaultStr, guid.0,    guid.1,    guid.2,
						 guid.3[0], guid.3[1], guid.3[2], guid.3[3],
						 guid.3[4], guid.3[5], guid.3[6], guid.3[7]),
			1 => format!("// {}DEFINE_GUID(<<name>>, \r\n0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x});\r\n",
						 defaultStr, guid.0,    guid.1,    guid.2,
						 guid.3[0], guid.3[1], guid.3[2], guid.3[3],
						 guid.3[4], guid.3[5], guid.3[6], guid.3[7]),
			2 => format!("// {}static const GUID <<name>> = \r\n{{ 0x{:x}, 0x{:x}, 0x{:x}, {{ 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x} }} }};\r\n",
						 defaultStr, guid.0,    guid.1,    guid.2,
						 guid.3[0], guid.3[1], guid.3[2], guid.3[3],
						 guid.3[4], guid.3[5], guid.3[6], guid.3[7]),
			_ => defaultStr,
		}
	}

	pub fn UpdateData(&mut self) {
		self.m_nGuidType = 0;
		if self.this.IsDlgButtonChecked(IDC_RADIO2 as i32) != 0 {
			self.m_nGuidType = 1;
		}
		if self.this.IsDlgButtonChecked(IDC_RADIO3 as i32) != 0 {
			self.m_nGuidType = 2;
		}
		if self.this.IsDlgButtonChecked(IDC_RADIO4 as i32) != 0 {
			self.m_nGuidType = 3;
		}
		//_ASSERTE(m_nGuidType >= 0 && m_nGuidType <= 3);
	}

	pub fn DisplayGUID(&self) {
		let szBuf = self.GetFormattedGuid();
		self.this.SetDlgItemText(IDC_RESULTS as i32, &szBuf);
	}

	pub fn NewGUID(&mut self) -> BOOL {
		self.m_guid = Uuid::new_v4();
		if Uuid::is_nil(&self.m_guid) {
			let mut szBuf: Vec<u16> = Vec::with_capacity(256);
			unsafe {
				szBuf.set_len(256);
				let hInstance = GetModuleHandleW(0 as *mut _);
				LoadStringW(hInstance, IDP_ERR_CREATE_GUID, szBuf.as_mut_ptr(), 255);
			}
			if let Ok(err_msg) = String::from_utf16(&szBuf) {
				self.this.MessageBox(&err_msg, "GUIDGen", MB_OK);
			}
			return FALSE;
		}
		TRUE
	}

	pub fn CloseDialog(&self, nVal: i32) {
		unsafe {
			DestroyWindow(self.this.GetHwnd());
			PostQuitMessage(nVal)
		};
	}

	////////////////////////////////////////
	// handlers
	pub fn this_msg(&mut self)->DlgMsg<T> {
		self.this.msg_handler()
	}

	pub fn btn_radio1_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDC_RADIO1 as u16)
	}

	pub fn btn_radio2_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDC_RADIO2 as u16)
	}

	pub fn btn_radio3_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDC_RADIO3 as u16)
	}

	pub fn btn_radio4_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDC_RADIO4 as u16)
	}

	pub fn btn_copy_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDOK as u16)
	}

	pub fn btn_newguid_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDC_NEWGUID as u16)
	}

	pub fn btn_exit_msg(&mut self)->BtnMsg<T>{
		self.this.btn_handler(IDCANCEL as u16)
	}
}
