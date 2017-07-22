use super::*;
use super::common::*;

use self::cocoa::appkit::NSView;
use self::cocoa::foundation::{NSRect, NSSize, NSPoint};
use self::cocoa::base::id;
use objc::runtime::{Class, Object};
use objc::declare::ClassDecl;

use std::mem;
use std::os::raw::c_void;

lazy_static! {
	static ref WINDOW_CLASS: RefClass = unsafe { register_window_class() };
}

use {layout, UiRole, UiRoleMut, UiControl, UiMember, UiContainer, UiMultiContainer, UiLinearLayout, Visibility};

#[repr(C)]
pub struct LinearLayout {
    base: CocoaControlBase,
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
    	self.base.visibility = visibility;
    }
    fn visibility(&self) -> Visibility {
    	self.base.visibility
    }
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
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
    	let mut x = x;
    	let mut y = y;
    	let my_h = self.size().1;
		for mut child in self.children.as_mut_slice() {
        	let child_size = child.size();
        	match self.orientation {
        		layout::Orientation::Horizontal => {
        			child.draw(x, y);
        			x += child_size.0
        		},
        		layout::Orientation::Vertical => {
        			child.draw(x, my_h-y-child_size.1);
        			y += child_size.1
        		},
        	}
        }
        if let Some(ref mut cb) = self.base.h_resize {
            unsafe {
            	let object: &Object = mem::transmute(self.base.control);
	            let saved: *mut c_void = *object.get_ivar("nativeUiLinearLayout");
	            let mut ll2: &mut LinearLayout = mem::transmute(saved);
	            (cb)(ll2,
	                 self.base.measured_size.0,
	                 self.base.measured_size.1);
            }
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
        if w == parent_width || h == parent_height {
        	let mut ww = 0;
	        let mut hh = 0;
	        for ref mut child in self.children.as_mut_slice() {
		        let (cw, ch) = child.measure(w, h);
		        ww += cw;
		        hh += ch;
		    }
	        match self.orientation {
	            layout::Orientation::Vertical => {
	                if let layout::Params::WrapContent = self.base.layout_height {
	                    h = hh;
	                }
	            }
	            layout::Orientation::Horizontal => {
	                if let layout::Params::WrapContent = self.base.layout_width {
	                    w = ww;
	                }
	            }
	        }
        }
        self.base.measured_size = (w, h);
        self.base.measured_size
    }
}

impl UiMultiContainer for LinearLayout {
    fn push_child(&mut self, child: Box<UiControl>) {
    	let len = self.children.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<UiControl>> {
    	let len = self.children.len();
        self.remove_child_from(len-1)
    }
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, index: usize, mut new: Box<UiControl>) -> Option<Box<UiControl>> {
        let old = self.remove_child_from(index);
        
        if !self.base.control.is_null() {
        	let (x,y) = {
        		let mut x = 0;
        		let mut y = 0;
        		for ref child in self.children.as_slice() {
        			let (xx, yy) = child.size();
		            match self.orientation {
		                layout::Orientation::Horizontal => x += xx,
		                layout::Orientation::Vertical => y += yy,
		            }
        		}
        		(x, y)
        	};
        	unsafe {
	        	let mut wc = common::cast_uicontrol_to_cocoa_mut(&mut new);
		        match self.orientation {
	        		layout::Orientation::Horizontal => {
	        			wc.on_added_to_container(self,x,y); //TODO padding
			        },
	        		layout::Orientation::Vertical => {
	        			let my_h = self.size().1;
					    wc.on_added_to_container(self,x,my_h-y); //TODO padding		        
	        		},
	        	}
				self.base.control.addSubview_(wc.base().control);
	        }
        }
        self.children.insert(index, new);
         
        old
    }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>> {
    	if index >= self.children.len() {
    		return None;
    	}
    	let mut child = self.children.remove(index);
    	unsafe {
    		let mut wc = common::cast_uicontrol_to_cocoa_mut(&mut child);
	        wc.on_removed_from_container(self);
    	}
        
    	Some(child)
    }
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>> {
        self.children.get(index)
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>> {
        self.children.get_mut(index)
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
    fn child(&self) -> Option<&Box<UiControl>> {
        self.children.get(0)
    }
    fn child_mut(&mut self) -> Option<&mut Box<UiControl>> {
        self.children.get_mut(0)
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

unsafe impl CocoaContainer for LinearLayout {
	unsafe fn id(&self) -> id {
		self.base.control
	}
}

unsafe impl CocoaControl for LinearLayout {
    unsafe fn on_added_to_container(&mut self, parent: &common::CocoaContainer, x: u16, y: u16) {
        let (pw, ph) = parent.size();
        let (w, h) = self.measure(pw, ph);

        let rect = NSRect::new(NSPoint::new(x as f64, y as f64),
                               NSSize::new(w as f64, h as f64));

        let base: id = msg_send![WINDOW_CLASS.0, alloc];
        let base: id = msg_send![base, initWithFrame: rect];

        self.base.control = msg_send![base, autorelease];
        (&mut *self.base.control).set_ivar("nativeUiLinearLayout",
                                           self as *mut _ as *mut ::std::os::raw::c_void);
        
        let mut x = 0;
        let mut y = 0;
        let ll2: &LinearLayout = mem::transmute(self as *mut _ as *mut ::std::os::raw::c_void);
        for ref mut child in self.children.as_mut_slice() {
        	let mut wc = common::cast_uicontrol_to_cocoa_mut(child);
	        let (xx, yy) = wc.size();
            match self.orientation {
                layout::Orientation::Horizontal => {
                	wc.on_added_to_container(ll2, x, y);
			        x += xx;
                },
                layout::Orientation::Vertical => {
                	wc.on_added_to_container(ll2, x, h-y-yy);
			        y += yy;
                },
            }
	        ll2.base.control.addSubview_(wc.base().control);
        }
    }
    unsafe fn on_removed_from_container(&mut self, _: &common::CocoaContainer) {
    	let ll2: &LinearLayout = mem::transmute(self as *mut _ as *mut ::std::os::raw::c_void);
        for ref mut child in self.children.as_mut_slice() {
        	let mut wc = common::cast_uicontrol_to_cocoa_mut(child);
	        wc.on_removed_from_container(ll2);
        }
    	self.base.on_removed_from_container();
    }

    unsafe fn base(&mut self) -> &mut common::CocoaControlBase {
        &mut self.base
    }
}

unsafe fn register_window_class() -> RefClass {
    let superclass = Class::get("NSView").unwrap();
    let mut decl = ClassDecl::new("NativeUiLinearLayout", superclass).unwrap();

    decl.add_ivar::<*mut c_void>("nativeUiLinearLayout");

    RefClass(decl.register())
}