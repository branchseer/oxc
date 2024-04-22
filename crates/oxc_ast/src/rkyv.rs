use std::cell::Cell;

use rkyv::{
    with::{ArchiveWith, DeserializeWith, SerializeWith},
    Archive, Archived, Deserialize, Fallible, Resolver, Serialize,
};

/// A wrapper that archives a cell with its contained value.
pub struct CellContent;

impl<T: Archive + Copy> ArchiveWith<Cell<T>> for CellContent {
    type Archived = Archived<T>;

    type Resolver = Resolver<T>;

    #[allow(unsafe_code)]
    unsafe fn resolve_with(
        field: &Cell<T>,
        pos: usize,
        resolver: Self::Resolver,
        out: *mut Self::Archived,
    ) {
        let content = field.get();
        content.resolve(pos, resolver, out)
    }
}

impl<T: Copy, S: Fallible + ?Sized> SerializeWith<Cell<T>, S> for CellContent
where
    T: Serialize<S>,
{
    fn serialize_with(field: &Cell<T>, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
        let content = field.get();
        content.serialize(serializer)
    }
}

impl<T: Archive + Copy, D: Fallible + ?Sized> DeserializeWith<Archived<T>, Cell<T>, D>
    for CellContent
where
    Archived<T>: Deserialize<T, D>,
{
    fn deserialize_with(
        field: &Archived<T>,
        deserializer: &mut D,
    ) -> Result<Cell<T>, <D as Fallible>::Error> {
        let content = field.deserialize(deserializer)?;
        Ok(Cell::new(content))
    }
}
