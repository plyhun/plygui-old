use super::common::*;
use super::*;

use gtk::{Cast, OrientableExt, Paned, PanedExt, Widget, WidgetExt};

pub type Splitted = Member<Control<MultiContainer<GtkSplitted>>>;

#[repr(C)]
pub struct GtkSplitted {
    base: common::GtkControlBase<Splitted>,
    splitter: f32,
    first: Box<controls::Control>,
    second: Box<controls::Control>,
}

impl GtkSplitted {
    fn update_splitter(&mut self) {
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        let orientation = self.layout_orientation();
        match orientation {
            layout::Orientation::Horizontal => self_widget.downcast::<Paned>().unwrap().set_position((self.base.measured_size.0 as f32 * self.splitter) as i32),
            layout::Orientation::Vertical => self_widget.downcast::<Paned>().unwrap().set_position((self.base.measured_size.1 as f32 * self.splitter) as i32),
        }
    }
    fn children_sizes(&self) -> (u16, u16) {
        let (w, h) = self.size();
        let o = self.layout_orientation();
        let handle = 6; // no access to handle-size
        let (target, start, end) = match o {
            layout::Orientation::Horizontal => (w, self.base.widget.get_margin_start(), self.base.widget.get_margin_end()),
            layout::Orientation::Vertical => (h, self.base.widget.get_margin_top(), self.base.widget.get_margin_bottom()),
        };
        (
            utils::coord_to_size((target as f32 * self.splitter) as i32 - start - (handle / 2)),
            utils::coord_to_size((target as f32 * (1.0 - self.splitter)) as i32 - end - (handle / 2)),
        )
    }
}

impl SplittedInner for GtkSplitted {
    fn with_content(first: Box<controls::Control>, second: Box<controls::Control>, orientation: layout::Orientation) -> Box<Splitted> {
        let mut ll = Box::new(Member::with_inner(
            Control::with_inner(
                MultiContainer::with_inner(
                    GtkSplitted {
                        base: common::GtkControlBase::with_gtk_widget(Paned::new(common::orientation_to_gtk(orientation)).upcast::<Widget>()),
                        first: first,
                        splitter: 0.5,
                        second: second,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        {
            let ptr = ll.as_ref() as *const _ as *mut std::os::raw::c_void;
            ll.as_inner_mut().as_inner_mut().as_inner_mut().base.set_pointer(ptr);
        }
        {
            use plygui_api::controls::Splitted;

            let self_widget: gtk::Widget = ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.clone().into();
            let gtk_self = self_widget.downcast::<Paned>().unwrap();
            let paned = gtk_self.downcast::<Paned>().unwrap();
            paned.pack1(common::cast_control_to_gtkwidget(ll.first()).as_ref(), false, false);
            paned.pack2(common::cast_control_to_gtkwidget(ll.second()).as_ref(), false, false);
            paned.connect_property_position_notify(on_property_position_notify);
        }
        ll.as_inner_mut().as_inner_mut().as_inner_mut().base.widget.connect_size_allocate(on_size_allocate);
        ll.as_inner_mut().as_inner_mut().as_inner_mut().update_splitter();
        ll
    }
    fn set_splitter(&mut self, _: &mut MemberBase, _: &mut ControlBase, pos: f32) {
        let pos = pos % 1.0;
        self.splitter = pos;
        self.update_splitter();
    }
    fn splitter(&self) -> f32 {
        self.splitter
    }

    fn first(&self) -> &controls::Control {
        self.first.as_ref()
    }
    fn second(&self) -> &controls::Control {
        self.second.as_ref()
    }
    fn first_mut(&mut self) -> &mut controls::Control {
        self.first.as_mut()
    }
    fn second_mut(&mut self) -> &mut controls::Control {
        self.second.as_mut()
    }
}

impl MemberInner for GtkSplitted {
    type Id = common::GtkWidget;

    fn size(&self) -> (u16, u16) {
        self.base.measured_size
    }

    fn on_set_visibility(&mut self, _: &mut MemberBase) {
        self.base.invalidate()
    }

    unsafe fn native_id(&self) -> Self::Id {
        self.base.widget.clone()
    }
}

impl Drawable for GtkSplitted {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase, coords: Option<(i32, i32)>) {
        self.base.draw(member, control, coords);
        for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
            child.draw(Some((0, 0)));
        }
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let orientation = self.layout_orientation();
        let old_size = self.base.measured_size;
        self.base.measured_size = match member.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent | layout::Size::WrapContent => parent_width,
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent | layout::Size::WrapContent => parent_height,
                };
                (w, h)
            }
        };
        let (first, second) = self.children_sizes();
        let (lm, tm, rm, bm) = self.base.margins().into();
        match orientation {
            layout::Orientation::Horizontal => {
                let size = cmp::max(0, parent_height as i32 - tm - bm) as u16;
                self.first.measure(first, size);
                self.second.measure(second, size);
            }
            layout::Orientation::Vertical => {
                let size = cmp::max(0, parent_width as i32 - lm - rm) as u16;
                self.first.measure(size, first);
                self.second.measure(size, second);
            }
        }
        (self.base.measured_size.0, self.base.measured_size.1, self.base.measured_size != old_size)
    }
    fn invalidate(&mut self, _: &mut MemberBase, _: &mut ControlBase) {
        self.base.invalidate()
    }
}

impl HasLayoutInner for GtkSplitted {
    fn on_layout_changed(&mut self, _: &mut MemberBase) {
        self.update_splitter();
        self.base.invalidate()
    }
}

impl ControlInner for GtkSplitted {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, _parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        self.base.measured_size = (pw, ph); // for update_splitter only
        self.update_splitter();
        self.measure(member, control, pw, ph);
        self.draw(member, control, Some((x, y)));

        let (first, second) = self.children_sizes();
        let o = self.layout_orientation();
        let (lm, tm, rm, bm) = self.base.margins().into();
        let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();

        match o {
            layout::Orientation::Horizontal => {
                let h = utils::coord_to_size(cmp::max(0, ph as i32 - tm - bm));
                self.first.on_added_to_container(self2, 0, 0, first, h);
                self.second.on_added_to_container(self2, 0, 0, second, h);
            }
            layout::Orientation::Vertical => {
                let w = utils::coord_to_size(cmp::max(0, pw as i32 - lm - rm));
                self.first.on_added_to_container(self2, 0, 0, w, first);
                self.second.on_added_to_container(self2, 0, 0, w, second);
            }
        }
    }
    fn on_removed_from_container(&mut self, _: &mut MemberBase, _: &mut ControlBase, _: &controls::Container) {
        let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();
        for mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
            child.on_removed_from_container(self2);
        }
    }

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
        use plygui_api::markup::MEMBER_TYPE_SPLITTED;

        fill_from_markup_base!(self, markup, registry, Splitted, [MEMBER_TYPE_SPLITTED]);
        fill_from_markup_children!(self, markup, registry);
    }
}

impl HasOrientationInner for GtkSplitted {
    fn layout_orientation(&self) -> layout::Orientation {
        let widget: Widget = self.base.widget.clone().into();
        let gtk_self = widget.downcast::<Paned>().unwrap();
        common::gtk_to_orientation(gtk_self.get_orientation())
    }
    fn set_layout_orientation(&mut self, _: &mut MemberBase, orientation: layout::Orientation) {
        let widget: Widget = self.base.widget.clone().into();
        let gtk_self = widget.downcast::<Paned>().unwrap();
        gtk_self.set_orientation(common::orientation_to_gtk(orientation));
        self.base.invalidate();
    }
}

impl ContainerInner for GtkSplitted {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        if self.first().as_member().id() == id {
            return Some(self.first_mut());
        }
        if self.second().as_member().id() == id {
            return Some(self.second_mut());
        }

        let self2: &mut GtkSplitted = unsafe { mem::transmute(self as *mut GtkSplitted) }; // bck is stupid
        if let Some(c) = self.first_mut().is_container_mut() {
            let ret = c.find_control_by_id_mut(id);
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self2.second_mut().is_container_mut() {
            let ret = c.find_control_by_id_mut(id);
            if ret.is_some() {
                return ret;
            }
        }

        None
    }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        if self.first().as_member().id() == id {
            return Some(self.first());
        }
        if self.second().as_member().id() == id {
            return Some(self.second());
        }

        if let Some(c) = self.first().is_container() {
            let ret = c.find_control_by_id(id);
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self.second().is_container() {
            let ret = c.find_control_by_id(id);
            if ret.is_some() {
                return ret;
            }
        }

        None
    }
}

impl MultiContainerInner for GtkSplitted {
    fn len(&self) -> usize {
        2
    }
    fn set_child_to(&mut self, _: &mut MemberBase, index: usize, mut child: Box<controls::Control>) -> Option<Box<controls::Control>> {
        let (pw, ph) = self.size();
        let orientation = self.layout_orientation();
        let (first, second) = self.children_sizes();
        let (lm, tm, rm, bm) = self.base.margins().into();
        let self_widget: gtk::Widget = self.base.widget.clone().into();
        let gtk_self = self_widget.downcast::<Paned>().unwrap();
        let self2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut self.base.widget).unwrap();

        match index {
            0 => {
                mem::swap(&mut self.first, &mut child);

                let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
                gtk_self.add1(widget.as_ref());
                child.on_removed_from_container(self2);
                match orientation {
                    layout::Orientation::Horizontal => {
                        self.first.on_added_to_container(self2, 0, 0, first, utils::coord_to_size(cmp::max(0, ph as i32 - tm - bm)));
                    }
                    layout::Orientation::Vertical => {
                        self.first.on_added_to_container(self2, 0, 0, utils::coord_to_size(cmp::max(0, pw as i32 - lm - rm)), first);
                    }
                }
            }
            1 => {
                mem::swap(&mut self.second, &mut child);

                let widget = common::cast_control_to_gtkwidget(self.first.as_mut());
                gtk_self.downcast::<Paned>().unwrap().add2(widget.as_ref());
                child.on_removed_from_container(self2);
                match orientation {
                    layout::Orientation::Horizontal => {
                        self.second.on_added_to_container(self2, 0, 0, second, utils::coord_to_size(cmp::max(0, ph as i32 - tm - bm)));
                    }
                    layout::Orientation::Vertical => {
                        self.second.on_added_to_container(self2, 0, 0, utils::coord_to_size(cmp::max(0, pw as i32 - lm - rm)), second);
                    }
                }
            }
            _ => return None,
        }

        Some(child)
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, _: usize) -> Option<Box<controls::Control>> {
        None
    }
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
        match index {
            0 => Some(self.first()),
            1 => Some(self.second()),
            _ => None,
        }
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
        match index {
            0 => Some(self.first_mut()),
            1 => Some(self.second_mut()),
            _ => None,
        }
    }
}

/*#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
	Splitted::with_orientation(layout::Orientation::Vertical).into_control()
}*/

fn on_size_allocate(this: &::gtk::Widget, _: &::gtk::Rectangle) {
    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();
    ll.as_inner_mut().as_inner_mut().as_inner_mut().update_splitter();

    let measured_size = ll.as_inner().as_inner().as_inner().base.measured_size;
    if let Some(ref mut cb) = ll.base_mut().handler_resize {
        let mut w2 = this.clone().upcast::<Widget>();
        let mut w2 = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut w2).unwrap();
        (cb.as_mut())(w2, measured_size.0 as u16, measured_size.1 as u16);
    }
}
fn on_property_position_notify(this: &::gtk::Paned) {
    use plygui_api::controls::{HasOrientation, Member};

    let position = this.get_position();
    if position < 1 {
        return;
    }

    let mut ll = this.clone().upcast::<Widget>();
    let ll = common::cast_gtk_widget_to_member_mut::<Splitted>(&mut ll).unwrap();
    let orientation = ll.layout_orientation();
    let (width, height) = ll.size();
    let splitter = position as f32 / match orientation {
        layout::Orientation::Vertical => if height > 0 {
            height as f32
        } else {
            position as f32 * 2.0
        },
        layout::Orientation::Horizontal => if width > 0 {
            width as f32
        } else {
            position as f32 * 2.0
        },
    };
    let old_splitter = ll.as_inner_mut().as_inner_mut().as_inner_mut().splitter;
    let member = unsafe { &mut *(ll.base_mut() as *mut MemberBase) };
    let control = unsafe { &mut *(ll.as_inner_mut().base_mut() as *mut ControlBase) };
    if (old_splitter - splitter).abs() > 0.001 {
        let ll = ll.as_inner_mut().as_inner_mut().as_inner_mut();
        ll.splitter = splitter;
        ll.measure(member, control, width, height);
        ll.draw(member, control, None);
    }
}

impl_all_defaults!(Splitted);
