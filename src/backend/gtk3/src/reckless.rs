use glib::object::{Downcast, IsA};
use glib::translate::*;
use gtk::{Buildable, Container, Fixed, Widget};

use glib_sys as glib_ffi;
use gobject_sys as gobject_ffi;
use gtk_sys as gtk_ffi;

use std::{mem, ptr};

pub mod ffi {
    extern "C" {
        pub fn reckless_fixed_get_type() -> ::glib_sys::GType;
        pub fn reckless_fixed_new() -> *mut ::gtk_sys::GtkWidget;
    }

    #[repr(C)]
    pub struct GtkRecklessFixedClass {
        _truncated_record_marker: ::libc::c_void,
        // /*Ignored*/field parent_class has incomplete type
    }
    impl ::std::fmt::Debug for GtkRecklessFixedClass {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessFixedClass @ {:?}", self as *const _)).finish()
        }
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct GtkRecklessFixed {
        pub container: ::gtk_sys::GtkFixed,
    }
    impl ::std::fmt::Debug for GtkRecklessFixed {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(&format!("GtkRecklessFixed @ {:?}", self as *const _)).field("container", &self.container).finish()
        }
    }
}

glib_wrapper! {
    pub struct RecklessFixed(Object<ffi::GtkRecklessFixed, ffi::GtkRecklessFixedClass>): [
         Fixed => gtk_ffi::GtkFixed,
         Container => gtk_ffi::GtkContainer,
         Widget => gtk_ffi::GtkWidget,
         Buildable => gtk_ffi::GtkBuildable,
    ];

    match fn {
        get_type => || ffi::reckless_fixed_get_type(),
    }
}

impl RecklessFixed {
    pub fn new() -> RecklessFixed {
        //assert_initialized_main_thread!(); // private module
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
        unsafe { Widget::from_glib_none(ffi::reckless_fixed_new()).downcast_unchecked() }
    }
}
impl Default for RecklessFixed {
    fn default() -> Self {
        Self::new()
    }
}
pub trait RecklessFixedExt {}

impl<O: IsA<RecklessFixed>> RecklessFixedExt for O {}
