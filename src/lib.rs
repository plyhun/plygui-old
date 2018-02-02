extern crate plygui_api;

pub use plygui_api::traits::*;
pub use plygui_api::ids::*;
pub use plygui_api::types::*;
pub use plygui_api::layout;
pub use plygui_api::members;
pub use plygui_api::utils;
pub use plygui_api::callbacks;
#[cfg(feature = "markup")]
pub use plygui_api::markup;

#[cfg(all(target_os = "macos", feature = "cocoa"))]
extern crate plygui_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa"))]
pub use plygui_cocoa::*;

#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use plygui_win32::*;

#[cfg(feature = "markup")]
pub fn register_markup_members(registry: &mut plygui_api::markup::MarkupRegistry) {
    register_members(registry);
}
