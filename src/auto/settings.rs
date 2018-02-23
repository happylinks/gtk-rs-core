// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use Action;
use SettingsBindFlags;
use SettingsSchema;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Settings(Object<ffi::GSettings, ffi::GSettingsClass>);

    match fn {
        get_type => || ffi::g_settings_get_type(),
    }
}

impl Settings {
    pub fn new(schema_id: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new(schema_id.to_glib_none().0))
        }
    }

    //pub fn new_full<'a, 'b, P: Into<Option<&'a /*Ignored*/SettingsBackend>>, Q: Into<Option<&'b str>>>(schema: &SettingsSchema, backend: P, path: Q) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_full() }
    //}

    //pub fn new_with_backend(schema_id: &str, backend: /*Ignored*/&SettingsBackend) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_with_backend() }
    //}

    //pub fn new_with_backend_and_path(schema_id: &str, backend: /*Ignored*/&SettingsBackend, path: &str) -> Settings {
    //    unsafe { TODO: call ffi::g_settings_new_with_backend_and_path() }
    //}

    pub fn new_with_path(schema_id: &str, path: &str) -> Settings {
        unsafe {
            from_glib_full(ffi::g_settings_new_with_path(schema_id.to_glib_none().0, path.to_glib_none().0))
        }
    }

    #[cfg_attr(feature = "v2_40", deprecated)]
    pub fn list_relocatable_schemas() -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_settings_list_relocatable_schemas())
        }
    }

    #[cfg_attr(feature = "v2_40", deprecated)]
    pub fn list_schemas() -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_settings_list_schemas())
        }
    }

    pub fn sync() {
        unsafe {
            ffi::g_settings_sync();
        }
    }

    pub fn unbind<P: IsA<glib::Object>>(object: &P, property: &str) {
        unsafe {
            ffi::g_settings_unbind(object.to_glib_none().0, property.to_glib_none().0);
        }
    }
}

pub trait SettingsExt {
    fn apply(&self);

    fn bind<P: IsA<glib::Object>>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags);

    //fn bind_with_mapping<P: IsA<glib::Object>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags, get_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindGetMapping, set_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindSetMapping, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn bind_writable<P: IsA<glib::Object>>(&self, key: &str, object: &P, property: &str, inverted: bool);

    fn create_action(&self, key: &str) -> Option<Action>;

    fn delay(&self);

    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    fn get_boolean(&self, key: &str) -> bool;

    fn get_child(&self, name: &str) -> Option<Settings>;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_default_value(&self, key: &str) -> Option<glib::Variant>;

    fn get_double(&self, key: &str) -> f64;

    fn get_enum(&self, key: &str) -> i32;

    fn get_flags(&self, key: &str) -> u32;

    fn get_has_unapplied(&self) -> bool;

    fn get_int(&self, key: &str) -> i32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_int64(&self, key: &str) -> i64;

    //fn get_mapped<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, key: &str, mapping: /*Unknown conversion*//*Unimplemented*/SettingsGetMapping, user_data: P) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[cfg_attr(feature = "v2_40", deprecated)]
    fn get_range(&self, key: &str) -> Option<glib::Variant>;

    fn get_string(&self, key: &str) -> Option<String>;

    fn get_strv(&self, key: &str) -> Vec<String>;

    fn get_uint(&self, key: &str) -> u32;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uint64(&self, key: &str) -> u64;

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_user_value(&self, key: &str) -> Option<glib::Variant>;

    fn get_value(&self, key: &str) -> Option<glib::Variant>;

    fn is_writable(&self, name: &str) -> bool;

    fn list_children(&self) -> Vec<String>;

    fn list_keys(&self) -> Vec<String>;

    #[cfg_attr(feature = "v2_40", deprecated)]
    fn range_check(&self, key: &str, value: &glib::Variant) -> bool;

    fn reset(&self, key: &str);

    fn revert(&self);

    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool;

    fn set_boolean(&self, key: &str, value: bool) -> bool;

    fn set_double(&self, key: &str, value: f64) -> bool;

    fn set_enum(&self, key: &str, value: i32) -> bool;

    fn set_flags(&self, key: &str, value: u32) -> bool;

    fn set_int(&self, key: &str, value: i32) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_int64(&self, key: &str, value: i64) -> bool;

    fn set_string(&self, key: &str, value: &str) -> bool;

    fn set_strv(&self, key: &str, value: &[&str]) -> bool;

    fn set_uint(&self, key: &str, value: u32) -> bool;

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_uint64(&self, key: &str, value: u64) -> bool;

    fn set_value(&self, key: &str, value: &glib::Variant) -> bool;

    //fn get_property_backend(&self) -> /*Ignored*/Option<SettingsBackend>;

    fn get_property_delay_apply(&self) -> bool;

    fn get_property_path(&self) -> Option<String>;

    #[deprecated]
    fn get_property_schema(&self) -> Option<String>;

    fn get_property_schema_id(&self) -> Option<String>;

    fn get_property_settings_schema(&self) -> Option<SettingsSchema>;

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_writable_change_event<F: Fn(&Self, u32) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_backend_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[deprecated]
    fn connect_property_schema_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_schema_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_settings_schema_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Settings> + IsA<glib::object::Object>> SettingsExt for O {
    fn apply(&self) {
        unsafe {
            ffi::g_settings_apply(self.to_glib_none().0);
        }
    }

    fn bind<P: IsA<glib::Object>>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags) {
        unsafe {
            ffi::g_settings_bind(self.to_glib_none().0, key.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0, flags.to_glib());
        }
    }

    //fn bind_with_mapping<P: IsA<glib::Object>, Q: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, key: &str, object: &P, property: &str, flags: SettingsBindFlags, get_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindGetMapping, set_mapping: /*Unknown conversion*//*Unimplemented*/SettingsBindSetMapping, user_data: Q, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::g_settings_bind_with_mapping() }
    //}

    fn bind_writable<P: IsA<glib::Object>>(&self, key: &str, object: &P, property: &str, inverted: bool) {
        unsafe {
            ffi::g_settings_bind_writable(self.to_glib_none().0, key.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0, inverted.to_glib());
        }
    }

    fn create_action(&self, key: &str) -> Option<Action> {
        unsafe {
            from_glib_full(ffi::g_settings_create_action(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn delay(&self) {
        unsafe {
            ffi::g_settings_delay(self.to_glib_none().0);
        }
    }

    //fn get(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::g_settings_get() }
    //}

    fn get_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_boolean(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_child(&self, name: &str) -> Option<Settings> {
        unsafe {
            from_glib_full(ffi::g_settings_get_child(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_default_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_default_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::g_settings_get_double(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_enum(&self, key: &str) -> i32 {
        unsafe {
            ffi::g_settings_get_enum(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_flags(&self, key: &str) -> u32 {
        unsafe {
            ffi::g_settings_get_flags(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    fn get_has_unapplied(&self) -> bool {
        unsafe {
            from_glib(ffi::g_settings_get_has_unapplied(self.to_glib_none().0))
        }
    }

    fn get_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::g_settings_get_int(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_int64(&self, key: &str) -> i64 {
        unsafe {
            ffi::g_settings_get_int64(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    //fn get_mapped<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, key: &str, mapping: /*Unknown conversion*//*Unimplemented*/SettingsGetMapping, user_data: P) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::g_settings_get_mapped() }
    //}

    fn get_range(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_range(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_string(&self, key: &str) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_settings_get_string(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_strv(&self, key: &str) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_get_strv(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_uint(&self, key: &str) -> u32 {
        unsafe {
            ffi::g_settings_get_uint(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn get_uint64(&self, key: &str) -> u64 {
        unsafe {
            ffi::g_settings_get_uint64(self.to_glib_none().0, key.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn get_user_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_user_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn get_value(&self, key: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_settings_get_value(self.to_glib_none().0, key.to_glib_none().0))
        }
    }

    fn is_writable(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_is_writable(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn list_children(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_children(self.to_glib_none().0))
        }
    }

    fn list_keys(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_settings_list_keys(self.to_glib_none().0))
        }
    }

    fn range_check(&self, key: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_range_check(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    fn reset(&self, key: &str) {
        unsafe {
            ffi::g_settings_reset(self.to_glib_none().0, key.to_glib_none().0);
        }
    }

    fn revert(&self) {
        unsafe {
            ffi::g_settings_revert(self.to_glib_none().0);
        }
    }

    //fn set(&self, key: &str, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> bool {
    //    unsafe { TODO: call ffi::g_settings_set() }
    //}

    fn set_boolean(&self, key: &str, value: bool) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_boolean(self.to_glib_none().0, key.to_glib_none().0, value.to_glib()))
        }
    }

    fn set_double(&self, key: &str, value: f64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_double(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    fn set_enum(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_enum(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    fn set_flags(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_flags(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    fn set_int(&self, key: &str, value: i32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_int(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_int64(&self, key: &str, value: i64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_int64(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    fn set_string(&self, key: &str, value: &str) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_string(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    fn set_strv(&self, key: &str, value: &[&str]) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_strv(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    fn set_uint(&self, key: &str, value: u32) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_uint(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    fn set_uint64(&self, key: &str, value: u64) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_uint64(self.to_glib_none().0, key.to_glib_none().0, value))
        }
    }

    fn set_value(&self, key: &str, value: &glib::Variant) -> bool {
        unsafe {
            from_glib(ffi::g_settings_set_value(self.to_glib_none().0, key.to_glib_none().0, value.to_glib_none().0))
        }
    }

    //fn get_property_backend(&self) -> /*Ignored*/Option<SettingsBackend> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "backend".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    fn get_property_delay_apply(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "delay-apply".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn get_property_path(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "path".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_schema(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "schema".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_schema_id(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "schema-id".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn get_property_settings_schema(&self) -> Option<SettingsSchema> {
        unsafe {
            let mut value = Value::from_type(<SettingsSchema as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "settings-schema".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    //fn connect_change_event<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented keys: *.CArray TypeId { ns_id: 2, id: 4 }
    //}

    fn connect_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_writable_change_event<F: Fn(&Self, u32) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "writable-change-event",
                transmute(writable_change_event_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_writable_changed<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "writable-changed",
                transmute(writable_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_backend_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::backend",
                transmute(notify_backend_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_delay_apply_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::delay-apply",
                transmute(notify_delay_apply_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_has_unapplied_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::has-unapplied",
                transmute(notify_has_unapplied_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::path",
                transmute(notify_path_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_schema_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::schema",
                transmute(notify_schema_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_schema_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::schema-id",
                transmute(notify_schema_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_settings_schema_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::settings-schema",
                transmute(notify_settings_schema_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline<P>(this: *mut ffi::GSettings, key: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(key))
}

unsafe extern "C" fn writable_change_event_trampoline<P>(this: *mut ffi::GSettings, key: libc::c_uint, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<Settings> {
    let f: &&(Fn(&P, u32) -> Inhibit + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked(), key).to_glib()
}

unsafe extern "C" fn writable_changed_trampoline<P>(this: *mut ffi::GSettings, key: *mut libc::c_char, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(key))
}

unsafe extern "C" fn notify_backend_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_delay_apply_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_has_unapplied_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_path_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_schema_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_schema_id_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_settings_schema_trampoline<P>(this: *mut ffi::GSettings, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Settings> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Settings::from_glib_borrow(this).downcast_unchecked())
}
