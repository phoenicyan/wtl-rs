
use winapi::shared::ntdef::*;
use winapi::shared::windef::*;
use winapi::um::winuser::*;
use wtl::atl::*;

use super::ui;

pub struct AboutDialogHandler;

impl ui::DialogHandler for AboutDialogHandler {
    fn register_handler(&self,r:&mut ui::Root){
        r.main_dlg.about_dialog.this_msg().on_init_dialog(|_,t|{
            t.main_dlg.about_dialog.this.CenterWindow(t.main_dlg.this.GetHwnd());
        });

        r.main_dlg.about_dialog.this_msg().on_close(|_,t|{
            t.main_dlg.this.EnableWindow(1);
            t.main_dlg.about_dialog.this.ShowWindow(SW_HIDE);
        });

        r.main_dlg.about_dialog.btn_ok_msg().on_click(|_,t|{
            t.main_dlg.this.EnableWindow(1);
            t.main_dlg.about_dialog.this.ShowWindow(SW_HIDE);
        });
    }
}
