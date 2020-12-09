pub use ffi;

macro_rules! skip_assert_initialized {
    () => {};
}

macro_rules! assert_initialized_main_thread {
    () => {};
}

#[allow(clippy::unreadable_literal)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::match_same_arms)]
mod auto;
pub use crate::auto::*;
mod address;
mod agent;
pub use address::Address;

pub mod prelude {
    pub use crate::auto::traits::*;
}
