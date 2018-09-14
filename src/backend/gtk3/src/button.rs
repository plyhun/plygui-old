use super::common::*;
use super::*;

use gtk::{Bin, BinExt, Button as GtkButtonSys, ButtonExt, Label, LabelExt};
use pango::LayoutExt;

use std::borrow::Cow;

pub type Button = Member<Control<GtkButton>>;

#[repr(C)]
pub struct GtkButton {
    base: GtkControlBase<Button>,

    h_left_clicked: Option<callbacks::Click>,
    h_right_clicked: Option<callbacks::Click>,
}

impl ButtonInner for GtkButton {
    fn with_label(label: &str) -> Box<Button> {
        let mut btn = Box::new(Member::with_inner(
            Control::with_inner(
                GtkButton {
                    base: common::GtkControlBase::with_gtk_widget(GtkButtonSys::new_with_label(label).upcast::<Widget>()),
                    h_left_clicked: None,
                    h_right_clicked: None,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = btn.as_ref() as *const _ as *mut std::os::raw::c_void;
            btn.as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            let self_widget: gtk::Widget = btn.as_inner_mut().as_inner_mut().base.widget.clone().into();
            let button = self_widget.downcast::<GtkButtonSys>().unwrap();
            button.connect_clicked(on_click);
        }
        btn.as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        btn
    }
}

impl HasLabelInner for GtkButton {
    fn label<'a>(&'a self) -> Cow<'a, str> {
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        Cow::Owned(self_widget.downcast::<GtkButtonSys>().unwrap().get_label().unwrap_or(String::new()))
    }
    fn set_label(&mut self, _: &mut MemberBase, label: &str) {
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        self_widget.downcast::<GtkButtonSys>().unwrap().set_label(label)
    }
}

impl ClickableInner for GtkButton {
    fn on_click(&mut self, cb: Option<callbacks::Click>) {
        self.h_left_clicked = cb;
    }
}

impl HasLayoutInner for GtkButton {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl ControlInner for GtkButton {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.measure(member, control, pw, ph);
        self.draw(member, control, Some((x, y)));
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &controls::Container) {}

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|m| m.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|m| m.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|m| m.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|m| m.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fill_from_markup_base!(self, base, markup, registry, Button, [MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, &mut base.member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::Click]);
    }
}

impl MemberInner for GtkButton {
    type Id = common::GtkWidget;

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }

    fn on_set_visibility(&mut self, _: &mut MemberBase) {
        self.base.invalidate()
    }

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone().into()
    }
}

impl Drawable for GtkButton {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut label_size = (-1i32, -1i32);

                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.0 < 0 {
                            let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let mut bin = self_widget.downcast::<Bin>().unwrap();
                            let mut label = bin.get_child().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        label_size.0 + self.base.widget.get_margin_start() + self.base.widget.get_margin_end()
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.1 < 0 {
                            let self_widget: gtk::Widget = self.base.widget.clone().into();
                            let mut bin = self_widget.downcast::<Bin>().unwrap();
                            let mut label = bin.get_child().unwrap().downcast::<Label>().unwrap();
                            label_size = label.get_layout().unwrap().get_pixel_size();
                        }
                        label_size.1 + self.base.widget.get_margin_top() + self.base.widget.get_margin_bottom()
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    Button::with_label("").into_control()
}

fn on_size_allocate(this: &::gtk::Widget, _allo: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Button>(&mut ll).unwrap();

    let measured_size = ll.as_inner().as_inner().base.measured_size;
    if let Some(ref mut cb) = ll.base_mut().handler_resize {
        let mut w2 = this.clone().upcast::<Widget>();
        let mut w2 = common::cast_gtk_widget_to_member_mut::<Button>(&mut w2).unwrap();
        (cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
    }
}

fn on_click(this: &GtkButtonSys) {
    let mut b = this.clone().upcast::<Widget>();
    let b = common::cast_gtk_widget_to_member_mut::<Button>(&mut b).unwrap();
    if let Some(ref mut cb) = b.as_inner_mut().as_inner_mut().h_left_clicked {
        let mut w2 = this.clone().upcast::<Widget>();
        let mut w2 = common::cast_gtk_widget_to_member_mut::<Button>(&mut w2).unwrap();
        (cb.as_mut())(w2);
    }
}

impl_all_defaults!(Button);
