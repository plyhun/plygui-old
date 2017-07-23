use super::*;
use super::common::WindowsContainer;

use std::{mem, thread};

use UiApplication;

pub struct Application {
	name: String,
	windows: Vec<winapi::HWND>,
}

impl UiApplication for Application {
	fn new_window(&mut self, title: &str, width: u16, height: u16, has_menu: bool) -> Box<Window> {
		let w = Window::new(title, width, height, has_menu);
		unsafe { self.windows.push(w.hwnd()); }
		w
	}
	fn name(&self) -> &str {
		self.name.as_str()
	}
	fn start(&mut self) {
		for i in (0..self.windows.len()).rev() {
			if i > 0 {
				thread::spawn(move || {
				
				});
			} else {
				start_window(self.windows[i]);
			}
		}
	}	
}

impl Application {
	pub fn with_name(name: &str) -> Box<Application> {
		Box::new(Application {
	        name: name.into(),
	        windows: Vec::with_capacity(1),
        })
	}	
}

fn start_window(hwnd: winapi::HWND) {
	let w: &mut Window = unsafe { mem::transmute(user32::GetWindowLongPtrW(hwnd, winapi::GWLP_USERDATA)) };
    w.start();            
}