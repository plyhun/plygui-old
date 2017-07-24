#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Params {
    MatchParent,
    WrapContent,
    Exact(u16),
}

pub type UiControlID = usize;

pub enum Neighborhood {
    Above(UiControlID),
    Below(UiControlID),
    ToLeftOf(UiControlID),
    ToRightOf(UiControlID),
    AlignTop(UiControlID),
    AlignBottom(UiControlID),
    AlignLeft(UiControlID),
    AlignRight(UiControlID),
    AlignParentLeft,
    AlignParentRight,
    AlignParentTop,
    AlignParentBottom,
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
