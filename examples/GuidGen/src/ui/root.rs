use super::main_dialog::MainDialog;

pub struct Root {
    pub main_dlg: MainDialog<Root>,
}

impl Root {
	#[inline(always)]
	pub fn new()->Root{
		Root{
			main_dlg:MainDialog::new()
		}
	}

	pub fn create(&mut self){
		let r = self as *mut _ ;
		self.main_dlg.create(r);
	}
}


/// all message handler must impl this trait to init callback closure
pub trait DialogHandler {
	fn register_handler(&self, r:&mut Root);
}
