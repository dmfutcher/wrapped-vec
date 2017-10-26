#![feature(concat_idents)]

#[macro_export]
macro_rules! wrapped_vec {
    ($wrapping_type:ident(Vec<$item_type:ty>)) => (
        _wrapped_vec!($wrapping_type, $item_type);
    )
}

macro_rules! _wrapped_vec {
    ($wrapping_type:ident, $item_type:ty) => (
        pub struct $wrapping_type(Vec<$item_type>);

        impl ::std::iter::FromIterator<$item_type> for $wrapping_type {
            fn from_iter<I: IntoIterator<Item=$item_type>>(iter: I) -> Self {
                let mut inner = vec![];
                inner.extend(iter);
                $wrapping_type(inner)
            }
        }

        impl From<Vec<$item_type>> for $wrapping_type {

            fn from(ids: Vec<$item_type>) -> $wrapping_type {
                let mut new = $wrapping_type::new();
                new.extend(ids);
                new
            }

        }

        impl IntoIterator for $wrapping_type {
            type Item = $item_type;
            type IntoIter = ::std::vec::IntoIter<$item_type>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a> IntoIterator for &'a $wrapping_type {
            type Item = &'a $item_type;
            type IntoIter = ::std::slice::Iter<'a, $item_type>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl Extend<$item_type> for $wrapping_type {
            fn extend<T: IntoIterator<Item=$item_type>>(&mut self, iter: T) {
                self.0.extend(iter);
            }
        }

        impl $wrapping_type {

            pub fn new() -> $wrapping_type {
                $wrapping_type(vec![])
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            pub fn len(&self) -> usize {
                self.0.len()
            }

            pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, $item_type> {
                self.into_iter()
            }

        }
    )
}

pub use example::{ExampleType, ExampleCollection};
mod example {
    pub struct ExampleType;
    wrapped_vec!(ExampleCollection(Vec<ExampleType>));
}
