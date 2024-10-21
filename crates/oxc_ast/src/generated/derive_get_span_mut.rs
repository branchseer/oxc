// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_span.rs`

#![allow(clippy::match_same_arms)]

use oxc_span::{ast_alloc::AstAllocator, GetSpanMut, Span};

#[allow(clippy::wildcard_imports)]
use crate::ast::comment::*;

#[allow(clippy::wildcard_imports)]
use crate::ast::js::*;

#[allow(clippy::wildcard_imports)]
use crate::ast::jsx::*;

#[allow(clippy::wildcard_imports)]
use crate::ast::literal::*;

#[allow(clippy::wildcard_imports)]
use crate::ast::ts::*;

impl GetSpanMut for BooleanLiteral {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for NullLiteral {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for NumericLiteral<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for BigIntLiteral<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for RegExpLiteral<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for StringLiteral<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Program<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Expression<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for IdentifierName<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for IdentifierReference<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for BindingIdentifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for LabelIdentifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for ThisExpression {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ArrayExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ArrayExpressionElement<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::SpreadElement(it) => GetSpanMut::span_mut(it),
            Self::Elision(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl GetSpanMut for Elision {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ObjectExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ObjectPropertyKind<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ObjectProperty(it) => GetSpanMut::span_mut(it),
            Self::SpreadProperty(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ObjectProperty<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for PropertyKey<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::StaticIdentifier(it) => GetSpanMut::span_mut(it),
            Self::PrivateIdentifier(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TemplateLiteral<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TaggedTemplateExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for TemplateElement<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for MemberExpression<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ComputedMemberExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for StaticMemberExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for PrivateFieldExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CallExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for NewExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for MetaProperty<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for SpreadElement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Argument<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::SpreadElement(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for UpdateExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for UnaryExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BinaryExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for PrivateInExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for LogicalExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ConditionalExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTarget<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrayAssignmentTarget(it) => GetSpanMut::span_mut(it),
            Self::ObjectAssignmentTarget(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for SimpleAssignmentTarget<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetPattern<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ArrayAssignmentTarget(it) => GetSpanMut::span_mut(it),
            Self::ObjectAssignmentTarget(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ArrayAssignmentTarget<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ObjectAssignmentTarget<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetRest<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetMaybeDefault<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::AssignmentTargetWithDefault(it) => GetSpanMut::span_mut(it),
            Self::AssignmentTargetIdentifier(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrayAssignmentTarget(it) => GetSpanMut::span_mut(it),
            Self::ObjectAssignmentTarget(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetWithDefault<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetProperty<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::AssignmentTargetPropertyIdentifier(it) => GetSpanMut::span_mut(it),
            Self::AssignmentTargetPropertyProperty(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetPropertyIdentifier<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentTargetPropertyProperty<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for SequenceExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for Super {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AwaitExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ChainExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ChainElement<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ParenthesizedExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Statement<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::BlockStatement(it) => GetSpanMut::span_mut(it),
            Self::BreakStatement(it) => GetSpanMut::span_mut(it),
            Self::ContinueStatement(it) => GetSpanMut::span_mut(it),
            Self::DebuggerStatement(it) => GetSpanMut::span_mut(it),
            Self::DoWhileStatement(it) => GetSpanMut::span_mut(it),
            Self::EmptyStatement(it) => GetSpanMut::span_mut(it),
            Self::ExpressionStatement(it) => GetSpanMut::span_mut(it),
            Self::ForInStatement(it) => GetSpanMut::span_mut(it),
            Self::ForOfStatement(it) => GetSpanMut::span_mut(it),
            Self::ForStatement(it) => GetSpanMut::span_mut(it),
            Self::IfStatement(it) => GetSpanMut::span_mut(it),
            Self::LabeledStatement(it) => GetSpanMut::span_mut(it),
            Self::ReturnStatement(it) => GetSpanMut::span_mut(it),
            Self::SwitchStatement(it) => GetSpanMut::span_mut(it),
            Self::ThrowStatement(it) => GetSpanMut::span_mut(it),
            Self::TryStatement(it) => GetSpanMut::span_mut(it),
            Self::WhileStatement(it) => GetSpanMut::span_mut(it),
            Self::WithStatement(it) => GetSpanMut::span_mut(it),
            Self::VariableDeclaration(it) => GetSpanMut::span_mut(it),
            Self::FunctionDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ClassDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAliasDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSInterfaceDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSEnumDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSModuleDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSImportEqualsDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ImportDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportAllDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportDefaultDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportNamedDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSExportAssignment(it) => GetSpanMut::span_mut(it),
            Self::TSNamespaceExportDeclaration(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for Directive<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for Hashbang<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BlockStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Declaration<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::VariableDeclaration(it) => GetSpanMut::span_mut(it),
            Self::FunctionDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ClassDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAliasDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSInterfaceDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSEnumDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSModuleDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSImportEqualsDeclaration(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for VariableDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for VariableDeclarator<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for EmptyStatement {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ExpressionStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for IfStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for DoWhileStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for WhileStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ForStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ForStatementInit<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::VariableDeclaration(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ForInStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ForStatementLeft<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::VariableDeclaration(it) => GetSpanMut::span_mut(it),
            Self::AssignmentTargetIdentifier(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrayAssignmentTarget(it) => GetSpanMut::span_mut(it),
            Self::ObjectAssignmentTarget(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ForOfStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ContinueStatement<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for BreakStatement<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ReturnStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for WithStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for SwitchStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for SwitchCase<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for LabeledStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ThrowStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TryStatement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CatchClause<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for CatchParameter<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for DebuggerStatement {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BindingPattern<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        GetSpanMut::span_mut(&mut self.kind)
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BindingPatternKind<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::BindingIdentifier(it) => GetSpanMut::span_mut(it),
            Self::ObjectPattern(it) => GetSpanMut::span_mut(it),
            Self::ArrayPattern(it) => GetSpanMut::span_mut(it),
            Self::AssignmentPattern(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AssignmentPattern<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ObjectPattern<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BindingProperty<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ArrayPattern<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for BindingRestElement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Function<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for FormalParameters<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for FormalParameter<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for FunctionBody<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ArrowFunctionExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for YieldExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ClassHead<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Class<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ClassBody<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ClassElement<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::StaticBlock(it) => GetSpanMut::span_mut(it),
            Self::MethodDefinition(it) => GetSpanMut::span_mut(it),
            Self::PropertyDefinition(it) => GetSpanMut::span_mut(it),
            Self::AccessorProperty(it) => GetSpanMut::span_mut(it),
            Self::TSIndexSignature(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for MethodDefinition<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for PropertyDefinition<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for PrivateIdentifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for StaticBlock<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ModuleDeclaration<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ImportDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportAllDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportDefaultDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ExportNamedDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSExportAssignment(it) => GetSpanMut::span_mut(it),
            Self::TSNamespaceExportDeclaration(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for AccessorProperty<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ImportExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ImportDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ImportDeclarationSpecifier<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ImportSpecifier(it) => GetSpanMut::span_mut(it),
            Self::ImportDefaultSpecifier(it) => GetSpanMut::span_mut(it),
            Self::ImportNamespaceSpecifier(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for ImportSpecifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ImportDefaultSpecifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ImportNamespaceSpecifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for WithClause<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ImportAttribute<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ImportAttributeKey<'a> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ExportNamedDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ExportDefaultDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ExportAllDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for ExportSpecifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for ExportDefaultDeclarationKind<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::FunctionDeclaration(it) => GetSpanMut::span_mut(it),
            Self::ClassDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSInterfaceDeclaration(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for ModuleExportName<'a> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::IdentifierName(it) => GetSpanMut::span_mut(it),
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSThisParameter<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSEnumDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSEnumMember<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSEnumMemberName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::StaticIdentifier(it) => GetSpanMut::span_mut(it),
            Self::StaticStringLiteral(it) => GetSpanMut::span_mut(it),
            Self::StaticTemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::StaticNumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeAnnotation<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSLiteralType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSLiteral<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSType<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::TSAnyKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSBigIntKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSBooleanKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSIntrinsicKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNeverKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNullKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNumberKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSObjectKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSStringKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSSymbolKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSUndefinedKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSUnknownKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSVoidKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSArrayType(it) => GetSpanMut::span_mut(it),
            Self::TSConditionalType(it) => GetSpanMut::span_mut(it),
            Self::TSConstructorType(it) => GetSpanMut::span_mut(it),
            Self::TSFunctionType(it) => GetSpanMut::span_mut(it),
            Self::TSImportType(it) => GetSpanMut::span_mut(it),
            Self::TSIndexedAccessType(it) => GetSpanMut::span_mut(it),
            Self::TSInferType(it) => GetSpanMut::span_mut(it),
            Self::TSIntersectionType(it) => GetSpanMut::span_mut(it),
            Self::TSLiteralType(it) => GetSpanMut::span_mut(it),
            Self::TSMappedType(it) => GetSpanMut::span_mut(it),
            Self::TSNamedTupleMember(it) => GetSpanMut::span_mut(it),
            Self::TSQualifiedName(it) => GetSpanMut::span_mut(it),
            Self::TSTemplateLiteralType(it) => GetSpanMut::span_mut(it),
            Self::TSThisType(it) => GetSpanMut::span_mut(it),
            Self::TSTupleType(it) => GetSpanMut::span_mut(it),
            Self::TSTypeLiteral(it) => GetSpanMut::span_mut(it),
            Self::TSTypeOperatorType(it) => GetSpanMut::span_mut(it),
            Self::TSTypePredicate(it) => GetSpanMut::span_mut(it),
            Self::TSTypeQuery(it) => GetSpanMut::span_mut(it),
            Self::TSTypeReference(it) => GetSpanMut::span_mut(it),
            Self::TSUnionType(it) => GetSpanMut::span_mut(it),
            Self::TSParenthesizedType(it) => GetSpanMut::span_mut(it),
            Self::JSDocNullableType(it) => GetSpanMut::span_mut(it),
            Self::JSDocNonNullableType(it) => GetSpanMut::span_mut(it),
            Self::JSDocUnknownType(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSConditionalType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSUnionType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSIntersectionType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSParenthesizedType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeOperator<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSArrayType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSIndexedAccessType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTupleType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSNamedTupleMember<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSOptionalType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSRestType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTupleElement<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::TSOptionalType(it) => GetSpanMut::span_mut(it),
            Self::TSRestType(it) => GetSpanMut::span_mut(it),
            Self::TSAnyKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSBigIntKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSBooleanKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSIntrinsicKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNeverKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNullKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSNumberKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSObjectKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSStringKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSSymbolKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSUndefinedKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSUnknownKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSVoidKeyword(it) => GetSpanMut::span_mut(it),
            Self::TSArrayType(it) => GetSpanMut::span_mut(it),
            Self::TSConditionalType(it) => GetSpanMut::span_mut(it),
            Self::TSConstructorType(it) => GetSpanMut::span_mut(it),
            Self::TSFunctionType(it) => GetSpanMut::span_mut(it),
            Self::TSImportType(it) => GetSpanMut::span_mut(it),
            Self::TSIndexedAccessType(it) => GetSpanMut::span_mut(it),
            Self::TSInferType(it) => GetSpanMut::span_mut(it),
            Self::TSIntersectionType(it) => GetSpanMut::span_mut(it),
            Self::TSLiteralType(it) => GetSpanMut::span_mut(it),
            Self::TSMappedType(it) => GetSpanMut::span_mut(it),
            Self::TSNamedTupleMember(it) => GetSpanMut::span_mut(it),
            Self::TSQualifiedName(it) => GetSpanMut::span_mut(it),
            Self::TSTemplateLiteralType(it) => GetSpanMut::span_mut(it),
            Self::TSThisType(it) => GetSpanMut::span_mut(it),
            Self::TSTupleType(it) => GetSpanMut::span_mut(it),
            Self::TSTypeLiteral(it) => GetSpanMut::span_mut(it),
            Self::TSTypeOperatorType(it) => GetSpanMut::span_mut(it),
            Self::TSTypePredicate(it) => GetSpanMut::span_mut(it),
            Self::TSTypeQuery(it) => GetSpanMut::span_mut(it),
            Self::TSTypeReference(it) => GetSpanMut::span_mut(it),
            Self::TSUnionType(it) => GetSpanMut::span_mut(it),
            Self::TSParenthesizedType(it) => GetSpanMut::span_mut(it),
            Self::JSDocNullableType(it) => GetSpanMut::span_mut(it),
            Self::JSDocNonNullableType(it) => GetSpanMut::span_mut(it),
            Self::JSDocUnknownType(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl GetSpanMut for TSAnyKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSStringKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSBooleanKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSNumberKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSNeverKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSIntrinsicKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSUnknownKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSNullKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSUndefinedKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSVoidKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSSymbolKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSThisType {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSObjectKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for TSBigIntKeyword {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeReference<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::QualifiedName(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSQualifiedName<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeParameterInstantiation<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeParameter<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeParameterDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeAliasDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSClassImplements<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSInterfaceDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSInterfaceBody<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSPropertySignature<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSSignature<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::TSIndexSignature(it) => GetSpanMut::span_mut(it),
            Self::TSPropertySignature(it) => GetSpanMut::span_mut(it),
            Self::TSCallSignatureDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSConstructSignatureDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSMethodSignature(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSIndexSignature<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSCallSignatureDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSMethodSignature<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSConstructSignatureDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSIndexSignatureName<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSInterfaceHeritage<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypePredicate<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypePredicateName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::This(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSModuleDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for TSModuleDeclarationName<'a> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSModuleDeclarationBody<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::TSModuleDeclaration(it) => GetSpanMut::span_mut(it),
            Self::TSModuleBlock(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSModuleBlock<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeLiteral<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSInferType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeQuery<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeQueryExprName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::TSImportType(it) => GetSpanMut::span_mut(it),
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::QualifiedName(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSImportType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSImportAttributes<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSImportAttribute<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for TSImportAttributeName<'a> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSFunctionType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSConstructorType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSMappedType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTemplateLiteralType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSAsExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSSatisfiesExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeAssertionAnnotation<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSTypeAssertion<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSImportEqualsDeclaration<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSModuleReference<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::ExternalModuleReference(it) => GetSpanMut::span_mut(it),
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::QualifiedName(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for TSExternalModuleReference<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSNonNullExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for Decorator<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSExportAssignment<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for TSNamespaceExportDeclaration<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for TSInstantiationExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSDocNullableType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSDocNonNullableType<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for JSDocUnknownType {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXElement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXOpeningElement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXClosingElement<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXFragment<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for JSXOpeningFragment {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for JSXClosingFragment {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXElementName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::NamespacedName(it) => GetSpanMut::span_mut(it),
            Self::MemberExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for JSXNamespacedName<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXMemberExpression<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXMemberExpressionObject<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::IdentifierReference(it) => GetSpanMut::span_mut(it),
            Self::MemberExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXExpressionContainer<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXExpression<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::EmptyExpression(it) => GetSpanMut::span_mut(it),
            Self::BooleanLiteral(it) => GetSpanMut::span_mut(it),
            Self::NullLiteral(it) => GetSpanMut::span_mut(it),
            Self::NumericLiteral(it) => GetSpanMut::span_mut(it),
            Self::BigIntLiteral(it) => GetSpanMut::span_mut(it),
            Self::RegExpLiteral(it) => GetSpanMut::span_mut(it),
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::TemplateLiteral(it) => GetSpanMut::span_mut(it),
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::MetaProperty(it) => GetSpanMut::span_mut(it),
            Self::Super(it) => GetSpanMut::span_mut(it),
            Self::ArrayExpression(it) => GetSpanMut::span_mut(it),
            Self::ArrowFunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::AssignmentExpression(it) => GetSpanMut::span_mut(it),
            Self::AwaitExpression(it) => GetSpanMut::span_mut(it),
            Self::BinaryExpression(it) => GetSpanMut::span_mut(it),
            Self::CallExpression(it) => GetSpanMut::span_mut(it),
            Self::ChainExpression(it) => GetSpanMut::span_mut(it),
            Self::ClassExpression(it) => GetSpanMut::span_mut(it),
            Self::ConditionalExpression(it) => GetSpanMut::span_mut(it),
            Self::FunctionExpression(it) => GetSpanMut::span_mut(it),
            Self::ImportExpression(it) => GetSpanMut::span_mut(it),
            Self::LogicalExpression(it) => GetSpanMut::span_mut(it),
            Self::NewExpression(it) => GetSpanMut::span_mut(it),
            Self::ObjectExpression(it) => GetSpanMut::span_mut(it),
            Self::ParenthesizedExpression(it) => GetSpanMut::span_mut(it),
            Self::SequenceExpression(it) => GetSpanMut::span_mut(it),
            Self::TaggedTemplateExpression(it) => GetSpanMut::span_mut(it),
            Self::ThisExpression(it) => GetSpanMut::span_mut(it),
            Self::UnaryExpression(it) => GetSpanMut::span_mut(it),
            Self::UpdateExpression(it) => GetSpanMut::span_mut(it),
            Self::YieldExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateInExpression(it) => GetSpanMut::span_mut(it),
            Self::JSXElement(it) => GetSpanMut::span_mut(it),
            Self::JSXFragment(it) => GetSpanMut::span_mut(it),
            Self::TSAsExpression(it) => GetSpanMut::span_mut(it),
            Self::TSSatisfiesExpression(it) => GetSpanMut::span_mut(it),
            Self::TSTypeAssertion(it) => GetSpanMut::span_mut(it),
            Self::TSNonNullExpression(it) => GetSpanMut::span_mut(it),
            Self::TSInstantiationExpression(it) => GetSpanMut::span_mut(it),
            Self::ComputedMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::StaticMemberExpression(it) => GetSpanMut::span_mut(it),
            Self::PrivateFieldExpression(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl GetSpanMut for JSXEmptyExpression {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXAttributeItem<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Attribute(it) => GetSpanMut::span_mut(it),
            Self::SpreadAttribute(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXAttribute<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXSpreadAttribute<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXAttributeName<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Identifier(it) => GetSpanMut::span_mut(it),
            Self::NamespacedName(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXAttributeValue<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::StringLiteral(it) => GetSpanMut::span_mut(it),
            Self::ExpressionContainer(it) => GetSpanMut::span_mut(it),
            Self::Element(it) => GetSpanMut::span_mut(it),
            Self::Fragment(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a> GetSpanMut for JSXIdentifier<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXChild<'a, A> {
    fn span_mut(&mut self) -> &mut Span {
        match self {
            Self::Text(it) => GetSpanMut::span_mut(it),
            Self::Element(it) => GetSpanMut::span_mut(it),
            Self::Fragment(it) => GetSpanMut::span_mut(it),
            Self::ExpressionContainer(it) => GetSpanMut::span_mut(it),
            Self::Spread(it) => GetSpanMut::span_mut(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpanMut for JSXSpreadChild<'a, A> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl<'a> GetSpanMut for JSXText<'a> {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}

impl GetSpanMut for Comment {
    #[inline]
    fn span_mut(&mut self) -> &mut Span {
        &mut self.span
    }
}
