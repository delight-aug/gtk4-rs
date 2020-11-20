// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct WaylandSeat(Object<ffi::GdkWaylandSeat, ffi::GdkWaylandSeatClass>) @extends gdk::Seat;

    match fn {
        get_type => || ffi::gdk_wayland_seat_get_type(),
    }
}

impl WaylandSeat {}

impl fmt::Display for WaylandSeat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandSeat")
    }
}
