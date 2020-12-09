use glib::translate::*;
use std::mem;

use libc::{sockaddr, sockaddr_in, sockaddr_in6};
// Unions
#[repr(C)]
#[derive(Copy, Clone)]
pub union Address_s {
    pub addr: sockaddr,
    pub ip4: sockaddr_in,
    pub ip6: sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Address {
    pub s: Address_s,
}

#[doc(hidden)]
impl Uninitialized for Address {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::NiceAddress> for Address {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::NiceAddress, Self> {
        let ptr: *const Address = &*self;
        Stash(ptr as *const ffi::NiceAddress, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::NiceAddress> for Address {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::NiceAddress, Self> {
        let ptr: *mut Address = &mut *self;
        StashMut(ptr as *mut ffi::NiceAddress, self)
    }
}
