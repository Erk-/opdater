//! # Opdater
//!
//! Small trait based approch to implement updating of structs.
//!
//! The main goal of this crate is to allow updating a struct continously
//! without writing a lot of boilerplate code.
//!
//! ## Example
//!
//! ```rust
//! use opdater::Opdater;
//!
//! #[derive(Debug, PartialEq, Opdater)]
//! struct Bla {
//!     a: Option<i32>,
//!     b: Option<f32>,
//! }
//!
//! let mut bla = Bla { a: None, b: None };
//! let bla_op = Bla {
//!     a: Some(10),
//!     b: Some(13.37),
//! };
//!
//! bla.update(bla_op);
//!
//! assert_eq!(
//!     bla,
//!     Bla {
//!         a: Some(10),
//!         b: Some(13.37)
//!     }
//! );
//!
//! let bla_op2 = Bla {
//!     a: Some(5),
//!     b: None,
//! };
//!
//! bla.update(bla_op2);
//!
//! assert_eq!(
//!     bla,
//!     Bla {
//!         a: Some(5),
//!         b: Some(13.37)
//!     }
//! );
//! ```
//!
//! ## Etmylogy
//!
//! Opdater means to update in Danish.
//!
//! ## License
//! This is licensed under the ISC License

pub use opdater_derive::Opdater;

/// Helper to implement struct Updates automatically.
///
/// True is returned if any updates were done.
///
/// There are default implementations for `Option<T> where T: PartialEq`
/// and `[T; N] where T: Opdater`.
///
/// # Example:
/// ```rust
/// use opdater::Opdater;
///
/// let mut bla: Option<u64> = None;
///
/// bla.update(Some(10));
///
/// assert_eq!(bla, Some(10));
///
/// bla.update(None);
///
/// assert_eq!(bla, Some(10));
///
/// bla.update(Some(42));
///
/// assert_eq!(bla, Some(42));
/// ```
pub trait Opdater {
    fn update(&mut self, other: Self) -> bool;
}

impl<T> Opdater for Option<T>
where
    T: PartialEq,
{
    fn update(&mut self, other: Self) -> bool {
        match (&self, &other) {
            (None, None) => false,
            (None, Some(_)) => {
                *self = other;
                true
            }
            (Some(_), None) => false,
            (Some(lhs), Some(rhs)) => {
                if lhs != rhs {
                    *self = other;
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl<T, const N: usize> Opdater for [T; N]
where
    T: Opdater,
{
    fn update(&mut self, other: Self) -> bool {
        let mut b = false;
        other.into_iter().enumerate().for_each(|(i, x)| {
            b |= self[i].update(x);
        });
        b
    }
}
