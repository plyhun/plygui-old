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

pub use inner::{Application, Window, Button, LinearLayout};

pub enum UiRole<'a> {
    Window(&'a UiWindow),
    Button(&'a UiButton),
    LinearLayout(&'a UiLinearLayout),
}
pub enum UiRoleMut<'a> {
    Window(&'a mut UiWindow),
    Button(&'a mut UiButton),
    LinearLayout(&'a mut UiLinearLayout),
}

pub trait UiApplication: Drop {
	fn new_window(&mut self, title: &str, width: u16, height: u16, has_menu: bool) -> Box<Window>;
	fn name(&self) -> &str;
	fn start(&mut self);
}

pub trait UiMember {
    fn set_visibility(&mut self, visibility: Visibility);
    fn visibility(&self) -> Visibility;
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<Box<FnMut(&mut UiMember, u16, u16)>>);
    
    fn role<'a>(&'a self) -> UiRole<'a>;
    fn role_mut<'a>(&'a mut self) -> UiRoleMut<'a>;
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
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>>;
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>>;
    fn clear(&mut self) {
    	let len = self.len();
    	for index in (0..len).rev() {
    		self.remove_child_from(index);
    	}
    }
}

pub trait UiWindow: UiMember {
	
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
	Visible,
	Invisible,
	Gone,
}