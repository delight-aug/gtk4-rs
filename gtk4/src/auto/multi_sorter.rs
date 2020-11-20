// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::Buildable;
use crate::Sorter;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct MultiSorter(Object<ffi::GtkMultiSorter, ffi::GtkMultiSorterClass>) @extends Sorter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || ffi::gtk_multi_sorter_get_type(),
    }
}

impl MultiSorter {
    pub fn new() -> MultiSorter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_multi_sorter_new()) }
    }
}

impl Default for MultiSorter {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MULTI_SORTER: Option<&MultiSorter> = None;

pub trait MultiSorterExt: 'static {
    fn append<P: IsA<Sorter>>(&self, sorter: &P);

    fn remove(&self, position: u32);
}

impl<O: IsA<MultiSorter>> MultiSorterExt for O {
    fn append<P: IsA<Sorter>>(&self, sorter: &P) {
        unsafe {
            ffi::gtk_multi_sorter_append(
                self.as_ref().to_glib_none().0,
                sorter.as_ref().to_glib_full(),
            );
        }
    }

    fn remove(&self, position: u32) {
        unsafe {
            ffi::gtk_multi_sorter_remove(self.as_ref().to_glib_none().0, position);
        }
    }
}

impl fmt::Display for MultiSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultiSorter")
    }
}
