extern crate plygui_api;

pub use plygui_api::controls::*;
pub use plygui_api::ids::*;
pub use plygui_api::types::*;
pub use plygui_api::callbacks;
pub use plygui_api::layout;
pub use plygui_api::utils;

#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "gtk3"))]
pub extern crate plygui_gtk as imp;

#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "qt5"))]
pub extern crate plygui_qt as imp;

#[cfg(all(target_os = "macos", feature = "cocoa"))]
pub extern crate plygui_cocoa as imp;

#[cfg(all(target_os = "windows", feature = "win32"))]
pub extern crate plygui_win32 as imp;
