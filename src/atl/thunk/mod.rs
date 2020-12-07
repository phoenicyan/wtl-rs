
pub mod stdthunk_x64;
pub mod stdthunk_x86;
pub mod thunk_pool;

#[cfg(target_arch = "x86_64")]
pub use self::stdthunk_x64::imp::Thunk;

#[cfg(target_arch = "x86")]
pub use self::stdthunk_x86::imp::Thunk;

pub use self::thunk_pool::{get_thunk, put_back, set_this, get_this, drop_pool};
