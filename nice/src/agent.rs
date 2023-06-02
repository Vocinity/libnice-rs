


//impl Agent {
//    #[cfg(any(feature = "v0_1_5", feature = "dox"))]
//    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
//    #[doc(alias = "nice_agent_recv")]
//    pub fn recv<P: IsA<gio::Cancellable>>(
//        &self,
//        stream_id: u32,
//        component_id: u32,
//        cancellable: Option<&P>,
//    ) -> Result<(isize, Vec<u8>), glib::Error> {
//        unsafe {
//            let mut buf = Vec::<u8>::uninitialized();
//            let mut buf_len = mem::MaybeUninit::uninit();
//            let mut error = ptr::null_mut();
//            let ret = ffi::nice_agent_recv(
//                self.to_glib_none().0,
//                stream_id,
//                component_id,
//                buf.to_glib_none_mut().0,
//                buf_len.as_mut_ptr(),
//                cancellable.map(|p| p.as_ref()).to_glib_none().0,
//                &mut error,
//            );
//            let buf_len = buf_len.assume_init();
//            if error.is_null() {
//                Ok((ret, buf))
//            } else {
//                Err(from_glib_full(error))
//            }
//        }
//    }
//
//    #[cfg(any(feature = "v0_1_5", feature = "dox"))]
//    #[cfg_attr(feature = "dox", doc(cfg(feature = "v0_1_5")))]
//    #[doc(alias = "nice_agent_recv_nonblocking")]
//    pub fn recv_nonblocking<P: IsA<gio::Cancellable>>(
//        &self,
//        stream_id: u32,
//        component_id: u32,
//        cancellable: Option<&P>,
//    ) -> Result<(isize, Vec<u8>), glib::Error> {
//        unsafe {
//            let mut buf = Vec::<u8>::uninitialized();
//            let mut buf_len = mem::MaybeUninit::uninit();
//            let mut error = ptr::null_mut();
//            let ret = ffi::nice_agent_recv_nonblocking(
//                self.to_glib_none().0,
//                stream_id,
//                component_id,
//                buf.to_glib_none_mut().0,
//                buf_len.as_mut_ptr(),
//                cancellable.map(|p| p.as_ref()).to_glib_none().0,
//                &mut error,
//            );
//            let buf_len = buf_len.assume_init();
//            if error.is_null() {
//                Ok((ret, buf))
//            } else {
//                Err(from_glib_full(error))
//            }
//        }
//    }
//}
