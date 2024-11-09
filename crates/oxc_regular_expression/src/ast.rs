// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use derive_where::derive_where;
use oxc_allocator::CloneIn;
use oxc_ast_macros::ast;
use oxc_span::{
    ast_alloc::{AstAllocator, Vec, Box}, cmp::ContentEq, hash::ContentHash, Atom, GetSpan, GetSpanMut, Span,
};
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use tsify::Tsify;

/// The root of the `PatternParser` result.
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Pattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: Disjunction<'a, A>,
}

/// Pile of [`Alternative`]s separated by `|`.
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Disjunction<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: Vec<'a, Alternative<'a, A>, A>,
}

/// Single unit of `|` separated alternatives.
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Alternative<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: Vec<'a, Term<'a, A>, A>,
}

/// Single unit of [`Alternative`], containing various kinds.
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum Term<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // Assertion, QuantifiableAssertion
    BoundaryAssertion(Box<'a, BoundaryAssertion, A>) = 0,
    LookAroundAssertion(Box<'a, LookAroundAssertion<'a, A>, A>) = 1,
    // Quantifier
    Quantifier(Box<'a, Quantifier<'a, A>, A>) = 2,
    // Atom, ExtendedAtom
    Character(Box<'a, Character, A>) = 3,
    Dot(Dot) = 4,
    CharacterClassEscape(Box<'a, CharacterClassEscape, A>) = 5,
    UnicodePropertyEscape(Box<'a, UnicodePropertyEscape<'a>, A>) = 6,
    CharacterClass(Box<'a, CharacterClass<'a, A>, A>) = 7,
    CapturingGroup(Box<'a, CapturingGroup<'a, A>, A>) = 8,
    IgnoreGroup(Box<'a, IgnoreGroup<'a, A>, A>) = 9,
    IndexedReference(Box<'a, IndexedReference, A>) = 10,
    NamedReference(Box<'a, NamedReference<'a>, A>) = 11,
}

/// Simple form of assertion.
/// e.g. `^`, `$`, `\b`, `\B`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct BoundaryAssertion {
    pub span: Span,
    pub kind: BoundaryAssertionKind,
}

#[ast]
#[derive(Debug, Clone, PartialEq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum BoundaryAssertionKind {
    Start = 0,
    End = 1,
    Boundary = 2,
    NegativeBoundary = 3,
}

/// Lookaround assertion.
/// e.g. `(?=...)`, `(?!...)`, `(?<=...)`, `(?<!...)`
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct LookAroundAssertion<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub kind: LookAroundAssertionKind,
    pub body: Disjunction<'a, A>,
}

#[ast]
#[derive(Debug, Clone, PartialEq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum LookAroundAssertionKind {
    Lookahead = 0,
    NegativeLookahead = 1,
    Lookbehind = 2,
    NegativeLookbehind = 3,
}

/// Quantifier holding a [`Term`] and its repetition count.
/// e.g. `a*`, `b+`, `c?`, `d{3}`, `e{4,}`, `f{5,6}`
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Quantifier<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub min: u64,
    /// `None` means no upper bound.
    pub max: Option<u64>,
    pub greedy: bool,
    pub body: Term<'a, A>,
}

/// Single character.
#[ast]
#[derive(Debug, Clone, Copy)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Character {
    /// This will be invalid position when `UnicodeMode` is disabled and `value` is a surrogate pair.
    #[serde(flatten)]
    pub span: Span,
    pub kind: CharacterKind,
    /// Unicode code point or UTF-16 code unit.
    pub value: u32,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum CharacterKind {
    ControlLetter = 0,
    HexadecimalEscape = 1,
    Identifier = 2,
    Null = 3,
    // To distinguish leading 0 cases like `\00` and `\000`
    Octal1 = 4,
    Octal2 = 5,
    Octal3 = 6,
    SingleEscape = 7,
    Symbol = 8,
    UnicodeEscape = 9,
}

/// Character class.
/// e.g. `\d`, `\D`, `\s`, `\S`, `\w`, `\W`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct CharacterClassEscape {
    #[serde(flatten)]
    pub span: Span,
    pub kind: CharacterClassEscapeKind,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum CharacterClassEscapeKind {
    D = 0,
    NegativeD = 1,
    S = 2,
    NegativeS = 3,
    W = 4,
    NegativeW = 5,
}

/// Unicode property.
/// e.g. `\p{ASCII}`, `\P{ASCII}`, `\p{sc=Hiragana}`, `\P{sc=Hiragana}`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct UnicodePropertyEscape<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub negative: bool,
    /// `true` if `UnicodeSetsMode` and `name` matches unicode property of strings.
    pub strings: bool,
    pub name: Atom<'a>,
    pub value: Option<Atom<'a>>,
}

/// The `.`.
#[ast]
#[derive(Debug, Clone, Copy)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Dot {
    #[serde(flatten)]
    pub span: Span,
}

/// Character class wrapped by `[]`.
/// e.g. `[a-z]`, `[^A-Z]`, `[abc]`, `[a&&b&&c]`, `[[a-z]--x--y]`
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct CharacterClass<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub negative: bool,
    /// `true` if:
    /// - `body` contains [`UnicodePropertyEscape`], nested [`CharacterClass`] or [`ClassStringDisjunction`] which `strings` is `true`
    /// - and matches each logic depends on `kind`
    pub strings: bool,
    pub kind: CharacterClassContentsKind,
    pub body: Vec<'a, CharacterClassContents<'a, A>, A>,
}

#[ast]
#[derive(Debug, PartialEq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum CharacterClassContentsKind {
    Union = 0,
    /// `UnicodeSetsMode` only.
    Intersection = 1,
    /// `UnicodeSetsMode` only.
    Subtraction = 2,
}

#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum CharacterClassContents<'a, A: AstAllocator = oxc_allocator::Allocator> {
    CharacterClassRange(Box<'a, CharacterClassRange, A>) = 0,
    CharacterClassEscape(Box<'a, CharacterClassEscape, A>) = 1,
    UnicodePropertyEscape(Box<'a, UnicodePropertyEscape<'a>, A>) = 2,
    Character(Box<'a, Character, A>) = 3,
    /// `UnicodeSetsMode` only
    NestedCharacterClass(Box<'a, CharacterClass<'a, A>, A>) = 4,
    /// `UnicodeSetsMode` only
    ClassStringDisjunction(Box<'a, ClassStringDisjunction<'a, A>, A>) = 5,
}

/// `-` separated range of characters.
/// e.g. `a-z`, `A-Z`, `0-9`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct CharacterClassRange {
    #[serde(flatten)]
    pub span: Span,
    pub min: Character,
    pub max: Character,
}

/// `|` separated string of characters wrapped by `\q{}`.
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct ClassStringDisjunction<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// `true` if body is empty or contains [`ClassString`] which `strings` is `true`.
    pub strings: bool,
    pub body: Vec<'a, ClassString<'a, A>, A>,
}

/// Single unit of [`ClassStringDisjunction`].
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct ClassString<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// `true` if body is empty or contain 2 more characters.
    pub strings: bool,
    pub body: Vec<'a, Character, A>,
}

/// Named or unnamed capturing group.
/// e.g. `(...)`, `(?<name>...)`
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct CapturingGroup<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Group name to be referenced by [`NamedReference`].
    pub name: Option<Atom<'a>>,
    pub body: Disjunction<'a, A>,
}

/// Pseudo-group for ignoring.
/// e.g. `(?:...)`
#[ast]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct IgnoreGroup<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub modifiers: Option<Modifiers>,
    pub body: Disjunction<'a, A>,
}

/// Modifiers in [`IgnoreGroup`].
/// e.g. `i` in `(?i:...)`, `-s` in `(?-s:...)`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Modifiers {
    #[serde(flatten)]
    pub span: Span,
    pub enabling: Option<Modifier>,
    pub disabling: Option<Modifier>,
}

/// Each part of modifier in [`Modifiers`].
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct Modifier {
    pub ignore_case: bool,
    pub multiline: bool,
    pub sticky: bool,
}

/// Backreference by index.
/// e.g. `\1`, `\2`, `\3`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct IndexedReference {
    #[serde(flatten)]
    pub span: Span,
    pub index: u32,
}

/// Backreference by name.
/// e.g. `\k<name>`
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct NamedReference<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub name: Atom<'a>,
}

// See `oxc_ast/src/lib.rs` for the details
#[cfg(target_pointer_width = "64")]
#[test]
fn size_asserts() {
    use std::mem::size_of;

    assert!(size_of::<Term>() == 16);
    assert!(size_of::<CharacterClassContents>() == 16);
}
