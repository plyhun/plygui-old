/*#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_builder;

#[cfg(target_os="windows")]
mod win32;
#[cfg(target_os="windows")]
use win32 as inner;
#[cfg(target_os="windows")]
pub use inner::common;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;

#[cfg(target_os = "macos")]
mod cocoa;
#[cfg(target_os = "macos")]
use cocoa as inner;
#[cfg(target_os = "macos")]
pub use inner::common;*/

#[macro_use]
extern crate derive_builder;

pub mod development;
pub mod layout;

mod ids;

pub use ids::Id;

pub use std::fmt::{Result as FmtResult, Formatter, Debug};

pub trait UiApplication {
    fn new_window(&mut self, title: &str, width: u16, height: u16, has_menu: bool) -> Box<UiWindow>;
    fn name(&self) -> &str;
    fn start(&mut self);
}

pub trait UiMember {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<Box<FnMut(&mut UiMember, u16, u16)>>);

    fn set_visibility(&mut self, visibility: Visibility);
    fn visibility(&self) -> Visibility;
    fn role<'a>(&'a self) -> UiRole<'a>;
    fn role_mut<'a>(&'a mut self) -> UiRoleMut<'a>;
    fn id(&self) -> Id;
    
    //fn native_id(&self) -> NativeId;
    
    fn is_control(&self) -> Option<&UiControl>;
    fn is_control_mut(&mut self) -> Option<&mut UiControl>;
}

pub trait UiControl: UiMember {
    fn layout_width(&self) -> layout::Size;
	fn layout_height(&self) -> layout::Size;
	fn layout_gravity(&self) -> layout::Gravity;
	fn layout_orientation(&self) -> layout::Orientation;
	fn layout_alignment(&self) -> layout::Alignment;
	
	fn set_layout_width(&mut self, layout::Size);
	fn set_layout_height(&mut self, layout::Size);
	fn set_layout_gravity(&mut self, layout::Gravity);
	fn set_layout_orientation(&mut self, layout::Orientation);
	fn set_layout_alignment(&mut self, layout::Alignment);
    
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);

    fn is_container_mut(&mut self) -> Option<&mut UiContainer>;
    fn is_container(&self) -> Option<&UiContainer>;

    fn parent(&self) -> Option<&UiContainer>;
    fn parent_mut(&mut self) -> Option<&mut UiContainer>;
    fn root(&self) -> Option<&UiContainer>;
    fn root_mut(&mut self) -> Option<&mut UiContainer>;
}

pub trait UiContainer: UiMember {
    fn set_child(&mut self, Option<Box<UiControl>>) -> Option<Box<UiControl>>;
    fn child(&self) -> Option<&UiControl>;
    fn child_mut(&mut self) -> Option<&mut UiControl>;

    fn find_control_by_id_mut(&mut self, id: Id) -> Option<&mut UiControl>;
    fn find_control_by_id(&self, id: Id) -> Option<&UiControl>;

    fn is_multi_mut(&mut self) -> Option<&mut UiMultiContainer>;
    fn is_multi(&self) -> Option<&UiMultiContainer>;
}

pub trait UiMultiContainer: UiContainer {
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
    fn push_child(&mut self, child: Box<UiControl>) {
        let len = self.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<UiControl>> {
        let len = self.len();
        if len > 0 {
        	self.remove_child_from(len - 1)
        } else {
        	None
        }
    }
}

pub trait UiWindow: UiMember + UiContainer {}

pub trait UiButton: UiControl {
    //fn new(label: &str) -> Box<Self>;
    fn label(&self) -> &str;
    fn on_left_click(&mut self, Option<Box<FnMut(&mut UiButton)>>);
}

pub trait UiLinearLayout: UiMultiContainer + UiControl {
    fn orientation(&self) -> layout::Orientation;
    fn set_orientation(&mut self, layout::Orientation);
}

pub trait UiRelativeLayout: UiMultiContainer + UiControl {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

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

impl<'a> Debug for UiRole<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            UiRole::Window(a) => write!(f, "UiWindow ({:?})", a.id()),
            UiRole::Button(a) => write!(f, "UiButton ({:?})", a.id()),
            UiRole::LinearLayout(a) => write!(f, "UiLinearLayout ({:?})", a.id()),
        }
    }
}
impl<'a> Debug for UiRoleMut<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            UiRoleMut::Window(ref a) => write!(f, "UiWindow ({:?})", a.id()),
            UiRoleMut::Button(ref a) => write!(f, "UiButton ({:?})", a.id()),
            UiRoleMut::LinearLayout(ref a) => write!(f, "UiLinearLayout ({:?})", a.id()),
        }
    }
}
