impl ::std::fmt::Debug for sockaddr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("sockaddr @ {:?}", self as *const _))
            .finish()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct sockaddr(libc::sockaddr);

impl ::std::fmt::Debug for sockaddr_in {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("sockaddr_in @ {:?}", self as *const _))
            .finish()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct sockaddr_in(libc::sockaddr_in);

impl ::std::fmt::Debug for sockaddr_in6 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("sockaddr_in6 @ {:?}", self as *const _))
            .finish()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct sockaddr_in6(libc::sockaddr_in6);
