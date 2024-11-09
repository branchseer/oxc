mod cast;
mod default;
pub mod traits;
mod void;

use crate::cmp::ContentEq;
use crate::hash::ContentHash;
use crate::{GetSpan, GetSpanMut, Span};
pub use cast::*;
use derive_where::derive_where;
use oxc_allocator::{Allocator, CloneIn, FromIn};
use std::fmt::Debug;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::mem::{transmute};
use std::ops::{Deref, DerefMut};
use serde::Serializer;
pub use traits::AstAllocator;
pub use void::VoidAllocator;
use traits::{Box as _, Vec as _};
use void::VoidVec;

#[derive_where(Debug)]
pub struct Vec<'a, T: Debug, A: AstAllocator = Allocator>(A::Vec<'static, ()>, PhantomData<(&'a (), T)>);

impl<'a, T: Debug, A: AstAllocator> Vec<'a, T, A> {
    #[inline]
    pub fn from_alloc_vec(value: A::Vec<'a, T>) -> Self {
        Self(unsafe { A::transmute_vec(value) }, PhantomData)
    }
    #[inline]
    pub fn into_alloc_vec(self) -> A::Vec<'a, T> {
        unsafe { A::transmute_vec(self.0) }
    }
    
    #[inline]
    pub fn specialize(self) -> Result<oxc_allocator::Vec<'a, T>, VoidVec<'a, T>> {
        self.into_alloc_vec().specialize()
    }
}

impl<'a, T: Debug, A: AstAllocator> Deref for Vec<'a, T, A> {
    type Target = A::Vec<'a, T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}
impl<'a, T: Debug, A: AstAllocator> DerefMut for Vec<'a, T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.0) }
    }
}

impl<'alloc, T: Debug> IntoIterator for &'alloc Vec<'alloc, T> {
    type IntoIter = std::slice::Iter<'alloc, T>;
    type Item = &'alloc T;

    fn into_iter(self) -> Self::IntoIter {
        self.deref().iter()
    }
}

impl<'a, T: Debug> IntoIterator for Vec<'a, T> {
    type Item = T;
    type IntoIter = <oxc_allocator::Vec<'a, T> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.into_alloc_vec().into_iter()
    }
}

impl<'old_alloc, T: Debug + CloneIn> CloneIn for Vec<'old_alloc, T>
where
        for<'a> <T as CloneIn>::Cloned<'a>: Debug,
{
    type Cloned<'a> = Vec<'a, T::Cloned<'a>>;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Vec::from_alloc_vec(self.deref().clone_in(allocator))
    }
}

impl<'a, T: Debug + ContentEq> ContentEq for Vec<'a, T> {
    fn content_eq(&self, other: &Self) -> bool {
        self.0.content_eq(&other.0)
    }
}
impl<'a, T: Debug + ContentHash> ContentHash for Vec<'a, T> {
    fn content_hash<H: Hasher>(&self, state: &mut H) {
        self.deref().content_hash(state)
    }
}

#[cfg(feature = "serialize")]
impl<'a, T: Debug + serde::Serialize, A: AstAllocator> serde::Serialize for Vec<'a, T, A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self.specialize_ref() {
            Ok(val) => val.serialize(serializer),
            Err(val) => val.serialize(serializer),
        }
    }
}

#[derive(Debug)]
struct VariantBoxPlaceholder;
impl GetSpan for VariantBoxPlaceholder {
    fn span(&self) -> Span {
        unimplemented!()
    }
}
impl GetSpanMut for VariantBoxPlaceholder {
    fn span_mut(&mut self) -> &mut Span {
        unimplemented!()
    }
}

#[derive_where(Debug)]
pub struct Box<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator = Allocator>(
    A::Box<'static, VariantBoxPlaceholder>,
    PhantomData<(&'a (), T)>,
);

impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> Deref for Box<'a, T, A> {
    type Target = A::Box<'a, T>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(&self.0) }
    }
}
impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> DerefMut for Box<'a, T, A> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(&mut self.0) }
    }
}

impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> Box<'a, T, A> {
    #[inline]
    pub fn from_alloc_box(value: A::Box<'a, T>) -> Self {
        Self(unsafe { A::transmute_box(value) }, PhantomData)
    }
    #[inline]
    pub fn into_alloc_box(self) -> A::Box<'a, T> {
        unsafe { A::transmute_box(self.0) }
    }
    
    #[inline]
    pub fn try_unbox(self) -> Result<T, Self> {
        match self.into_alloc_box().try_unbox() {
            Ok(value) => Ok(value),
            Err(alloc_box) => Err(Self::from_alloc_box(alloc_box)),
        }
    }
}
impl<'a, T: Debug + GetSpan + GetSpanMut> Box<'a, T> {
    #[inline]
    pub const unsafe fn dangling() -> Self {
        Self(oxc_allocator::Box::dangling(), PhantomData)
    }
    #[inline]
    pub fn unbox(self) -> T {
        self.into_alloc_box().unbox()
    }
}

impl<'old_alloc, T: Debug + GetSpan + GetSpanMut + CloneIn> CloneIn for Box<'old_alloc, T>
where
    for<'a> <T as CloneIn>::Cloned<'a>: Debug + GetSpan + GetSpanMut,
{
    type Cloned<'a> = Box<'a, T::Cloned<'a>>;

    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Box::from_alloc_box(self.deref().clone_in(allocator))
    }
}

impl<'a, T: Debug + GetSpan + GetSpanMut + ContentEq> ContentEq for Box<'a, T> {
    fn content_eq(&self, other: &Self) -> bool {
        self.deref().content_eq(other.deref())
    }
}
impl<'a, T: Debug + GetSpan + GetSpanMut + ContentHash> ContentHash for Box<'a, T> {
    fn content_hash<H: Hasher>(&self, state: &mut H) {
        self.deref().content_hash(state)
    }
}

impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> GetSpan for Box<'a, T, A> {
    fn span(&self) -> Span {
        self.deref().span()
    }
}
impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> GetSpanMut for Box<'a, T, A> {
    fn span_mut(&mut self) -> &mut Span {
        self.deref_mut().span_mut()
    }
}

impl<'a, T: Debug + GetSpan + GetSpanMut, A: AstAllocator> FromIn<'a, T, A> for Box<'a, T, A> {
    fn from_in(value: T, allocator: &'a A) -> Self {
        Self::from_alloc_box(A::Box::from_in(value, allocator))
    }
}


#[cfg(feature = "serialize")]
impl<'a, T: Debug + serde::Serialize + GetSpan + GetSpanMut, A: AstAllocator> serde::Serialize for Box<'a, T, A> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self.specialize_ref() {
            Ok(val) => val.serialize(serializer),
            Err(val) => val.serialize(serializer),
        }
    }
}

#[allow(dead_code)]
mod _test_variance {
    use super::*;
    #[derive(Debug)]
    struct TestAstNodeWithLifetime<'a>(&'a mut Span);
    impl GetSpan for TestAstNodeWithLifetime<'_> {
        fn span(&self) -> Span {
            unimplemented!()
        }
    }
    impl GetSpanMut for TestAstNodeWithLifetime<'_> {
        fn span_mut(&mut self) -> &mut Span {
            unimplemented!()
        }
    }
    fn _assert_box_variance<'a: 'b, 'b>(val: Box<'a, TestAstNodeWithLifetime<'a>>) -> Box<'b, TestAstNodeWithLifetime<'b>> {
        val
    }

    fn _assert_vec_variance<'a: 'b, 'b>(val: Vec<'a, TestAstNodeWithLifetime<'a>>) -> Vec<'b, TestAstNodeWithLifetime<'b>> {
        val
    }
}

