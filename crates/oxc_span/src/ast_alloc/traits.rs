use std::fmt::Debug;
use std::ops::{Deref as _, DerefMut as _};

use crate::{GetSpan, GetSpanMut, Span};
use oxc_allocator::{CloneIn, FromIn};

use super::void::VoidVec;
// pub use crate::ast_alloc::cast::*;
// pub use crate::ast_alloc::void::VoidAllocator;

pub(super) trait Sealed {}

/// Safety: Box and Vec are variant on its lifetime `'a`
#[allow(private_bounds)]
pub unsafe trait AstAllocator: Sized + 'static + Sealed {
    // For runtime specialization
    const IS_VOID: bool;
    type Box<'a, T: Debug + GetSpan + GetSpanMut>: Box<'a, Target = T>
        + GetSpan
        + GetSpanMut
        + FromIn<'a, T, Self>
        + Debug
    where
        Self: 'a;

    type Vec<'a, T: Debug>: Vec<'a, Item = T> + Debug
    where
        Self: 'a;

    fn alloc<'a, T: Debug + GetSpan + GetSpanMut>(&'a self, value: T) -> Self::Box<'a, T>;
    fn alloc_str<'a>(&'a self, src: &str) -> &'a str;

    fn box_from_span<'a, T: Debug + GetSpan + GetSpanMut>(span: Span) -> Option<Self::Box<'a, T>>;

    fn vec<'a, T: Debug>(&'a self) -> Self::Vec<'a, T>;
    fn vec_with_capacity<'a, T: Debug>(&'a self, capacity: usize) -> Self::Vec<'a, T>;
    fn vec_from_iter<'a, T: Debug, I: IntoIterator<Item = T>>(
        &'a self,
        iter: I,
    ) -> Self::Vec<'a, T>;
}

pub trait Box<'a>: Sized {
    type Target: ?Sized;

    fn deref_or_else<U, FSome: FnOnce(&Self::Target) -> U, FNone: FnOnce() -> U>(
        &self,
        f_some: FSome,
        f_none: FNone,
    ) -> U;
    fn unbox_or_else<U, FSome: FnOnce(Self::Target) -> U, FNone: FnOnce(Self) -> U>(
        self,
        f_some: FSome,
        f_none: FNone,
    ) -> U
    where
        Self::Target: Sized;
    fn try_deref(&self) -> Option<&Self::Target>;
    fn try_unbox(self) -> Result<Self::Target, Self>
    where
        Self::Target: Sized;
}
impl<'a, T> Box<'a> for oxc_allocator::Box<'a, T> {
    type Target = T;

    #[inline]
    fn deref_or_else<U, FSome: FnOnce(&Self::Target) -> U, FNone: FnOnce() -> U>(
        &self,
        f_some: FSome,
        _f_none: FNone,
    ) -> U {
        f_some(self.deref())
    }

    #[inline]
    fn unbox_or_else<U, FSome: FnOnce(Self::Target) -> U, FNone: FnOnce(Self) -> U>(
        self,
        f_some: FSome,
        _f_none: FNone,
    ) -> U
    where
        Self::Target: Sized,
    {
        f_some(self.unbox())
    }

    #[inline]
    fn try_deref(&self) -> Option<&Self::Target> {
        Some(self.deref())
    }

    fn try_unbox(self) -> Result<Self::Target, Self>
    where
        Self::Target: Sized,
    {
        Ok(self.unbox())
    }
}

pub trait Vec<'a>: TryInto<oxc_allocator::Vec<'a, Self::Item>> {
    type Item;
    type Iterator: Iterator<Item = Self::Item>;

    fn push(&mut self, val: Self::Item);
    fn append(&mut self, other: &mut Self);
    fn try_as_slice(&self) -> Option<&[Self::Item]>;
    fn try_as_mut_slice(&mut self) -> Option<&mut [Self::Item]>;
    fn into_iter_or_empty(self) -> Self::Iterator;

    #[inline]
    fn as_slice_or_empty(&self) -> &[Self::Item] {
        self.try_as_slice().unwrap_or(&[])
    }
    #[inline]
    fn as_mut_slice_or_empty(&mut self) -> &mut [Self::Item] {
        self.try_as_mut_slice().unwrap_or(&mut [])
    }

    fn specialize(self) -> Result<oxc_allocator::Vec<'a, Self::Item>, VoidVec<'a, Self::Item>>;

    fn specialize_ref(
        &self,
    ) -> Result<&oxc_allocator::Vec<'a, Self::Item>, &VoidVec<'a, Self::Item>>;

    fn specialize_mut(
        &mut self,
    ) -> Result<&mut oxc_allocator::Vec<'a, Self::Item>, &mut VoidVec<'a, Self::Item>>;
}

impl<'a, T> Vec<'a> for oxc_allocator::Vec<'a, T> {
    type Item = T;
    type Iterator = <oxc_allocator::Vec<'a, T> as IntoIterator>::IntoIter;

    #[inline]
    fn push(&mut self, val: Self::Item) {
        self.deref_mut().push(val);
    }

    #[inline]
    fn append(&mut self, other: &mut Self) {
        self.deref_mut().append(other);
    }

    #[inline]
    fn try_as_slice(&self) -> Option<&[Self::Item]> {
        Some(self.as_slice())
    }

    fn try_as_mut_slice(&mut self) -> Option<&mut [Self::Item]> {
        Some(self.deref_mut())
    }

    #[inline]
    fn into_iter_or_empty(self) -> Self::Iterator {
        self.into_iter()
    }

    #[inline]
    fn specialize(self) -> Result<oxc_allocator::Vec<'a, Self::Item>, VoidVec<'a, Self::Item>> {
        Ok(self)
    }

    #[inline]
    fn specialize_ref(
        &self,
    ) -> Result<&oxc_allocator::Vec<'a, Self::Item>, &VoidVec<'a, Self::Item>> {
        Ok(self)
    }
    #[inline]
    fn specialize_mut(
        &mut self,
    ) -> Result<&mut oxc_allocator::Vec<'a, Self::Item>, &mut VoidVec<'a, Self::Item>> {
        Ok(self)
    }
}

impl Sealed for oxc_allocator::Allocator {}

// pub type Vec<'a, T, A: AstAllocator> = Vec<'a, T, A>;

unsafe impl AstAllocator for oxc_allocator::Allocator {
    const IS_VOID: bool = false;
    type Box<'a, T: Debug + GetSpan + GetSpanMut> = oxc_allocator::Box<'a, T>;
    type Vec<'a, T: Debug> = oxc_allocator::Vec<'a, T>;

    #[inline]
    fn alloc<'a, T: Debug + GetSpan + GetSpanMut>(&'a self, value: T) -> Self::Box<'a, T> {
        oxc_allocator::Box::new_in(value, self)
    }

    #[inline]
    fn alloc_str<'a>(&'a self, src: &str) -> &'a str {
        self.deref().alloc_str(src)
    }

    #[inline]
    fn box_from_span<'a, T: Debug + GetSpan + GetSpanMut>(_span: Span) -> Option<Self::Box<'a, T>> {
        None
    }

    #[inline]
    fn vec<'a, T: Debug>(&'a self) -> Self::Vec<'a, T> {
        oxc_allocator::Vec::new_in(self)
    }

    #[inline]
    fn vec_with_capacity<'a, T: Debug>(&'a self, capacity: usize) -> Self::Vec<'a, T> {
        oxc_allocator::Vec::with_capacity_in(capacity, self)
    }

    #[inline]
    fn vec_from_iter<'a, T: Debug, I: IntoIterator<Item = T>>(
        &'a self,
        iter: I,
    ) -> Self::Vec<'a, T> {
        oxc_allocator::Vec::from_iter_in(iter, self)
    }
}
