use super::*;

use std::{ptr, mem, str};
use std::os::raw::c_void;
use self::cocoa::base::{class, id as cocoa_id};
use self::cocoa::foundation::{NSString, NSRect, NSRange};
use self::cocoa::appkit::{NSWindow, NSView};
use objc::runtime::{Class, Object, Ivar, YES, NO, class_copyIvarList};

use {layout, UiContainer, UiMember, UiControl, UiRoleMut, UiRole, Visibility};

pub struct RefClass(pub *const Class);
unsafe impl Sync for RefClass {}

#[repr(C)]
pub struct CocoaControlBase {
    pub control: cocoa_id,
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
    pub fn set_visibility(&mut self, visibility: Visibility) {
        if self.visibility != visibility {
            self.visibility = visibility;
            unsafe {
                match self.visibility {
                    Visibility::Invisible => {
                        msg_send![self.control, setHidden: YES];
                    }
                    _ => {
                        msg_send![self.control, setHidden: NO];
                    }
                }
            }
        }
    }
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }
    pub fn id(&self) -> Id {
        self.control
    }
    pub fn parent(&self) -> Option<&UiContainer> {
        unsafe {
            let id_: cocoa_id = msg_send![self.control, superview];
            if id_.is_null() {
                return None;
            }
            let mut ivar_count = 0;
            let ivars = class_copyIvarList(msg_send![id_, class], &mut ivar_count);
            /*let ivars = from_raw_parts(ivars, ivar_count as usize);
			for ivar in ivars {
				let ivar: &Ivar = mem::transmute(*ivar);
				println!("ivar {}", ivar.name());
			}
			if ivar_count < 1 {
				return None;
			}*/
            let ivar: &Ivar = mem::transmute(*ivars);
            let id_: &Object = mem::transmute(id_);
            let saved: *mut c_void = *id_.get_ivar(ivar.name());
            match ivar.name() {
                super::layout_linear::IVAR => {
                    let ll: &LinearLayout = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(ll)
                }
                super::window::IVAR => {
                    let w: &Window = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(w)
                }
                _ => None,
            }
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut UiContainer> {
        unsafe {
            let id_: cocoa_id = msg_send![self.control, superview];
            if id_.is_null() {
                return None;
            }
            let mut ivar_count = 0;
            let ivars = class_copyIvarList(msg_send![id_, class], &mut ivar_count);
            let ivar: &Ivar = mem::transmute(*ivars);
            let id_: &Object = mem::transmute(id_);
            let saved: *mut c_void = *id_.get_ivar(ivar.name());
            match ivar.name() {
                super::layout_linear::IVAR => {
                    let ll: &mut LinearLayout = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(ll)
                }
                super::window::IVAR => {
                    let w: &mut Window = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(w)
                }
                _ => None,
            }
        }
    }
    pub fn root(&self) -> Option<&UiContainer> {
        unsafe {
            let w: cocoa_id = msg_send![self.control, window];
            if w.is_null() {
                return None;
            }
            let dlg = w.delegate();
            let mut ivar_count = 0;
            let ivars = class_copyIvarList(msg_send![dlg, class], &mut ivar_count);
            let ivar: &Ivar = mem::transmute(*ivars);
            let id_: &Object = mem::transmute(dlg);
            let saved: *mut c_void = *id_.get_ivar(ivar.name());
            match ivar.name() {
                super::layout_linear::IVAR => {
                    let ll: &LinearLayout = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(ll)
                }
                super::window::IVAR => {
                    let w: &Window = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(w)
                }
                _ => None,
            }
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut UiContainer> {
        unsafe {
            let w: cocoa_id = msg_send![self.control, window];
            if w.is_null() {
                return None;
            }
            let dlg = w.delegate();
            let mut ivar_count = 0;
            let ivars = class_copyIvarList(msg_send![dlg, class], &mut ivar_count);
            let ivar: &Ivar = mem::transmute(*ivars);
            let id_: &Object = mem::transmute(dlg);
            let saved: *mut c_void = *id_.get_ivar(ivar.name());
            match ivar.name() {
                super::layout_linear::IVAR => {
                    let ll: &mut LinearLayout = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(ll)
                }
                super::window::IVAR => {
                    let w: &mut Window = mem::transmute(saved as *mut _ as *mut ::std::os::raw::c_void);
                    Some(w)
                }
                _ => None,
            }
        }
    }
}

pub unsafe trait CocoaControl: UiMember {
    unsafe fn on_added_to_container(&mut self, &CocoaContainer, x: u16, y: u16);
    unsafe fn on_removed_from_container(&mut self, &CocoaContainer);
    unsafe fn base(&mut self) -> &mut CocoaControlBase;
}

pub unsafe trait CocoaContainer: UiContainer + UiMember {
    unsafe fn cocoa_id(&self) -> cocoa_id;
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
        _ => {
            unimplemented!();
        }
    }
}

/*pub unsafe fn cast_uicontrol_to_cocoa(input: &Box<UiControl>) -> &CocoaControl {
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
        _ =>{
	        unimplemented!();
        }
    }
}*/

pub unsafe fn measure_string(text: &str) -> (u16, u16) {
    let title = NSString::alloc(cocoa::base::nil).init_str(text);

    let text_storage: cocoa_id = msg_send![class("NSTextStorage"), alloc];
    let text_storage: cocoa_id = msg_send![text_storage, initWithString: title];
    let layout_manager: cocoa_id = msg_send![class("NSLayoutManager"), alloc];
    let layout_manager: cocoa_id = msg_send![layout_manager, init];
    let text_container: cocoa_id = msg_send![class("NSTextContainer"), alloc];
    let text_container: cocoa_id = msg_send![text_container, init];

    msg_send![layout_manager, addTextContainer: text_container];
    msg_send![text_container, release];
    msg_send![text_storage, addLayoutManager: layout_manager];
    msg_send![layout_manager, release];

    let num = msg_send![layout_manager, numberOfGlyphs];
    let range = NSRange::new(0, num);

    let string_rect: NSRect = msg_send![layout_manager, boundingRectForGlyphRange:range inTextContainer:text_container];
    (string_rect.size.width as u16, string_rect.size.height as u16)
}
