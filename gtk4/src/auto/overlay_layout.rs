// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::LayoutManager;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct OverlayLayout(Object<ffi::GtkOverlayLayout, ffi::GtkOverlayLayoutClass>) @extends LayoutManager;

    match fn {
        get_type => || ffi::gtk_overlay_layout_get_type(),
    }
}

impl OverlayLayout {
    pub fn new() -> OverlayLayout {
        assert_initialized_main_thread!();
        unsafe { LayoutManager::from_glib_full(ffi::gtk_overlay_layout_new()).unsafe_cast() }
    }
}

impl Default for OverlayLayout {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_OVERLAY_LAYOUT: Option<&OverlayLayout> = None;

impl fmt::Display for OverlayLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OverlayLayout")
    }
}
