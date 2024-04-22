use std::{cmp, convert::Infallible};

use crate::{Allocator, Box, Vec};
use rkyv::{
    boxed::{ArchivedBox, BoxResolver},
    ser::{ScratchSpace, Serializer},
    string::ArchivedString,
    vec::{ArchivedVec, VecResolver},
    Archive, ArchivePointee, ArchiveUnsized, Archived, Deserialize, DeserializeUnsized, Fallible,
    Serialize, SerializeUnsized,
};

impl<'alloc> Fallible for &'alloc Allocator {
    type Error = Infallible;
}

impl<'alloc, T> Archive for Vec<'alloc, T>
where
    T: Archive,
{
    type Archived = ArchivedVec<Archived<T>>;

    type Resolver = VecResolver;

    #[allow(unsafe_code)]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedVec::resolve_from_slice(self.as_slice(), pos, resolver, out)
    }
}

impl<'alloc, T, S> Serialize<S> for Vec<'alloc, T>
where
    T: Serialize<S>,
    S: ScratchSpace + Serializer + ?Sized,
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedVec::serialize_from_slice(self.as_slice(), serializer)
    }
}

impl<'alloc, T: Archive> Deserialize<Vec<'alloc, T>, &'alloc Allocator> for ArchivedVec<Archived<T>>
where
    Archived<T>: Deserialize<T, &'alloc Allocator>,
{
    fn deserialize(
        &self,
        deserializer: &mut &'alloc Allocator,
    ) -> Result<Vec<'alloc, T>, Infallible> {
        let mut result = Vec::with_capacity_in(self.len(), *deserializer);
        for item in self.as_slice() {
            result.push(item.deserialize(deserializer)?)
        }
        Ok(result)
    }
}

impl<'alloc, T: ArchiveUnsized + ?Sized> Archive for Box<'alloc, T> {
    type Archived = ArchivedBox<T::Archived>;
    type Resolver = BoxResolver<T::MetadataResolver>;

    #[inline]
    #[allow(unsafe_code)]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        ArchivedBox::resolve_from_ref(&**self, pos, resolver, out);
    }
}

impl<'alloc, T: SerializeUnsized<S> + ?Sized, S: Fallible + ?Sized> Serialize<S>
    for Box<'alloc, T>
{
    #[inline]
    fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        ArchivedBox::serialize_from_ref(&**self, serializer)
    }
}

impl<'alloc, T> Deserialize<Box<'alloc, T>, &'alloc Allocator> for ArchivedBox<T::Archived>
where
    T: ArchiveUnsized + ?Sized,
    T::Archived: DeserializeUnsized<T, &'alloc Allocator>,
{
    #[inline]
    #[allow(unsafe_code)]
    fn deserialize(
        &self,
        deserializer: &mut &'alloc Allocator,
    ) -> Result<Box<'alloc, T>, Infallible> {
        let allocator = *deserializer;
        unsafe {
            let data_address = self.get().deserialize_unsized(deserializer, |layout| {
                allocator.alloc_layout(layout).as_ptr()
            })?;
            let metadata = self.get().deserialize_metadata(deserializer)?;
            let ptr = ptr_meta::from_raw_parts_mut(data_address, metadata);
            Ok(Box::from_raw(ptr))
        }
    }
}

impl<'alloc, T: ArchivePointee + PartialEq<U> + ?Sized, U: ?Sized> PartialEq<Box<'alloc, U>>
    for ArchivedBox<T>
{
    #[inline]
    fn eq(&self, other: &Box<U>) -> bool {
        self.get().eq(&**other)
    }
}

impl<'alloc, T: ArchivePointee + PartialOrd<U> + ?Sized, U: ?Sized> PartialOrd<Box<'alloc, U>>
    for ArchivedBox<T>
{
    #[inline]
    fn partial_cmp(&self, other: &Box<U>) -> Option<cmp::Ordering> {
        self.get().partial_cmp(&**other)
    }
}

impl<'alloc> Deserialize<&'alloc str, &'alloc Allocator> for ArchivedString {
    fn deserialize(&self, deserializer: &mut &'alloc Allocator) -> Result<&'alloc str, Infallible> {
        Ok(deserializer.alloc_str(self.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use rkyv::Deserialize;

    use crate::{Allocator, Box, Vec};

    #[test]
    fn rkyv_box() {
        let allocator = Allocator::default();

        let value = Box::new_in(42u8, &allocator);

        let bytes = rkyv::to_bytes::<_, 0>(&value).unwrap();

        #[allow(unsafe_code)]
        let archived = unsafe { rkyv::archived_root::<Box<'_, u8>>(bytes.as_slice()) };

        let deserialized: Box<'_, u8> = archived.deserialize(&mut &allocator).unwrap();

        assert_eq!(*deserialized, 42);
    }

    #[test]
    fn rkyv_vec() {
        let allocator = Allocator::default();

        let value = Vec::from_iter_in([1u8, 2, 3], &allocator);

        let bytes = rkyv::to_bytes::<_, 0>(&value).unwrap();

        #[allow(unsafe_code)]
        let archived = unsafe { rkyv::archived_root::<Vec<'_, u8>>(bytes.as_slice()) };

        let deserialized: Vec<'_, u8> = archived.deserialize(&mut &allocator).unwrap();

        assert_eq!(deserialized.as_slice(), [1, 2, 3]);
    }
}
