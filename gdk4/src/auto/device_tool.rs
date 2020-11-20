// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::AxisFlags;
use crate::DeviceToolType;
use glib::object::Cast;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::fmt;

glib::glib_wrapper! {
    pub struct DeviceTool(Object<ffi::GdkDeviceTool>);

    match fn {
        get_type => || ffi::gdk_device_tool_get_type(),
    }
}

impl DeviceTool {
    pub fn get_axes(&self) -> AxisFlags {
        unsafe { from_glib(ffi::gdk_device_tool_get_axes(self.to_glib_none().0)) }
    }

    pub fn get_hardware_id(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_hardware_id(self.to_glib_none().0) }
    }

    pub fn get_serial(&self) -> u64 {
        unsafe { ffi::gdk_device_tool_get_serial(self.to_glib_none().0) }
    }

    pub fn get_tool_type(&self) -> DeviceToolType {
        unsafe { from_glib(ffi::gdk_device_tool_get_tool_type(self.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
pub struct DeviceToolBuilder {
    axes: Option<AxisFlags>,
    hardware_id: Option<u64>,
    serial: Option<u64>,
    tool_type: Option<DeviceToolType>,
}

impl DeviceToolBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> DeviceTool {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref axes) = self.axes {
            properties.push(("axes", axes));
        }
        if let Some(ref hardware_id) = self.hardware_id {
            properties.push(("hardware-id", hardware_id));
        }
        if let Some(ref serial) = self.serial {
            properties.push(("serial", serial));
        }
        if let Some(ref tool_type) = self.tool_type {
            properties.push(("tool-type", tool_type));
        }
        let ret = glib::Object::new(DeviceTool::static_type(), &properties)
            .expect("object new")
            .downcast::<DeviceTool>()
            .expect("downcast");
        ret
    }

    pub fn axes(mut self, axes: AxisFlags) -> Self {
        self.axes = Some(axes);
        self
    }

    pub fn hardware_id(mut self, hardware_id: u64) -> Self {
        self.hardware_id = Some(hardware_id);
        self
    }

    pub fn serial(mut self, serial: u64) -> Self {
        self.serial = Some(serial);
        self
    }

    pub fn tool_type(mut self, tool_type: DeviceToolType) -> Self {
        self.tool_type = Some(tool_type);
        self
    }
}

impl fmt::Display for DeviceTool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceTool")
    }
}
