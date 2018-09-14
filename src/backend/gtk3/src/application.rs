use super::*;

use gtk::Widget;

use plygui_api::development;
use plygui_api::{controls, ids, types};

pub struct GtkApplication {
    name: String,
    windows: Vec<Widget>,
    selfptr: *mut Application,
}

pub type Application = development::Application<GtkApplication>;

impl development::ApplicationInner for GtkApplication {
    fn with_name(name: &str) -> Box<Application> {
        use plygui_api::development::HasInner;
        use std::ptr;

        if gtk::init().is_err() {
            panic!("Failed to initialize GTK");
        }
        let mut a = Box::new(development::Application::with_inner(
            GtkApplication {
                name: name.into(),
                windows: Vec::with_capacity(1),
                selfptr: ptr::null_mut(),
            },
            (),
        ));
        a.as_inner_mut().selfptr = a.as_mut() as *mut Application;
        a
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
        use plygui_api::development::{MemberInner, WindowInner};

        let w = window::GtkWindow::with_params(title, size, menu);
        let widget = {
            use gtk::{Inhibit, WidgetExt};
            use plygui_api::controls::AsAny;
            use plygui_api::development::HasInner;

            let widget = unsafe { w.as_any().downcast_ref::<window::Window>().unwrap().as_inner().native_id().as_ref().clone() };
            let selfptr = self.selfptr.clone();
            widget.connect_delete_event(move |window, _| {
                let a = unsafe { &mut *selfptr }.as_inner_mut();
                a.windows.iter().position(|item| item == window).map(|e| a.windows.remove(e));
                if a.windows.len() < 1 {
                    gtk::main_quit();
                }
                Inhibit(false)
            });
            widget
        };
        self.windows.push(widget);

        w
    }
    fn name(&self) -> ::std::borrow::Cow<str> {
        ::std::borrow::Cow::Borrowed(self.name.as_ref())
    }
    fn start(&mut self) {
        gtk::main()
    }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Member> {
        use plygui_api::controls::{Container, Member, SingleContainer};

        for window in self.windows.as_mut_slice() {
            let window: &mut window::Window = common::cast_gtk_widget_to_member_mut(window).unwrap();
            if window.id() == id {
                return Some(window.as_single_container_mut().as_container_mut().as_member_mut());
            } else {
                return window.find_control_by_id_mut(id).map(|control| control.as_member_mut());
            }
        }
        None
    }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&controls::Member> {
        use plygui_api::controls::{Container, Member, SingleContainer};

        for window in self.windows.as_slice() {
            let window: &window::Window = common::cast_gtk_widget_to_member(window).unwrap();
            if window.id() == id {
                return Some(window.as_single_container().as_container().as_member());
            } else {
                return window.find_control_by_id(id).map(|control| control.as_member());
            }
        }

        None
    }
}
