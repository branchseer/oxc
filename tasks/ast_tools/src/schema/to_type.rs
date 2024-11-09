use proc_macro2::TokenStream;
use quote::{format_ident, ToTokens};
use syn::{parse_quote, PathSegment};

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

// `Vec<...>` -> `Vec<..., A>`
struct InsertAllocatorGenericParam;
impl syn::visit_mut::VisitMut for InsertAllocatorGenericParam {
    fn visit_path_segment_mut(&mut self, path_segment: &mut PathSegment) {
        if path_segment.ident == "Vec" || path_segment.ident == "Box" {
            let syn::PathArguments::AngleBracketed(arguments) = &mut path_segment.arguments else {
                panic!("Vec/Box not followed by <...>");
            };
            arguments.args.push(syn::GenericArgument::Type(syn::parse_str("A").unwrap()))
        }
        syn::visit_mut::visit_path_segment_mut(self, path_segment);
    }
}

impl ToType for TypeRef {
    fn to_type(&self) -> syn::Type {
        syn::parse_str(self.raw()).unwrap()
    }
    fn to_type_with_generic_allocator(&self) -> syn::Type {
        let mut ty = self.to_type();
        syn::visit_mut::VisitMut::visit_type_mut(&mut InsertAllocatorGenericParam, &mut ty);
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
