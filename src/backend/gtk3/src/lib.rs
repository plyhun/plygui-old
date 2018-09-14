#[macro_use]
extern crate plygui_api;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate glib;
extern crate gdk;
extern crate gtk;
extern crate libc;
extern crate pango;

extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk_sys;

pub mod reckless;

#[macro_use]
pub mod common;

mod application;
mod button;
mod frame;
mod layout_linear;
mod splitted;
mod window;

#[cfg(feature = "markup")]
pub fn register_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    registry.register_member(plygui_api::markup::MEMBER_TYPE_BUTTON.into(), button::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT.into(), layout_linear::spawn);
    registry.register_member(plygui_api::markup::MEMBER_TYPE_FRAME.into(), frame::spawn).unwrap();
}

pub mod prelude {
	pub use plygui_api::controls::*;
	pub use plygui_api::ids::*;
	pub use plygui_api::types::*;
	pub use plygui_api::callbacks;
	pub use plygui_api::layout;
	pub use plygui_api::utils; 
	
	pub mod imp {
		pub use ::application::Application;
		pub use ::window::Window;
		pub use ::button::Button;
		pub use ::layout_linear::LinearLayout;
		pub use ::frame::Frame;
		pub use ::splitted::Splitted;
	}
}
