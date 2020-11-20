// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Display;
use gio;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>) @extends gio::AppLaunchContext;

    match fn {
        get_type => || ffi::gdk_app_launch_context_get_type(),
    }
}

impl AppLaunchContext {
    pub fn get_display(&self) -> Option<Display> {
        unsafe {
            from_glib_none(ffi::gdk_app_launch_context_get_display(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn set_desktop(&self, desktop: i32) {
        unsafe {
            ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop);
        }
    }

    pub fn set_icon<P: IsA<gio::Icon>>(&self, icon: Option<&P>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon(
                self.to_glib_none().0,
                icon.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    pub fn set_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(
                self.to_glib_none().0,
                icon_name.to_glib_none().0,
            );
        }
    }

    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe {
            ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp);
        }
    }
}

#[derive(Clone, Default)]
pub struct AppLaunchContextBuilder {
    display: Option<Display>,
}

impl AppLaunchContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> AppLaunchContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref display) = self.display {
            properties.push(("display", display));
        }
        let ret = glib::Object::new(AppLaunchContext::static_type(), &properties)
            .expect("object new")
            .downcast::<AppLaunchContext>()
            .expect("downcast");
        ret
    }

    pub fn display(mut self, display: &Display) -> Self {
        self.display = Some(display.clone());
        self
    }
}

impl fmt::Display for AppLaunchContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AppLaunchContext")
    }
}
