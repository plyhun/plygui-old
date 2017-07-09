#[macro_use]
extern crate lazy_static;

#[cfg(target_os="windows")]
mod win32;
#[cfg(target_os="windows")]
use win32 as inner;


#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
mod cocoa;
#[cfg(target_os = "macos")]
use cocoa as inner;

pub mod layout;

//pub use inner::{Window, Button, LinearLayout};
pub use inner::{Window, Button};

pub enum UiRole<'a> {
    Window(&'a mut UiWindow),
    Button(&'a mut UiButton),
    LinearLayout(&'a mut UiLinearLayout),
}

/*struct UiControlBase {
	pub signature: usize,
	pub os_signature: usize,
	pub type_signature: usize,
}*/

pub trait UiMember {
    fn show(&mut self);
    fn hide(&mut self);
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<Box<FnMut(&mut UiMember, u16, u16)>>);
    
    /*fn handle(&mut self);
	fn parent(&self) -> Option<&UiControl>;
	fn top_level(&self) -> bool;
	fn visible(&self) -> bool;
	fn enabled(&self) -> bool;
	fn enable(&mut self);
	fn disable(&mut self);*/

    fn role<'a>(&'a mut self) -> UiRole<'a>;
}

pub trait UiControl: UiMember {
    fn layout_params(&self) -> (layout::Params, layout::Params);
    fn set_layout_params(&mut self, layout::Params, layout::Params);
    fn draw(&mut self, x: u16, y: u16);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16);
}

pub trait UiContainer: UiMember {
	fn set_child(&mut self, Option<Box<UiControl>>) -> Option<Box<UiControl>>;
	fn child(&self) -> Option<&Box<UiControl>>;
	fn child_mut(&mut self) -> Option<&mut Box<UiControl>>;
}

pub trait UiMultiContainer {
    fn push_child(&mut self, Box<UiControl>);
    fn pop_child(&mut self) -> Option<Box<UiControl>>;
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<UiControl>) -> Option<Box<UiControl>>;
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>>;
}

pub trait UiWindow: UiMember {
    //fn new(title: &str, width: u16, height: u16, has_menu: bool) -> Box<Self>;
    fn start(&mut self);
}

pub trait UiButton: UiControl {
    //fn new(label: &str) -> Box<Self>;
    fn label(&self) -> &str;
    fn on_left_click(&mut self, Option<Box<FnMut(&mut UiButton)>>);
}

pub trait UiLinearLayout: UiMultiContainer + UiControl {
	fn orientation(&self) -> layout::Orientation;
	fn set_orientation(&mut self, layout::Orientation);
}

pub trait UiRelativeLayout: UiMultiContainer + UiControl {
	
}
