//! TypeScript Definitions
//!
//! - [AST Spec](https://github.com/typescript-eslint/typescript-eslint/tree/main/packages/ast-spec)
//! - [Archived TypeScript spec](https://github.com/microsoft/TypeScript/blob/3c99d50da5a579d9fa92d02664b1b66d4ff55944/doc/spec-ARCHIVED.md)

// NB: `#[span]`, `#[scope(...)]`,`#[visit(...)]` and `#[generate_derive(...)]` do NOT do anything to the code.
// They are purely markers for codegen used in `tasks/ast_tools` and `crates/oxc_traverse/scripts`. See docs in those crates.
// Read [`macro@oxc_ast_macros::ast`] for more information.

// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use std::cell::Cell;

use oxc_allocator::CloneIn;
use oxc_ast_macros::ast;
use oxc_span::{cmp::ContentEq, hash::ContentHash, Atom, GetSpan, GetSpanMut, Span};
use oxc_syntax::scope::ScopeId;
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use tsify::Tsify;

use super::{inherit_variants, js::*, jsx::*, literal::*};
use derive_where::derive_where;
use oxc_span::ast_alloc::AstAllocator;

#[cfg(feature = "serialize")]
#[wasm_bindgen::prelude::wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export interface TSIndexSignatureName extends Span {
    type: "Identifier",
    name: Atom,
    typeAnnotation: TSTypeAnnotation,
}
"#;

/// TypeScript `this` parameter
///
/// ## Example
/// ```ts
/// type T = (this: string, a: number) => void
/// //        ^^^^^^^^^^^^
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - `this` parameters](https://www.typescriptlang.org/docs/handbook/2/functions.html#this-parameters)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSThisParameter<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub this_span: Span,
    /// Type type the `this` keyword will have in the function
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

/// Enum Declaration
///
/// `const_opt` enum `BindingIdentifier` { `EnumBody_opt` }
///
/// ## Examples
///
/// ```ts
/// enum Foo {
///     A,
///     B
/// }
/// // `Bar` has `r#const` set to `true`
/// const enum Bar {
///     A,
///     B
/// }
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Enums](https://www.typescriptlang.org/docs/handbook/enums.html)
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSEnumDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub id: BindingIdentifier<'a>,
    #[scope(enter_before)]
    pub members: A::Vec<'a, TSEnumMember<'a, A>>,
    /// `true` for const enums
    pub r#const: bool,
    pub declare: bool,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// Enum Member
///
/// A member property in a [`TSEnumDeclaration`].
///
/// ## Example
/// ```ts
/// enum Foo {
/// //  _ id
///     A = 1,
/// //      ^ initializer
///     B // initializer will be `None`
///
/// }
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Enums](https://www.typescriptlang.org/docs/handbook/enums.html)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSEnumMember<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub id: TSEnumMemberName<'a, A>,
    pub initializer: Option<Expression<'a, A>>,
}

inherit_variants! {
/// TS Enum Member Name
///
/// Used in [`TSEnumMember`]. Inherits variants from [`Expression`]. See [`ast` module docs] for
/// explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum TSEnumMemberName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    StaticIdentifier(A::Box<'a, IdentifierName<'a>>) = 64,
    StaticStringLiteral(A::Box<'a, StringLiteral<'a>>) = 65,
    StaticTemplateLiteral(A::Box<'a, TemplateLiteral<'a, A>>) = 66,
    // Invalid Grammar `enum E { 1 }`
    StaticNumericLiteral(A::Box<'a, NumericLiteral<'a>>) = 67,
    // Invalid Grammar `enum E { [computed] }`
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// TypeScript Type Annotation
///
/// An annotation on a variable declaration, parameter, etc.
///
/// ## Example
/// ```ts
/// const x: number = 1;
/// //     ^^^^^^^^
///
/// function foo(x: number): number { return x; }
/// //            ^^^^^^^^ ^^^^^^^^
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeAnnotation<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    /// starts at the `:` token and ends at the end of the type annotation
    pub span: Span,
    /// The actual type in the annotation
    pub type_annotation: TSType<'a, A>,
}

/// TypeScript Literal Type
///
/// A type that is a literal value. Wraps a [`TSLiteral`].
///
/// ## Example
/// ```ts
/// const x: 'foo' = 'foo';
/// //       ^^^^^
///
/// type NonZero<N> = N extends 0 ? never : N;
/// //                          ^
/// type Three = NonZero<3>;
/// //                   ^
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSLiteralType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub literal: TSLiteral<'a, A>,
}

/// A literal in a [`TSLiteralType`].
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSLiteral<'a, A: AstAllocator = oxc_allocator::Allocator> {
    BooleanLiteral(A::Box<'a, BooleanLiteral>) = 0,
    NullLiteral(A::Box<'a, NullLiteral>) = 1,
    NumericLiteral(A::Box<'a, NumericLiteral<'a>>) = 2,
    BigIntLiteral(A::Box<'a, BigIntLiteral<'a>>) = 3,
    RegExpLiteral(A::Box<'a, RegExpLiteral<'a, A>>) = 4,
    StringLiteral(A::Box<'a, StringLiteral<'a>>) = 5,
    TemplateLiteral(A::Box<'a, TemplateLiteral<'a, A>>) = 6,
    UnaryExpression(A::Box<'a, UnaryExpression<'a, A>>) = 7,
}

/// TypeScript Type
///
/// This is the root-level type for TypeScript types, kind of like [`Expression`] is for
/// expressions.
///
/// ## Examples
/// ```ts
/// // Foo is a TSTypeAlias
/// type Foo = number | string
/// //         ^^^^^^^^^^^^^^^ TSType::TSUnionType
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // Keyword
    TSAnyKeyword(A::Box<'a, TSAnyKeyword>) = 0,
    TSBigIntKeyword(A::Box<'a, TSBigIntKeyword>) = 1,
    TSBooleanKeyword(A::Box<'a, TSBooleanKeyword>) = 2,
    TSIntrinsicKeyword(A::Box<'a, TSIntrinsicKeyword>) = 3,
    TSNeverKeyword(A::Box<'a, TSNeverKeyword>) = 4,
    TSNullKeyword(A::Box<'a, TSNullKeyword>) = 5,
    TSNumberKeyword(A::Box<'a, TSNumberKeyword>) = 6,
    TSObjectKeyword(A::Box<'a, TSObjectKeyword>) = 7,
    TSStringKeyword(A::Box<'a, TSStringKeyword>) = 8,
    TSSymbolKeyword(A::Box<'a, TSSymbolKeyword>) = 9,
    TSUndefinedKeyword(A::Box<'a, TSUndefinedKeyword>) = 11,
    TSUnknownKeyword(A::Box<'a, TSUnknownKeyword>) = 12,
    TSVoidKeyword(A::Box<'a, TSVoidKeyword>) = 13,
    // Compound
    TSArrayType(A::Box<'a, TSArrayType<'a, A>>) = 14,
    TSConditionalType(A::Box<'a, TSConditionalType<'a, A>>) = 15,
    TSConstructorType(A::Box<'a, TSConstructorType<'a, A>>) = 16,
    TSFunctionType(A::Box<'a, TSFunctionType<'a, A>>) = 17,
    TSImportType(A::Box<'a, TSImportType<'a, A>>) = 18,
    TSIndexedAccessType(A::Box<'a, TSIndexedAccessType<'a, A>>) = 19,
    TSInferType(A::Box<'a, TSInferType<'a, A>>) = 20,
    TSIntersectionType(A::Box<'a, TSIntersectionType<'a, A>>) = 21,
    TSLiteralType(A::Box<'a, TSLiteralType<'a, A>>) = 22,
    TSMappedType(A::Box<'a, TSMappedType<'a, A>>) = 23,
    TSNamedTupleMember(A::Box<'a, TSNamedTupleMember<'a, A>>) = 24,
    TSQualifiedName(A::Box<'a, TSQualifiedName<'a, A>>) = 25,
    TSTemplateLiteralType(A::Box<'a, TSTemplateLiteralType<'a, A>>) = 26,
    TSThisType(A::Box<'a, TSThisType>) = 10,
    TSTupleType(A::Box<'a, TSTupleType<'a, A>>) = 27,
    TSTypeLiteral(A::Box<'a, TSTypeLiteral<'a, A>>) = 28,
    TSTypeOperatorType(A::Box<'a, TSTypeOperator<'a, A>>) = 29,
    TSTypePredicate(A::Box<'a, TSTypePredicate<'a, A>>) = 30,
    TSTypeQuery(A::Box<'a, TSTypeQuery<'a, A>>) = 31,
    TSTypeReference(A::Box<'a, TSTypeReference<'a, A>>) = 32,
    TSUnionType(A::Box<'a, TSUnionType<'a, A>>) = 33,
    TSParenthesizedType(A::Box<'a, TSParenthesizedType<'a, A>>) = 34,
    // JSDoc
    JSDocNullableType(A::Box<'a, JSDocNullableType<'a, A>>) = 35,
    JSDocNonNullableType(A::Box<'a, JSDocNonNullableType<'a, A>>) = 36,
    JSDocUnknownType(A::Box<'a, JSDocUnknownType>) = 37,
}

/// Macro for matching `TSType`'s variants.
#[macro_export]
macro_rules! match_ts_type {
    ($ty:ident) => {
        $ty::TSAnyKeyword(_)
            | $ty::TSBigIntKeyword(_)
            | $ty::TSBooleanKeyword(_)
            | $ty::TSIntrinsicKeyword(_)
            | $ty::TSNeverKeyword(_)
            | $ty::TSNullKeyword(_)
            | $ty::TSNumberKeyword(_)
            | $ty::TSObjectKeyword(_)
            | $ty::TSStringKeyword(_)
            | $ty::TSSymbolKeyword(_)
            | $ty::TSThisType(_)
            | $ty::TSUndefinedKeyword(_)
            | $ty::TSUnknownKeyword(_)
            | $ty::TSVoidKeyword(_)
            | $ty::TSArrayType(_)
            | $ty::TSConditionalType(_)
            | $ty::TSConstructorType(_)
            | $ty::TSFunctionType(_)
            | $ty::TSImportType(_)
            | $ty::TSIndexedAccessType(_)
            | $ty::TSInferType(_)
            | $ty::TSIntersectionType(_)
            | $ty::TSLiteralType(_)
            | $ty::TSMappedType(_)
            | $ty::TSNamedTupleMember(_)
            | $ty::TSQualifiedName(_)
            | $ty::TSTemplateLiteralType(_)
            | $ty::TSTupleType(_)
            | $ty::TSTypeLiteral(_)
            | $ty::TSTypeOperatorType(_)
            | $ty::TSTypePredicate(_)
            | $ty::TSTypeQuery(_)
            | $ty::TSTypeReference(_)
            | $ty::TSUnionType(_)
            | $ty::TSParenthesizedType(_)
            | $ty::JSDocNullableType(_)
            | $ty::JSDocNonNullableType(_)
            | $ty::JSDocUnknownType(_)
    };
}
pub use match_ts_type;

/// TypeScript Conditional Type
///
/// ## Example
/// ```ts
/// type GetProperty<T extends string> =
/// //  _ check_type
///     T extends `${string}.${infer U}`  // <- extends_type
///         ? U                           // <- true_type
///         : never;                      // <- false_type
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Conditional Types](https://www.typescriptlang.org/docs/handbook/2/conditional-types.html)
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSConditionalType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The type before `extends` in the test expression.
    pub check_type: TSType<'a, A>,
    /// The type `check_type` is being tested against.
    #[scope(enter_before)]
    pub extends_type: TSType<'a, A>,
    /// The type evaluated to if the test is true.
    pub true_type: TSType<'a, A>,
    /// The type evaluated to if the test is false.
    #[scope(exit_before)]
    pub false_type: TSType<'a, A>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// TypeScript Union Type
///
/// ## Example
/// ```ts
///  string | string[] | (() => string) | { s: string }
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Union Types](https://www.typescriptlang.org/docs/handbook/typescript-in-5-minutes-func.html#unions)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSUnionType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The types in the union.
    pub types: A::Vec<'a, TSType<'a, A>>,
}

/// TypeScript Intersection Type
///
/// ## Example
/// ```ts
/// type Colorful = { color: string };
/// type Circle = { radius: number };
///
/// // `types` will be `[Colorful, Circle]`
/// type ColorfulCircle = Colorful & Circle;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Intersection Types](https://www.typescriptlang.org/docs/handbook/2/objects.html#intersection-types)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSIntersectionType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub types: A::Vec<'a, TSType<'a, A>>,
}

/// Parenthesized Type
///
/// Like [`ParenthesizedExpression`], but for types.
///
/// ## Example
/// ```ts
/// type Foo = (string | number);
/// //          ^^^^^^^^^^^^^^^^ type_annotation
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSParenthesizedType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_annotation: TSType<'a, A>,
}

/// TypeScript Type Operators
///
/// Includes
/// - `keyof`
/// - `unique`
/// - `readonly`
///
/// ## References
/// * [TypeScript Handbook - Keyof Types](https://www.typescriptlang.org/docs/handbook/2/keyof-types.html)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeOperator<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub operator: TSTypeOperatorOperator,
    /// The type being operated on
    pub type_annotation: TSType<'a, A>,
}

/// Operator in a [`TSTypeOperator`].
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum TSTypeOperatorOperator {
    Keyof = 0,
    Unique = 1,
    Readonly = 2,
}

/// TypeScript Array Type
///
/// Does not include tuple types, which are stored as [`TSTupleType`].
///
/// ## Example
///
/// ```ts
/// let myArray: string[] = ["hello", "world"];
/// ```
///
/// <https://www.typescriptlang.org/docs/handbook/2/objects.html#the-array-type>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSArrayType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub element_type: TSType<'a, A>,
}

/// TypeScript Index Access Type
///
/// This is the type equivalent to expression member access.
///
/// ## Example
///
/// ```ts
/// type I1 = Person["age" | "name"];
/// ```
///
/// <https://www.typescriptlang.org/docs/handbook/2/indexed-access-types.html#handbook-content>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSIndexedAccessType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub object_type: TSType<'a, A>,
    pub index_type: TSType<'a, A>,
}

/// TypeScript Tuple Type
///
/// ## Example
///
/// ```ts
/// type `StringNumberPair` = [string, number];
/// ```
///
/// <https://www.typescriptlang.org/docs/handbook/2/objects.html#tuple-types>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTupleType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub element_types: A::Vec<'a, TSTupleElement<'a, A>>,
}

/// TypeScript Named Tuple Member
///
/// ## Example
/// ```ts
/// type Foo = [first: string, second: number];
/// //          ^^^^^^^^^^^^^
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Tuple Types](https://www.typescriptlang.org/docs/handbook/2/objects.html#tuple-types)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSNamedTupleMember<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub element_type: TSTupleElement<'a, A>,
    pub label: IdentifierName<'a>,
    pub optional: bool,
}

/// TypeScript Optional Type
///
/// Note that this does not cover optional object or class properties.
///
/// ## Example
/// ```ts
/// type Foo = [number?]
/// //          ^^^^^^ type_annotation
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSOptionalType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_annotation: TSType<'a, A>,
}

/// TypeScript Rest Type
///
/// ## Example
/// ```ts
/// //                  ___________ this is the rest type
/// type Foo = [number, ...string[]]
/// //                     ^^^^^^^^ type_annotation
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSRestType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_annotation: TSType<'a, A>,
}

inherit_variants! {
/// TS Tuple Element
///
/// Inherits variants from [`TSType`]. See [`ast` module docs] for explanation of inheritance.
///
/// See [`TSNamedTupleMember`] for named tuple elements.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSTupleElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // Discriminants start at 64, so that `TSTupleElement::is_ts_type` is a single
    // bitwise AND operation on the discriminant (`discriminant & 63 != 0`).
    TSOptionalType(A::Box<'a, TSOptionalType<'a, A>>) = 64,
    TSRestType(A::Box<'a, TSRestType<'a, A>>) = 65,
    // `TSType` variants added here by `inherit_variants!` macro
    @inherit TSType
}
}

/// TypeScript `any` keyword
///
/// ## Example
/// ```ts
/// type Foo = any;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Any Type](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#any)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSAnyKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `string` keyword
///
/// ## Example
/// ```ts
/// type Foo = string;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSStringKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `boolean` keyword
///
/// ## Example
/// ```ts
/// type Foo = boolean;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSBooleanKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `number` keyword
///
/// ## Example
/// ```ts
/// type Foo = boolean;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#the-primitives-string-number-and-boolean)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSNumberKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `never` Keyword
///
/// ## Example
/// ```ts
/// type Foo<T> = T extends string ? never : T;
/// //                               ^^^^^
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Advanced Topics](https://www.typescriptlang.org/docs/handbook/type-compatibility.html#advanced-topics)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSNeverKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `intrinsic` Keyword
///
/// Intrinsic types are built into TypeScript and are not user-defined.
/// ## Example
/// `type Uppercase<T extends character> = intrinsic;`
///
/// ### References
/// * [TypeScript Handbook - Intrinsic String Manipulation
/// Types](https://www.typescriptlang.org/docs/handbook/2/template-literal-types.html#intrinsic-string-manipulation-types)
/// * [microsoft/TypeScript #40580](https://github.com/microsoft/TypeScript/pull/40580)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSIntrinsicKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `unknown` Keyword
///
/// This is like `any`, but is not assignable to anything except `any` and `unknown`.
///
/// ## Example
/// ```ts
/// type Foo = unknown;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Advanced Topics](https://www.typescriptlang.org/docs/handbook/type-compatibility.html#advanced-topics)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSUnknownKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `null` Keyword
///
/// ## Example
/// ```ts
/// type Foo = string | null;
/// //                  ^^^^
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#null-and-undefined)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSNullKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript `null` Keyword
///
/// ## Example
/// ```ts
/// type Foo = string | undefined;
/// //                  ^^^^^^^^^
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#null-and-undefined)
/// ## Reference
/// * [TypeScript Handbook - Everyday Types](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#null-and-undefined)
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSUndefinedKeyword {
    #[serde(flatten)]
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSVoidKeyword {
    #[serde(flatten)]
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSSymbolKeyword {
    #[serde(flatten)]
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSThisType {
    #[serde(flatten)]
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSObjectKeyword {
    #[serde(flatten)]
    pub span: Span,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TSBigIntKeyword {
    #[serde(flatten)]
    pub span: Span,
}

/// TypeScript Type Reference
///
/// ## Example
/// ```ts
/// type C = A;
/// type D = B.a;
/// type E = D.c.b.a;
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeReference<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_name: TSTypeName<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

/// TypeName:
///     IdentifierReference
///     NamespaceName . IdentifierReference
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum TSTypeName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    IdentifierReference(A::Box<'a, IdentifierReference<'a>>) = 0,
    QualifiedName(A::Box<'a, TSQualifiedName<'a, A>>) = 1,
}

/// Macro for matching `TSTypeName`'s variants.
#[macro_export]
macro_rules! match_ts_type_name {
    ($ty:ident) => {
        $ty::IdentifierReference(_) | $ty::QualifiedName(_)
    };
}
pub use match_ts_type_name;

/// TypeScript Qualified Name
///
/// A [type reference](TSTypeReference) qualified by a namespace.
///
/// ## Example
/// ```ts
/// type Foo = A.B.C;
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSQualifiedName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: TSTypeName<'a, A>,
    pub right: IdentifierName<'a>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeParameterInstantiation<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub params: A::Vec<'a, TSType<'a, A>>,
}

/// TypeScript Type Parameter
///
/// This is a type parameter in a generic type or function.
///
/// ## Example
/// ```ts
/// //                 ______ constraint
/// type A::Box<T extends string = 'foo'> = { value: T };
/// // name  ^                  ^^^^^ default
///
/// function add<in T>(a: T, b: T): T { return a + b; }
/// //           ^^ in: true
/// ```
///
/// ## References
/// * [TypeScript Handbook - Generics](https://www.typescriptlang.org/docs/handbook/2/generics.html)
/// * [TypeScript Handbook - Variance Annotations](https://www.typescriptlang.org/docs/handbook/2/generics.html#variance-annotations)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeParameter<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the parameter, e.g. `T` in `type Foo<T> = ...`.
    pub name: BindingIdentifier<'a>,
    /// Constrains what types can be passed to the type parameter.
    pub constraint: Option<TSType<'a, A>>,
    /// Default value of the type parameter if no type is provided when using the type.
    pub default: Option<TSType<'a, A>>,
    /// Was an `in` modifier keyword present?
    pub r#in: bool,
    /// Was an `out` modifier keyword present?
    pub out: bool,
    /// Was a `const` modifier keyword present?
    pub r#const: bool,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeParameterDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub params: A::Vec<'a, TSTypeParameter<'a, A>>,
}

/// TypeScript Type Alias Declaration Statement
///
/// ## Example
/// ```ts
/// //   _____ id
/// type Maybe<T> = T | null | undefined;
/// //         ^ type_parameters
/// ```
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeAliasDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Type alias's identifier, e.g. `Foo` in `type Foo = number`.
    pub id: BindingIdentifier<'a>,
    #[scope(enter_before)]
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub type_annotation: TSType<'a, A>,
    pub declare: bool,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum TSAccessibility {
    Private = 0,
    Protected = 1,
    Public = 2,
}

/// TypeScript Class Interface Heritage
///
/// `implements` clause of a [class declaration](Class).
///
/// ## Example
/// ```ts
/// //                   ___ expression
/// class Foo implements Bar, Baz<number, string> {}
/// //            type_parameters ^^^^^^^^^^^^^^
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSClassImplements<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: TSTypeName<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

/// TypeScriptInterface Declaration
///
///   interface `BindingIdentifier` `TypeParameters_opt` `InterfaceExtendsClause_opt` `ObjectType`
///
/// ## Example
/// ```ts
/// //                       ___ extends
/// interface Foo<T> extends Bar {
/// //     id ^^^ ^ type_parameters
/// }
/// ```
///
/// ## References
/// * [TypeScript in 5 Minutes - Interfaces](https://www.typescriptlang.org/docs/handbook/typescript-tooling-in-5-minutes.html#interfaces)
/// * [TypeScript Handbook - Interfaces](https://www.typescriptlang.org/docs/handbook/2/objects.html#interfaces)
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSInterfaceDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The identifier (name) of the interface.
    pub id: BindingIdentifier<'a>,
    /// Other interfaces/types this interface extends.
    #[scope(enter_before)]
    pub extends: Option<A::Vec<'a, TSInterfaceHeritage<'a, A>>>,
    /// Type parameters that get bound to the interface.
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub body: A::Box<'a, TSInterfaceBody<'a, A>>,
    /// `true` for `declare interface Foo {}`
    pub declare: bool,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// Body of a [`TSInterfaceDeclaration`].
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSInterfaceBody<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: A::Vec<'a, TSSignature<'a, A>>,
}

/// TypeScript Property Signature
///
/// Used in [classes](Class), [interfaces](TSInterfaceDeclaration), [mapped types](TSMappedType),
/// etc. Part of a [`TSSignature`].
///
/// ## Example
/// ```ts
/// interface Foo {
/// //  ___ key
///     bar: number
/// //     ^^^^^^^^ type_annotation
///     baz?: string          // <- optional
///     readony bang: boolean // <- readonly
/// }
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSPropertySignature<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub computed: bool,
    pub optional: bool,
    pub readonly: bool,
    pub key: PropertyKey<'a, A>,
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSSignature<'a, A: AstAllocator = oxc_allocator::Allocator> {
    TSIndexSignature(A::Box<'a, TSIndexSignature<'a, A>>) = 0,
    TSPropertySignature(A::Box<'a, TSPropertySignature<'a, A>>) = 1,
    TSCallSignatureDeclaration(A::Box<'a, TSCallSignatureDeclaration<'a, A>>) = 2,
    TSConstructSignatureDeclaration(A::Box<'a, TSConstructSignatureDeclaration<'a, A>>) = 3,
    TSMethodSignature(A::Box<'a, TSMethodSignature<'a, A>>) = 4,
}

/// An index signature within a class, type alias, etc.
///
/// ## Example
/// [playground link](https://oxc-playground.netlify.app/?code=3YCAAIC9gICAgICAgIC6nsrEgtem3AB/pQsrWlLnujiFhkHVtfeFMq5RMD7X5AzJnZ5R/ecQ5KG1FUFjzXvrxFXH0m6HpS+Ob3TC8gQXeRQygA%3D%3D)
/// ```ts
/// type MapOf<T> = {
/// //   _________ parameters (vec with 1 element)
///     [K: string]: T
/// //               - type_annotation
/// }
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSIndexSignature<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub parameters: A::Vec<'a, TSIndexSignatureName<'a, A>>,
    pub type_annotation: A::Box<'a, TSTypeAnnotation<'a, A>>,
    pub readonly: bool,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSCallSignatureDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub this_param: Option<TSThisParameter<'a, A>>,
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    pub return_type: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum TSMethodSignatureKind {
    Method = 0,
    Get = 1,
    Set = 2,
}

/// TypeScript Method Signature
///
/// Similar to a [`TSFunctionType`], but only for method shorthand syntax.
///
/// ## Example
/// ```ts
/// interface Foo {
///     bar(a: number): string;
/// //  ^^^ key
/// }
/// ```
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSMethodSignature<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub key: PropertyKey<'a, A>,
    pub computed: bool,
    pub optional: bool,
    pub kind: TSMethodSignatureKind,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub this_param: Option<A::Box<'a, TSThisParameter<'a, A>>>,
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    pub return_type: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// TypeScript Constructor Signature Declaration
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSConstructSignatureDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    pub return_type: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "Identifier", rename_all = "camelCase")]
pub struct TSIndexSignatureName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub name: Atom<'a>,
    pub type_annotation: A::Box<'a, TSTypeAnnotation<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSInterfaceHeritage<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

/// TypeScript Type Predicate
///
/// ## Examples
/// ```ts
/// function isString(x: unknown): x is string {
/// //              parameter_name ^    ^^^^^^ type_annotation
///     return typeof x === 'string';
/// }
/// ```
///
/// ```ts
/// function assertString(x: unknown): asserts x is string {
/// //                                 ^^^^^^^ asserts: true
///     if (typeof x !== 'string') throw new TypeError('x is not a string');
/// }
/// ```
///
/// ## References
/// * [TypeScript Handbook - Type Predicates](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#using-type-predicates)
/// * [TypeScript Handbook - Assertion Functions](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypePredicate<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The identifier the predicate operates on
    pub parameter_name: TSTypePredicateName<'a, A>,
    /// Does this predicate include an `asserts` modifier?
    ///
    /// ## Example
    /// ```ts
    /// declare function isString(x: any): asserts x is string; // true
    /// ```
    pub asserts: bool,
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSTypePredicateName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    Identifier(A::Box<'a, IdentifierName<'a>>) = 0,
    This(TSThisType) = 1,
}

/// TypeScript Module and Namespace Declarations
///
/// ## Examples
/// ```ts
/// declare module 'foo' {
/// // kind ^^^^^^ ^^^^^ id
/// }
/// ```
///
/// ```ts
/// namespace Foo { }
/// declare namespace Bar { }
/// ```
///
/// ```ts
/// declare global {
///     interface Window {
///        customProp: string;
///     }
/// }
/// ```
///
/// ## References
/// * [TypeScript Handbook - Namespaces](https://www.typescriptlang.org/docs/handbook/2/modules.html#namespaces)
/// * [TypeScript Handbook - Module Augmentation](https://www.typescriptlang.org/docs/handbook/declaration-merging.html#module-augmentation)
/// * [TypeScript Handbook - Global Augmentation](https://www.typescriptlang.org/docs/handbook/declaration-merging.html#global-augmentation)
#[ast(visit)]
#[scope(
    flags(ScopeFlags::TsModuleBlock),
    strict_if(self.body.as_ref().is_some_and(TSModuleDeclarationBody::is_strict)),
)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSModuleDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the module/namespace being declared.
    ///
    /// Note that for `declare global {}`, no symbol will be created for the module name.
    pub id: TSModuleDeclarationName<'a>,
    #[scope(enter_before)]
    pub body: Option<TSModuleDeclarationBody<'a, A>>,
    /// The keyword used to define this module declaration.
    ///
    /// Helps discriminate between global overrides vs module declarations vs namespace
    /// declarations.
    ///
    /// ```ts
    /// namespace Foo {}
    /// ^^^^^^^^^
    /// module 'foo' {}
    /// ^^^^^^
    /// declare global {}
    ///         ^^^^^^
    /// ```
    pub kind: TSModuleDeclarationKind,
    pub declare: bool,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum TSModuleDeclarationKind {
    /// `declare global {}`
    Global = 0,
    /// `declare module 'foo' {}`
    Module = 1,
    /// `namespace Foo {}`
    Namespace = 2,
}

/// The name of a TypeScript [namespace or module declaration](TSModuleDeclaration).
///
/// Note that it is a syntax error for namespace declarations to have a string literal name.
/// Modules may have either kind.
///
/// ## Examples
/// ```ts
/// // TSModuleDeclarationName::StringLiteral
/// declare module "*.css" {
///     const styles: { [key: string]: string };
///     export default styles;
/// }
/// ```
///
/// ```ts
/// // TSModuleDeclarationName::Identifier
/// namespace Foo {
///    export const bar = 42;
/// }
/// ```
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum TSModuleDeclarationName<'a> {
    Identifier(BindingIdentifier<'a>) = 0,
    StringLiteral(StringLiteral<'a>) = 1,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum TSModuleDeclarationBody<'a, A: AstAllocator = oxc_allocator::Allocator> {
    TSModuleDeclaration(A::Box<'a, TSModuleDeclaration<'a, A>>) = 0,
    TSModuleBlock(A::Box<'a, TSModuleBlock<'a, A>>) = 1,
}

// See serializer in serialize.rs
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSModuleBlock<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[serde(skip)]
    pub directives: A::Vec<'a, Directive<'a>>,
    pub body: A::Vec<'a, Statement<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeLiteral<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub members: A::Vec<'a, TSSignature<'a, A>>,
}

/// TypeScript `infer` type
///
/// Used in a [`TSConditionalType`] to bind a type parameter when some tested type extends a
/// desired type.
///
/// ## Example
/// ```ts
/// type Foo<T> = T extends infer U ? U : never;
/// //                            ^ type_parameter
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Inferring With Conditional Types](https://www.typescriptlang.org/docs/handbook/2/conditional-types.html#inferring-within-conditional-types)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSInferType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The type bound when the
    pub type_parameter: A::Box<'a, TSTypeParameter<'a, A>>,
}

/// Type Query
///
/// ## Example
/// ```ts
/// type Foo = typeof Bar;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Typeof Type Operator](https://www.typescriptlang.org/docs/handbook/2/typeof-types.html)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeQuery<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expr_name: TSTypeQueryExprName<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

inherit_variants! {
/// TS Type Query Expr Name
///
/// Inherits variants from [`TSTypeName`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum TSTypeQueryExprName<'a, A: AstAllocator = oxc_allocator::Allocator> {
    TSImportType(A::Box<'a, TSImportType<'a, A>>) = 2,
    // `TSTypeName` variants added here by `inherit_variants!` macro
    @inherit TSTypeName
}
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSImportType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// `true` for `typeof import("foo")`
    pub is_type_of: bool,
    pub parameter: TSType<'a, A>,
    pub qualifier: Option<TSTypeName<'a, A>>,
    pub attributes: Option<A::Box<'a, TSImportAttributes<'a, A>>>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSImportAttributes<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub attributes_keyword: IdentifierName<'a>, // `with` or `assert`
    pub elements: A::Vec<'a, TSImportAttribute<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSImportAttribute<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub name: TSImportAttributeName<'a>,
    pub value: Expression<'a, A>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum TSImportAttributeName<'a> {
    Identifier(IdentifierName<'a>) = 0,
    StringLiteral(StringLiteral<'a>) = 1,
}

/// TypeScript Function Type
///
/// ## Examples
/// ```ts
/// //       __________ this is the TSFunctionType
/// type T = () => void
/// //             ^^^^ return_type
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSFunctionType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Generic type parameters
    ///
    /// ```ts
    /// type T = <U>(x: U) => U;
    /// //        ^
    /// ```
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    /// `this` parameter
    ///
    /// ```ts
    /// type T = (this: string, a: number) => void;
    /// //        ^^^^^^^^^^^^
    /// ```
    pub this_param: Option<A::Box<'a, TSThisParameter<'a, A>>>,
    /// Function parameters. Akin to [`Function::params`].
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    /// Return type of the function.
    /// ```ts
    /// type T = () => void;
    /// //             ^^^^
    /// ```
    pub return_type: A::Box<'a, TSTypeAnnotation<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSConstructorType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub r#abstract: bool,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    pub return_type: A::Box<'a, TSTypeAnnotation<'a, A>>,
}

/// TypeScript Mapped Type
///
/// ## Examples
/// ```ts
/// type Maybe<T> = {
/// //        _____ constraint
///     [P in keyof T]?: T[P]
/// //   ^ type_parameter
/// }
/// ```
///
/// ```ts
/// type ReadonlyDefinite<T> = {
/// //           _ type parameter
///    readonly [P in keyof T]-?: T[P]
/// //                        ^^ `optional` modifier
/// };
/// ```
///
/// ## References
/// * [TypeScript Handbook - Mapped Types](https://www.typescriptlang.org/docs/handbook/2/mapped-types.html)
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSMappedType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Key type parameter, e.g. `P` in `[P in keyof T]`.
    pub type_parameter: A::Box<'a, TSTypeParameter<'a, A>>,
    pub name_type: Option<TSType<'a, A>>,
    pub type_annotation: Option<TSType<'a, A>>,
    /// Optional modifier on type annotation
    ///
    /// ## Examples
    /// ```ts
    /// type Foo = { [P in keyof T]?: T[P] }
    /// //                         ^^ True
    /// type Bar = { [P in keyof T]+?: T[P] }
    /// //                         ^^ Plus
    /// type Baz = { [P in keyof T]-?: T[P] }
    /// //                         ^^ Minus
    /// type Qux = { [P in keyof T]: T[P] }
    /// //                         ^ None
    /// ```
    pub optional: TSMappedTypeModifierOperator,
    /// Readonly modifier before keyed index signature
    ///
    /// ## Examples
    /// ```ts
    /// type Foo = { readonly [P in keyof T]: T[P] }  // True
    /// type Bar = { +readonly [P in keyof T]: T[P] } // Plus
    /// type Baz = { -readonly [P in keyof T]: T[P] } // Minus
    /// type Qux = { [P in keyof T]: T[P] }           // None
    /// ```
    pub readonly: TSMappedTypeModifierOperator,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum TSMappedTypeModifierOperator {
    /// e.g. `?` in `{ [P in K]?: T }`
    True = 0,
    /// e.g. `+?` in `{ [P in K]+?: T }`
    #[serde(rename = "+")]
    Plus = 1,
    /// e.g. `-?` in `{ [P in K]-?: T }`
    #[serde(rename = "-")]
    Minus = 2,
    /// No modifier present
    None = 3,
}

/// TypeScript Template Literal Type
///
/// ## Example
/// ```ts
/// // Each string part is an element in `quasis`, including empty strings at the beginning/end.
/// // In this example, `quasis` has 3 elements: ["", ".", ""]
/// type Dot<T, U> = `${T}.${U}`;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - Template Literal Types](https://www.typescriptlang.org/docs/handbook/2/template-literal-types.html#handbook-content)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTemplateLiteralType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The string parts of the template literal.
    pub quasis: A::Vec<'a, TemplateElement<'a>>,
    /// The interpolated expressions in the template literal.
    pub types: A::Vec<'a, TSType<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSAsExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
    pub type_annotation: TSType<'a, A>,
}

/// TypeScript `satisfies` Expression
///
/// ## Example
/// ```ts
/// const user = {
///     id: 0,
///     name: 'Alice',
/// } satisfies User;
/// ```
///
/// ## Reference
/// * [TypeScript Handbook - The `satisfies` Operator](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html#the-satisfies-operator)
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSSatisfiesExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The value expression being constrained.
    pub expression: Expression<'a, A>,
    /// The type `expression` must satisfy.
    pub type_annotation: TSType<'a, A>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSTypeAssertion<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
    pub type_annotation: TSType<'a, A>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSImportEqualsDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub id: BindingIdentifier<'a>,
    pub module_reference: TSModuleReference<'a, A>,
    pub import_kind: ImportOrExportKind,
}

inherit_variants! {
/// TS Module Reference
///
/// Inherits variants from [`TSTypeName`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged, rename_all = "camelCase")]
pub enum TSModuleReference<'a, A: AstAllocator = oxc_allocator::Allocator> {
    ExternalModuleReference(A::Box<'a, TSExternalModuleReference<'a>>) = 2,
    // `TSTypeName` variants added here by `inherit_variants!` macro
    @inherit TSTypeName
}
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSExternalModuleReference<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: StringLiteral<'a>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSNonNullExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
}

/// Decorator
///
/// Decorators are annotations on classes, methods, properties, and parameters.
/// They are usually either an [`IdentifierReference`] or an [`CallExpression`].
///
/// ## Example
/// ```ts
/// @Foo                        // class decorator
/// @Bar()                      // class decorator factory
/// class SomeClass {
///     @Freeze                 // property decorator
///     public x: number;
///
///     @MethodDecorator        // method decorator
///     public method(
///         @LogParam x: number // parameter decorator
///     ) {
///       // ...
///     }
/// }
/// ```
///
/// [`IdentifierReference`]: crate::ast::js::IdentifierReference
/// [`CallExpression`]: crate::ast::js::CallExpression
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct Decorator<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
}

/// Export Assignment in non-module files
///
/// `export = foo`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSExportAssignment<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
}

/// Namespace Export Declaration in declaration files
///
/// `export as namespace foo`
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSNamespaceExportDeclaration<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub id: IdentifierName<'a>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TSInstantiationExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
    pub type_parameters: A::Box<'a, TSTypeParameterInstantiation<'a, A>>,
}

/// See [TypeScript - Type-Only Imports and Exports](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-8.html)
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum ImportOrExportKind {
    /// `import { foo } from './foo'`;
    Value = 0,
    /// `import type { foo } from './foo'`;
    Type = 1,
}

// [`JSDoc`](https://github.com/microsoft/TypeScript/blob/54a554d8af2657630307cbfa8a3e4f3946e36507/src/compiler/types.ts#L393)

/// `type foo = ty?` or `type foo = ?ty`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct JSDocNullableType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_annotation: TSType<'a, A>,
    /// Was `?` after the type annotation?
    pub postfix: bool,
}

/// `type foo = ty!` or `type foo = !ty`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct JSDocNonNullableType<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub type_annotation: TSType<'a, A>,
    pub postfix: bool,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct JSDocUnknownType {
    #[serde(flatten)]
    pub span: Span,
}
