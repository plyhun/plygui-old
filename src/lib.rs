extern crate plygui_api;

pub use plygui_api::traits::*;
pub use plygui_api::ids::*;
pub use plygui_api::types::*;
pub use plygui_api::callbacks;
pub use plygui_api::layout;
pub use plygui_api::members;
pub use plygui_api::utils;

#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "gtk3"))]
extern crate plygui_gtk;
#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "gtk3"))]
pub use plygui_gtk::*;

#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "qt5"))]
extern crate plygui_qt;
#[cfg(all(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"), feature = "qt5"))]
pub use plygui_qt::*;

#[cfg(all(target_os = "macos", feature = "cocoa"))]
extern crate plygui_cocoa;
#[cfg(all(target_os = "macos", feature = "cocoa"))]
pub use plygui_cocoa::*;

#[cfg(all(target_os = "windows", feature = "win32"))]
extern crate plygui_win32;
#[cfg(all(target_os = "windows", feature = "win32"))]
pub use plygui_win32::*;
