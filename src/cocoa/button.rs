use super::*;

use self::cocoa::appkit::{NSBezelStyle, NSButton, NSEvent};
use self::cocoa::foundation::{NSString, NSAutoreleasePool, NSRect, NSSize, NSPoint};
use self::cocoa::base::{id, nil};
use objc::runtime::{Class, Object, Sel, BOOL, YES, NO};
use objc::declare::ClassDecl;

use std::mem;
use std::os::raw::c_void;

struct RefClass(*const Class);
unsafe impl Sync for RefClass {}

lazy_static! {
	static ref WINDOW_CLASS: RefClass = unsafe { register_window_class() };
}

use {layout, UiRole, UiControl, UiButton, UiMember};

#[repr(C)]
pub struct Button {
    base: common::CocoaControlBase,

    label: String,
    h_left_clicked: Option<Box<FnMut(&mut UiButton)>>,
    h_right_clicked: Option<Box<FnMut(&mut UiButton)>>,
}

impl Button {
    pub fn new(label: &str) -> Box<Button> {
        Box::new(Button {
            base: Default::default(),
            label: label.to_owned(),
            h_left_clicked: None,
            h_right_clicked: None,
        })
    }
}

impl UiButton for Button {
    fn label(&self) -> &str {
        self.label.as_ref()
    }
    fn on_left_click(&mut self, cb: Option<Box<FnMut(&mut UiButton)>>) {
        self.h_left_clicked = cb;
    }
    /*fn on_right_click(&mut self, cb: Option<Box<FnMut(&mut UiButton)>>) {
        self.h_right_clicked = cb;
    }*/
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
            let mut frame: NSRect = msg_send![self.base.control, frame];
            frame.size = NSSize::new(self.base.measured_size.0 as f64,
                                     self.base.measured_size.1 as f64);
            frame.origin = NSPoint::new(x as f64, y as f64);
            msg_send![self.base.control, setFrame: frame];

            if let Some(ref mut cb) = self.base.h_resize {
                let object: &Object = mem::transmute(self.base.control);
                let saved: *mut c_void = *object.get_ivar("nativeUiButton");
                let mut button2: &mut Button = mem::transmute(saved);
                (cb)(button2,
                     self.base.measured_size.0,
                     self.base.measured_size.1);
            }
        }
    }
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16) {
        self.base.measured_size = (w, h);
        self.base.measured_size
    }
}

impl UiMember for Button {
    fn show(&mut self) {}
    fn hide(&mut self) {}
    fn size(&self) -> (u16, u16) {
        self.base.measured_size
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
        //unsafe { msg_send![self.base.control, dealloc]; }
    }
}

unsafe impl common::CocoaControl for Button {
    unsafe fn on_added_to_container(&mut self, parent: &common::CocoaContainer, x: u16, y: u16) {
        let (pw, ph) = parent.size();
        let (w, h) = self.measure(pw, ph);

        let rect = NSRect::new(NSPoint::new(x as f64, y as f64),
                               NSSize::new(w as f64, h as f64));

        let base: id = msg_send![WINDOW_CLASS.0, alloc];
        let base: id = msg_send![base, initWithFrame: rect];

        self.base.control = msg_send![base, autorelease];

        let title = NSString::alloc(cocoa::base::nil).init_str(self.label.as_ref());
        self.base.control.setTitle_(title);
        self.base.control.setBezelStyle_(NSBezelStyle::NSRoundedBezelStyle);

        (&mut *self.base.control).set_ivar("nativeUiButton",
                                           self as *mut _ as *mut ::std::os::raw::c_void);
    }
    unsafe fn on_removed_from_container(&mut self, _: &common::CocoaContainer) {
        msg_send![self.base.control, dealloc];
    }

    unsafe fn base(&mut self) -> &mut common::CocoaControlBase {
        &mut self.base
    }
}

unsafe fn register_window_class() -> RefClass {
    let superclass = Class::get("NSButton").unwrap();
    let mut decl = ClassDecl::new("NativeUiButton", superclass).unwrap();

    decl.add_method(sel!(mouseDown:),
                    button_left_click as extern "C" fn(&Object, Sel, id));
    decl.add_method(sel!(rightMouseDown:),
                    button_right_click as extern "C" fn(&Object, Sel, id));
    decl.add_ivar::<*mut c_void>("nativeUiButton");

    RefClass(decl.register())
}

extern "C" fn button_left_click(this: &Object, _: Sel, param: id) {
    unsafe {
        let saved: *mut c_void = *this.get_ivar("nativeUiButton");
        let button: &mut Button = mem::transmute(saved.clone());
        if let Some(ref mut cb) = button.h_left_clicked {
            let b2: &mut Button = mem::transmute(saved);
            (cb)(b2);
        }
        msg_send![super(button.base.control, Class::get("NSButton").unwrap()), mouseDown: param];
    }
}
extern "C" fn button_right_click(this: &Object, _: Sel, param: id) {
    println!("right!");
    unsafe {
        let saved: *mut c_void = *this.get_ivar("nativeUiButton");
        let button: &mut Button = mem::transmute(saved.clone());
        if let Some(ref mut cb) = button.h_right_clicked {
            let b2: &mut Button = mem::transmute(saved);
            (cb)(b2);
        }
        msg_send![super(button.base.control, Class::get("NSButton").unwrap()), rightMouseDown: param];
    }
}
