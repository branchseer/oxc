// NB: `#[span]`, `#[scope(...)]`,`#[visit(...)]` and `#[generate_derive(...)]` do NOT do anything to the code.
// They are purely markers for codegen used in `tasks/ast_tools` and `crates/oxc_traverse/scripts`. See docs in those crates.
// Read [`macro@oxc_ast_macros::ast`] for more information.

// Silence erroneous warnings from Rust Analyser for `#[derive(Tsify)]`
#![allow(non_snake_case)]

use std::cell::Cell;

use oxc_allocator::CloneIn;
use oxc_ast_macros::ast;
use oxc_span::{cmp::ContentEq, hash::ContentHash, Atom, GetSpan, GetSpanMut, SourceType, Span};
use oxc_syntax::{
    operator::{
        AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
    },
    reference::ReferenceId,
    scope::ScopeId,
    symbol::SymbolId,
};
#[cfg(feature = "serialize")]
use serde::Serialize;
#[cfg(feature = "serialize")]
use tsify::Tsify;

use derive_where::derive_where;

use super::{macros::inherit_variants, *};
use oxc_span::ast_alloc::AstAllocator;

/// Represents the root of a JavaScript abstract syntax tree (AST), containing metadata about the source, directives, top-level statements, and scope information.
#[ast(visit)]
#[scope(
    flags(ScopeFlags::Top),
    strict_if(self.source_type.is_strict() || self.directives.iter().any(Directive::is_use_strict)),
)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct Program<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub source_type: SourceType,
    #[serde(skip)]
    pub source_text: &'a str,
    /// Sorted comments
    #[serde(skip)]
    pub comments: A::Vec<'a, Comment>,
    pub hashbang: Option<Hashbang<'a>>,
    pub directives: A::Vec<'a, Directive<'a>>,
    pub body: A::Vec<'a, Statement<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

inherit_variants! {
/// Represents a type for AST nodes corresponding to JavaScript's expressions.
///
/// Inherits variants from [`MemberExpression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum Expression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// See [`BooleanLiteral`] for AST node details.
    BooleanLiteral(A::Box<'a, BooleanLiteral>) = 0,
    /// See [`NullLiteral`] for AST node details.
    NullLiteral(A::Box<'a, NullLiteral>) = 1,
    /// See [`NumericLiteral`] for AST node details.
    NumericLiteral(A::Box<'a, NumericLiteral<'a>>) = 2,
    /// See [`BigIntLiteral`] for AST node details.
    BigIntLiteral(A::Box<'a, BigIntLiteral<'a>>) = 3,
    /// See [`RegExpLiteral`] for AST node details.
    RegExpLiteral(A::Box<'a, RegExpLiteral<'a, A>>) = 4,
    /// See [`StringLiteral`] for AST node details.
    StringLiteral(A::Box<'a, StringLiteral<'a>>) = 5,
    /// See [`TemplateLiteral`] for AST node details.
    TemplateLiteral(A::Box<'a, TemplateLiteral<'a, A>>) = 6,

    /// See [`IdentifierReference`] for AST node details.
    Identifier(A::Box<'a, IdentifierReference<'a>>) = 7,

    /// See [`MetaProperty`] for AST node details.
    MetaProperty(A::Box<'a, MetaProperty<'a>>) = 8,
    /// See [`Super`] for AST node details.
    Super(A::Box<'a, Super>) = 9,

    /// See [`ArrayExpression`] for AST node details.
    ArrayExpression(A::Box<'a, ArrayExpression<'a, A>>) = 10,
    /// See [`ArrowFunctionExpression`] for AST node details.
    ArrowFunctionExpression(A::Box<'a, ArrowFunctionExpression<'a, A>>) = 11,
    /// See [`AssignmentExpression`] for AST node details.
    AssignmentExpression(A::Box<'a, AssignmentExpression<'a, A>>) = 12,
    /// See [`AwaitExpression`] for AST node details.
    AwaitExpression(A::Box<'a, AwaitExpression<'a, A>>) = 13,
    /// See [`BinaryExpression`] for AST node details.
    BinaryExpression(A::Box<'a, BinaryExpression<'a, A>>) = 14,
    /// See [`CallExpression`] for AST node details.
    CallExpression(A::Box<'a, CallExpression<'a, A>>) = 15,
    /// See [`ChainExpression`] for AST node details.
    ChainExpression(A::Box<'a, ChainExpression<'a, A>>) = 16,
    /// See [`Class`] for AST node details.
    ClassExpression(A::Box<'a, Class<'a, A>>) = 17,
    /// See [`ConditionalExpression`] for AST node details.
    ConditionalExpression(A::Box<'a, ConditionalExpression<'a, A>>) = 18,
    /// See [`Function`] for AST node details.
    #[visit(args(flags = ScopeFlags::Function))]
    FunctionExpression(A::Box<'a, Function<'a, A>>) = 19,
    /// See [`ImportExpression`] for AST node details.
    ImportExpression(A::Box<'a, ImportExpression<'a, A>>) = 20,
    /// See [`LogicalExpression`] for AST node details.
    LogicalExpression(A::Box<'a, LogicalExpression<'a, A>>) = 21,
    /// See [`NewExpression`] for AST node details.
    NewExpression(A::Box<'a, NewExpression<'a, A>>) = 22,
    /// See [`ObjectExpression`] for AST node details.
    ObjectExpression(A::Box<'a, ObjectExpression<'a, A>>) = 23,
    /// See [`ParenthesizedExpression`] for AST node details.
    ParenthesizedExpression(A::Box<'a, ParenthesizedExpression<'a, A>>) = 24,
    /// See [`SequenceExpression`] for AST node details.
    SequenceExpression(A::Box<'a, SequenceExpression<'a, A>>) = 25,
    /// See [`TaggedTemplateExpression`] for AST node details.
    TaggedTemplateExpression(A::Box<'a, TaggedTemplateExpression<'a, A>>) = 26,
    /// See [`ThisExpression`] for AST node details.
    ThisExpression(A::Box<'a, ThisExpression>) = 27,
    /// See [`UnaryExpression`] for AST node details.
    UnaryExpression(A::Box<'a, UnaryExpression<'a, A>>) = 28,
    /// See [`UpdateExpression`] for AST node details.
    UpdateExpression(A::Box<'a, UpdateExpression<'a, A>>) = 29,
    /// See [`YieldExpression`] for AST node details.
    YieldExpression(A::Box<'a, YieldExpression<'a, A>>) = 30,
    /// See [`PrivateInExpression`] for AST node details.
    PrivateInExpression(A::Box<'a, PrivateInExpression<'a, A>>) = 31,

    /// See [`JSXElement`] for AST node details.
    JSXElement(A::Box<'a, JSXElement<'a, A>>) = 32,
    /// See [`JSXFragment`] for AST node details.
    JSXFragment(A::Box<'a, JSXFragment<'a, A>>) = 33,

    /// See [`TSAsExpression`] for AST node details.
    TSAsExpression(A::Box<'a, TSAsExpression<'a, A>>) = 34,
    /// See [`TSSatisfiesExpression`] for AST node details.
    TSSatisfiesExpression(A::Box<'a, TSSatisfiesExpression<'a, A>>) = 35,
    /// See [`TSTypeAssertion`] for AST node details.
    TSTypeAssertion(A::Box<'a, TSTypeAssertion<'a, A>>) = 36,
    /// See [`TSNonNullExpression`] for AST node details.
    TSNonNullExpression(A::Box<'a, TSNonNullExpression<'a, A>>) = 37,
    /// See [`TSInstantiationExpression`] for AST node details.
    TSInstantiationExpression(A::Box<'a, TSInstantiationExpression<'a, A>>) = 38,

    // `MemberExpression` variants added here by `inherit_variants!` macro
    @inherit MemberExpression
}
}

/// Macro for matching `Expression`'s variants.
/// Includes `MemberExpression`'s variants.
#[macro_export]
macro_rules! match_expression {
    ($ty:ident) => {
        $ty::BooleanLiteral(_)
            | $ty::NullLiteral(_)
            | $ty::NumericLiteral(_)
            | $ty::BigIntLiteral(_)
            | $ty::RegExpLiteral(_)
            | $ty::StringLiteral(_)
            | $ty::TemplateLiteral(_)
            | $ty::Identifier(_)
            | $ty::MetaProperty(_)
            | $ty::Super(_)
            | $ty::ArrayExpression(_)
            | $ty::ArrowFunctionExpression(_)
            | $ty::AssignmentExpression(_)
            | $ty::AwaitExpression(_)
            | $ty::BinaryExpression(_)
            | $ty::CallExpression(_)
            | $ty::ChainExpression(_)
            | $ty::ClassExpression(_)
            | $ty::ConditionalExpression(_)
            | $ty::FunctionExpression(_)
            | $ty::ImportExpression(_)
            | $ty::LogicalExpression(_)
            | $ty::NewExpression(_)
            | $ty::ObjectExpression(_)
            | $ty::ParenthesizedExpression(_)
            | $ty::SequenceExpression(_)
            | $ty::TaggedTemplateExpression(_)
            | $ty::ThisExpression(_)
            | $ty::UnaryExpression(_)
            | $ty::UpdateExpression(_)
            | $ty::YieldExpression(_)
            | $ty::PrivateInExpression(_)
            | $ty::JSXElement(_)
            | $ty::JSXFragment(_)
            | $ty::TSAsExpression(_)
            | $ty::TSSatisfiesExpression(_)
            | $ty::TSTypeAssertion(_)
            | $ty::TSNonNullExpression(_)
            | $ty::TSInstantiationExpression(_)
            | $ty::ComputedMemberExpression(_)
            | $ty::StaticMemberExpression(_)
            | $ty::PrivateFieldExpression(_)
    };
}
pub use match_expression;

/// `foo` in `let foo = 1;`
///
/// Fundamental syntactic structure used for naming variables, functions, and properties. It must start with a Unicode letter (including $ and _) and can be followed by Unicode letters, digits, $, or _.
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "Identifier")]
pub struct IdentifierName<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub name: Atom<'a>,
}

/// `x` inside `func` in `const x = 0; function func() { console.log(x); }`
///
/// Represents an identifier reference, which is a reference to a variable, function, class, or object.
///
/// See: [13.1 Identifiers](https://tc39.es/ecma262/#sec-identifiers)
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "Identifier")]
pub struct IdentifierReference<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the identifier being referenced.
    pub name: Atom<'a>,
    /// Reference ID
    ///
    /// Identifies what identifier this refers to, and how it is used. This is
    /// set in the bind step of semantic analysis, and will always be [`None`]
    /// immediately after parsing.
    #[serde(skip)]
    #[clone_in(default)]
    pub reference_id: Cell<Option<ReferenceId>>,
}

/// `x` in `const x = 0;`
///
/// Represents a binding identifier, which is an identifier that is used to declare a variable, function, class, or object.
///
/// See: [13.1 Identifiers](https://tc39.es/ecma262/#sec-identifiers)
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "Identifier")]
pub struct BindingIdentifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The identifier name being bound.
    pub name: Atom<'a>,
    /// Unique identifier for this binding.
    ///
    /// This gets initialized during [`semantic analysis`] in the bind step. If
    /// you choose to skip semantic analysis, this will always be [`None`].
    ///
    /// [`semantic analysis`]: <https://docs.rs/oxc_semantic/latest/oxc_semantic/struct.SemanticBuilder.html>
    #[serde(skip)]
    #[clone_in(default)]
    pub symbol_id: Cell<Option<SymbolId>>,
}

/// `loop` in `loop: while (true) { break loop; }`
///
/// Represents a label identifier, which is an identifier that is used to label a statement.
///
/// See: [13.1 Identifiers](https://tc39.es/ecma262/#sec-identifiers)
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "Identifier")]
pub struct LabelIdentifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub name: Atom<'a>,
}

/// `this` in `return this.prop;`
///
/// Represents a `this` expression, which is a reference to the current object.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ThisExpression {
    #[serde(flatten)]
    pub span: Span,
}

/// `[1, 2, ...[3, 4], null]` in `const array = [1, 2, ...[3, 4], null];`
///
/// Represents an array literal, which can include elements, spread elements, or null values.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ArrayExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[tsify(type = "Array<SpreadElement | Expression | null>")]
    pub elements: A::Vec<'a, ArrayExpressionElement<'a, A>>,
    /// Array trailing comma
    /// <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#arrays>
    #[serde(skip)]
    pub trailing_comma: Option<Span>,
}

inherit_variants! {
/// Represents a element in an array literal.
///
/// Inherits variants from [`Expression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(untagged)]
pub enum ArrayExpressionElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `...[3, 4]` in `const array = [1, 2, ...[3, 4], null];`
    SpreadElement(A::Box<'a, SpreadElement<'a, A>>) = 64,
    /// `<empty>` in `const array = [1, , 2];`
    ///
    /// Array hole for sparse arrays
    /// <https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#arrays>
    Elision(Elision) = 65,
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// empty slot in `const array = [1, , 2];`
///
/// Array Expression Elision Element
/// Serialized as `null` in JSON AST. See `serialize.rs`.
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
pub struct Elision {
    pub span: Span,
}

/// `{ a: 1 }` in `const obj = { a: 1 };`
///
/// Represents an object literal, which can include properties, spread properties, or computed properties and trailing comma.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ObjectExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Properties declared in the object
    pub properties: A::Vec<'a, ObjectPropertyKind<'a, A>>,
    #[serde(skip)]
    pub trailing_comma: Option<Span>,
}

/// Represents a property in an object literal.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ObjectPropertyKind<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `a: 1` in `const obj = { a: 1 };`
    ObjectProperty(A::Box<'a, ObjectProperty<'a, A>>) = 0,
    /// `...{ a: 1 }` in `const obj = { ...{ a: 1 } };`
    SpreadProperty(A::Box<'a, SpreadElement<'a, A>>) = 1,
}

/// `a: 1` in `const obj = { a: 1 };`
///
/// Represents a property in an object literal.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ObjectProperty<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub kind: PropertyKind,
    pub key: PropertyKey<'a, A>,
    pub value: Expression<'a, A>,
    pub init: Option<Expression<'a, A>>, // for `CoverInitializedName`
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
}

inherit_variants! {
/// Property Key
///
/// Inherits variants from [`Expression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum PropertyKey<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `a` in `const obj = { a: 1 }; obj.a;`
    StaticIdentifier(A::Box<'a, IdentifierName<'a>>) = 64,
    /// `#a` in `class C { #a = 1; }; const c = new C(); c.#a;`
    PrivateIdentifier(A::Box<'a, PrivateIdentifier<'a>>) = 65,
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// Represents the kind of property in an object literal or class.
#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum PropertyKind {
    /// `a: 1` in `const obj = { a: 1 };`
    Init = 0,
    /// `get a() { return 1; }` in `const obj = { get a() { return 1; } };`
    Get = 1,
    /// `set a(value) { this._a = value; }` in `const obj = { set a(value) { this._a = value; } };`
    Set = 2,
}

/// `` `Hello, ${name}` `` in `` const foo = `Hello, ${name}` ``
///
/// Represents a template literal, which can include quasi elements and expression elements.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TemplateLiteral<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub quasis: A::Vec<'a, TemplateElement<'a>>,
    pub expressions: A::Vec<'a, Expression<'a, A>>,
}

/// `` foo`Hello, ${name}` `` in `` const foo = foo`Hello, ${name}` ``
///
/// Represents a tagged template expression, which can include a tag and a quasi.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct TaggedTemplateExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub tag: Expression<'a, A>,
    pub quasi: TemplateLiteral<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

/// `Hello, ` in `` `Hello, ${name}` ``
///
/// Represents a quasi element in a template literal.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TemplateElement<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub tail: bool,
    pub value: TemplateElementValue<'a>,
}

/// See [template-strings-cooked-vs-raw](https://exploringjs.com/js/book/ch_template-literals.html#template-strings-cooked-vs-raw)
#[ast]
#[derive(Debug)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct TemplateElementValue<'a> {
    /// A raw interpretation where backslashes do not have special meaning.
    /// For example, \t produces two characters – a backslash and a t.
    /// This interpretation of the template strings is stored in property .raw of the first argument (an Array).
    pub raw: Atom<'a>,
    /// A cooked interpretation where backslashes have special meaning.
    /// For example, \t produces a tab character.
    /// This interpretation of the template strings is stored as an Array in the first argument.
    /// cooked = None when template literal has invalid escape sequence
    pub cooked: Option<Atom<'a>>,
}

/// Represents a member access expression, which can include computed member access, static member access, or private field access.
///
/// <https://tc39.es/ecma262/#prod-MemberExpression>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum MemberExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `ar[0]` in `const ar = [1, 2]; ar[0];`
    ComputedMemberExpression(A::Box<'a, ComputedMemberExpression<'a, A>>) = 48,
    /// `console.log` in `console.log('Hello, World!');`
    StaticMemberExpression(A::Box<'a, StaticMemberExpression<'a, A>>) = 49,
    /// `c.#a` in `class C { #a = 1; }; const c = new C(); c.#a;`
    PrivateFieldExpression(A::Box<'a, PrivateFieldExpression<'a, A>>) = 50,
}

/// Macro for matching `MemberExpression`'s variants.
#[macro_export]
macro_rules! match_member_expression {
    ($ty:ident) => {
        $ty::ComputedMemberExpression(_)
            | $ty::StaticMemberExpression(_)
            | $ty::PrivateFieldExpression(_)
    };
}
pub use match_member_expression;

/// `ar[0]` in `const ar = [1, 2]; ar[0];`
///
/// Represents a computed member access expression, which can include an object and an expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ComputedMemberExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub object: Expression<'a, A>,
    pub expression: Expression<'a, A>,
    pub optional: bool, // for optional chaining
}

/// `console.log` in `console.log('Hello, World!');`
///
/// Represents a static member access expression, which can include an object and a property.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct StaticMemberExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub object: Expression<'a, A>,
    pub property: IdentifierName<'a>,
    pub optional: bool, // for optional chaining
}

/// `c.#a` in `class C { #a = 1; }; const c = new C(); c.#a;`
///
/// Represents a private field access expression, which can include an object and a private identifier.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct PrivateFieldExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub object: Expression<'a, A>,
    pub field: PrivateIdentifier<'a>,
    pub optional: bool, // for optional chaining
}

/// `foo()` in `function foo() { return 1; }; foo();`
///
/// Represents a call expression, which can include a callee and arguments.
///
/// ## Examples
/// ```ts
/// //        ___ callee
/// const x = foo(1, 2)
///
/// //            ^^^^ arguments
/// const y = foo.bar?.(1, 2)
/// //               ^ optional
///
/// const z = foo<number, string>(1, 2)
/// //            ^^^^^^^^^^^^^^ type_parameters
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct CallExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub callee: Expression<'a, A>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
    pub arguments: A::Vec<'a, Argument<'a, A>>,
    pub optional: bool, // for optional chaining
}

/// `new C()` in `class C {}; new C();`
///
/// Represents a new expression, which can include a callee and arguments.
///
/// ## Example
/// ```ts
/// //           callee         arguments
/// //              ↓↓↓         ↓↓↓↓
/// const foo = new Foo<number>(1, 2)
/// //                 ↑↑↑↑↑↑↑↑
/// //                 type_parameters
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct NewExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub callee: Expression<'a, A>,
    pub arguments: A::Vec<'a, Argument<'a, A>>,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
}

/// `import.meta` in `console.log(import.meta);`
///
/// Represents a meta property. The following syntaxes are supported. `import.meta`, `new.target`.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct MetaProperty<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub meta: IdentifierName<'a>,
    pub property: IdentifierName<'a>,
}

/// `...[1, 2]` in `const arr = [...[1, 2]];`
///
/// Represents a spread element, which can include an argument.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct SpreadElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The expression being spread.
    pub argument: Expression<'a, A>,
}

inherit_variants! {
/// Argument
///
/// Inherits variants from [`Expression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum Argument<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `...[1, 2]` in `const arr = [...[1, 2]];`
    SpreadElement(A::Box<'a, SpreadElement<'a, A>>) = 64,
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// `++i` in `let i = 0; ++i;`
///
/// Represents an update expression, which can include an operator and an argument. The following syntaxes are supported. `++a`, `a++`, `--a`, `a--`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct UpdateExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub operator: UpdateOperator,
    pub prefix: bool,
    pub argument: SimpleAssignmentTarget<'a, A>,
}

/// `typeof` in `typeof a === "string"`
///
/// Represents a unary expression, which can include an operator and an argument. The following syntaxes are supported. `+a`, `-a`, `~a`, `!a`, `delete a`, `void a`, `typeof a`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct UnaryExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub operator: UnaryOperator,
    pub argument: Expression<'a, A>,
}

/// `1 + 1` in `const two = 1 + 1;`
///
/// Represents a binary expression, which can include a left expression, an operator, and a right expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct BinaryExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: Expression<'a, A>,
    pub operator: BinaryOperator,
    pub right: Expression<'a, A>,
}

/// `#brand in obj` in `class Foo { #brand; static isFoo(obj) { return #brand in obj; } }`
///
/// Represents a private in expression, which can include a private identifier, an operator, and a expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct PrivateInExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: PrivateIdentifier<'a>,
    pub operator: BinaryOperator, // BinaryOperator::In
    pub right: Expression<'a, A>,
}

/// `||` in `const foo = bar || 2;`
///
/// Represents a logical expression, which can include a left expression, an operator, and a right expression. The following syntaxes are supported. `||`, `&&` and `??`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct LogicalExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: Expression<'a, A>,
    pub operator: LogicalOperator,
    pub right: Expression<'a, A>,
}

/// `bar ? 1 : 2` in `const foo = bar ? 1 : 2;`
///
/// Represents a conditional expression, which can include a test, a consequent, and an alternate.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ConditionalExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub test: Expression<'a, A>,
    pub consequent: Expression<'a, A>,
    pub alternate: Expression<'a, A>,
}

/// `foo = 1` in `let foo; foo = 1;`
///
/// Represents an assignment expression, which can include an operator, a target, and a expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AssignmentExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub operator: AssignmentOperator,
    pub left: AssignmentTarget<'a, A>,
    pub right: Expression<'a, A>,
}

inherit_variants! {
/// Destructuring Assignment
///
/// Inherits variants from [`SimpleAssignmentTarget`] and [`AssignmentTargetPattern`].
/// See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum AssignmentTarget<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // `SimpleAssignmentTarget` variants added here by `inherit_variants!` macro
    @inherit SimpleAssignmentTarget
    // `AssignmentTargetPattern` variants added here by `inherit_variants!` macro
    @inherit AssignmentTargetPattern
}
}

inherit_variants! {
/// Simple Assignment Target
///
/// Inherits variants from [`MemberExpression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum SimpleAssignmentTarget<'a, A: AstAllocator = oxc_allocator::Allocator> {
    AssignmentTargetIdentifier(A::Box<'a, IdentifierReference<'a>>) = 0,
    TSAsExpression(A::Box<'a, TSAsExpression<'a, A>>) = 1,
    TSSatisfiesExpression(A::Box<'a, TSSatisfiesExpression<'a, A>>) = 2,
    TSNonNullExpression(A::Box<'a, TSNonNullExpression<'a, A>>) = 3,
    TSTypeAssertion(A::Box<'a, TSTypeAssertion<'a, A>>) = 4,
    TSInstantiationExpression(A::Box<'a, TSInstantiationExpression<'a, A>>) = 5,
    // `MemberExpression` variants added here by `inherit_variants!` macro
    @inherit MemberExpression
}
}

/// Macro for matching `AssignmentTarget`'s variants.
/// Includes `SimpleAssignmentTarget`'s and `AssignmentTargetPattern`'s variants.
#[macro_export]
macro_rules! match_assignment_target {
    ($ty:ident) => {
        $ty::AssignmentTargetIdentifier(_)
            | $ty::ComputedMemberExpression(_)
            | $ty::StaticMemberExpression(_)
            | $ty::PrivateFieldExpression(_)
            | $ty::TSAsExpression(_)
            | $ty::TSSatisfiesExpression(_)
            | $ty::TSNonNullExpression(_)
            | $ty::TSTypeAssertion(_)
            | $ty::TSInstantiationExpression(_)
            | $ty::ArrayAssignmentTarget(_)
            | $ty::ObjectAssignmentTarget(_)
    };
}
pub use match_assignment_target;

/// Macro for matching `SimpleAssignmentTarget`'s variants.
/// Includes `MemberExpression`'s variants
#[macro_export]
macro_rules! match_simple_assignment_target {
    ($ty:ident) => {
        $ty::AssignmentTargetIdentifier(_)
            | $ty::ComputedMemberExpression(_)
            | $ty::StaticMemberExpression(_)
            | $ty::PrivateFieldExpression(_)
            | $ty::TSAsExpression(_)
            | $ty::TSSatisfiesExpression(_)
            | $ty::TSNonNullExpression(_)
            | $ty::TSTypeAssertion(_)
            | $ty::TSInstantiationExpression(_)
    };
}
pub use match_simple_assignment_target;

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum AssignmentTargetPattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    ArrayAssignmentTarget(A::Box<'a, ArrayAssignmentTarget<'a, A>>) = 8,
    ObjectAssignmentTarget(A::Box<'a, ObjectAssignmentTarget<'a, A>>) = 9,
}

/// Macro for matching `AssignmentTargetPattern`'s variants.
#[macro_export]
macro_rules! match_assignment_target_pattern {
    ($ty:ident) => {
        $ty::ArrayAssignmentTarget(_) | $ty::ObjectAssignmentTarget(_)
    };
}
pub use match_assignment_target_pattern;

/// `[a, b]` in `[a, b] = arr;`
///
/// Represents an array assignment target, which can include elements and a rest element.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ArrayAssignmentTarget<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[tsify(type = "Array<AssignmentTargetMaybeDefault | AssignmentTargetRest | null>")]
    pub elements: A::Vec<'a, Option<AssignmentTargetMaybeDefault<'a, A>>>,
    #[serde(skip)]
    pub rest: Option<AssignmentTargetRest<'a, A>>,
    #[serde(skip)]
    pub trailing_comma: Option<Span>,
}

/// `{ foo }` in `({ foo } = obj);`
///
/// Represents an object assignment target, which can include properties and a rest element.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ObjectAssignmentTarget<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[tsify(type = "Array<AssignmentTargetProperty | AssignmentTargetRest>")]
    pub properties: A::Vec<'a, AssignmentTargetProperty<'a, A>>,
    #[serde(skip)]
    pub rest: Option<AssignmentTargetRest<'a, A>>,
}

/// `rest` in `[foo, ...rest] = arr;`
///
/// Represents a rest element in an array assignment target, which can include a target.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "RestElement")]
pub struct AssignmentTargetRest<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[serde(rename = "argument")]
    pub target: AssignmentTarget<'a, A>,
}

inherit_variants! {
/// Assignment Target Maybe Default
///
/// Inherits variants from [`AssignmentTarget`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum AssignmentTargetMaybeDefault<'a, A: AstAllocator = oxc_allocator::Allocator> {
    AssignmentTargetWithDefault(A::Box<'a, AssignmentTargetWithDefault<'a, A>>) = 16,
    // `AssignmentTarget` variants added here by `inherit_variants!` macro
    @inherit AssignmentTarget
}
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AssignmentTargetWithDefault<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub binding: AssignmentTarget<'a, A>,
    pub init: Expression<'a, A>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum AssignmentTargetProperty<'a, A: AstAllocator = oxc_allocator::Allocator> {
    AssignmentTargetPropertyIdentifier(A::Box<'a, AssignmentTargetPropertyIdentifier<'a, A>>) = 0,
    AssignmentTargetPropertyProperty(A::Box<'a, AssignmentTargetPropertyProperty<'a, A>>) = 1,
}

/// `foo` in `({ foo } = obj);`
///
/// Represents an assignment target property identifier, which can include a binding and an init expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AssignmentTargetPropertyIdentifier<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub binding: IdentifierReference<'a>,
    pub init: Option<Expression<'a, A>>,
}

/// `foo: bar` in `({ foo: bar } = obj);`
///
/// Represents an assignment target property property, which can include a name and a binding.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AssignmentTargetPropertyProperty<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub name: PropertyKey<'a, A>,
    pub binding: AssignmentTargetMaybeDefault<'a, A>,
}

/// `a++, b++` in `let a = 1, b = 2; let result = (a++, b++);`
///
/// Represents a sequence expression, which can include expressions.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct SequenceExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expressions: A::Vec<'a, Expression<'a, A>>,
}

/// `super` in `class C extends B { constructor() { super(); } }`
///
/// Represents a super expression.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct Super {
    #[serde(flatten)]
    pub span: Span,
}

/// `await` in `await foo();`
///
/// Represents an await expression, which can include an argument.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AwaitExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub argument: Expression<'a, A>,
}

/// `foo?.bar` in `foo?.bar;`
///
/// Represents a chain expression, which can include an expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ChainExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: ChainElement<'a, A>,
}

inherit_variants! {
/// Chain Element
///
/// Inherits variants from [`MemberExpression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum ChainElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    CallExpression(A::Box<'a, CallExpression<'a, A>>) = 0,
    // `MemberExpression` variants added here by `inherit_variants!` macro
    @inherit MemberExpression
}
}

/// `(a + b)` in `const res = (a + b) / c;`
///
/// Represents a parenthesized expression, which can include an expression.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ParenthesizedExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
}

inherit_variants! {
/// Statement
///
/// Inherits variants from [`Declaration`] and [`ModuleDeclaration`].
/// See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // Statements
    BlockStatement(A::Box<'a, BlockStatement<'a, A>>) = 0,
    BreakStatement(A::Box<'a, BreakStatement<'a>>) = 1,
    ContinueStatement(A::Box<'a, ContinueStatement<'a>>) = 2,
    DebuggerStatement(A::Box<'a, DebuggerStatement>) = 3,
    DoWhileStatement(A::Box<'a, DoWhileStatement<'a, A>>) = 4,
    EmptyStatement(A::Box<'a, EmptyStatement>) = 5,
    ExpressionStatement(A::Box<'a, ExpressionStatement<'a, A>>) = 6,
    ForInStatement(A::Box<'a, ForInStatement<'a, A>>) = 7,
    ForOfStatement(A::Box<'a, ForOfStatement<'a, A>>) = 8,
    ForStatement(A::Box<'a, ForStatement<'a, A>>) = 9,
    IfStatement(A::Box<'a, IfStatement<'a, A>>) = 10,
    LabeledStatement(A::Box<'a, LabeledStatement<'a, A>>) = 11,
    ReturnStatement(A::Box<'a, ReturnStatement<'a, A>>) = 12,
    SwitchStatement(A::Box<'a, SwitchStatement<'a, A>>) = 13,
    ThrowStatement(A::Box<'a, ThrowStatement<'a, A>>) = 14,
    TryStatement(A::Box<'a, TryStatement<'a, A>>) = 15,
    WhileStatement(A::Box<'a, WhileStatement<'a, A>>) = 16,
    WithStatement(A::Box<'a, WithStatement<'a, A>>) = 17,
    // `Declaration` variants added here by `inherit_variants!` macro
    @inherit Declaration
    // `ModuleDeclaration` variants added here by `inherit_variants!` macro
    @inherit ModuleDeclaration
}
}

/// `"use strict";` in `"use strict";`
///
/// Represents a directive statement, which can include a string literal.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct Directive<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// Directive with any escapes unescaped
    pub expression: StringLiteral<'a>,
    /// Raw content of directive as it appears in source, any escapes left as is
    pub directive: Atom<'a>,
}

/// `#! /usr/bin/env node` in `#! /usr/bin/env node`
///
/// Represents a hashbang directive, which can include a value.
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct Hashbang<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub value: Atom<'a>,
}

/// `{ let foo = 1; }` in `if(true) { let foo = 1; }`
///
/// Represents a block statement, which can include a body.
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct BlockStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: A::Vec<'a, Statement<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// Declarations and the Variable Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum Declaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    VariableDeclaration(A::Box<'a, VariableDeclaration<'a, A>>) = 32,
    #[visit(args(flags = ScopeFlags::Function))]
    FunctionDeclaration(A::Box<'a, Function<'a, A>>) = 33,
    ClassDeclaration(A::Box<'a, Class<'a, A>>) = 34,

    TSTypeAliasDeclaration(A::Box<'a, TSTypeAliasDeclaration<'a, A>>) = 35,
    TSInterfaceDeclaration(A::Box<'a, TSInterfaceDeclaration<'a, A>>) = 36,
    TSEnumDeclaration(A::Box<'a, TSEnumDeclaration<'a, A>>) = 37,
    TSModuleDeclaration(A::Box<'a, TSModuleDeclaration<'a, A>>) = 38,
    TSImportEqualsDeclaration(A::Box<'a, TSImportEqualsDeclaration<'a, A>>) = 39,
}

/// Macro for matching `Declaration`'s variants.
#[macro_export]
macro_rules! match_declaration {
    ($ty:ident) => {
        $ty::VariableDeclaration(_)
            | $ty::FunctionDeclaration(_)
            | $ty::ClassDeclaration(_)
            | $ty::TSTypeAliasDeclaration(_)
            | $ty::TSInterfaceDeclaration(_)
            | $ty::TSEnumDeclaration(_)
            | $ty::TSModuleDeclaration(_)
            | $ty::TSImportEqualsDeclaration(_)
    };
}
pub use match_declaration;

/// `let a;` in `let a; a = 1;`
///
/// Represents a variable declaration, which can include a kind, declarations, and modifiers.
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct VariableDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub kind: VariableDeclarationKind,
    pub declarations: A::Vec<'a, VariableDeclarator<'a, A>>,
    pub declare: bool,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum VariableDeclarationKind {
    Var = 0,
    Const = 1,
    Let = 2,
    Using = 3,
    #[serde(rename = "await using")]
    AwaitUsing = 4,
}

/// A single variable declaration in a list of [variable declarations](VariableDeclaration).
///
/// ## Examples
/// ```ts
/// // declarators may or may not have initializers
/// let foo, b = 1;
/// //  ^^^ id   ^ init
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct VariableDeclarator<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[serde(skip)]
    pub kind: VariableDeclarationKind,
    pub id: BindingPattern<'a, A>,
    pub init: Option<Expression<'a, A>>,
    pub definite: Option<TSDefiniteMark>,
}

/// Empty Statement
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct EmptyStatement {
    #[serde(flatten)]
    pub span: Span,
}

/// Expression Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ExpressionStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub expression: Expression<'a, A>,
}

/// If Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct IfStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub test: Expression<'a, A>,
    pub consequent: Statement<'a, A>,
    pub alternate: Option<Statement<'a, A>>,
}

/// Do-While Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct DoWhileStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: Statement<'a, A>,
    pub test: Expression<'a, A>,
}

/// While Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct WhileStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub test: Expression<'a, A>,
    pub body: Statement<'a, A>,
}

/// For Statement
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ForStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub init: Option<ForStatementInit<'a, A>>,
    pub test: Option<Expression<'a, A>>,
    pub update: Option<Expression<'a, A>>,
    pub body: Statement<'a, A>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

inherit_variants! {
/// For Statement Init
///
/// Inherits variants from [`Expression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum ForStatementInit<'a, A: AstAllocator = oxc_allocator::Allocator> {
    VariableDeclaration(A::Box<'a, VariableDeclaration<'a, A>>) = 64,
    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// For-In Statement
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ForInStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: ForStatementLeft<'a, A>,
    pub right: Expression<'a, A>,
    pub body: Statement<'a, A>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

inherit_variants! {
/// For Statement Left
///
/// Inherits variants from [`AssignmentTarget`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum ForStatementLeft<'a, A: AstAllocator = oxc_allocator::Allocator> {
    VariableDeclaration(A::Box<'a, VariableDeclaration<'a, A>>) = 16,
    // `AssignmentTarget` variants added here by `inherit_variants!` macro
    @inherit AssignmentTarget
}
}
/// For-Of Statement
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ForOfStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub r#await: bool,
    pub left: ForStatementLeft<'a, A>,
    pub right: Expression<'a, A>,
    pub body: Statement<'a, A>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// Continue Statement
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ContinueStatement<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub label: Option<LabelIdentifier<'a>>,
}

/// Break Statement
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct BreakStatement<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub label: Option<LabelIdentifier<'a>>,
}

/// Return Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ReturnStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub argument: Option<Expression<'a, A>>,
}

/// With Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct WithStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub object: Expression<'a, A>,
    pub body: Statement<'a, A>,
}

/// Switch Statement
#[ast(visit)]
#[scope]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct SwitchStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub discriminant: Expression<'a, A>,
    #[scope(enter_before)]
    pub cases: A::Vec<'a, SwitchCase<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct SwitchCase<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub test: Option<Expression<'a, A>>,
    pub consequent: A::Vec<'a, Statement<'a, A>>,
}

/// Labelled Statement
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct LabeledStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub label: LabelIdentifier<'a>,
    pub body: Statement<'a, A>,
}

/// Throw Statement
///
/// # Example
/// ```ts
/// throw new Error('something went wrong!');
/// //    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ argument
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ThrowStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The expression being thrown, e.g. `err` in `throw err;`
    pub argument: Expression<'a, A>,
}

/// Try Statement
///
/// # Example
/// ```ts
/// var x;
/// let didRun = false;
///
/// try {                 // block
///     x = 1;
/// } catch (e) {         // handler
///     console.error(e);
/// } finally {           // finalizer
///     didRun = true;
/// }
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct TryStatement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Statements in the `try` block
    pub block: A::Box<'a, BlockStatement<'a, A>>,
    /// The `catch` clause, including the parameter and the block statement
    pub handler: Option<A::Box<'a, CatchClause<'a, A>>>,
    /// The `finally` clause
    #[visit(as(FinallyClause))]
    pub finalizer: Option<A::Box<'a, BlockStatement<'a, A>>>,
}

/// Catch Clause in a [`try/catch` statement](TryStatement).
///
/// This node creates a new scope inside its `body`.
///
/// # Example
/// ```ts
/// try {
///   throw new Error('foo');
/// } catch (e) {             // `param` is `e`
///   console.error(e);       // `body`
/// }
/// ```
#[ast(visit)]
#[scope(flags(ScopeFlags::CatchClause))]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct CatchClause<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The caught error parameter, e.g. `e` in `catch (e) {}`
    pub param: Option<CatchParameter<'a, A>>,
    /// The statements run when an error is caught
    pub body: A::Box<'a, BlockStatement<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// A caught error parameter in a [catch clause](CatchClause).
///
/// # Examples
///
/// ```ts
/// try {} catch (err) {}
/// //            ^^^ pattern
/// ```
///
/// ```ts
/// try {} catch ({ err }) {}
/// //            ^^^^^^^  pattern
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct CatchParameter<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// The bound error
    pub pattern: BindingPattern<'a, A>,
}

/// Debugger Statement
///
/// # Example
/// ```ts
/// let x = 1;
/// debugger; // <--
/// ```
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub span: Span,
}

/// Destructuring Binding Patterns
/// * <https://tc39.es/ecma262/#prod-BindingPattern>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct BindingPattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    // serde(flatten) the attributes because estree has no `BindingPattern`
    #[serde(flatten)]
    #[tsify(type = "(BindingIdentifier | ObjectPattern | ArrayPattern | AssignmentPattern)")]
    #[span]
    pub kind: BindingPatternKind<'a, A>,
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
    pub optional: Option<TSOptionalMark>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum BindingPatternKind<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `const a = 1`
    BindingIdentifier(A::Box<'a, BindingIdentifier<'a>>) = 0,
    /// `const {a} = 1`
    ObjectPattern(A::Box<'a, ObjectPattern<'a, A>>) = 1,
    /// `const [a] = 1`
    ArrayPattern(A::Box<'a, ArrayPattern<'a, A>>) = 2,
    /// A defaulted binding pattern, i.e.:
    /// `const {a = 1} = 1`
    /// the assignment pattern is `a = 1`
    /// it has an inner left that has a BindingIdentifier
    AssignmentPattern(A::Box<'a, AssignmentPattern<'a, A>>) = 3,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct AssignmentPattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub left: BindingPattern<'a, A>,
    pub right: Expression<'a, A>,
}

// See serializer in serialize.rs
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ObjectPattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[tsify(type = "Array<BindingProperty | BindingRestElement>")]
    pub properties: A::Vec<'a, BindingProperty<'a, A>>,
    #[serde(skip)]
    pub rest: Option<A::Box<'a, BindingRestElement<'a, A>>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct BindingProperty<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub key: PropertyKey<'a, A>,
    pub value: BindingPattern<'a, A>,
    pub shorthand: bool,
    pub computed: bool,
}

// See serializer in serialize.rs
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ArrayPattern<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    #[tsify(type = "Array<BindingPattern | BindingRestElement | null>")]
    pub elements: A::Vec<'a, Option<BindingPattern<'a, A>>>,
    #[serde(skip)]
    pub rest: Option<A::Box<'a, BindingRestElement<'a, A>>>,
}

/// A `...rest` binding in an [array](ArrayPattern) or [object](ObjectPattern) destructure.
///
/// ## Examples
/// ```ts
/// const [a, ...rest] = [1, 2, 3];
/// //           ^^^^  argument
/// const { x, y, ...others} = foo.bar();
/// //               ^^^^^^  argument
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize), serde(bound = ""))]
#[serde(tag = "type", rename = "RestElement")]
pub struct BindingRestElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub argument: BindingPattern<'a, A>,
}

/// Function Statement or Expression
///
/// Includes generator functions and function-valued class properties.
/// Arrow functions are represented by [`ArrowFunctionExpression`].
///
/// # Examples
/// ```ts
/// //    id ___             ____ return_type
/// function foo(a: number): void {
/// //           ^^^^^^^^^ params
///     console.log(a);
/// }
/// ```
///
/// ```ts
/// // `async` and `generator` are true
/// async function* foo() {
///     yield 1;
/// }
/// ```
///
/// ```js
/// // function.id is None
/// // use function.r#type to check if a node is a function expression.
/// const foo = function() { }
/// ```
///
/// ```ts
/// // Function overloads will not have a body
/// function add(a: number, b: number): number; // <-- No body
/// function add(a: string, b: string): string; // <-- No body
/// function add(a: any, b: any): any {         // <-- Body is between `{}`, inclusive.
///    return a + b;
/// }
/// ```
#[ast(visit)]
#[scope(
    // `flags` passed in to visitor via parameter defined by `#[visit(args(flags = ...))]` on parents
    flags(flags),
    strict_if(self.is_strict()),
)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct Function<'a, A: AstAllocator = oxc_allocator::Allocator> {
    pub r#type: FunctionType,
    #[serde(flatten)]
    pub span: Span,
    /// The function identifier. [`None`] for anonymous function expressions.
    pub id: Option<BindingIdentifier<'a>>,
    /// Is this a generator function?
    ///
    /// ```ts
    /// function* foo() { } // <- generator: true
    /// function bar() { }  // <- generator: false
    /// ```
    pub generator: bool,
    pub r#async: bool,
    pub declare: bool,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    /// Declaring `this` in a Function <https://www.typescriptlang.org/docs/handbook/2/functions.html#declaring-this-in-a-function>
    ///
    /// The JavaScript specification states that you cannot have a parameter called `this`,
    /// and so TypeScript uses that syntax space to let you declare the type for `this` in the function body.
    ///
    /// ```ts
    /// interface DB {
    ///     filterUsers(filter: (this: User) => boolean): User[];
    ///     //                   ^^^^
    /// }
    ///
    /// const db = getDB();
    /// const admins = db.filterUsers(function (this: User) {
    ///     return this.admin;
    /// });
    /// ```
    pub this_param: Option<A::Box<'a, TSThisParameter<'a, A>>>,
    /// Function parameters.
    ///
    /// Does not include `this` parameters used by some TypeScript functions.
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    /// The TypeScript return type annotation.
    pub return_type: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
    /// The function body.
    ///
    /// [`None`] for function declarations, e.g.
    /// ```ts
    /// // TypeScript function declarations have no body
    /// declare function foo(a: number): number;
    ///
    /// function bar(a: number): number; // <- overloads have no body
    /// function bar(a: number): number {
    ///     return a;
    /// }
    /// ```
    pub body: Option<A::Box<'a, FunctionBody<'a, A>>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum FunctionType {
    FunctionDeclaration = 0,
    FunctionExpression = 1,
    TSDeclareFunction = 2,
    /// <https://github.com/typescript-eslint/typescript-eslint/pull/1289>
    TSEmptyBodyFunctionExpression = 3,
}

/// <https://tc39.es/ecma262/#prod-FormalParameters>
// See serializer in serialize.rs
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct FormalParameters<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub kind: FormalParameterKind,
    #[tsify(type = "Array<FormalParameter | FormalParameterRest>")]
    pub items: A::Vec<'a, FormalParameter<'a, A>>,
    #[serde(skip)]
    pub rest: Option<A::Box<'a, BindingRestElement<'a, A>>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub struct FormalParameterModifiers {
    #[serde(flatten)]
    pub span: Span,
    pub accessibility: Option<TSAccessibility>,
    pub readonly: bool,
    pub r#override: bool,
}
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct FormalParameter<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub decorators: A::Vec<'a, Decorator<'a, A>>,
    pub modifiers: Option<FormalParameterModifiers>,
    pub pattern: BindingPattern<'a, A>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum FormalParameterKind {
    /// <https://tc39.es/ecma262/#prod-FormalParameters>
    FormalParameter = 0,
    /// <https://tc39.es/ecma262/#prod-UniqueFormalParameters>
    UniqueFormalParameters = 1,
    /// <https://tc39.es/ecma262/#prod-ArrowFormalParameters>
    ArrowFormalParameters = 2,
    /// Part of TypeScript type signatures
    Signature = 3,
}

/// <https://tc39.es/ecma262/#prod-FunctionBody>
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct FunctionBody<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub directives: A::Vec<'a, Directive<'a>>,
    pub statements: A::Vec<'a, Statement<'a, A>>,
}

/// Arrow Function Definitions
#[ast(visit)]
#[scope(
    flags(ScopeFlags::Function | ScopeFlags::Arrow),
    strict_if(self.body.has_use_strict_directive()),
)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ArrowFunctionExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Is the function body an arrow expression? i.e. `() => expr` instead of `() => {}`
    pub expression: bool,
    pub r#async: bool,
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    pub params: A::Box<'a, FormalParameters<'a, A>>,
    pub return_type: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
    /// See `expression` for whether this arrow expression returns an expression.
    pub body: A::Box<'a, FunctionBody<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// Generator Function Definitions
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct YieldExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub delegate: bool,
    pub argument: Option<Expression<'a, A>>,
}

/// Class Definitions

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct ClassModifiers {
    #[serde(flatten)]
    pub span: Span,
    /// Whether the class is abstract
    ///
    /// ## Example
    /// ```ts
    /// class Foo {}          // true
    /// abstract class Bar {} // false
    /// ```
    pub r#abstract: bool,
    /// Whether the class was `declare`ed
    ///
    /// ## Example
    /// ```ts
    /// declare class Foo {}
    /// ```
    pub declare: bool,
}

#[ast(visit)]
#[scope(flags(ScopeFlags::StrictMode))]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct Class<'a, A: AstAllocator = oxc_allocator::Allocator> {
    pub r#type: ClassType,
    #[serde(flatten)]
    pub span: Span,
    /// Decorators applied to the class.
    ///
    /// Decorators are currently a stage 3 proposal. Oxc handles both TC39 and
    /// legacy TypeScript decorators.
    ///
    /// ## Example
    /// ```ts
    /// @Bar() // <-- Decorator
    /// class Foo {}
    /// ```
    pub decorators: A::Vec<'a, Decorator<'a, A>>,

    pub modifiers: ClassModifiers,
    /// Class identifier, AKA the name
    pub id: Option<BindingIdentifier<'a>>,

    #[scope(enter_before)]
    pub type_parameters: Option<A::Box<'a, TSTypeParameterDeclaration<'a, A>>>,
    /// Super class. When present, this will usually be an [`IdentifierReference`].
    ///
    /// ## Example
    /// ```ts
    /// class Foo extends Bar {}
    /// //                ^^^
    /// ```
    #[visit(as(ClassHeritage))]
    pub super_class: Option<Expression<'a, A>>,
    /// Type parameters passed to super class.
    ///
    /// ## Example
    /// ```ts
    /// class Foo<T> extends Bar<T> {}
    /// //                       ^
    /// ```
    pub super_type_parameters: Option<A::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
    /// Interface implementation clause for TypeScript classes.
    ///
    /// ## Example
    /// ```ts
    /// interface Bar {}
    /// class Foo implements Bar {}
    /// //                   ^^^
    /// ```
    pub implements: Option<TSClassImplements<'a, A>>,
    pub body: A::Box<'a, ClassBody<'a, A>>,
    /// Id of the scope created by the [`Class`], including type parameters and
    /// statements within the [`ClassBody`].
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum ClassType {
    /// Class declaration statement
    /// ```ts
    /// class Foo { }
    /// ```
    ClassDeclaration = 0,
    /// Class expression
    ///
    /// ```ts
    /// const Foo = class {}
    /// ```
    ClassExpression = 1,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ClassBody<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: A::Vec<'a, ClassElement<'a, A>>,
}

/// Class Body Element
///
/// ## Example
/// ```ts
/// class Foo {
///   [prop: string]: string // ClassElement::TSIndexSignature
///
///   public x: number // ClassElement::PropertyDefinition
///
///   accessor z() { return 5 } // ClassElement::AccessorProperty
///
///   // These are all ClassElement::MethodDefinitions
///   get y() { return 5 }
///   set y(value) { }
///   static foo() {}
///   bar() {}
/// }
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ClassElement<'a, A: AstAllocator = oxc_allocator::Allocator> {
    StaticBlock(A::Box<'a, StaticBlock<'a, A>>) = 0,
    /// Class Methods
    ///
    /// Includes static and non-static methods, constructors, getters, and setters.
    MethodDefinition(A::Box<'a, MethodDefinition<'a, A>>) = 1,
    PropertyDefinition(A::Box<'a, PropertyDefinition<'a, A>>) = 2,
    AccessorProperty(A::Box<'a, AccessorProperty<'a, A>>) = 3,
    /// Index Signature
    ///
    /// ## Example
    /// ```ts
    /// class Foo {
    ///   [keys: string]: string
    /// }
    /// ```
    TSIndexSignature(A::Box<'a, TSIndexSignature<'a, A>>) = 4,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct MethodDefinition<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub decorators: A::Vec<'a, Decorator<'a, A>>,
    pub modifiers: ClassElementModifiers,
    pub key: PropertyKey<'a, A>,
    #[visit(args(flags = match self.kind {
        MethodDefinitionKind::Get => ScopeFlags::Function | ScopeFlags::GetAccessor,
        MethodDefinitionKind::Set => ScopeFlags::Function | ScopeFlags::SetAccessor,
        MethodDefinitionKind::Constructor => ScopeFlags::Function | ScopeFlags::Constructor,
        MethodDefinitionKind::Method => ScopeFlags::Function,
    }))]
    pub value: A::Box<'a, Function<'a, A>>, // FunctionExpression
    pub kind: MethodDefinitionKind,
    pub computed: bool,
    pub optional: Option<TSOptionalMark>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum MethodDefinitionType {
    MethodDefinition = 0,
    TSAbstractMethodDefinition = 1,
}

#[ast(visit)]
#[derive(Debug, Clone, Copy)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct ClassElementModifiers {
    #[serde(flatten)]
    pub span: Span,

    pub r#async: bool,

    pub r#abstract: bool,

    /// Property was declared with a `static` modifier
    pub r#static: bool,
    /// Property is declared with a `declare` modifier.
    ///
    /// ## Example
    /// ```ts
    /// class Foo {
    ///   x: number         // false
    ///   declare y: string // true
    /// }
    ///
    /// declare class Bar {
    ///   x: number         // false
    /// }
    /// ```
    pub declare: bool,
    pub r#override: bool,
    /// `true` when declared with a `readonly` modifier
    pub readonly: bool,

    /// Accessibility modifier.
    ///
    /// Only ever [`Some`] for TypeScript files.
    ///
    /// ## Example
    ///
    /// ```ts
    /// class Foo {
    ///   public w: number     // Some(TSAccessibility::Public)
    ///   private x: string    // Some(TSAccessibility::Private)
    ///   protected y: boolean // Some(TSAccessibility::Protected)
    ///   readonly z           // None
    /// }
    /// ```
    pub accessibility: Option<TSAccessibility>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct PropertyDefinition<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Decorators applied to the property.
    ///
    /// See [`Decorator`] for more information.
    pub decorators: A::Vec<'a, Decorator<'a, A>>,

    pub modifiers: ClassElementModifiers,
    /// The expression used to declare the property.
    pub key: PropertyKey<'a, A>,

    pub optional: Option<TSOptionalMark>,
    pub definite: Option<TSDefiniteMark>,

    /// Initialized value in the declaration.
    ///
    /// ## Example
    /// ```
    /// class Foo {
    ///   x = 5     // Some(NumericLiteral)
    ///   y: string // None
    ///
    ///   constructor() {
    ///     this.y = "hello"
    ///   }
    /// }
    /// ```
    pub value: Option<Expression<'a, A>>,

    /// Property was declared with a computed key
    ///
    /// ## Example
    /// ```ts
    /// class Foo {
    ///   ["a"]: string // true
    ///   b: number     // false
    /// }
    /// ```
    pub computed: bool,
    /// Type annotation on the property.
    ///
    /// Will only ever be [`Some`] for TypeScript files.
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum PropertyDefinitionType {
    PropertyDefinition = 0,
    TSAbstractPropertyDefinition = 1,
}

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub enum MethodDefinitionKind {
    /// Class constructor
    Constructor = 0,
    /// Static or instance method
    Method = 1,
    /// Getter method
    Get = 2,
    /// Setter method
    Set = 3,
}

/// An identifier for a private class member.
///
/// See: [MDN - Private class fields](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Private_class_fields)
#[ast(visit)]
#[derive(Debug, Clone)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct PrivateIdentifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub name: Atom<'a>,
}

/// Class Static Block
///
/// See: [MDN - Static initialization blocks](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Static_initialization_blocks)
///
/// ## Example
///
/// ```ts
/// class Foo {
///     static {
///         this.someStaticProperty = 5;
///     }
/// }
/// ```
#[ast(visit)]
#[scope(flags(ScopeFlags::ClassStaticBlock))]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct StaticBlock<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub body: A::Vec<'a, Statement<'a, A>>,
    #[serde(skip)]
    #[clone_in(default)]
    pub scope_id: Cell<Option<ScopeId>>,
}

/// ES6 Module Declaration
///
/// An ESM import or export statement.
///
/// ## Example
///
/// ```ts
/// // ImportDeclaration
/// import { foo } from 'foo';
/// import bar from 'bar';
/// import * as baz from 'baz';
///
/// // Not a ModuleDeclaration
/// export const a = 5;
///
/// const b = 6;
///
/// export { b };             // ExportNamedDeclaration
/// export default b;         // ExportDefaultDeclaration
/// export * as c from './c'; // ExportAllDeclaration
/// export = b;               // TSExportAssignment
/// export as namespace d;    // TSNamespaceExportDeclaration
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ModuleDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// `import hello from './world.js';`
    /// `import * as t from './world.js';`
    ImportDeclaration(A::Box<'a, ImportDeclaration<'a, A>>) = 64,
    /// `export * as numbers from '../numbers.js'`
    ExportAllDeclaration(A::Box<'a, ExportAllDeclaration<'a, A>>) = 65,
    /// `export default 5;`
    ExportDefaultDeclaration(A::Box<'a, ExportDefaultDeclaration<'a, A>>) = 66,
    /// `export {five} from './numbers.js';`
    /// `export {six, seven};`
    ExportNamedDeclaration(A::Box<'a, ExportNamedDeclaration<'a, A>>) = 67,

    /// `export = 5;`
    TSExportAssignment(A::Box<'a, TSExportAssignment<'a, A>>) = 68,
    /// `export as namespace React;`
    TSNamespaceExportDeclaration(A::Box<'a, TSNamespaceExportDeclaration<'a>>) = 69,
}

/// Macro for matching `ModuleDeclaration`'s variants.
#[macro_export]
macro_rules! match_module_declaration {
    ($ty:ident) => {
        $ty::ImportDeclaration(_)
            | $ty::ExportAllDeclaration(_)
            | $ty::ExportDefaultDeclaration(_)
            | $ty::ExportNamedDeclaration(_)
            | $ty::TSExportAssignment(_)
            | $ty::TSNamespaceExportDeclaration(_)
    };
}
pub use match_module_declaration;

#[ast]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[generate_derive(CloneIn, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
pub enum AccessorPropertyType {
    AccessorProperty = 0,
    TSAbstractAccessorProperty = 1,
}

/// Class Accessor Property
///
/// ## Example
/// ```ts
/// class Foo {
///   accessor y: string
/// }
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(rename_all = "camelCase")]
pub struct AccessorProperty<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// Decorators applied to the accessor property.
    ///
    /// See [`Decorator`] for more information.
    pub decorators: A::Vec<'a, Decorator<'a, A>>,

    pub modifiers: ClassElementModifiers,

    /// The expression used to declare the property.
    pub key: PropertyKey<'a, A>,
    /// Initialized value in the declaration, if present.
    pub value: Option<Expression<'a, A>>,
    /// Property was declared with a computed key
    pub computed: bool,
    /// Property has a `!` after its key.
    pub definite: Option<TSDefiniteMark>,
    /// Type annotation on the property.
    ///
    /// Will only ever be [`Some`] for TypeScript files.
    pub type_annotation: Option<A::Box<'a, TSTypeAnnotation<'a, A>>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ImportExpression<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub source: Expression<'a, A>,
    pub arguments: A::Vec<'a, Expression<'a, A>>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ImportDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// `None` for `import 'foo'`, `Some([])` for `import {} from 'foo'`
    pub specifiers: Option<A::Vec<'a, ImportDeclarationSpecifier<'a, A>>>,
    pub source: StringLiteral<'a>,
    /// Some(vec![]) for empty assertion
    pub with_clause: Option<A::Box<'a, WithClause<'a, A>>>,
    /// `import type { foo } from 'bar'`
    pub import_kind: ImportOrExportKind,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ImportDeclarationSpecifier<'a, A: AstAllocator = oxc_allocator::Allocator> {
    /// import {imported} from "source"
    /// import {imported as local} from "source"
    ImportSpecifier(A::Box<'a, ImportSpecifier<'a>>) = 0,
    /// import local from "source"
    ImportDefaultSpecifier(A::Box<'a, ImportDefaultSpecifier<'a>>) = 1,
    /// import * as local from "source"
    ImportNamespaceSpecifier(A::Box<'a, ImportNamespaceSpecifier<'a>>) = 2,
}

// import {imported} from "source"
// import {imported as local} from "source"
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ImportSpecifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub imported: ModuleExportName<'a>,
    /// The name of the imported symbol.
    ///
    /// ## Example
    /// ```ts
    /// // local and imported name are the same
    /// import { Foo } from 'foo';
    /// //       ^^^
    /// // imports can be renamed, changing the local name
    /// import { Foo as Bar } from 'foo';
    /// //              ^^^
    /// ```
    pub local: BindingIdentifier<'a>,
    pub import_kind: ImportOrExportKind,
}

/// Default Import Specifier
///
/// ## Example
/// ```ts
/// import local from "source";
/// ```
///
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ImportDefaultSpecifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    /// The name of the imported symbol.
    pub local: BindingIdentifier<'a>,
}

/// Namespace import specifier
///
/// ## Example
/// ```ts
/// import * as local from "source";
/// ```
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ImportNamespaceSpecifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub local: BindingIdentifier<'a>,
}

#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct WithClause<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub attributes_keyword: IdentifierName<'a>, // `with` or `assert`
    pub with_entries: A::Vec<'a, ImportAttribute<'a>>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ImportAttribute<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub key: ImportAttributeKey<'a>,
    pub value: StringLiteral<'a>,
}

#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ImportAttributeKey<'a> {
    Identifier(IdentifierName<'a>) = 0,
    StringLiteral(StringLiteral<'a>) = 1,
}

/// Named Export Declaration
///
/// ## Example
///
/// ```ts
/// //       ________ specifiers
/// export { Foo, Bar };
/// export type { Baz } from 'baz';
/// //     ^^^^              ^^^^^
/// // export_kind           source
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ExportNamedDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub declaration: Option<Declaration<'a, A>>,
    pub specifiers: A::Vec<'a, ExportSpecifier<'a>>,
    pub source: Option<StringLiteral<'a>>,
    /// `export type { foo }`
    pub export_kind: ImportOrExportKind,
    /// Some(vec![]) for empty assertion
    pub with_clause: Option<A::Box<'a, WithClause<'a, A>>>,
}

/// Export Default Declaration
///
/// ## Example
///
/// ```ts
/// export default HoistableDeclaration
/// export default ClassDeclaration
/// export default AssignmentExpression
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type")]
pub struct ExportDefaultDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    pub declaration: ExportDefaultDeclarationKind<'a, A>,
    pub exported: ModuleExportName<'a>, // the `default` Keyword
}

/// Export All Declaration
///
/// ## Example
///
/// ```ts
/// //          _______ exported
/// export * as numbers from '../numbers.js';
/// //                       ^^^^^^^^^^^^^^^ source
/// ```
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ExportAllDeclaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[serde(flatten)]
    pub span: Span,
    /// If this declaration is re-named
    pub exported: Option<ModuleExportName<'a>>,
    pub source: StringLiteral<'a>,
    /// Will be `Some(vec![])` for empty assertion
    pub with_clause: Option<A::Box<'a, WithClause<'a, A>>>, // Some(vec![]) for empty assertion
    pub export_kind: ImportOrExportKind, // `export type *`
}

/// Export Specifier
///
/// Each [`ExportSpecifier`] is one of the named exports in an [`ExportNamedDeclaration`].
///
/// ## Example
///
/// ```ts
/// //       ____ export_kind
/// import { type Foo as Bar } from './foo';
/// //   exported ^^^    ^^^ local
/// ```
#[ast(visit)]
#[derive(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(tag = "type", rename_all = "camelCase")]
pub struct ExportSpecifier<'a> {
    #[serde(flatten)]
    pub span: Span,
    pub local: ModuleExportName<'a>,
    pub exported: ModuleExportName<'a>,
    pub export_kind: ImportOrExportKind, // `export type *`
}

inherit_variants! {
/// Export Default Declaration Kind
///
/// Inherits variants from [`Expression`]. See [`ast` module docs] for explanation of inheritance.
///
/// [`ast` module docs]: `super`
#[ast(visit)]
#[derive_where(Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify),  serde(bound = ""))]
#[serde(untagged)]
pub enum ExportDefaultDeclarationKind<'a, A: AstAllocator = oxc_allocator::Allocator> {
    #[visit(args(flags = ScopeFlags::Function))]
    FunctionDeclaration(A::Box<'a, Function<'a, A>>) = 64,
    ClassDeclaration(A::Box<'a, Class<'a, A>>) = 65,

    TSInterfaceDeclaration(A::Box<'a, TSInterfaceDeclaration<'a, A>>) = 66,

    // `Expression` variants added here by `inherit_variants!` macro
    @inherit Expression
}
}

/// Module Export Name
///
/// Supports:
///   * `import {"\0 any unicode" as foo} from ""`
///   * `export {foo as "\0 any unicode"}`
/// * es2022: <https://github.com/estree/estree/blob/master/es2022.md#modules>
/// * <https://github.com/tc39/ecma262/pull/2154>
#[ast(visit)]
#[derive(Clone, Debug)]
#[generate_derive(CloneIn, GetSpan, GetSpanMut, ContentEq, ContentHash)]
#[cfg_attr(feature = "serialize", derive(Serialize, Tsify), serde(bound = ""))]
#[serde(untagged)]
pub enum ModuleExportName<'a> {
    IdentifierName(IdentifierName<'a>) = 0,
    /// For `local` in `ExportSpecifier`: `foo` in `export { foo }`
    IdentifierReference(IdentifierReference<'a>) = 1,
    StringLiteral(StringLiteral<'a>) = 2,
}
