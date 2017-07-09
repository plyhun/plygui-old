use super::*;

use self::cocoa::appkit::{NSApp, NSApplication, NSWindow, NSRunningApplication, NSApplicationActivationPolicyRegular, NSClosableWindowMask, NSResizableWindowMask, NSMiniaturizableWindowMask, NSTitledWindowMask, NSBackingStoreBuffered};
use self::cocoa::foundation::{NSString, NSAutoreleasePool, NSRect, NSSize, NSPoint};
use self::cocoa::base::{id, nil};
use objc::runtime::{Class, Object, Sel, BOOL, YES, NO};
use objc::declare::ClassDecl;

use std::mem;
use std::os::raw::c_void;

use {UiRole, UiWindow, UiControl, UiMember, UiContainer};

struct RefClass(*const Class);
unsafe impl Sync for RefClass {}

lazy_static! {
	static ref WINDOW_CLASS: RefClass = unsafe { register_window_class() };
}

pub struct Window {
    app: id,
    window: id,
    container: id,

    child: Option<Box<UiControl>>,
    h_resize: Option<Box<FnMut(&mut UiMember, u16, u16)>>,
}

impl Window {
    pub fn new(title: &str, width: u16, height: u16, has_menu: bool) -> Box<Window> {
    	use self::cocoa::appkit::NSView;
    	
        unsafe {
        	let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicyRegular);

            let window = NSWindow::alloc(nil)
                .initWithContentRect_styleMask_backing_defer_(NSRect::new(NSPoint::new(0.0, 0.0),
                                                                          NSSize::new(width as f64, height as f64)),
                                                              NSClosableWindowMask | NSResizableWindowMask | NSMiniaturizableWindowMask | NSTitledWindowMask,
                                                              NSBackingStoreBuffered,
                                                              NO)
                .autorelease();
            window.cascadeTopLeftFromPoint_(NSPoint::new(20., 20.));
            window.center();
            let title = NSString::alloc(cocoa::base::nil).init_str(title);
            window.setTitle_(title);
            window.makeKeyAndOrderFront_(cocoa::base::nil);
            let current_app = cocoa::appkit::NSRunningApplication::currentApplication(cocoa::base::nil);
            current_app.activateWithOptions_(cocoa::appkit::NSApplicationActivateIgnoringOtherApps);
			
			let view = NSView::alloc(nil).initWithFrame_(NSRect::new(NSPoint::new(0.0, 0.0),
                                                                          NSSize::new(width as f64, height as f64))).autorelease();
            window.setContentView_(view);

            let mut window = Box::new(Window {
                app: app,
                window: window,
                container: view,

                child: None,
                h_resize: None,
            });

            let delegate: *mut Object = msg_send!(WINDOW_CLASS.0, new);
			(&mut *delegate).set_ivar("nativeUiWindow", window.as_mut() as *mut _ as *mut ::std::os::raw::c_void);
            window.window.setDelegate_(delegate);
            
            window
        }
    }
}

impl UiWindow for Window {
    fn start(&mut self) {
        use self::cocoa::appkit::NSApplication;
        unsafe { self.app.run() };
    }
}

impl UiContainer for Window {
    fn set_child(&mut self, mut child: Option<Box<UiControl>>) -> Option<Box<UiControl>> {
    	use self::cocoa::appkit::NSView;
    	
        unsafe {
            let mut old = self.child.take();
            if let Some(old) = old.as_mut() {
                let mut wc = common::cast_uicontrol_to_cocoa(old);
                wc.on_removed_from_container(self);
                self.container.removeFromSuperview();
            }
            if let Some(new) = child.as_mut() {
                let mut wc = common::cast_uicontrol_to_cocoa(new);
                wc.on_added_to_container(self,0,0); //TODO padding
				self.container.addSubview_(wc.base().control);
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
    fn show(&mut self) {}
    fn hide(&mut self) {}
    fn size(&self) -> (u16, u16) {
        unsafe {
            let size = self.window.contentView().frame().size;
            (size.width as u16, size.height as u16)
        }
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
		unsafe { 
			msg_send![self.container, dealloc]; 
			msg_send![self.window, dealloc]; 
			msg_send![self.app, dealloc]; 
		}
	}
}

unsafe impl common::CocoaContainer for Window {
	unsafe fn id(&self) -> id {
		self.window
	}
}

unsafe fn register_window_class() -> RefClass {
	let superclass = Class::get("NSObject").unwrap();
    let mut decl = ClassDecl::new("NativeUiWindowDelegate", superclass).unwrap();

    decl.add_method(sel!(windowShouldClose:), window_should_close as extern "C" fn(&Object, Sel, id) -> BOOL);
    decl.add_method(sel!(windowDidResize:), window_did_resize as extern "C" fn(&Object, Sel, id));
    decl.add_method(sel!(windowDidChangeScreen:), window_did_change_screen as extern "C" fn(&Object, Sel, id));

    //decl.add_method(sel!(windowDidBecomeKey:), window_did_become_key as extern "C" fn(&Object, Sel, id));
    //decl.add_method(sel!(windowDidResignKey:), window_did_resign_key as extern "C" fn(&Object, Sel, id));

    decl.add_ivar::<*mut c_void>("nativeUiWindow");

    RefClass(decl.register())
}

extern "C" fn window_did_resize(this: &Object, _: Sel, _: id) {
    unsafe {
    	
        let saved: *mut c_void = *this.get_ivar("nativeUiWindow");
        let window: &mut Window = mem::transmute(saved.clone());
        let size = window.window.contentView().frame().size;
            
        if let Some(ref mut child) = window.child {
        	child.measure(size.width as u16, size.height as u16);
        	child.draw(0,0); //TODO padding
        }
        
        if let Some(ref mut cb) = window.h_resize {
        	let w2: &mut Window = mem::transmute(saved);
            (cb)(w2, size.width as u16, size.height as u16);
        }
    }
}

extern "C" fn window_did_change_screen(this: &Object, _: Sel, _: id) {
    unsafe {
    	
        let saved: *mut c_void = *this.get_ivar("nativeUiWindow");
        let window: &mut Window = mem::transmute(saved.clone());
        if let Some(ref mut cb) = window.h_resize {
        	let size = window.window.contentView().frame().size;
            let w2: &mut Window = mem::transmute(saved);
            (cb)(w2, size.width as u16, size.height as u16);
        }
    }
}
extern "C" fn window_should_close(this: &Object, _: Sel, _: id) -> BOOL {
	YES
}