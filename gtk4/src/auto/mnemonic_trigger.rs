// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::ShortcutTrigger;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct MnemonicTrigger(Object<ffi::GtkMnemonicTrigger, ffi::GtkMnemonicTriggerClass>) @extends ShortcutTrigger;

    match fn {
        get_type => || ffi::gtk_mnemonic_trigger_get_type(),
    }
}

impl MnemonicTrigger {
    pub fn new(keyval: u32) -> MnemonicTrigger {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_mnemonic_trigger_new(keyval)) }
    }

    pub fn get_keyval(&self) -> u32 {
        unsafe { ffi::gtk_mnemonic_trigger_get_keyval(self.to_glib_none().0) }
    }
}

#[derive(Clone, Default)]
pub struct MnemonicTriggerBuilder {
    keyval: Option<u32>,
}

impl MnemonicTriggerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> MnemonicTrigger {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref keyval) = self.keyval {
            properties.push(("keyval", keyval));
        }
        let ret = glib::Object::new(MnemonicTrigger::static_type(), &properties)
            .expect("object new")
            .downcast::<MnemonicTrigger>()
            .expect("downcast");
        ret
    }

    pub fn keyval(mut self, keyval: u32) -> Self {
        self.keyval = Some(keyval);
        self
    }
}

impl fmt::Display for MnemonicTrigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MnemonicTrigger")
    }
}
