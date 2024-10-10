// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_span.rs`

#![allow(clippy::match_same_arms)]

use oxc_span::{ast_alloc::AstAllocator, GetSpan, Span};

#[allow(clippy::wildcard_imports)]
use crate::ast::*;

impl<'a, A: AstAllocator> GetSpan for Pattern<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Disjunction<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Alternative<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Term<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::BoundaryAssertion(it) => GetSpan::span(it),
            Self::LookAroundAssertion(it) => GetSpan::span(it),
            Self::Quantifier(it) => GetSpan::span(it),
            Self::Character(it) => GetSpan::span(it),
            Self::Dot(it) => GetSpan::span(it),
            Self::CharacterClassEscape(it) => GetSpan::span(it),
            Self::UnicodePropertyEscape(it) => GetSpan::span(it),
            Self::CharacterClass(it) => GetSpan::span(it),
            Self::CapturingGroup(it) => GetSpan::span(it),
            Self::IgnoreGroup(it) => GetSpan::span(it),
            Self::IndexedReference(it) => GetSpan::span(it),
            Self::NamedReference(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for BoundaryAssertion {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for LookAroundAssertion<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Quantifier<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Character {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for CharacterClassEscape {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for UnicodePropertyEscape<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Dot {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CharacterClass<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CharacterClassContents<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::CharacterClassRange(it) => GetSpan::span(it),
            Self::CharacterClassEscape(it) => GetSpan::span(it),
            Self::UnicodePropertyEscape(it) => GetSpan::span(it),
            Self::Character(it) => GetSpan::span(it),
            Self::NestedCharacterClass(it) => GetSpan::span(it),
            Self::ClassStringDisjunction(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for CharacterClassRange {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ClassStringDisjunction<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ClassString<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CapturingGroup<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for IgnoreGroup<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Modifiers {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for IndexedReference {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for NamedReference<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
