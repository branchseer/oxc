// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/clone_in.rs`

#![allow(clippy::default_trait_access)]

use oxc_allocator::{Allocator, CloneIn};

#[allow(clippy::wildcard_imports)]
use crate::ast::*;

impl<'old_alloc> CloneIn for Pattern<'old_alloc> {
    type Cloned<'a> = Pattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Pattern {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Disjunction<'old_alloc> {
    type Cloned<'a> = Disjunction<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Disjunction {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Alternative<'old_alloc> {
    type Cloned<'a> = Alternative<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Alternative {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Term<'old_alloc> {
    type Cloned<'a> = Term<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::BoundaryAssertion(it) => {
                Term::BoundaryAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::LookAroundAssertion(it) => {
                Term::LookAroundAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::Quantifier(it) => Term::Quantifier(CloneIn::clone_in(it, allocator)),
            Self::Character(it) => Term::Character(CloneIn::clone_in(it, allocator)),
            Self::Dot(it) => Term::Dot(CloneIn::clone_in(it, allocator)),
            Self::CharacterClassEscape(it) => {
                Term::CharacterClassEscape(CloneIn::clone_in(it, allocator))
            }
            Self::UnicodePropertyEscape(it) => {
                Term::UnicodePropertyEscape(CloneIn::clone_in(it, allocator))
            }
            Self::CharacterClass(it) => Term::CharacterClass(CloneIn::clone_in(it, allocator)),
            Self::CapturingGroup(it) => Term::CapturingGroup(CloneIn::clone_in(it, allocator)),
            Self::IgnoreGroup(it) => Term::IgnoreGroup(CloneIn::clone_in(it, allocator)),
            Self::IndexedReference(it) => Term::IndexedReference(CloneIn::clone_in(it, allocator)),
            Self::NamedReference(it) => Term::NamedReference(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl CloneIn for BoundaryAssertion {
    type Cloned<'a> = BoundaryAssertion;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BoundaryAssertion {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
        }
    }
}

impl CloneIn for BoundaryAssertionKind {
    type Cloned<'a> = BoundaryAssertionKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Start => BoundaryAssertionKind::Start,
            Self::End => BoundaryAssertionKind::End,
            Self::Boundary => BoundaryAssertionKind::Boundary,
            Self::NegativeBoundary => BoundaryAssertionKind::NegativeBoundary,
        }
    }
}

impl<'old_alloc> CloneIn for LookAroundAssertion<'old_alloc> {
    type Cloned<'a> = LookAroundAssertion<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        LookAroundAssertion {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl CloneIn for LookAroundAssertionKind {
    type Cloned<'a> = LookAroundAssertionKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Lookahead => LookAroundAssertionKind::Lookahead,
            Self::NegativeLookahead => LookAroundAssertionKind::NegativeLookahead,
            Self::Lookbehind => LookAroundAssertionKind::Lookbehind,
            Self::NegativeLookbehind => LookAroundAssertionKind::NegativeLookbehind,
        }
    }
}

impl<'old_alloc> CloneIn for Quantifier<'old_alloc> {
    type Cloned<'a> = Quantifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Quantifier {
            span: CloneIn::clone_in(&self.span, allocator),
            min: CloneIn::clone_in(&self.min, allocator),
            max: CloneIn::clone_in(&self.max, allocator),
            greedy: CloneIn::clone_in(&self.greedy, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl CloneIn for Character {
    type Cloned<'a> = Character;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Character {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl CloneIn for CharacterKind {
    type Cloned<'a> = CharacterKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ControlLetter => CharacterKind::ControlLetter,
            Self::HexadecimalEscape => CharacterKind::HexadecimalEscape,
            Self::Identifier => CharacterKind::Identifier,
            Self::Null => CharacterKind::Null,
            Self::Octal1 => CharacterKind::Octal1,
            Self::Octal2 => CharacterKind::Octal2,
            Self::Octal3 => CharacterKind::Octal3,
            Self::SingleEscape => CharacterKind::SingleEscape,
            Self::Symbol => CharacterKind::Symbol,
            Self::UnicodeEscape => CharacterKind::UnicodeEscape,
        }
    }
}

impl CloneIn for CharacterClassEscape {
    type Cloned<'a> = CharacterClassEscape;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CharacterClassEscape {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
        }
    }
}

impl CloneIn for CharacterClassEscapeKind {
    type Cloned<'a> = CharacterClassEscapeKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::D => CharacterClassEscapeKind::D,
            Self::NegativeD => CharacterClassEscapeKind::NegativeD,
            Self::S => CharacterClassEscapeKind::S,
            Self::NegativeS => CharacterClassEscapeKind::NegativeS,
            Self::W => CharacterClassEscapeKind::W,
            Self::NegativeW => CharacterClassEscapeKind::NegativeW,
        }
    }
}

impl<'old_alloc> CloneIn for UnicodePropertyEscape<'old_alloc> {
    type Cloned<'a> = UnicodePropertyEscape<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        UnicodePropertyEscape {
            span: CloneIn::clone_in(&self.span, allocator),
            negative: CloneIn::clone_in(&self.negative, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl CloneIn for Dot {
    type Cloned<'a> = Dot;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Dot { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for CharacterClass<'old_alloc> {
    type Cloned<'a> = CharacterClass<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CharacterClass {
            span: CloneIn::clone_in(&self.span, allocator),
            negative: CloneIn::clone_in(&self.negative, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl CloneIn for CharacterClassContentsKind {
    type Cloned<'a> = CharacterClassContentsKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Union => CharacterClassContentsKind::Union,
            Self::Intersection => CharacterClassContentsKind::Intersection,
            Self::Subtraction => CharacterClassContentsKind::Subtraction,
        }
    }
}

impl<'old_alloc> CloneIn for CharacterClassContents<'old_alloc> {
    type Cloned<'a> = CharacterClassContents<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::CharacterClassRange(it) => {
                CharacterClassContents::CharacterClassRange(CloneIn::clone_in(it, allocator))
            }
            Self::CharacterClassEscape(it) => {
                CharacterClassContents::CharacterClassEscape(CloneIn::clone_in(it, allocator))
            }
            Self::UnicodePropertyEscape(it) => {
                CharacterClassContents::UnicodePropertyEscape(CloneIn::clone_in(it, allocator))
            }
            Self::Character(it) => {
                CharacterClassContents::Character(CloneIn::clone_in(it, allocator))
            }
            Self::NestedCharacterClass(it) => {
                CharacterClassContents::NestedCharacterClass(CloneIn::clone_in(it, allocator))
            }
            Self::ClassStringDisjunction(it) => {
                CharacterClassContents::ClassStringDisjunction(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for CharacterClassRange {
    type Cloned<'a> = CharacterClassRange;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CharacterClassRange {
            span: CloneIn::clone_in(&self.span, allocator),
            min: CloneIn::clone_in(&self.min, allocator),
            max: CloneIn::clone_in(&self.max, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ClassStringDisjunction<'old_alloc> {
    type Cloned<'a> = ClassStringDisjunction<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ClassStringDisjunction {
            span: CloneIn::clone_in(&self.span, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ClassString<'old_alloc> {
    type Cloned<'a> = ClassString<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ClassString {
            span: CloneIn::clone_in(&self.span, allocator),
            strings: CloneIn::clone_in(&self.strings, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for CapturingGroup<'old_alloc> {
    type Cloned<'a> = CapturingGroup<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CapturingGroup {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for IgnoreGroup<'old_alloc> {
    type Cloned<'a> = IgnoreGroup<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        IgnoreGroup {
            span: CloneIn::clone_in(&self.span, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl CloneIn for Modifiers {
    type Cloned<'a> = Modifiers;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Modifiers {
            span: CloneIn::clone_in(&self.span, allocator),
            enabling: CloneIn::clone_in(&self.enabling, allocator),
            disabling: CloneIn::clone_in(&self.disabling, allocator),
        }
    }
}

impl CloneIn for Modifier {
    type Cloned<'a> = Modifier;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Modifier {
            ignore_case: CloneIn::clone_in(&self.ignore_case, allocator),
            multiline: CloneIn::clone_in(&self.multiline, allocator),
            sticky: CloneIn::clone_in(&self.sticky, allocator),
        }
    }
}

impl CloneIn for IndexedReference {
    type Cloned<'a> = IndexedReference;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        IndexedReference {
            span: CloneIn::clone_in(&self.span, allocator),
            index: CloneIn::clone_in(&self.index, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for NamedReference<'old_alloc> {
    type Cloned<'a> = NamedReference<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        NamedReference {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}
