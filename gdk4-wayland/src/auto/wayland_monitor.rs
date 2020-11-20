// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct WaylandMonitor(Object<ffi::GdkWaylandMonitor, ffi::GdkWaylandMonitorClass>) @extends gdk::Monitor;

    match fn {
        get_type => || ffi::gdk_wayland_monitor_get_type(),
    }
}

impl WaylandMonitor {}

impl fmt::Display for WaylandMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandMonitor")
    }
}
