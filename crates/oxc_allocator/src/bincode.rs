use std::ops::Deref;

use bincode::{
    de::{read::Reader as _, BorrowDecoder},
    error::DecodeError,
    BorrowDecode, Decode, Encode,
};

use crate::{Allocator, Box, Vec};

impl<'alloc, T: Encode> Encode for Vec<'alloc, T> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.as_slice().encode(encoder)
    }
}

impl<'alloc, T: Decode<&'alloc Allocator>> Decode<&'alloc Allocator> for Vec<'alloc, T> {
    fn decode<D: bincode::de::Decoder<Ctx = &'alloc Allocator>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let len = u64::decode(decoder)?;
        let len = usize::try_from(len).map_err(|_| DecodeError::OutsideUsizeRange(len))?;

        let allocator = *decoder.ctx();

        if unty::type_equal::<T, u8>() {
            decoder.claim_container_read::<T>(len)?;
            // optimize for reading u8 vecs
            let mut vec = bumpalo::vec![in allocator.deref(); 0u8; len];

            decoder.reader().read(&mut vec)?;
            // Safety: Vec<T> is Vec<u8>
            #[allow(unsafe_code)]
            Ok(Self(unsafe { core::mem::transmute(vec) }))
        } else {
            decoder.claim_container_read::<T>(len)?;

            let mut vec = Vec::with_capacity_in(len, allocator);
            for _ in 0..len {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());

                vec.push(T::decode(decoder)?);
            }
            Ok(vec)
        }
    }
}

impl<'de, 'alloc, T: BorrowDecode<'de, &'alloc Allocator>> BorrowDecode<'de, &'alloc Allocator>
    for Vec<'alloc, T>
{
    fn borrow_decode<D: BorrowDecoder<'de, Ctx = &'alloc Allocator>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let len = u64::decode(decoder)?;
        let len = usize::try_from(len).map_err(|_| DecodeError::OutsideUsizeRange(len))?;
        decoder.claim_container_read::<T>(len)?;

        let allocator = *decoder.ctx();
        if unty::type_equal::<T, u8>() {
            // optimize for reading u8 vecs
            let mut vec = bumpalo::vec![in allocator.deref(); 0u8; len];
            decoder.reader().read(&mut vec)?;
            // Safety: Vec<T> is Vec<u8>
            #[allow(unsafe_code)]
            Ok(Self(unsafe { core::mem::transmute(vec) }))
        } else {
            let mut vec = Vec::with_capacity_in(len, allocator);
            for _ in 0..len {
                // See the documentation on `unclaim_bytes_read` as to why we're doing this here
                decoder.unclaim_bytes_read(core::mem::size_of::<T>());

                vec.push(T::borrow_decode(decoder)?);
            }
            Ok(vec)
        }
    }
}

impl<'alloc, T: Encode> Encode for Box<'alloc, T> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.deref().encode(encoder)
    }
}

impl<'alloc, T: Decode<&'alloc Allocator>> Decode<&'alloc Allocator> for Box<'alloc, T> {
    fn decode<D: bincode::de::Decoder<Ctx = &'alloc Allocator>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let t = T::decode(decoder)?;
        Ok(Box(decoder.ctx().alloc(t)))
    }
}

impl<'de, 'alloc, T: BorrowDecode<'de, &'alloc Allocator>> BorrowDecode<'de, &'alloc Allocator>
    for Box<'alloc, T>
{
    fn borrow_decode<D: BorrowDecoder<'de, Ctx = &'alloc Allocator>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let t = T::borrow_decode(decoder)?;
        Ok(Box(decoder.ctx().alloc(t)))
    }
}

#[cfg(test)]
mod tests {
    use bincode::{config, decode_from_slice_with_ctx, encode_to_vec};

    use super::*;

    #[test]
    fn test_encode_decode_vec() {
        let config = config::standard();
        let allocator = Allocator::default();

        let v: Vec<'_, i32> = Vec::from_iter_in([1, 2, 3], &allocator);
        let bytes = encode_to_vec(&v, config).unwrap();
        assert_eq!(
            decode_from_slice_with_ctx::<_, Vec<'_, i32>, _>(&bytes, config, &allocator).unwrap().0,
            v
        );

        let v: Vec<'_, u8> = Vec::from_iter_in([1, 2, 3], &allocator);
        let bytes = encode_to_vec(&v, config).unwrap();
        assert_eq!(
            decode_from_slice_with_ctx::<_, Vec<'_, u8>, _>(&bytes, config, &allocator).unwrap().0,
            v
        );
    }
}
