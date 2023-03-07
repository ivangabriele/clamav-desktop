//! Outlined format+panic code.
//!
//! This reduces the code bloat caused by our macros, improving performance in
//! the case that the assertions are not triggered.
use core::fmt;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub enum AssertType {
    Lt,
    Gt,
    Le,
    Ge,
    // TODO: `matches`? `contains`?
}

#[cold]
#[track_caller]
pub(crate) fn assert_failed_nomsg_impl(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    ty: AssertType,
) -> ! {
    assert_failed_impl(left, right, ty, None);
}

#[cold]
#[track_caller]
pub(crate) fn assert_failed_msg_impl(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    ty: AssertType,
    msg: fmt::Arguments<'_>,
) -> ! {
    assert_failed_impl(left, right, ty, Some(msg))
}

#[cold]
#[track_caller]
#[inline(never)]
fn assert_failed_impl(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    ty: AssertType,
    msg: Option<fmt::Arguments<'_>>,
) -> ! {
    let compare = match ty {
        AssertType::Lt => "<",
        AssertType::Gt => ">",
        AssertType::Le => "<=",
        AssertType::Ge => ">=",
    };
    if let Some(msg) = msg {
        panic!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`: {}",
            compare, left, right, msg,
        );
    } else {
        panic!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`",
            compare, left, right,
        );
    }
}
