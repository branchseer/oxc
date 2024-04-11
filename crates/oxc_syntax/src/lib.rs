//! Common code for JavaScript Syntax

pub mod class;
pub mod identifier;
pub mod keyword;
pub mod module_graph_visitor;
pub mod module_record;
pub mod node;
pub mod operator;
pub mod precedence;
pub mod reference;
pub mod scope;
pub mod symbol;
pub mod xml_entities;

#[cfg(feature = "bincode")]
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
pub enum NumberBase {
    Float,
    Decimal,
    Binary,
    Octal,
    Hex,
}

impl NumberBase {
    pub fn is_base_10(&self) -> bool {
        matches!(self, Self::Float | Self::Decimal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(Decode, Encode))]
pub enum BigintBase {
    Decimal,
    Binary,
    Octal,
    Hex,
}

impl BigintBase {
    pub fn is_base_10(&self) -> bool {
        self == &Self::Decimal
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_bincode_for_bitflags {
    ($t:ty) => {
        impl ::bincode::Encode for $t {
            fn encode<E: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), ::bincode::error::EncodeError> {
                self.bits().encode(encoder)
            }
        }

        impl<C> ::bincode::Decode<C> for $t {
            fn decode<D: ::bincode::de::Decoder<Ctx = C>>(
                decoder: &mut D,
            ) -> Result<Self, ::bincode::error::DecodeError> {
                let bits = <Self as ::bitflags::Flags>::Bits::decode(decoder)?;
                Ok(Self::from_bits_retain(bits))
            }
        }
        ::bincode::impl_borrow_decode!($t);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_bincode_for_index_type {
    ($t:ty, $raw_type:ty) => {
        impl ::bincode::Encode for $t {
            fn encode<E: ::bincode::enc::Encoder>(
                &self,
                encoder: &mut E,
            ) -> Result<(), ::bincode::error::EncodeError> {
                self.raw().encode(encoder)
            }
        }

        impl<C> ::bincode::Decode<C> for $t {
            fn decode<D: ::bincode::de::Decoder<Ctx = C>>(
                decoder: &mut D,
            ) -> Result<Self, ::bincode::error::DecodeError> {
                let raw = <$raw_type as ::bincode::Decode<C>>::decode(decoder)?;
                Ok(Self::from_raw(raw))
            }
        }
        ::bincode::impl_borrow_decode!($t);
    };
}
