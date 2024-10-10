use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::parse_quote;

use super::{
    defs::{EnumDef, StructDef, TypeDef, TypeRef},
    GetGenerics, GetIdent,
};

pub trait ToType {
    fn to_type(&self) -> syn::Type;

    fn to_type_with_generic_allocator(&self) -> syn::Type;
    fn to_type_elide(&self) -> syn::Type;
    fn to_type_with_explicit_generics(&self, generics: TokenStream) -> syn::Type;
}

// `Vec<...>` -> `A::Vec<...>`
struct InsertAllocatorPrefix;
impl syn::visit_mut::VisitMut for InsertAllocatorPrefix {
    fn visit_type_path_mut(&mut self, type_path: &mut syn::TypePath) {
        if type_path
            .path
            .segments
            .first()
            .is_some_and(|first_seg| first_seg.ident == "Vec" || first_seg.ident == "Box")
        {
            type_path.path.segments.insert(
                0,
                syn::PathSegment { ident: format_ident!("A"), arguments: syn::PathArguments::None },
            );
        }
        syn::visit_mut::visit_type_path_mut(self, type_path)
    }
}

impl ToType for TypeRef {
    fn to_type(&self) -> syn::Type {
        syn::parse_str(self.raw()).unwrap()
    }
    fn to_type_with_generic_allocator(&self) -> syn::Type {
        let mut ty = self.to_type();
        syn::visit_mut::VisitMut::visit_type_mut(&mut InsertAllocatorPrefix, &mut ty);
        ty
    }

    fn to_type_elide(&self) -> syn::Type {
        self.to_type_with_explicit_generics(proc_macro2::TokenStream::default())
    }

    fn to_type_with_explicit_generics(&self, generics: proc_macro2::TokenStream) -> syn::Type {
        let ident = self.name().first_ident();
        parse_quote!(#ident #generics)
    }
}

auto_impl_to_type! {
    TypeDef,
    EnumDef,
    StructDef,
}

macro_rules! auto_impl_to_type {
    ($($ty:ty,)+) => (
        $(
            impl ToType for $ty {
                fn to_type(&self) -> syn::Type {
                    self.to_type_with_explicit_generics(self.generics().to_token_stream())
                }
                fn to_type_with_generic_allocator(&self) -> syn::Type {
                    self.to_type_with_explicit_generics(self.generics_with_allocator().to_token_stream())
                }
                fn to_type_elide(&self) -> syn::Type {
                    self.to_type_with_explicit_generics(TokenStream::default())
                }

                fn to_type_with_explicit_generics(&self, generics: TokenStream) -> syn::Type {
                    let name = self.ident();
                    parse_quote!(#name #generics)
                }
            }
        )+
    )
}

use auto_impl_to_type;
