#![no_std]
// #![deny(missing_docs)]

mod inner;

// From use with macros. Not public API.
#[doc(hidden)]
pub extern crate core as __core;

// From use with macros. Not public API.
#[doc(hidden)]
pub mod __private {
    pub use crate::inner::AssertType;
    // Wrap the outlined functions with generic versions in an effort to improve
    // the error message given when using one of these macros on a type which
    // doesn't impl `core::fmt::Debug`.
    #[cold]
    #[track_caller]
    pub fn assert_failed_nomsg<A, B>(left: &A, right: &B, ty: AssertType) -> !
    where
        A: core::fmt::Debug,
        B: core::fmt::Debug,
    {
        crate::inner::assert_failed_nomsg_impl(left, right, ty);
    }

    #[cold]
    #[track_caller]
    #[doc(hidden)]
    pub fn assert_failed_msg<A, B>(
        left: &A,
        right: &B,
        ty: AssertType,
        msg: core::fmt::Arguments<'_>,
    ) -> !
    where
        A: core::fmt::Debug,
        B: core::fmt::Debug,
    {
        crate::inner::assert_failed_msg_impl(left, right, ty, msg);
    }
}

/// Panics if the first expression is not strictly less than the second.
///
/// Requires that the values implement [`Debug`](core::fmt::Debug) and [`PartialOrd`](core::cmp::PartialOrd).
///
/// On failure, panics and prints the values out in a manner similar to [`assert_eq!`](core::assert_eq).
///
/// # Example
///
/// ```rust
/// use jest;
///
/// jest::assert_ends_with_next!(3, 4);
/// jest::assert_ends_with_next!(3, 4, "With a message");
/// jest::assert_ends_with_next!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! assert_ends_with_next {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left, right) => if !(left < right) {
                $crate::__private::assert_failed_nomsg(
                    left, right, $crate::__private::AssertType::Lt,
                );
            }
        }
    };
    ($left:expr, $right:expr, ) => {
        $crate::assert_is_less_than!($left, $right)
    };
    ($left:expr, $right:expr, $($msg_args:tt)+) => {
        match (&$left, &$right) {
            (left, right) => if !(left < right) {
                $crate::__private::assert_failed_msg(
                    left, right, $crate::__private::AssertType::Lt,
                    $crate::__core::format_args!($($msg_args)+),
                );
            }
        }
    };
}

/// Panics if the first expression is not strictly less than the second.
///
/// Requires that the values implement [`Debug`](core::fmt::Debug) and [`PartialOrd`](core::cmp::PartialOrd).
///
/// On failure, panics and prints the values out in a manner similar to [`assert_eq!`](core::assert_eq).
///
/// # Example
///
/// ```rust
/// use jest;
///
/// jest::assert_is_less_than!(3, 4);
/// jest::assert_is_less_than!(3, 4, "With a message");
/// jest::assert_is_less_than!(3, 4, "With a formatted message: {}", "oh no");
/// ```
#[macro_export]
macro_rules! assert_is_less_than {
    ($left:expr, $right:expr) => {
        match (&$left, &$right) {
            (left, right) => if !(left < right) {
                $crate::__private::assert_failed_nomsg(
                    left, right, $crate::__private::AssertType::Lt,
                );
            }
        }
    };
    ($left:expr, $right:expr, ) => {
        $crate::assert_is_less_than!($left, $right)
    };
    ($left:expr, $right:expr, $($msg_args:tt)+) => {
        match (&$left, &$right) {
            (left, right) => if !(left < right) {
                $crate::__private::assert_failed_msg(
                    left, right, $crate::__private::AssertType::Lt,
                    $crate::__core::format_args!($($msg_args)+),
                );
            }
        }
    };
}

// TODO Convert that into a macro using `macro_rules!`.
pub fn assert_ends_with<B: AsRef<str>, S: AsRef<str>>(base_string: B, end_string: S) -> () {
    let result = base_string.as_ref().ends_with(end_string.as_ref());

    if !result {
        panic!(
            "\"{}\" doesn't end with \"{}\".",
            base_string.as_ref(),
            end_string.as_ref()
        )
    }
}
