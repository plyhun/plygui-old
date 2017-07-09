use super::*;

use std::{ptr, mem, str};
use self::cocoa::base::id;

use {layout, UiContainer, UiMember, UiControl, UiRole};

#[repr(C)]
pub struct CocoaControlBase {
	pub control: id,
    pub layout_width: layout::Params,
    pub layout_height: layout::Params,
    pub measured_size: (u16, u16),

    pub h_resize: Option<Box<FnMut(&mut UiMember, u16, u16)>>,
}

impl Default for CocoaControlBase {
    fn default() -> CocoaControlBase {
        CocoaControlBase {
        	control: ptr::null_mut(),
            h_resize: None,
            layout_width: layout::Params::MatchParent,
            layout_height: layout::Params::WrapContent,
            measured_size: (0, 0),
        }
    }
}

pub unsafe trait CocoaControl: UiMember {
    unsafe fn on_added_to_container(&mut self, &CocoaContainer, x: u16, y: u16);
    unsafe fn on_removed_from_container(&mut self, &CocoaContainer);
    unsafe fn base(&mut self) -> &mut CocoaControlBase;
    //unsafe fn measure(&mut self, hwnd: winapi::HWND, parent_width: u16, parent_height: u16) -> (u16, u16);
}

pub unsafe trait CocoaContainer: UiContainer + UiMember {
    unsafe fn id(&self) -> id;
}



pub unsafe fn cast_uicontrol_to_cocoa(input: &mut Box<UiControl>) -> &mut CocoaControl {
    use std::ops::DerefMut;
    match input.role() {
        UiRole::Button(_) => {
            let a: &mut Box<button::Button> = mem::transmute(input);
            a.deref_mut()
        }
        //UiRole::LinearLayout(_) => {
        //    let a: &mut Box<layout_linear::LinearLayout> = mem::transmute(input);
        //    a.deref_mut()
        //}
        UiRole::Window(_) => {
            panic!("Window as a container child is impossible!");
        }
        _=>{
	        unimplemented!();
        }
    }
}
