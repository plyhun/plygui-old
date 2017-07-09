use super::*;
use super::common::*;

use {UiRole, UiWindow, UiControl, UiMember, UiContainer};

use std::{ptr, mem, str};
use std::os::raw::c_void;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

lazy_static! {
	pub static ref WINDOW_CLASS: Vec<u16> = unsafe { register_window_class() };
	//pub static ref INSTANCE: winapi::HINSTANCE = unsafe { kernel32::GetModuleHandleW(ptr::null()) };
}

#[repr(C)]
pub struct Window {
    hwnd: winapi::HWND,
    child: Option<Box<UiControl>>,
    h_resize: Option<Box<FnMut(&mut UiMember, u16, u16)>>,
}

impl Window {
    pub fn new(title: &str, width: u16, height: u16, has_menu: bool) -> Box<Window> {
        unsafe {
            let mut rect = winapi::RECT {
                left: 0,
                top: 0,
                right: width as i32,
                bottom: height as i32,
            };
            let style = winapi::WS_OVERLAPPEDWINDOW;
            let exstyle = winapi::WS_EX_APPWINDOW;

            user32::AdjustWindowRectEx(&mut rect, style, winapi::FALSE, exstyle);
            let window_name = OsStr::new(title)
                .encode_wide()
                .chain(Some(0).into_iter())
                .collect::<Vec<_>>();

            let mut w = Box::new(Window {
                                     hwnd: 0 as winapi::HWND,
                                     child: None,
                                     h_resize: None,
                                 });

            if INSTANCE as usize == 0 {
                INSTANCE = kernel32::GetModuleHandleW(ptr::null());
            }

            let hwnd = user32::CreateWindowExW(exstyle,
                                               WINDOW_CLASS.as_ptr(),
                                               window_name.as_ptr() as winapi::LPCWSTR,
                                               style | winapi::WS_VISIBLE,
                                               winapi::CW_USEDEFAULT,
                                               winapi::CW_USEDEFAULT,
                                               rect.right - rect.left,
                                               rect.bottom - rect.top,
                                               ptr::null_mut(),
                                               ptr::null_mut(),
                                               INSTANCE,
                                               w.as_mut() as *mut _ as *mut c_void);

            w.hwnd = hwnd;
            w
        }
    }
}

impl UiWindow for Window {
    fn start(&mut self) {
        loop {
            unsafe {
                let mut msg: winapi::MSG = mem::zeroed();
                if user32::GetMessageW(&mut msg, ptr::null_mut(), 0, 0) <= 0 {
                    break;
                } else {
                    user32::TranslateMessage(&mut msg);
                    user32::DispatchMessageW(&mut msg);
                }
            }
        }
    }
}

impl UiContainer for Window {
	fn set_child(&mut self, mut child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
        unsafe {
            let mut old = self.child.take();
            if let Some(old) = old.as_mut() {
                let mut wc = common::cast_uicontrol_to_windows(old);
                wc.on_removed_from_container(self);
            }
            if let Some(new) = child.as_mut() {
                let mut wc = common::cast_uicontrol_to_windows(new);
                wc.on_added_to_container(self,0,0); //TODO padding

            }
            self.child = child;

            old
        }
    }
	fn child(&self) -> Option<&Box<UiControl>> {
		self.child.as_ref()
	}
	fn child_mut(&mut self) -> Option<&mut Box<UiControl>> {
		self.child.as_mut()
	}
}

impl UiMember for Window {
    fn show(&mut self) {
        unsafe {
            user32::ShowWindow(self.hwnd, winapi::SW_SHOW);
        }
    }
    fn hide(&mut self) {
        unsafe {
            user32::ShowWindow(self.hwnd, winapi::SW_HIDE);
        }
    }
    fn size(&self) -> (u16, u16) {
    	let rect = unsafe { window_rect(self.hwnd) };
    	((rect.right-rect.left) as u16, (rect.bottom-rect.top) as u16)
    } 
    
    fn on_resize(&mut self, handler: Option<Box<FnMut(&mut UiMember, u16, u16)>>) {
        self.h_resize = handler;
    }

    fn role<'a>(&'a mut self) -> UiRole<'a> {
        UiRole::Window(self)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        self.set_child(None);
        self.hide();
        destroy_hwnd(self.hwnd, 0, None);
    }
}

unsafe impl WindowsContainer for Window {
    unsafe fn hwnd(&self) -> winapi::HWND {
        self.hwnd
    }
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name = OsStr::new("NativeUiWindowClass")
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>();

    let class = winapi::WNDCLASSEXW {
        cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
        style: winapi::CS_DBLCLKS,
        lpfnWndProc: Some(handler),
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

unsafe extern "system" fn handler(hwnd: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> winapi::LRESULT {
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
            let width = lparam as u16;
            let height = (lparam >> 16) as u16;
            let mut w: &mut window::Window = mem::transmute(ww);
            
            if let Some(ref mut child) = w.child {
            	child.measure(width, height);
            	child.draw(0,0); //TODO padding
            }
            
            if let Some(ref mut cb) = w.h_resize {
                let w2: &mut Window = mem::transmute(user32::GetWindowLongPtrW(hwnd, winapi::GWLP_USERDATA));
                (cb)(w2, width, height);
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
