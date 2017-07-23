use super::*;
use super::common::*;

use {layout, UiRole, UiRoleMut, UiControl, UiMember, UiContainer, UiMultiContainer, UiLinearLayout, Visibility};

use std::{ptr, mem};
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

lazy_static! {
	pub static ref WINDOW_CLASS: Vec<u16> = unsafe { register_window_class() };
	//pub static ref INSTANCE: winapi::HINSTANCE = unsafe { kernel32::GetModuleHandleW(ptr::null()) };
}

#[repr(C)]
pub struct LinearLayout {
    base: WindowsControlBase,
    orientation: layout::Orientation,
    children: Vec<Box<UiControl>>,
}

impl LinearLayout {
    pub fn new(orientation: layout::Orientation) -> Box<LinearLayout> {
        Box::new(LinearLayout {
                     base: Default::default(),
                     orientation: orientation,
                     children: Vec::new(),
                 })
    }
}

impl UiMember for LinearLayout {
    fn set_visibility(&mut self, visibility: Visibility) {
    	self.base.set_visibility(visibility);
    }
    fn visibility(&self) -> Visibility {
    	self.base.visibility()
    }
    
    fn id(&self) -> Id {
    	self.base.hwnd
    }
    fn size(&self) -> (u16, u16) {
        let rect = unsafe { window_rect(self.base.hwnd) };
        ((rect.right - rect.left) as u16, (rect.bottom - rect.top) as u16)
    }

    fn on_resize(&mut self, handler: Option<Box<FnMut(&mut UiMember, u16, u16)>>) {
        self.base.h_resize = handler;
    }

    fn role<'a>(&'a self) -> UiRole<'a> {
        UiRole::LinearLayout(self)
    }
    fn role_mut<'a>(&'a mut self) -> UiRoleMut<'a> {
        UiRoleMut::LinearLayout(self)
    }
}

impl UiControl for LinearLayout {
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
        let mut w = parent_width;
        let mut h = parent_height;

        if let layout::Params::Exact(ew) = self.base.layout_width {
            w = ew;
        }
        if let layout::Params::Exact(eh) = self.base.layout_height {
            w = eh;
        }
        match self.orientation {
            layout::Orientation::Vertical => {
                if let layout::Params::WrapContent = self.base.layout_height {
                    let mut hh = 0;
                    for ref mut child in self.children.as_mut_slice() {
                        let (_, ch) = child.measure(w, h);
                        hh += ch;
                    }
                    h = hh;
                }
            }
            layout::Orientation::Horizontal => {
                if let layout::Params::WrapContent = self.base.layout_width {
                    let mut ww = 0;
                    for ref mut child in self.children.as_mut_slice() {
                        let (cw, _) = child.measure(w, h);
                        ww += cw;
                    }
                    w = ww;
                }
            }
        }
        let ret = (w, h);
        self.base.measured_size = ret;
        ret
    }
    fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
    	Some(self)
    }
    fn is_container(&self) -> Option<&UiContainer> {
    	Some(self)
    }
    
    fn parent(&self) -> Option<&UiContainer> {
    	None
    }
    fn parent_mut(&mut self) -> Option<&mut UiContainer> {
    	None
    }
    fn root(&self) -> Option<&UiContainer> {
    	None
    }
    fn root_mut(&mut self) -> Option<&mut UiContainer> {
    	None
    }
}

impl UiContainer for LinearLayout {
    fn set_child(&mut self, child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
        let old = if let Some(child) = child {
            self.children.push(child);
            None
        } else {
            let old = self.children.pop();
            self.children.clear();
            old
        };
        old
    }
    fn child(&self) -> Option<&UiControl> {
        self.children.get(0).map(|c|c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut UiControl> {
        //self.children.get_mut(0).map(|c|c.as_mut()) // WTF??
        if self.children.len() > 0 {
        	Some(self.children[0].as_mut())
        } else {
        	None
        }
    }
    fn find_control_by_id_mut(&mut self, id_: Id) -> Option<&mut UiControl> {
    	if self.id() == id_ {
    		return Some(self);
    	}
    	for child in self.children.as_mut_slice() {
    		if child.id() == id_ {
    			return Some(child.as_mut());
    		} else if let Some(c) = child.is_container_mut() {
    			let ret = c.find_control_by_id_mut(id_);
    			if ret.is_none() {
    				continue;
    			}
    			return ret;
    		}
    	}
    	None
    }
	fn find_control_by_id(&self, id_: Id) -> Option<&UiControl> {
		if self.id() == id_ {
    		return Some(self);
    	}
    	for child in self.children.as_slice() {
    		if child.id() == id_ {
    			return Some(child.as_ref());
    		} else if let Some(c) = child.is_container() {
    			let ret = c.find_control_by_id(id_);
    			if ret.is_none() {
    				continue;
    			}
    			return ret;
    		}
    	}
    	None
	}
	fn is_multi_mut(&mut self) -> Option<&mut UiMultiContainer> {
		Some(self)
	}
	fn is_multi(&self) -> Option<&UiMultiContainer> {
		Some(self)
	}
}

impl UiMultiContainer for LinearLayout {
    fn push_child(&mut self, child: Box<UiControl>) {
        self.children.push(child)
    }
    fn pop_child(&mut self) -> Option<Box<UiControl>> {
        self.children.pop()
    }
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, index: usize, child: Box<UiControl>) -> Option<Box<UiControl>> {
        //TODO yes this is ineffective, need a way to swap old item with new
        self.children.insert(index, child);
        if (index + 1) >= self.children.len() {
            return None;
        }
        Some(self.children.remove(index + 1))
    }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>> {
    	if index < self.children.len() {
    		Some(self.children.remove(index))
    	} else {
    		None
    	}
    }
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>> {
        self.children.get(index)
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>> {
        self.children.get_mut(index)
    }
}

impl UiLinearLayout for LinearLayout {
    fn orientation(&self) -> layout::Orientation {
        self.orientation
    }
    fn set_orientation(&mut self, orientation: layout::Orientation) {
        self.orientation = orientation;
    }
}

unsafe impl WindowsContainer for LinearLayout {
    unsafe fn hwnd(&self) -> winapi::HWND {
        self.base.hwnd
    }
}

unsafe impl WindowsControl for LinearLayout {
    unsafe fn on_added_to_container(&mut self, parent: &WindowsContainer, px: u16, py: u16) {
        let selfptr = self as *mut _ as *mut c_void;
        let (pw, ph) = parent.size();
        self.base.hwnd = parent.hwnd(); // required for measure, as we don't have own hwnd yet
        let (width, height) = self.measure(pw, ph);
        let (hwnd, id) = common::create_control_hwnd(px as i32,
                                                     py as i32,
                                                     width as i32,
                                                     height as i32,
                                                     parent.hwnd(),
                                                     winapi::WS_EX_CONTROLPARENT,
                                                     WINDOW_CLASS.as_ptr(),
                                                     "",
                                                     0,
                                                     selfptr,
                                                     None);
        self.base.hwnd = hwnd;
        self.base.subclass_id = id;
        let mut x = 0;
        let mut y = 0;
        for ref mut child in self.children.as_mut_slice() {
            let mut wc = common::cast_uicontrol_to_windows(child);
            let self2: &mut LinearLayout = mem::transmute(selfptr);
            wc.on_added_to_container(self2, x, y);
            let (xx, yy) = wc.size();
            match self.orientation {
                layout::Orientation::Horizontal => x += xx,
                layout::Orientation::Vertical => y += yy,
            }
        }
    }
    unsafe fn on_removed_from_container(&mut self, _: &WindowsContainer) {
        let selfptr = self as *mut _ as *mut c_void;
        for ref mut child in self.children.as_mut_slice() {
            let mut wc = common::cast_uicontrol_to_windows(child);
            let self2: &mut LinearLayout = mem::transmute(selfptr);
            wc.on_removed_from_container(self2);
        }
        destroy_hwnd(self.base.hwnd, self.base.subclass_id, None);
        self.base.hwnd = 0 as winapi::HWND;
        self.base.subclass_id = 0;
    }
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name = OsStr::new("NativeUiContainerClass")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    let class = winapi::WNDCLASSEXW {
        cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
        style: winapi::CS_DBLCLKS,
        lpfnWndProc: Some(whandler),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: kernel32::GetModuleHandleW(ptr::null()),
        hIcon: user32::LoadIconW(ptr::null_mut(), winapi::IDI_APPLICATION),
        hCursor: user32::LoadCursorW(ptr::null_mut(), winapi::IDC_ARROW),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut(),
    };
    user32::RegisterClassExW(&class);
    class_name
}

unsafe extern "system" fn whandler(hwnd: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> winapi::LRESULT {
    let ww = user32::GetWindowLongPtrW(hwnd, winapi::GWLP_USERDATA);
    if ww == 0 {
        if winapi::WM_CREATE == msg {
            let cs: &mut winapi::CREATESTRUCTW = mem::transmute(lparam);
            user32::SetWindowLongPtrW(hwnd, winapi::GWLP_USERDATA, cs.lpCreateParams as i64);
        }
        return user32::DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    match msg {
        winapi::WM_SIZE => {
            let mut width = lparam as u16;
            let mut height = (lparam >> 16) as u16;
            let mut ll: &mut LinearLayout = mem::transmute(ww);
            let o = ll.orientation;

            let mut x = 0;
            let mut y = 0;
            for ref mut child in ll.children.as_mut_slice() {
                let (cw, ch) = child.measure(width, height);
                child.draw(x, y); //TODO padding
                match o {
                    layout::Orientation::Horizontal if width >= cw => {
                        x += cw;
                        width -= cw;
                    }
                    layout::Orientation::Vertical if height >= ch => {
                        y += ch;
                        height -= ch;
                    }
                    _ => {},
                }
            }

            if let Some(ref mut cb) = ll.base.h_resize {
                let mut ll2: &mut LinearLayout = mem::transmute(ww);
                (cb)(ll2, width, height);
            }
        }
        winapi::WM_DESTROY => {
            user32::PostQuitMessage(0);
            return 0;
        }
        /*winapi::WM_NOTIFY => {
        	let hdr: winapi::LPNMHDR = mem::transmute(lparam);
        	println!("notify for {:?}", hdr);
        },
        winapi::WM_COMMAND => {
        	let hdr: winapi::LPNMHDR = mem::transmute(lparam);
        	
        	println!("command for {:?}", hdr);
        }*/
        _ => {}
    }

    user32::DefWindowProcW(hwnd, msg, wparam, lparam)
}
