#![allow(
    clippy::fn_params_excessive_bools,
    clippy::must_use_candidate, // must_use_candidate is too annoying for this file
    clippy::too_many_arguments,
    clippy::unused_self,
)]

use oxc_allocator::{Allocator, FromIn, String};
use oxc_span::ast_alloc::{AstAllocator, Vec, Box};
use oxc_span::{ast_alloc::traits::{Vec as _}, Atom, GetSpan, GetSpanMut, Span};
use oxc_syntax::{number::NumberBase, operator::UnaryOperator};
use std::fmt::Debug;
use std::mem;

#[allow(clippy::wildcard_imports)]
use crate::ast::*;
use crate::handle::Handler;
use crate::{AstBuilder, AstBuilderWithHandler, Visit};

/// Type that can be used in any AST builder method call which requires an `IntoIn<'a, Anything<'a>>`.
/// Pass `NONE` instead of `None::<Anything<'a>>`.
#[allow(clippy::upper_case_acronyms)]
pub struct NONE;

impl<'a, T, A> FromIn<'a, NONE, A> for Option<T> {
    fn from_in(_: NONE, _: &'a A) -> Self {
        None
    }
}

impl<'a> AstBuilder<'a> {
    #[inline]
    pub fn new(allocator: &'a oxc_allocator::Allocator) -> Self {
        Self { allocator }
    }

    #[inline]
    pub fn alloc<T: Debug + GetSpan + GetSpanMut>(self, value: T) -> Box<'a, T> {
        AstAllocator::alloc(self.allocator, value)
    }

    #[inline]
    pub fn vec<T: Debug>(self) -> Vec<'a, T> {
        AstAllocator::vec(self.allocator)
    }

    #[inline]
    pub fn vec_with_capacity<T: Debug>(self, capacity: usize) -> Vec<'a, T> {
        AstAllocator::vec_with_capacity(self.allocator, capacity)
    }

    #[inline]
    pub fn vec_from_iter<T: Debug, I: IntoIterator<Item = T>>(self, iter: I) -> Vec<'a, T> {
        AstAllocator::vec_from_iter(self.allocator, iter)
    }

    #[inline]
    pub fn vec1<T: Debug>(self, value: T) -> Vec<'a, T> {
        let mut vec = self.vec_with_capacity(1);
        vec.push(value);
        vec
    }
}

impl<'a> AstBuilder<'a> {
    #[inline]
    pub fn atom(self, value: &str) -> Atom<'a> {
        Atom::from(String::from_str_in(value, self.allocator).into_bump_str())
    }

    /// # SAFETY
    /// This method is completely unsound and should not be used.
    /// We need to remove all uses of it. Please don't add any more!
    /// <https://github.com/oxc-project/oxc/issues/3483>
    #[allow(clippy::missing_safety_doc)]
    #[inline]
    pub unsafe fn copy<T>(self, src: &T) -> T {
        // SAFETY: Not safe (see above)
        unsafe { std::mem::transmute_copy(src) }
    }

    /// Moves the expression out by replacing it with a null expression.
    #[inline]
    pub fn move_expression(self, expr: &mut Expression<'a>) -> Expression<'a> {
        let null_expr = self.expression_null_literal(expr.span());
        mem::replace(expr, null_expr)
    }

    #[inline]
    pub fn move_statement(self, stmt: &mut Statement<'a>) -> Statement<'a> {
        let empty_stmt = self.empty_statement(stmt.span());
        mem::replace(stmt, Statement::EmptyStatement(self.alloc(empty_stmt)))
    }

    #[inline]
    pub fn move_assignment_target(self, target: &mut AssignmentTarget<'a>) -> AssignmentTarget<'a> {
        let dummy =
            self.simple_assignment_target_identifier_reference(Span::default(), Atom::from(""));
        mem::replace(target, dummy.into())
    }

    #[inline]
    pub fn move_declaration(self, decl: &mut Declaration<'a>) -> Declaration<'a> {
        let empty_decl = self.variable_declaration(
            Span::default(),
            VariableDeclarationKind::Var,
            self.vec(),
            false,
        );
        let empty_decl = Declaration::VariableDeclaration(self.alloc(empty_decl));
        mem::replace(decl, empty_decl)
    }

    #[inline]
    pub fn move_variable_declaration(
        self,
        decl: &mut VariableDeclaration<'a>,
    ) -> VariableDeclaration<'a> {
        let empty_decl = self.variable_declaration(
            Span::default(),
            VariableDeclarationKind::Var,
            self.vec(),
            false,
        );
        mem::replace(decl, empty_decl)
    }

    #[inline]
    pub fn move_vec<T: Debug>(self, vec: &mut Vec<'a, T>) -> Vec<'a, T> {
        mem::replace(vec, self.vec())
    }

    /* ---------- Constructors ---------- */

    /// `0`
    #[inline]
    pub fn number_0(self) -> Expression<'a> {
        self.expression_numeric_literal(Span::default(), 0.0, "0", NumberBase::Decimal)
    }

    /// `void 0`
    #[inline]
    pub fn void_0(self, span: Span) -> Expression<'a> {
        let num = self.number_0();
        Expression::UnaryExpression(self.alloc_unary_expression(span, UnaryOperator::Void, num))
    }

    /* ---------- Functions ---------- */

    #[inline]
    pub fn plain_formal_parameter(
        self,
        span: Span,
        pattern: BindingPattern<'a>,
    ) -> FormalParameter<'a> {
        self.formal_parameter(span, self.vec(), None, pattern)
    }

    #[inline]
    pub fn plain_function(
        self,
        r#type: FunctionType,
        span: Span,
        id: Option<BindingIdentifier<'a>>,
        params: FormalParameters<'a>,
        body: Option<FunctionBody<'a>>,
    ) -> Box<'a, Function<'a>> {
        self.alloc_function(
            r#type,
            span,
            id,
            false,
            false,
            false,
            NONE,
            NONE,
            params,
            NONE,
            body.map(|body| self.alloc(body)),
        )
    }

    /* ---------- Modules ---------- */

    #[inline]
    pub fn plain_export_named_declaration_declaration(
        self,
        span: Span,
        declaration: Declaration<'a>,
    ) -> Box<'a, ExportNamedDeclaration<'a>> {
        self.alloc_export_named_declaration(
            span,
            Some(declaration),
            self.vec(),
            None,
            ImportOrExportKind::Value,
            NONE,
        )
    }

    #[inline]
    pub fn plain_export_named_declaration(
        self,
        span: Span,
        specifiers: Vec<'a, ExportSpecifier<'a>>,
        source: Option<StringLiteral<'a>>,
    ) -> Box<'a, ExportNamedDeclaration<'a>> {
        self.alloc_export_named_declaration(
            span,
            None,
            specifiers,
            source,
            ImportOrExportKind::Value,
            NONE,
        )
    }
}

impl<'a, A: AstAllocator, H: Handler<'a, A>> AstBuilderWithHandler<'a, H, A> {
    #[inline]
    pub fn new(allocator: &'a A, handler: H) -> Self {
        Self { allocator, handler }
    }
    #[inline]
    pub fn alloc<T: Debug + GetSpan + GetSpanMut>(&self, value: T) -> Box<'a, T, A> {
        self.allocator.alloc(value)
    }

    #[inline]
    pub fn vec<T: Debug>(&self) -> Vec<'a, T, A> {
        self.allocator.vec()
    }

    #[inline]
    pub fn vec_with_capacity<T: Debug>(&self, capacity: usize) -> Vec<'a, T, A> {
        self.allocator.vec_with_capacity(capacity)
    }

    #[inline]
    pub fn vec1<T: Debug>(&self, value: T) -> Vec<'a, T, A> {
        let mut vec = self.vec_with_capacity(1);
        vec.push(value);
        vec
    }

    #[inline]
    pub fn str(&self, src: &str) -> &'a str {
        self.allocator.alloc_str(src)
    }

    #[inline]
    pub fn vec_from_iter<T: Debug, I: IntoIterator<Item = T>>(&self, iter: I) -> Vec<'a, T, A> {
        self.allocator.vec_from_iter(iter)
    }

    // #[inline]
    // pub fn str(&self, value: &str) -> &'a str {
    //     String::from_str_in(value, self.allocator).into_bump_str()
    // }

    #[inline]
    pub fn map_alloc<T: Debug + GetSpan + GetSpanMut>(
        &self,
        value: Option<T>,
    ) -> Option<Box<'a, T, A>> {
        Some(self.alloc(value?))
    }
}

impl<'a, A: AstAllocator, H: Handler<'a, A>> AstBuilderWithHandler<'a, H, A> {
    #[inline]
    pub fn plain_formal_parameter(
        &mut self,
        span: Span,
        pattern: BindingPattern<'a, A>,
    ) -> FormalParameter<'a, A> {
        let param = self.formal_parameter(span, self.vec(), None, pattern);
        self.handler.handle_formal_parameter(&param);
        param
    }
    #[inline]
    pub fn jsx_opening_fragment(&mut self, span: Span) -> JSXOpeningFragment {
        // Not visitable
        JSXOpeningFragment { span }
    }
    #[inline]
    pub fn jsx_closing_fragment(&mut self, span: Span) -> JSXClosingFragment {
        // Not visitable
        JSXClosingFragment { span }
    }

    /* ---------- TypeScript ---------- */

    #[inline]
    pub fn ts_interface_heritages(
        &mut self,
        extends: A::Vec<
            'a,
            (Expression<'a, A>, Option<Box<'a, TSTypeParameterInstantiation<'a, A>, A>>, Span),
        >,
    ) -> Vec<'a, TSInterfaceHeritage<'a, A>, A> {
        let Ok(extends) = extends.specialize() else {
            return self.vec();
        };
        let mut vec = self.vec_with_capacity(extends.len());
        for (expression, type_parameters, span) in extends {
            vec.push(self.ts_interface_heritage(span, expression, type_parameters));
        }
        vec
    }
}
