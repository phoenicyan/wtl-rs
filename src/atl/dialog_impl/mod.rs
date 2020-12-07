/*
							impl_root
						/				\
					win_impl_base		dlg_impl_base
					/					/	  |		\
				win_impl		ax_dlg_impl	dlg_imp	simple_dlg

*/

pub use self::dlg_msg::DlgMsg;
pub use self::dialog::Dialog;
//pub use self::event::Event;


mod dialog;
mod dlg_msg;
