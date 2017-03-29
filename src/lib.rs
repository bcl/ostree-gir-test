
extern crate libc;
extern crate gio;
extern crate gio_sys as gio_ffi;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gobject_sys as gobject_ffi;
extern crate ostree_sys as ffi;

/// No-op
macro_rules! callback_guard {
    () => ()
}

//pub use ffi::GUdevDeviceNumber as DeviceNumber;

pub use auto::*;
mod auto;
