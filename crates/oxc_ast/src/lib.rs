#![allow(clippy::wildcard_imports)]
// TODO: I'm not sure if it is a but or intentional but clippy needs this allowed both on this
// module and the generated one.
#![allow(clippy::self_named_module_files)]
#![recursion_limit = "256"]

//! # Oxc AST
//!
//! Abstract Syntax Tree nodes for Oxc. Supports both TypeScript and JavaScript.
//!
//! This is almost similar to [estree](https://github.com/estree/estree) except a few places:
//! * `Identifier` is replaced with explicit [`BindingIdentifier`], [`IdentifierReference`], [`IdentifierName`] per spec
//! * `AssignmentExpression`.`left` `Pattern` is replaced with [`AssignmentTarget`]
//!
//! ## Parsing
//!
//! You can obtain an AST by parsing source code with a [`Parser`] from [`oxc_parser`].
//!
//! ## Cargo Features
//! * `"serde"` enables support for serde serialization
//!
//! [`BindingIdentifier`]: ast::BindingIdentifier
//! [`IdentifierReference`]: ast::IdentifierReference
//! [`IdentifierName`]: ast::IdentifierName
//! [`AssignmentTarget`]: ast::AssignmentTarget
//! [`oxc_parser`]: <https://docs.rs/oxc_parser>
//! [`Parser`]: <https://docs.rs/oxc_parser/latest/oxc_parser/struct.Parser.html>

#[cfg(feature = "serialize")]
mod serialize;

pub mod ast;
mod ast_builder_impl;
mod ast_impl;
mod ast_kind_impl;
pub mod precedence;
mod trivia;

mod generated {
    pub mod assert_layouts;
    pub mod ast_builder;
    pub mod ast_kind;
    pub mod derive_clone_in;
    pub mod derive_content_eq;
    pub mod derive_content_hash;

    pub mod derive_get_span;

    pub mod handle;

    pub mod derive_get_span_mut;
    pub mod visit;
    pub mod visit_mut;
}

pub use generated::handle;

pub mod visit {
    pub use crate::generated::{visit::*, visit_mut::*};
}

pub use crate::{
    ast::comment::{Comment, CommentKind, CommentPosition},
    ast_builder::{AstBuilder, AstBuilderWithHandler},
    ast_builder_impl::NONE,
    ast_kind::{AstKind, AstType},
    trivia::{comments_range, has_comments_between, CommentsRange},
    visit::{Visit, VisitMut},
};
pub use generated::{ast_builder, ast_kind};
pub use num_bigint::BigUint;
use oxc_span::ast_alloc::AstAllocator;

impl<'a, A: AstAllocator> handle::Handler<'a, A> for () {}

pub trait AstScopeNode {
    const SCOPE_TYPE: ScopeType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeType {
    Program,
    BlockStatement,
    ForStatement,
    ForInStatement,
    ForOfStatement,
    SwitchStatement,
    CatchClause,
    Function,
    ArrowFunctionExpression,
    Class,
    StaticBlock,
    TSEnumDeclaration,
    TSConditionalType,
    TSTypeAliasDeclaration,
    TSInterfaceDeclaration,
    TSMethodSignature,
    TSConstructSignatureDeclaration,
    TSModuleDeclaration,
    TSMappedType,
}

// After experimenting with two types of boxed enum variants:
//   1.
//   ```
//      enum Expression {
//          Variant(Box<Struct>)
//      }
//      struct Struct {
//          expression: Expression
//      }
//   ```
//   2.
//   ```
//      enum Expression {
//          Variant(Struct)
//      }
//      struct Struct {
//          expression: Box<Expression>
//      }
//   ```
//  I have concluded that the first options is more performant and more ergonomic to use.
//  The following test make sure all enum variants are boxed, resulting 16 bytes for each enum.
//  Read `https://nnethercote.github.io/perf-book/type-sizes.html` for more details.
#[cfg(target_pointer_width = "64")]
#[test]
fn size_asserts() {
    use std::mem::size_of;

    use crate::ast;

    assert!(size_of::<ast::Statement>() == 16);
    assert!(size_of::<ast::Expression>() == 16);
    assert!(size_of::<ast::Declaration>() == 16);
    assert!(size_of::<ast::BindingPatternKind>() == 16);
    assert!(size_of::<ast::ModuleDeclaration>() == 16);
    assert!(size_of::<ast::ClassElement>() == 16);
    assert!(size_of::<ast::ExportDefaultDeclarationKind>() == 16);
    assert!(size_of::<ast::AssignmentTargetPattern>() == 16);
    assert!(size_of::<ast::AssignmentTargetMaybeDefault>() == 16);
    assert!(size_of::<ast::AssignmentTargetProperty>() == 16);
    assert!(size_of::<ast::TSLiteral>() == 16);
    assert!(size_of::<ast::TSType>() == 16);
}

#[test]
fn lifetime_variance() {
    type Node<'a, A = oxc_allocator::Allocator> = <A as AstAllocator>::Vec<'a, u8>;

    // Variance infer doesn't work on concrete GAT types.
    // The struct below doesn't pass the variance test but the type alias above does.
    // Could be a rust compiler limitation.
    // struct Node<'a, A: AstAllocator = oxc_allocator::Allocator>(Vec<'a, u8, A>);

    fn _assert_program_variant_lifetime<'a: 'b, 'b>(program: Node<'a>) -> Node<'b> {
        program
    }
}
