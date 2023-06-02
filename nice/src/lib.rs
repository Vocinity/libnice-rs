pub use ffi;
pub use glib;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

mod auto;
pub use crate::auto::*;
mod address;
mod agent;
pub use address::Address;

//pub mod prelude {
//    pub use crate::auto::traits::*;
//}
