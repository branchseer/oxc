use std::cell::Cell;

use crate::{Allocator, Box, Vec};

/// A trait to explicitly clone an object into an arena allocator.
///
/// As a convention `Cloned` associated type should always be the same as `Self`,
/// It'd only differ in the lifetime, Here's an example:
///
/// ```
/// impl<'old_alloc, 'new_alloc> CloneIn<'new_alloc> for Struct<'old_alloc> {
///     type Cloned = Struct<'new_alloc>;
///     fn clone_in(&self, allocator: &'new_alloc Allocator) -> Self::Cloned {
///         Struct { a: self.a.clone_in(allocator), b: self.b.clone_in(allocator) }
///     }
/// }
/// ```
///
/// Implementations of this trait on non-allocated items usually short-circuit to `Clone::clone`;
/// However, it **isn't** guaranteed.
///
pub trait CloneIn<A = Allocator>: Sized {
    type Cloned<'a>
    where
        A: 'a;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc A) -> Self::Cloned<'new_alloc>;
}

impl<T, A> CloneIn<A> for Option<T>
where
    T: CloneIn<A>,
{
    type Cloned<'a> = Option<T::Cloned<'a>> where A: 'a;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc A) -> Self::Cloned<'new_alloc> {
        self.as_ref().map(|it| it.clone_in(allocator))
    }
}

impl<'old_alloc, T> CloneIn for Box<'old_alloc, T>
where
    T: CloneIn,
{
    type Cloned<'a> = Box<'a, T::Cloned<'a>>;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Box::new_in(self.as_ref().clone_in(allocator), allocator)
    }
}

impl<'old_alloc, T> CloneIn for Vec<'old_alloc, T>
where
    T: CloneIn,
{
    type Cloned<'a> = Vec<'a, T::Cloned<'a>>;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Vec::from_iter_in(self.iter().map(|it| it.clone_in(allocator)), allocator)
    }
}

impl<T: Copy, A> CloneIn<A> for Cell<T> {
    type Cloned<'a> = Cell<T> where A: 'a;

    fn clone_in<'new_alloc>(&self, _: &'new_alloc A) -> Self::Cloned<'new_alloc> {
        Cell::new(self.get())
    }
}

impl<'old_alloc> CloneIn for &'old_alloc str {
    type Cloned<'a> = &'a str;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        allocator.alloc_str(self)
    }
}

macro_rules! impl_clone_in {
    ($($t:ty)*) => {
        $(
            impl<A> CloneIn<A> for $t {
                type Cloned<'a> = Self where A: 'a;
                #[inline(always)]
                fn clone_in<'new_alloc>(&self, _: &'new_alloc A) -> Self::Cloned<'new_alloc> {
                    *self
                }
            }
        )*
    }
}

impl_clone_in! {
    usize u8 u16 u32 u64 u128
    isize i8 i16 i32 i64 i128
    f32 f64
    bool char
}
