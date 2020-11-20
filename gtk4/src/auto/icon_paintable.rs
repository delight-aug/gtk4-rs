// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use std;
use std::fmt;

glib::glib_wrapper! {
    pub struct IconPaintable(Object<ffi::GtkIconPaintable>) @implements gdk::Paintable;

    match fn {
        get_type => || ffi::gtk_icon_paintable_get_type(),
    }
}

impl IconPaintable {
    pub fn new_for_file<P: IsA<gio::File>>(file: &P, size: i32, scale: i32) -> IconPaintable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_icon_paintable_new_for_file(
                file.as_ref().to_glib_none().0,
                size,
                scale,
            ))
        }
    }

    pub fn get_file(&self) -> Option<gio::File> {
        unsafe { from_glib_full(ffi::gtk_icon_paintable_get_file(self.to_glib_none().0)) }
    }

    pub fn get_icon_name(&self) -> Option<std::path::PathBuf> {
        unsafe { from_glib_none(ffi::gtk_icon_paintable_get_icon_name(self.to_glib_none().0)) }
    }

    pub fn is_symbolic(&self) -> bool {
        unsafe { from_glib(ffi::gtk_icon_paintable_is_symbolic(self.to_glib_none().0)) }
    }

    pub fn get_property_is_symbolic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"is-symbolic\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `is-symbolic` getter")
                .unwrap()
        }
    }
}

#[derive(Clone, Default)]
pub struct IconPaintableBuilder {
    file: Option<gio::File>,
    icon_name: Option<String>,
    is_symbolic: Option<bool>,
}

impl IconPaintableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> IconPaintable {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref file) = self.file {
            properties.push(("file", file));
        }
        if let Some(ref icon_name) = self.icon_name {
            properties.push(("icon-name", icon_name));
        }
        if let Some(ref is_symbolic) = self.is_symbolic {
            properties.push(("is-symbolic", is_symbolic));
        }
        let ret = glib::Object::new(IconPaintable::static_type(), &properties)
            .expect("object new")
            .downcast::<IconPaintable>()
            .expect("downcast");
        ret
    }

    pub fn file<P: IsA<gio::File>>(mut self, file: &P) -> Self {
        self.file = Some(file.clone().upcast());
        self
    }

    pub fn icon_name(mut self, icon_name: &str) -> Self {
        self.icon_name = Some(icon_name.to_string());
        self
    }

    pub fn is_symbolic(mut self, is_symbolic: bool) -> Self {
        self.is_symbolic = Some(is_symbolic);
        self
    }
}

impl fmt::Display for IconPaintable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IconPaintable")
    }
}
