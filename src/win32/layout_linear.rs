use super::*;
use super::common::*;

use {development, layout, Id, UiRole, UiRoleMut, UiControl, UiMember, UiContainer, UiMultiContainer, UiLinearLayout, Visibility};

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

    fn native_id(&self) -> NativeId {
        self.base.hwnd
    }
    fn id(&self) -> Id {
    	self.base.id()
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
    fn is_control(&self) -> Option<&UiControl> {
    	Some(self)
    }
    fn is_control_mut(&mut self) -> Option<&mut UiControl> {
    	Some(self)
    }     
}

impl UiControl for LinearLayout {
    fn layout_width(&self) -> layout::Size {
    	self.base.layout_width()
    }
	fn layout_height(&self) -> layout::Size {
		self.base.layout_height()
	}
	fn layout_gravity(&self) -> layout::Gravity {
		self.base.layout_gravity()
	}
	fn layout_orientation(&self) -> layout::Orientation {
		self.base.layout_orientation()
	}
	fn layout_alignment(&self) -> layout::Alignment {
		self.base.layout_alignment()
	}
	
	fn set_layout_width(&mut self, width: layout::Size) {
		self.base.set_layout_width(width);
	}
	fn set_layout_height(&mut self, height: layout::Size) {
		self.base.set_layout_height(height);
	}
	fn set_layout_gravity(&mut self, gravity: layout::Gravity) {
		self.base.set_layout_gravity(gravity);
	}
	fn set_layout_orientation(&mut self, orientation: layout::Orientation) {
		self.base.set_layout_orientation(orientation);
	}
	fn set_layout_alignment(&mut self, alignment: layout::Alignment) {
		self.base.set_layout_alignment(alignment);
	}
    fn draw(&mut self, coords: Option<(u16, u16)>) {
    	if coords.is_some() {
    		self.base.coords = coords;
    	}
        if let Some((x, y)) = self.base.coords {
        	unsafe {
	            user32::SetWindowPos(self.base.hwnd,
	                                 ptr::null_mut(),
	                                 x as i32,
	                                 y as i32,
	                                 self.base.measured_size.0 as i32,
	                                 self.base.measured_size.1 as i32,
	                                 0);
	        }
        	let mut x = 0;
	        let mut y = 0;
	        for ref mut child in self.children.as_mut_slice() {
	            child.draw(Some((x, y)));
	            let (xx, yy) = child.size();
	            match self.orientation {
	                layout::Orientation::Horizontal => x += xx,
	                layout::Orientation::Vertical => y += yy,
	            }
	        }
        }
    }
    fn measure(&mut self, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
    	let old_size = self.base.measured_size;
        self.base.measured_size = match self.visibility() {
        	Visibility::Gone => (0,0),
        	_ => {
        		let mut w = parent_width;
		        let mut h = parent_height;
		
		        if let layout::Size::Exact(ew) = self.base.layout_width() {
		            w = ew;
		        }
		        if let layout::Size::Exact(eh) = self.base.layout_height() {
		            w = eh;
		        }
		        match self.orientation {
		            layout::Orientation::Vertical => {
		                if let layout::Size::WrapContent = self.base.layout_height() {
		                    let mut hh = 0;
		                    for ref mut child in self.children.as_mut_slice() {
		                        let (_, ch, _) = child.measure(w, h);
		                        hh += ch;
		                    }
		                    h = hh;
		                }
		            }
		            layout::Orientation::Horizontal => {
		                if let layout::Size::WrapContent = self.base.layout_width() {
		                    let mut ww = 0;
		                    for ref mut child in self.children.as_mut_slice() {
		                        let (cw, _, _) = child.measure(w, h);
		                        ww += cw;
		                    }
		                    w = ww;
		                }
		            }
		        }
		        (w, h)
        	}
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn is_container_mut(&mut self) -> Option<&mut UiContainer> {
        Some(self)
    }
    fn is_container(&self) -> Option<&UiContainer> {
        Some(self)
    }

    fn parent(&self) -> Option<&UiContainer> {
        self.base.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut UiContainer> {
        self.base.parent_mut()
    }
    fn root(&self) -> Option<&UiContainer> {
        self.base.root()
    }
    fn root_mut(&mut self) -> Option<&mut UiContainer> {
        self.base.root_mut()
    }
}

impl UiContainer for LinearLayout {
    fn set_child(&mut self, child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
        let old = self.children.pop();
        self.children.clear();

        if let Some(child) = child {
            self.set_child_to(0, child);
        }

        old
    }
    fn child(&self) -> Option<&UiControl> {
        self.children.get(0).map(|c| c.as_ref())
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
        let (width, height, _) = self.measure(pw, ph);
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
        self.base.coords = Some((px, py));
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
    fn as_base(&self) -> &WindowsControlBase {
    	&self.base
    }
    fn as_base_mut(&mut self) -> &mut WindowsControlBase {
    	&mut self.base
    }    
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name = OsStr::new(development::CLASS_ID_LAYOUT_LINEAR)
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
                let (cw, ch, _) = child.measure(width, height);
                child.draw(Some((x, y))); //TODO padding
                match o {
                    layout::Orientation::Horizontal if width >= cw => {
                        x += cw;
                        width -= cw;
                    }
                    layout::Orientation::Vertical if height >= ch => {
                        y += ch;
                        height -= ch;
                    }
                    _ => {}
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

impl Drop for LinearLayout {
    fn drop(&mut self) {
        self.set_visibility(Visibility::Gone);
        common::destroy_hwnd(self.base.hwnd, 0, None);
    }
}