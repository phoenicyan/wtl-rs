#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::um::libloaderapi::{GetModuleHandleW,LoadStringW,};
use winapi::um::winuser::*;
use winreg::enums::*;
use winreg::RegKey;
use wtl::atl::*;

use super::ui;
use crate::about;

pub struct MainDialogHandler{
    about_dlg_handler: about::AboutDialogHandler,
}

impl MainDialogHandler {
    #[inline(always)]    
    pub fn new()->MainDialogHandler{
        MainDialogHandler{
            about_dlg_handler: about::AboutDialogHandler,
        }
    }
}

impl ui::DialogHandler for MainDialogHandler {
    fn register_handler(&self,r:&mut ui::Root) {
        r.main_dlg.this_msg().on_init_dialog(|_,t| {
            t.main_dlg.this.CenterWindow(NULL_HWND);
            unsafe {
                let hInstance = GetModuleHandleW(0 as *mut _);
                let hIcon = LoadImageW(hInstance,
                                       MAKEINTRESOURCEW(IDR_MAINFRAME as u16),
                                       IMAGE_ICON,
                                       GetSystemMetrics(SM_CXICON),
                                       GetSystemMetrics(SM_CYICON),
                                       LR_DEFAULTCOLOR) as HICON;
                t.main_dlg.this.SetIcon(hIcon, TRUE);

                let hIconSmall = LoadImageW(hInstance,
                                            MAKEINTRESOURCEW(IDR_MAINFRAME as u16),
                                            IMAGE_ICON,
                                            GetSystemMetrics(SM_CXSMICON),
                                            GetSystemMetrics(SM_CYSMICON),
                                            LR_DEFAULTCOLOR) as HICON;
                t.main_dlg.this.SetIcon(hIconSmall, FALSE);

                let SysMenu = t.main_dlg.this.GetSystemMenu(FALSE);
                if IsMenu(SysMenu) == TRUE {
                    let mut szAboutMenu: Vec<u16> = Vec::with_capacity(256);
                    szAboutMenu.set_len(256);
                    if LoadStringW(0 as HINSTANCE, IDS_ABOUTBOX, szAboutMenu.as_mut_ptr(), 255) > 0 {
                        AppendMenuW(SysMenu, MF_SEPARATOR, 0, 0 as *mut _);
                        AppendMenuW(SysMenu, MF_STRING, IDM_ABOUTBOX as usize, szAboutMenu.as_mut_ptr());
                    }
                }
                //SysMenu.Detach();
            }

            // // register object for message filtering
            // CMessageLoop* pLoop = _Module.GetMessageLoop();
            // pLoop->AddMessageFilter(this);

            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(reg) = hkcu.open_subkey("Software\\Microsoft\\WTL Samples\\GUIDGEN") {
                if let Ok(lRet) = reg.get_value::<u32, &str>("GUID Type") {
                    t.main_dlg.m_nGuidType = lRet;
                }
            }

            t.main_dlg.this.CheckRadioButton(IDC_RADIO1 as i32,IDC_RADIO4 as i32,
                                             (IDC_RADIO1 + t.main_dlg.m_nGuidType) as i32);

            if t.main_dlg.NewGUID() == FALSE {
                t.main_dlg.CloseDialog(IDABORT);
            }

            t.main_dlg.DisplayGUID();
        });

        r.main_dlg.this_msg().on_close(|_,t|{
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(reg) = hkcu.create_subkey("Software\\Microsoft\\WTL Samples\\GUIDGEN") {
                reg.0.set_value("GUID Type", &t.main_dlg.m_nGuidType);
            }
            unsafe{PostQuitMessage(0)};
        }).set_user_priority(100);       // only for test

        r.main_dlg.this_msg().on_sys_command(|e,t|{
            if (e.get_wparam() & 0xFFF0) as u32 == IDM_ABOUTBOX {
                // // Due to limitation of rust-lang/rust#35978, using generic requires trait
                // // object or redundant verbose struct wrapper to work. So this PR uses enum
                // // instead to get around the problem and make the types simpler.
                // let about_dialog = ui::AboutDialog::new();
                // about_dialog.this.DoModal(e.get_hwnd(), 0, &mut about_dialog); // 0 as *mut _
                t.main_dlg.about_dialog.this.ShowWindow(SW_SHOW);
                t.main_dlg.this.EnableWindow(0);
            } else {
                e.set_result(0);
                unsafe{DefWindowProcA(e.get_hwnd(), e.get_msg(), e.get_wparam(), e.get_lparam());}
            }
        });

        r.main_dlg.btn_radio1_msg().on_click(|_,t|{
            t.main_dlg.UpdateData();
            t.main_dlg.DisplayGUID();
        });

        r.main_dlg.btn_radio2_msg().on_click(|_,t|{
            t.main_dlg.UpdateData();
            t.main_dlg.DisplayGUID();
        });

        r.main_dlg.btn_radio3_msg().on_click(|_,t|{
            t.main_dlg.UpdateData();
            t.main_dlg.DisplayGUID();
        });

        r.main_dlg.btn_radio4_msg().on_click(|_,t|{
            t.main_dlg.UpdateData();
            t.main_dlg.DisplayGUID();
        });

        r.main_dlg.btn_copy_msg().on_click(|_,t|{
            t.main_dlg.UpdateData();

            let strResult = t.main_dlg.GetFormattedGuid();

            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            ctx.set_contents(strResult.to_owned()).unwrap();
        });

        r.main_dlg.btn_newguid_msg().on_click(|_,t|{
            if t.main_dlg.NewGUID() == TRUE {
                t.main_dlg.DisplayGUID();
            }
        });

        r.main_dlg.btn_exit_msg().on_click(|_,t|{
            let hkcu = RegKey::predef(HKEY_CURRENT_USER);
            if let Ok(reg) = hkcu.create_subkey("Software\\Microsoft\\WTL Samples\\GUIDGEN") {
                reg.0.set_value("GUID Type", &t.main_dlg.m_nGuidType);
            }
            unsafe{PostQuitMessage(0)};
        });

        self.about_dlg_handler.register_handler(r);
    }
}
