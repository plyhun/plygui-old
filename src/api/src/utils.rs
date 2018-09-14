use super::*;

#[inline]
pub unsafe fn base_to_impl<T>(this: &development::MemberBase) -> &T
    where T: controls::Member + Sized
{
    &*(this as *const _ as *const T)
}
#[inline]
pub unsafe fn base_to_impl_mut<T>(this: &mut development::MemberBase) -> &mut T
    where T: controls::Member + Sized
{
    &mut *(this as *mut _ as *mut T)
}

#[inline]
pub fn coord_to_size(a: i32) -> u16 {
    ::std::cmp::max(0, a) as u16
}