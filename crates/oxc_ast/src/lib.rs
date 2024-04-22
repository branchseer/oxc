#![allow(clippy::wildcard_imports)]

//! # Oxc AST
//!
//! This is almost similar to [estree](https://github.com/estree/estree) except a few places:
//! * `Identifier` is replaced with explicit `BindingIdentifier`, `IdentifierReference`, `IdentifierName` per spec
//! * `AssignmentExpression`.`left` `Pattern` is replaced with `AssignmentTarget`
//!
//! ## Cargo Features
//! * `"serde"` enables support for serde serialization

#[cfg(feature = "serialize")]
mod serialize;

#[cfg(feature = "rkyv")]
mod rkyv;

pub mod ast;
mod ast_builder;
mod ast_kind;
pub mod precedence;
mod span;
pub mod syntax_directed_operations;
mod trivia;
pub mod visit;

pub use num_bigint::BigUint;

pub use crate::{
    ast_builder::AstBuilder,
    ast_kind::{AstKind, AstType},
    trivia::{Comment, CommentKind, Trivias, TriviasMap},
    visit::{Visit, VisitMut},
};

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
    use crate::ast;
    use oxc_index::assert_eq_size;

    assert_eq_size!(ast::Statement, [u8; 16]);
    assert_eq_size!(ast::Expression, [u8; 16]);
    assert_eq_size!(ast::Declaration, [u8; 16]);
    assert_eq_size!(ast::BindingPatternKind, [u8; 16]);
    assert_eq_size!(ast::ModuleDeclaration, [u8; 16]);
    assert_eq_size!(ast::ClassElement, [u8; 16]);
    assert_eq_size!(ast::ExportDefaultDeclarationKind, [u8; 16]);
    assert_eq_size!(ast::AssignmentTargetPattern, [u8; 16]);
    assert_eq_size!(ast::AssignmentTargetMaybeDefault, [u8; 24]);
    assert_eq_size!(ast::AssignmentTargetProperty, [u8; 16]);
    assert_eq_size!(ast::TSLiteral, [u8; 16]);
    assert_eq_size!(ast::TSType, [u8; 16]);
}

#[test]
fn lifetime_variance() {
    use crate::ast;

    fn _assert_program_variant_lifetime<'a: 'b, 'b>(program: ast::Program<'a>) -> ast::Program<'b> {
        program
    }
}

#[cfg(feature = "rkyv")]
#[test]
fn rkyv_smoke() {
    use std::cell::Cell;

    use oxc_allocator::{Allocator, Box, Vec};
    use oxc_syntax::reference::{ReferenceFlag, ReferenceId};

    let allocator = Allocator::default();
    let program = ast::Program {
        span: oxc_span::SPAN,
        source_type: oxc_span::SourceType::default(),
        directives: Vec::new_in(&allocator),
        hashbang: None,
        body: Vec::from_iter_in(
            [ast::Statement::ExpressionStatement(Box::new_in(
                ast::ExpressionStatement {
                    span: oxc_span::SPAN,
                    expression: ast::Expression::Identifier(Box::new_in(
                        ast::IdentifierReference {
                            span: oxc_span::SPAN,
                            name: "hello".into(),
                            reference_id: Cell::new(Some(ReferenceId::from_raw(42))),
                            reference_flag: ReferenceFlag::empty(),
                        },
                        &allocator,
                    )),
                },
                &allocator,
            ))],
            &allocator,
        ),
    };
    let bytes = ::rkyv::util::to_bytes::<_, 0>(&program).unwrap();
    dbg!(bytes.len());
}
