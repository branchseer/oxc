use derive_where::derive_where;
use oxc_allocator::FromIn;
use std::marker::PhantomData;

use crate::ast_alloc::AstNode;
use crate::{GetSpan, GetSpanMut, Span};

#[derive(Default, Debug)]
pub struct VoidAllocator(());

impl VoidAllocator {
    pub const fn new() -> Self {
        VoidAllocator(())
    }
}

impl super::Sealed for VoidAllocator {}

#[derive_where(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(bound = ""))]
pub struct VoidBox<'a, T> {
    span: Span,
    _phantom: PhantomData<(&'a (), T)>,
}

impl<'a, T> GetSpan for VoidBox<'a, T> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, T> GetSpanMut for VoidBox<'a, T> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, T: GetSpan> FromIn<'a, T, VoidAllocator> for VoidBox<'a, T> {
    #[inline]
    fn from_in(value: T, _allocator: &'a VoidAllocator) -> Self {
        Self { span: value.span(), _phantom: PhantomData }
    }
}

impl<'a, T> super::Box<'a> for VoidBox<'a, T> {
    type Target = T;

    #[inline]
    fn deref_or_else<U, FSome: FnOnce(&Self::Target) -> U, FNone: FnOnce() -> U>(
        &self,
        _f_some: FSome,
        f_none: FNone,
    ) -> U {
        f_none()
    }

    #[inline]
    fn unbox_or_else<U, FSome: FnOnce(Self::Target) -> U, FNone: FnOnce(Self) -> U>(
        self,
        _f_some: FSome,
        f_none: FNone,
    ) -> U
    where
        Self::Target: Sized,
    {
        f_none(self)
    }

    fn try_deref(&self) -> Option<&Self::Target> {
        None
    }

    fn try_unbox(self) -> Result<Self::Target, Self>
    where
        Self::Target: Sized,
    {
        Err(self)
    }
}

#[derive_where(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(bound = ""))]
pub struct VoidVec<'a, T>(PhantomData<(&'a (), T)>);

impl<'a, T> TryFrom<VoidVec<'a, T>> for oxc_allocator::Vec<'a, T> {
    type Error = VoidVec<'a, T>;

    fn try_from(value: VoidVec<'a, T>) -> Result<Self, Self::Error> {
        Err(value)
    }
}

impl<'a, T> super::Vec<'a> for VoidVec<'a, T> {
    type Item = T;
    type Iterator = std::iter::Empty<Self::Item>;

    #[inline]
    fn push(&mut self, val: Self::Item) {}

    #[inline]
    fn append(&mut self, other: &mut Self) {}

    #[inline]
    fn try_as_slice(&self) -> Option<&[Self::Item]> {
        None
    }

    #[inline]
    fn try_as_mut_slice(&mut self) -> Option<&mut [Self::Item]> {
        None
    }

    fn into_iter_or_empty(self) -> Self::Iterator {
        std::iter::empty()
    }
    fn specialize(self) -> Result<oxc_allocator::Vec<'a, Self::Item>, VoidVec<'a, Self::Item>> {
        Err(self)
    }
    fn specialize_ref(
        &self,
    ) -> Result<&oxc_allocator::Vec<'a, Self::Item>, &VoidVec<'a, Self::Item>> {
        Err(self)
    }
    #[inline]
    fn specialize_mut(
        &mut self,
    ) -> Result<&mut oxc_allocator::Vec<'a, Self::Item>, &mut VoidVec<'a, Self::Item>> {
        Err(self)
    }
}

impl super::AstAllocator for VoidAllocator {
    const IS_VOID: bool = true;
    type Box<'a, T: AstNode + GetSpan + GetSpanMut> = VoidBox<'a, T>;
    type Vec<'a, T: AstNode> = VoidVec<'a, T>;

    fn alloc<'a, T: AstNode + GetSpan + GetSpanMut>(&'a self, value: T) -> Self::Box<'a, T> {
        VoidBox::from_in(value, self)
    }

    fn alloc_str<'a>(&'a self, src: &str) -> &'a str {
        ""
    }

    #[inline]
    fn box_from_span<'a, T: AstNode + GetSpan + GetSpanMut>(
        span: Span,
    ) -> Option<Self::Box<'a, T>> {
        Some(VoidBox { span, _phantom: PhantomData })
    }

    fn vec<'a, T: AstNode>(&'a self) -> Self::Vec<'a, T> {
        VoidVec(PhantomData)
    }

    fn vec_with_capacity<'a, T: AstNode>(&'a self, capacity: usize) -> Self::Vec<'a, T> {
        VoidVec(PhantomData)
    }

    fn vec_from_iter<'a, T: AstNode, I: IntoIterator<Item = T>>(
        &'a self,
        iter: I,
    ) -> Self::Vec<'a, T> {
        VoidVec(PhantomData)
    }
}
