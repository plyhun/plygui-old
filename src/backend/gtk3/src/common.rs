pub use plygui_api::development::*;
pub use plygui_api::{callbacks, controls, ids, layout, types, utils};

pub use glib::translate::ToGlibPtr;
pub use gtk::{Cast, Orientation as GtkOrientation, Widget, WidgetExt};
pub use gtk_sys::GtkWidget as WidgetSys;

pub use std::borrow::Cow;
pub use std::ffi::CString;
pub use std::marker::PhantomData;
pub use std::os::raw::{c_char, c_void};
pub use std::{cmp, mem, ops};

lazy_static! {
    pub static ref PROPERTY: CString = CString::new("plygui").unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GtkWidget(Widget);

impl From<Widget> for GtkWidget {
    fn from(a: Widget) -> GtkWidget {
        GtkWidget(a)
    }
}
impl From<GtkWidget> for Widget {
    fn from(a: GtkWidget) -> Widget {
        a.0
    }
}
impl From<GtkWidget> for usize {
    fn from(a: GtkWidget) -> usize {
        let aa: *mut WidgetSys = a.0.to_glib_full();
        aa as usize
    }
}
impl From<usize> for GtkWidget {
    fn from(a: usize) -> GtkWidget {
        use glib::translate::FromGlibPtrFull;

        unsafe { GtkWidget(Widget::from_glib_full(a as *mut WidgetSys)) }
    }
}
impl cmp::PartialOrd for GtkWidget {
    fn partial_cmp(&self, other: &GtkWidget) -> Option<cmp::Ordering> {
        pointer(&self.0).partial_cmp(&pointer(&other.0))
    }
}
impl cmp::Ord for GtkWidget {
    fn cmp(&self, other: &GtkWidget) -> cmp::Ordering {
        pointer(&self.0).cmp(&pointer(&other.0))
    }
}
impl ops::Deref for GtkWidget {
    type Target = Widget;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl ops::DerefMut for GtkWidget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl AsRef<Widget> for GtkWidget {
    fn as_ref(&self) -> &Widget {
        &self.0
    }
}
impl AsMut<Widget> for GtkWidget {
    fn as_mut(&mut self) -> &mut Widget {
        &mut self.0
    }
}
impl NativeId for GtkWidget {}

#[repr(C)]
pub struct GtkControlBase<T: controls::Control + Sized> {
    pub widget: GtkWidget,
    pub coords: Option<(i32, i32)>,
    pub measured_size: (u16, u16),
    //pub dirty: bool,
    _marker: PhantomData<T>,
}

impl<T: controls::Control + Sized> GtkControlBase<T> {
    pub fn with_gtk_widget(widget: Widget) -> GtkControlBase<T> {
        let base = GtkControlBase {
            widget: widget.into(),
            coords: None,
            measured_size: (0, 0),
            //dirty: true,
            _marker: PhantomData,
        };
        base
    }
    pub fn set_pointer(&mut self, ptr: *mut c_void) {
        set_pointer(&mut self.widget, ptr)
    }
    pub fn pointer(&self) -> *mut c_void {
        pointer(&self.widget)
    }
    pub fn margins(&self) -> layout::BoundarySize {
        layout::BoundarySize::Distinct(self.widget.get_margin_start(), self.widget.get_margin_top(), self.widget.get_margin_end(), self.widget.get_margin_bottom())
    }
    pub fn parent(&self) -> Option<&MemberBase> {
        if let Some(w) = self.widget.get_parent() {
            if pointer(&w).is_null() {
                w.get_parent().map(|w| cast_gtk_widget(&w).unwrap())
            } else {
                Some(cast_gtk_widget(&w).unwrap())
            }
        } else {
            None
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut MemberBase> {
        if let Some(mut w) = self.widget.get_parent() {
            if pointer(&w).is_null() {
                w.get_parent().map(|mut w| cast_gtk_widget_mut(&mut w).unwrap())
            } else {
                Some(cast_gtk_widget_mut(&mut w).unwrap())
            }
        } else {
            None
        }
    }
    pub fn root(&self) -> Option<&MemberBase> {
        self.widget.get_toplevel().map(|w| cast_gtk_widget(&w).unwrap())
    }
    pub fn root_mut(&mut self) -> Option<&mut MemberBase> {
        self.widget.get_toplevel().map(|mut w| cast_gtk_widget_mut(&mut w).unwrap())
    }
    pub fn invalidate(&mut self) {
        use gtk::WidgetExt;

        if let Some(mut parent_widget) = self.widget.get_parent() {
            if pointer(&parent_widget).is_null() {
                parent_widget = parent_widget.get_parent().unwrap();
            }
            if let Some(mparent) = cast_gtk_widget_to_base_mut(&mut parent_widget) {
                let (pw, ph) = mparent.as_member().size();
                let this: &mut T = cast_gtk_widget_to_member_mut(&mut self.widget).unwrap();
                let (_, _, changed) = this.measure(pw, ph);
                this.draw(None);

                if let Some(cparent) = mparent.as_member_mut().is_control_mut() {
                    if changed && !cparent.is_skip_draw() {
                        cparent.invalidate();
                    }
                }
            }
        }
    }
    pub fn draw(&mut self, member: &mut MemberBase, _control: &mut ControlBase, coords: Option<(i32, i32)>) {
        if coords.is_some() {
            self.coords = coords;
        }
        if self.coords.is_some() {
            self.widget.set_size_request(self.measured_size.0 as i32, self.measured_size.1 as i32);
            if let types::Visibility::Gone = member.visibility {
                self.widget.hide();
            } else {
                self.widget.show();
            }
            if let types::Visibility::Invisible = member.visibility {
                self.widget.set_sensitive(false);
                self.widget.set_opacity(0.0);
            } else {
                self.widget.set_sensitive(true);
                self.widget.set_opacity(1.0);
            }
        }
    }
}

pub fn set_pointer(this: &mut Widget, ptr: *mut c_void) {
    unsafe {
        ::gobject_sys::g_object_set_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char, ptr as *mut ::libc::c_void);
    }
}
pub fn pointer(this: &Widget) -> *mut c_void {
    unsafe { ::gobject_sys::g_object_get_data(this.to_glib_none().0, PROPERTY.as_ptr() as *const c_char) as *mut c_void }
}
pub fn cast_control_to_gtkwidget(control: &controls::Control) -> GtkWidget {
    unsafe { control.native_id().into() }
}

fn cast_gtk_widget_mut<'a, T>(this: &mut Widget) -> Option<&'a mut T>
where
    T: Sized,
{
    unsafe {
        let ptr = pointer(this);
        if !ptr.is_null() {
            Some(::std::mem::transmute(ptr))
        } else {
            None
        }
    }
}
fn cast_gtk_widget<'a, T>(this: &Widget) -> Option<&'a T>
where
    T: Sized,
{
    unsafe {
        let ptr = pointer(this);
        if !ptr.is_null() {
            Some(::std::mem::transmute(ptr))
        } else {
            None
        }
    }
}
pub fn cast_gtk_widget_to_member_mut<'a, T>(object: &'a mut Widget) -> Option<&'a mut T>
where
    T: controls::Member + Sized,
{
    cast_gtk_widget_mut(object)
}
pub fn cast_gtk_widget_to_member<'a, T>(object: &'a Widget) -> Option<&'a T>
where
    T: controls::Member + Sized,
{
    cast_gtk_widget(object)
}
pub fn cast_gtk_widget_to_base_mut<'a>(object: &'a mut Widget) -> Option<&'a mut MemberBase> {
    cast_gtk_widget_mut(object)
}
pub fn cast_gtk_widget_to_base<'a>(object: &'a Widget) -> Option<&'a MemberBase> {
    cast_gtk_widget(object)
}
pub fn orientation_to_gtk(a: layout::Orientation) -> GtkOrientation {
    match a {
        layout::Orientation::Horizontal => GtkOrientation::Horizontal,
        layout::Orientation::Vertical => GtkOrientation::Vertical,
    }
}
pub fn gtk_to_orientation(a: GtkOrientation) -> layout::Orientation {
    match a {
        GtkOrientation::Horizontal => layout::Orientation::Horizontal,
        GtkOrientation::Vertical => layout::Orientation::Vertical,
        _ => panic!("Unsupported GtkOrientation"),
    }
}
