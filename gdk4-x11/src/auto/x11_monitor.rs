// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use gdk;
use glib::translate::*;
use std::fmt;
use x11::xlib;

glib::glib_wrapper! {
    pub struct X11Monitor(Object<ffi::GdkX11Monitor, ffi::GdkX11MonitorClass>) @extends gdk::Monitor;

    match fn {
        get_type => || ffi::gdk_x11_monitor_get_type(),
    }
}

impl X11Monitor {
    pub fn get_output(&self) -> xlib::XID {
        unsafe { ffi::gdk_x11_monitor_get_output(self.to_glib_none().0) }
    }

    pub fn get_workarea(&self) -> gdk::Rectangle {
        unsafe {
            let mut workarea = gdk::Rectangle::uninitialized();
            ffi::gdk_x11_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }
}

impl fmt::Display for X11Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Monitor")
    }
}
