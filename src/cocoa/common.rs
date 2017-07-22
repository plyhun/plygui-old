use super::*;

use std::{ptr, mem, str};
use self::cocoa::base::{class, id};
use self::cocoa::foundation::{NSString, NSRect, NSRange};
use self::cocoa::appkit::NSView;
use objc::runtime::Class;

use {layout, UiContainer, UiMember, UiControl, UiRoleMut, UiRole, Visibility};

pub struct RefClass(pub *const Class);
unsafe impl Sync for RefClass {}

#[repr(C)]
pub struct CocoaControlBase {
	pub control: id,
    pub layout_width: layout::Params,
    pub layout_height: layout::Params,
    pub visibility: Visibility,
    pub measured_size: (u16, u16),

    pub h_resize: Option<Box<FnMut(&mut UiMember, u16, u16)>>,
}

impl Default for CocoaControlBase {
    fn default() -> CocoaControlBase {
        CocoaControlBase {
        	control: ptr::null_mut(),
            h_resize: None,
            visibility: Visibility::Visible,
            layout_width: layout::Params::MatchParent,
            layout_height: layout::Params::WrapContent,
            measured_size: (0, 0),
        }
    }
}
impl CocoaControlBase {
	pub unsafe fn on_removed_from_container(&mut self) {
    	self.control.removeFromSuperview();
        msg_send![self.control, dealloc];
        self.control = ptr::null_mut();
    }	
}

pub unsafe trait CocoaControl: UiMember {
    unsafe fn on_added_to_container(&mut self, &CocoaContainer, x: u16, y: u16);
    unsafe fn on_removed_from_container(&mut self, &CocoaContainer);
    unsafe fn base(&mut self) -> &mut CocoaControlBase;
}

pub unsafe trait CocoaContainer: UiContainer + UiMember {
    unsafe fn id(&self) -> id;
}

pub unsafe fn cast_uicontrol_to_cocoa_mut(input: &mut Box<UiControl>) -> &mut CocoaControl {
    use std::ops::DerefMut;
    match input.role_mut() {
        UiRoleMut::Button(_) => {
            let a: &mut Box<button::Button> = mem::transmute(input);
            a.deref_mut()
        }
        UiRoleMut::LinearLayout(_) => {
            let a: &mut Box<layout_linear::LinearLayout> = mem::transmute(input);
            a.deref_mut()
        }
        UiRoleMut::Window(_) => {
            panic!("Window as a container child is impossible!");
        }
        _=>{
	        unimplemented!();
        }
    }
}

pub unsafe fn cast_uicontrol_to_cocoa(input: &Box<UiControl>) -> &CocoaControl {
    use std::ops::Deref;
    match input.role() {
        UiRole::Button(_) => {
            let a: &Box<button::Button> = mem::transmute(input);
            a.deref()
        }
        UiRole::LinearLayout(_) => {
            let a: &Box<layout_linear::LinearLayout> = mem::transmute(input);
            a.deref()
        }
        UiRole::Window(_) => {
            panic!("Window as a container child is impossible!");
        }
        _=>{
	        unimplemented!();
        }
    }
}

pub unsafe fn measure_string(text: &str) -> (u16, u16) {
	let title = NSString::alloc(cocoa::base::nil).init_str(text);
	                    
    let text_storage: id = msg_send![class("NSTextStorage"), alloc];
    let text_storage: id = msg_send![text_storage, initWithString:title];
    let layout_manager: id = msg_send![class("NSLayoutManager"), alloc];
    let layout_manager: id = msg_send![layout_manager, init];
    let text_container: id = msg_send![class("NSTextContainer"), alloc];
	let text_container: id = msg_send![text_container, init];
	
	msg_send![layout_manager, addTextContainer:text_container];
	msg_send![text_container, release];
	msg_send![text_storage, addLayoutManager:layout_manager];
	msg_send![layout_manager, release];
	
	let num = msg_send![layout_manager, numberOfGlyphs];
	let range = NSRange::new(0, num);
	
	let string_rect: NSRect = msg_send![layout_manager, boundingRectForGlyphRange:range inTextContainer:text_container];
	(string_rect.size.width as u16, string_rect.size.height as u16)
}