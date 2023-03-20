use std::fmt::Debug;

use num::ToPrimitive;

// macro_rules! implement_expectation_shared_methods {
//     ($trait_fragment:tt$(<$type_fragment:tt>)?) => {
//         impl<T: $trait_fragment$(<$type_fragment>)? + Debug> Expectation<T> {
//             fn panic_with_assertion(&self, right_value: T, comparator: &str) {
//                 panic!(
//                     "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`",
//                     comparator, self.left_value, right_value,
//                 );
//             }
//         }
//     };
// }

// pub fn expect<T: Debug>(left_value: T) -> Expectation<T> {
//     Expectation::new(left_value)
// }

#[macro_export]
macro_rules! expect {
    ($left_value:expr) => {{
        jest::Expectation::new($left_value)
    }};
}

pub struct Expectation<L: Debug> {
    left_value: L,
}
impl<L: Debug> Expectation<L> {
    pub fn new(left_value: L) -> Self {
        Self { left_value }
    }
}
impl<L: Debug> Expectation<L> {
    fn panic_with_assertion<R: Debug>(&self, right_value: R, comparator: &str) {
        panic!(
            "assertion failed: `(left {} right)`\n  left: `{:?}`,\n right: `{:?}`",
            comparator, self.left_value, right_value,
        );
    }
}

impl<L: Debug + Eq + PartialEq> Expectation<L> {
    pub fn to_be(&self, right_value: L) {
        if !(self.left_value == right_value) {
            self.panic_with_assertion(right_value, "===");
        }
    }

    // pub fn to_equal<R: 'static + Debug + Eq + Into<L> + PartialEq>(&self, right_value: &R)
    // where
    //     &'static R: Into<&'static L>,
    // {
    //     if !(&self.left_value == right_value.into()) {
    //         self.panic_with_assertion(right_value, "==");
    //     }
    // }
}

// implement_expectation_shared_methods! { ToPrimitive }
impl<T: ToPrimitive + Debug> Expectation<T> {
    pub fn to_be_greater_than(&self, right_value: T) {
        if !(self.left_value.to_f32() > right_value.to_f32()) {
            self.panic_with_assertion(right_value, ">");
        }
    }

    pub fn to_be_greater_than_or_equal(&self, right_value: T) {
        if !(self.left_value.to_f32() >= right_value.to_f32()) {
            self.panic_with_assertion(right_value, ">=");
        }
    }

    pub fn to_be_less_than(&self, right_value: T) {
        if !(self.left_value.to_f32() < right_value.to_f32()) {
            self.panic_with_assertion(right_value, "<");
        }
    }

    pub fn to_be_less_than_or_equal(&self, right_value: T) {
        if !(self.left_value.to_f32() <= right_value.to_f32()) {
            self.panic_with_assertion(right_value, "<=");
        }
    }
}

// implement_expectation_shared_methods! { AsRef<str> }
impl<T: AsRef<str> + Debug> Expectation<T> {
    pub fn to_end_with<R: AsRef<str> + Debug>(&self, right_value: R) {
        if !self.left_value.as_ref().ends_with(right_value.as_ref()) {
            self.panic_with_assertion(right_value, "ends with");
        }
    }

    pub fn to_start_with<R: AsRef<str> + Debug>(&self, right_value: R) {
        if !self.left_value.as_ref().starts_with(right_value.as_ref()) {
            self.panic_with_assertion(right_value, "starts with");
        }
    }
}
