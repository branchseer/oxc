use super::traits::{Box, Sealed, Vec};
use derive_where::derive_where;
use oxc_allocator::FromIn;
use std::fmt::Debug;
use std::marker::PhantomData;

use crate::{GetSpan, GetSpanMut, Span};

#[derive(Default, Debug)]
pub struct VoidAllocator(());

impl VoidAllocator {
    pub const fn new() -> Self {
        VoidAllocator(())
    }
}

impl Sealed for VoidAllocator {}

#[derive_where(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize), serde(bound = ""))]
pub struct VoidBox<'a, T: ?Sized> {
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

impl<'a, T> Box<'a> for VoidBox<'a, T> {
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

    fn specialize_ref(
        &self,
    ) -> Result<&oxc_allocator::Box<'a, Self::Target>, &VoidBox<'a, Self::Target>> {
        Err(&self)
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

impl<'a, T> Vec<'a> for VoidVec<'a, T> {
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

unsafe impl super::AstAllocator for VoidAllocator {
    const IS_VOID: bool = true;
    type Box<'a, T: Debug + GetSpan + GetSpanMut> = VoidBox<'a, T>;
    type Vec<'a, T: Debug> = VoidVec<'a, T>;

    fn alloc<'a, T: Debug + GetSpan + GetSpanMut>(&'a self, value: T) -> super::Box<'a, T, Self> {
        super::Box::from_alloc_box(VoidBox::from_in(value, self))
    }

    fn alloc_str<'a>(&'a self, src: &str) -> &'a str {
        ""
    }

    #[inline]
    fn box_from_span<'a, T: Debug + GetSpan + GetSpanMut>(
        span: Span,
    ) -> Option<super::Box<'a, T, Self>> {
        Some(super::Box::from_alloc_box(VoidBox { span, _phantom: PhantomData }))
    }

    #[inline]
    fn vec<'a, T: Debug>(&'a self) -> super::Vec<'a, T, Self> {
        super::Vec::from_alloc_vec(VoidVec(PhantomData))
    }

    #[inline]
    fn vec_with_capacity<'a, T: Debug>(&'a self, _capacity: usize) -> super::Vec<'a, T, Self> {
        super::Vec::from_alloc_vec(VoidVec(PhantomData))
    }

    #[inline]
    fn vec_from_iter<'a, T: Debug, I: IntoIterator<Item = T>>(
        &'a self,
        _iter: I,
    ) -> super::Vec<'a, T, Self> {
        super::Vec::from_alloc_vec(VoidVec(PhantomData))
    }

    unsafe fn transmute_vec<'a, 'b, T1: Debug, T2: Debug>(
        val: Self::Vec<'a, T1>,
    ) -> Self::Vec<'b, T2> {
        unsafe { std::mem::transmute(val) }
    }

    unsafe fn transmute_box<
        'a,
        'b,
        T1: Debug + GetSpan + GetSpanMut,
        T2: Debug + GetSpan + GetSpanMut,
    >(
        val: Self::Box<'a, T1>,
    ) -> Self::Box<'b, T2> {
        unsafe { std::mem::transmute(val) }
    }
}
