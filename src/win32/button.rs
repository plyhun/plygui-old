use super::*;

use {layout, UiRole, UiControl, UiButton, UiMember};

use std::{ptr, mem, str};
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

lazy_static! {
	static ref WINDOW_CLASS: Vec<u16> = OsStr::new("button")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();
}

#[repr(C)]
pub struct Button {
    base: common::WindowsControlBase,
    label: String,
    h_left_clicked: Option<Box<FnMut(&mut UiButton)>>,
}

impl Button {
    pub fn new(label: &str) -> Box<Button> {
        let b = Box::new(Button {
                             base: Default::default(),
                             h_left_clicked: None,
                             label: label.to_owned(),
                         });

        b
    }
}

impl UiButton for Button {
    fn on_left_click(&mut self, handle: Option<Box<FnMut(&mut UiButton)>>) {
        self.h_left_clicked = handle;
    }
    fn label(&self) -> &str {
        self.label.as_ref()
    }
}

impl UiControl for Button {
    fn layout_params(&self) -> (layout::Params, layout::Params) {
        (self.base.layout_width, self.base.layout_height)
    }
    fn set_layout_params(&mut self, wp: layout::Params, hp: layout::Params) {
    	self.base.layout_width = wp;
    	self.base.layout_height = hp;
    }
    fn draw(&mut self, x: u16, y: u16) {
        unsafe {
            user32::SetWindowPos(self.base.hwnd,
                                 ptr::null_mut(),
                                 x as i32,
                                 y as i32,
                                 self.base.measured_size.0 as i32,
                                 self.base.measured_size.1 as i32,
                                 0);
        }
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16) {
        unsafe {
        	let mut label_size: winapi::SIZE = mem::zeroed();
	        let w = match self.base.layout_width {
	            layout::Params::MatchParent => parent_width,
	            layout::Params::Exact(w) => w,
	            layout::Params::WrapContent => {
	                if label_size.cx < 1 {
	                    let label = OsStr::new(self.label.as_str())
	                        .encode_wide()
	                        .chain(Some(0).into_iter())
	                        .collect::<Vec<_>>();
	                    gdi32::GetTextExtentPointW(user32::GetDC(self.base.hwnd),
	                                               label.as_ptr(),
	                                               self.label.len() as i32,
	                                               &mut label_size);
	                }
	                label_size.cx as u16
	            } 
	        };
	        let h = match self.base.layout_height {
	            layout::Params::MatchParent => parent_height,
	            layout::Params::Exact(h) => h,
	            layout::Params::WrapContent => {
	                if label_size.cy < 1 {
	                    let label = OsStr::new(self.label.as_str())
	                        .encode_wide()
	                        .chain(Some(0).into_iter())
	                        .collect::<Vec<_>>();
	                    gdi32::GetTextExtentPointW(user32::GetDC(self.base.hwnd),
	                                               label.as_ptr(),
	                                               self.label.len() as i32,
	                                               &mut label_size);
	                }
	                label_size.cy as u16
	            } 
	        };
	        let ret = (w, h);
	        self.base.measured_size = ret;
	        ret
        }
    }
}

impl UiMember for Button {
    fn show(&mut self) {
        unsafe {
            user32::ShowWindow(self.base.hwnd, winapi::SW_SHOW);
        }
    }
    fn hide(&mut self) {
        unsafe {
            user32::ShowWindow(self.base.hwnd, winapi::SW_HIDE);
        }
    }
    fn size(&self) -> (u16, u16) {
        let rect = unsafe { common::window_rect(self.base.hwnd) };
        ((rect.right - rect.left) as u16, (rect.bottom - rect.top) as u16)
    }

    fn on_resize(&mut self, handler: Option<Box<FnMut(&mut UiMember, u16, u16)>>) {
        self.base.h_resize = handler;
    }

    fn role<'a>(&'a mut self) -> UiRole<'a> {
        UiRole::Button(self)
    }
}

impl Drop for Button {
    fn drop(&mut self) {
        self.hide();
        common::destroy_hwnd(self.base.hwnd, 0, None);
    }
}

unsafe impl common::WindowsControl for Button {
    unsafe fn on_added_to_container(&mut self, parent: &common::WindowsContainer, px: u16, py: u16) {
        let selfptr = self as *mut _ as *mut c_void;
        let (pw, ph) = parent.size();
        self.base.hwnd = parent.hwnd(); // required for measure, as we don't have own hwnd yet
        let (w, h) = self.measure(pw, ph);
        let (hwnd, id) = common::create_control_hwnd(px as i32,
                                                     py as i32,
                                                     w as i32,
                                                     h as i32,
                                                     parent.hwnd(),
                                                     0,
                                                     WINDOW_CLASS.as_ptr(),
                                                     self.label.as_str(),
                                                     winapi::BS_PUSHBUTTON | winapi::WS_TABSTOP,
                                                     selfptr,
                                                     Some(handler));
        self.base.hwnd = hwnd;
        self.base.subclass_id = id;
    }
    unsafe fn on_removed_from_container(&mut self, _: &common::WindowsContainer) {
        common::destroy_hwnd(self.base.hwnd, self.base.subclass_id, Some(handler));
        self.base.hwnd = 0 as winapi::HWND;
        self.base.subclass_id = 0;
    }
}

unsafe extern "system" fn handler(hwnd: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM, _: u64, param: u64) -> i64 {
    let mut button: &mut Button = mem::transmute(param);
    match msg {
        winapi::WM_LBUTTONDOWN => {
            if let Some(ref mut cb) = button.h_left_clicked {
                let mut button2: &mut Button = mem::transmute(param);
                (cb)(button2);
            }
        }
        winapi::WM_SIZE => {
            let width = lparam as u16;
            let height = (lparam >> 16) as u16;

            if let Some(ref mut cb) = button.base.h_resize {
                let mut button2: &mut Button = mem::transmute(param);
                (cb)(button2, width, height);
            }
        }
        _ => {}
    }

    comctl32::DefSubclassProc(hwnd, msg, wparam, lparam)
}
