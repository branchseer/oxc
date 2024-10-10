use syn::parse_quote;

use super::{
    defs::{EnumDef, StructDef, TypeDef},
    with_either,
};

pub trait GetGenerics {
    fn has_lifetime(&self) -> bool {
        false
    }

    fn has_generic_allocator(&self) -> bool;

    fn generics(&self) -> Option<syn::Generics> {
        if self.has_lifetime() {
            Some(parse_quote!(<'a>))
        } else {
            None
        }
    }
    fn generics_with_allocator(&self) -> Option<syn::Generics> {
        if self.has_lifetime() {
            Some(if self.has_generic_allocator() {
                parse_quote!(<'a, A>)
            } else {
                parse_quote!(<'a>)
            })
        } else {
            None
        }
    }
    fn generics_decl_with_allocator(&self) -> Option<syn::Generics> {
        if self.has_lifetime() {
            Some(if self.has_generic_allocator() {
                parse_quote!(<'a, A: AstAllocator>)
            } else {
                parse_quote!(<'a>)
            })
        } else {
            None
        }
    }
}

impl GetGenerics for TypeDef {
    fn has_lifetime(&self) -> bool {
        with_either!(self, it => it.has_lifetime())
    }

    fn has_generic_allocator(&self) -> bool {
        with_either!(self, it => it.has_generic_allocator())
    }
}

impl GetGenerics for StructDef {
    fn has_lifetime(&self) -> bool {
        self.has_lifetime
    }

    fn has_generic_allocator(&self) -> bool {
        self.has_generic_allocator
    }
}

impl GetGenerics for EnumDef {
    fn has_lifetime(&self) -> bool {
        self.has_lifetime
    }

    fn has_generic_allocator(&self) -> bool {
        self.has_generic_allocator
    }
}
