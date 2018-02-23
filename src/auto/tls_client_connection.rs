// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Error;
use IOStream;
use SocketConnectable;
use TlsCertificateFlags;
use TlsConnection;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TlsClientConnection(Object<ffi::GTlsClientConnection, ffi::GTlsClientConnectionInterface>): TlsConnection, IOStream;

    match fn {
        get_type => || ffi::g_tls_client_connection_get_type(),
    }
}

impl TlsClientConnection {
    pub fn new<'a, P: IsA<IOStream>, Q: IsA<SocketConnectable> + 'a, R: Into<Option<&'a Q>>>(base_io_stream: &P, server_identity: R) -> Result<TlsClientConnection, Error> {
        let server_identity = server_identity.into();
        let server_identity = server_identity.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_client_connection_new(base_io_stream.to_glib_none().0, server_identity.0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait TlsClientConnectionExt {
    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn copy_session_state<P: IsA<TlsClientConnection>>(&self, source: &P);

    //fn get_accepted_cas(&self) -> /*Ignored*/Vec<glib::ByteArray>;

    fn get_server_identity(&self) -> Option<SocketConnectable>;

    fn get_use_ssl3(&self) -> bool;

    fn get_validation_flags(&self) -> TlsCertificateFlags;

    fn set_server_identity<P: IsA<SocketConnectable>>(&self, identity: &P);

    fn set_use_ssl3(&self, use_ssl3: bool);

    fn set_validation_flags(&self, flags: TlsCertificateFlags);

    fn connect_property_accepted_cas_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_server_identity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_ssl3_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsClientConnection> + IsA<glib::object::Object>> TlsClientConnectionExt for O {
    #[cfg(any(feature = "v2_46", feature = "dox"))]
    fn copy_session_state<P: IsA<TlsClientConnection>>(&self, source: &P) {
        unsafe {
            ffi::g_tls_client_connection_copy_session_state(self.to_glib_none().0, source.to_glib_none().0);
        }
    }

    //fn get_accepted_cas(&self) -> /*Ignored*/Vec<glib::ByteArray> {
    //    unsafe { TODO: call ffi::g_tls_client_connection_get_accepted_cas() }
    //}

    fn get_server_identity(&self) -> Option<SocketConnectable> {
        unsafe {
            from_glib_none(ffi::g_tls_client_connection_get_server_identity(self.to_glib_none().0))
        }
    }

    fn get_use_ssl3(&self) -> bool {
        unsafe {
            from_glib(ffi::g_tls_client_connection_get_use_ssl3(self.to_glib_none().0))
        }
    }

    fn get_validation_flags(&self) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_client_connection_get_validation_flags(self.to_glib_none().0))
        }
    }

    fn set_server_identity<P: IsA<SocketConnectable>>(&self, identity: &P) {
        unsafe {
            ffi::g_tls_client_connection_set_server_identity(self.to_glib_none().0, identity.to_glib_none().0);
        }
    }

    fn set_use_ssl3(&self, use_ssl3: bool) {
        unsafe {
            ffi::g_tls_client_connection_set_use_ssl3(self.to_glib_none().0, use_ssl3.to_glib());
        }
    }

    fn set_validation_flags(&self, flags: TlsCertificateFlags) {
        unsafe {
            ffi::g_tls_client_connection_set_validation_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn connect_property_accepted_cas_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::accepted-cas",
                transmute(notify_accepted_cas_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_server_identity_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::server-identity",
                transmute(notify_server_identity_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_use_ssl3_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::use-ssl3",
                transmute(notify_use_ssl3_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_validation_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::validation-flags",
                transmute(notify_validation_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_accepted_cas_trampoline<P>(this: *mut ffi::GTlsClientConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsClientConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsClientConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_server_identity_trampoline<P>(this: *mut ffi::GTlsClientConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsClientConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsClientConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_use_ssl3_trampoline<P>(this: *mut ffi::GTlsClientConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsClientConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsClientConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_validation_flags_trampoline<P>(this: *mut ffi::GTlsClientConnection, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsClientConnection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsClientConnection::from_glib_borrow(this).downcast_unchecked())
}
