// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Event;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct ButtonEvent(Object<ffi::GdkButtonEvent>) @extends Event;

    match fn {
        get_type => || ffi::gdk_button_event_get_type(),
    }
}

impl ButtonEvent {
    pub fn get_button(&self) -> u32 {
        unsafe { ffi::gdk_button_event_get_button(self.to_glib_none().0) }
    }
}

impl fmt::Display for ButtonEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ButtonEvent")
    }
}
