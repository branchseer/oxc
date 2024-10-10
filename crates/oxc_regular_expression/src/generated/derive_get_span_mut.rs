// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_span.rs`

#![allow(clippy::match_same_arms)]

use oxc_span::{ast_alloc::AstAllocator, GetSpanMut, Span};

#[allow(clippy::wildcard_imports)]
use crate::ast::*;

impl<'a, A: AstAllocator> GetSpanMut for Pattern<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Disjunction<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Alternative<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Term<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::BoundaryAssertion(it) => GetSpanMut::span_mut(it),
            Self::LookAroundAssertion(it) => GetSpanMut::span_mut(it),
            Self::Quantifier(it) => GetSpanMut::span_mut(it),
            Self::Character(it) => GetSpanMut::span_mut(it),
            Self::Dot(it) => GetSpanMut::span_mut(it),
            Self::CharacterClassEscape(it) => GetSpanMut::span_mut(it),
            Self::UnicodePropertyEscape(it) => GetSpanMut::span_mut(it),
            Self::CharacterClass(it) => GetSpanMut::span_mut(it),
            Self::CapturingGroup(it) => GetSpanMut::span_mut(it),
            Self::IgnoreGroup(it) => GetSpanMut::span_mut(it),
            Self::IndexedReference(it) => GetSpanMut::span_mut(it),
            Self::NamedReference(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl GetSpanMut for BoundaryAssertion {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for LookAroundAssertion<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Quantifier<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for Character {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for CharacterClassEscape {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for UnicodePropertyEscape<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for Dot {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CharacterClass<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CharacterClassContents<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::CharacterClassRange(it) => GetSpanMut::span_mut(it),
            Self::CharacterClassEscape(it) => GetSpanMut::span_mut(it),
            Self::UnicodePropertyEscape(it) => GetSpanMut::span_mut(it),
            Self::Character(it) => GetSpanMut::span_mut(it),
            Self::NestedCharacterClass(it) => GetSpanMut::span_mut(it),
            Self::ClassStringDisjunction(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl GetSpanMut for CharacterClassRange {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ClassStringDisjunction<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ClassString<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CapturingGroup<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for IgnoreGroup<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for Modifiers {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for IndexedReference {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for NamedReference<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}
