// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/get_span.rs`

#![allow(clippy::match_same_arms)]

use oxc_span::{ast_alloc::AstAllocator, GetSpan, Span};

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

impl GetSpan for BooleanLiteral {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for NullLiteral {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for NumericLiteral<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for BigIntLiteral<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for RegExpLiteral<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for StringLiteral<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Program<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Expression<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for IdentifierName<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for IdentifierReference<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for BindingIdentifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for LabelIdentifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ThisExpression {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ArrayExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ArrayExpressionElement<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::SpreadElement(it) => GetSpan::span(it),
            Self::Elision(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for Elision {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ObjectExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ObjectPropertyKind<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ObjectProperty(it) => GetSpan::span(it),
            Self::SpreadProperty(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ObjectProperty<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for PropertyKey<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::StaticIdentifier(it) => GetSpan::span(it),
            Self::PrivateIdentifier(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TemplateLiteral<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TaggedTemplateExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for TemplateElement<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for MemberExpression<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ComputedMemberExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for StaticMemberExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for PrivateFieldExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CallExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for NewExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for MetaProperty<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for SpreadElement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Argument<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::SpreadElement(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for UpdateExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for UnaryExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for BinaryExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for PrivateInExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for LogicalExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ConditionalExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTarget<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
            Self::ArrayAssignmentTarget(it) => GetSpan::span(it),
            Self::ObjectAssignmentTarget(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for SimpleAssignmentTarget<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::AssignmentTargetIdentifier(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetPattern<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ArrayAssignmentTarget(it) => GetSpan::span(it),
            Self::ObjectAssignmentTarget(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ArrayAssignmentTarget<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ObjectAssignmentTarget<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetRest<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetMaybeDefault<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::AssignmentTargetWithDefault(it) => GetSpan::span(it),
            Self::AssignmentTargetIdentifier(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
            Self::ArrayAssignmentTarget(it) => GetSpan::span(it),
            Self::ObjectAssignmentTarget(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetWithDefault<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetProperty<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::AssignmentTargetPropertyIdentifier(it) => GetSpan::span(it),
            Self::AssignmentTargetPropertyProperty(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetPropertyIdentifier<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentTargetPropertyProperty<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for SequenceExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Super {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for AwaitExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ChainExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ChainElement<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ParenthesizedExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Statement<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::BlockStatement(it) => GetSpan::span(it),
            Self::BreakStatement(it) => GetSpan::span(it),
            Self::ContinueStatement(it) => GetSpan::span(it),
            Self::DebuggerStatement(it) => GetSpan::span(it),
            Self::DoWhileStatement(it) => GetSpan::span(it),
            Self::EmptyStatement(it) => GetSpan::span(it),
            Self::ExpressionStatement(it) => GetSpan::span(it),
            Self::ForInStatement(it) => GetSpan::span(it),
            Self::ForOfStatement(it) => GetSpan::span(it),
            Self::ForStatement(it) => GetSpan::span(it),
            Self::IfStatement(it) => GetSpan::span(it),
            Self::LabeledStatement(it) => GetSpan::span(it),
            Self::ReturnStatement(it) => GetSpan::span(it),
            Self::SwitchStatement(it) => GetSpan::span(it),
            Self::ThrowStatement(it) => GetSpan::span(it),
            Self::TryStatement(it) => GetSpan::span(it),
            Self::WhileStatement(it) => GetSpan::span(it),
            Self::WithStatement(it) => GetSpan::span(it),
            Self::VariableDeclaration(it) => GetSpan::span(it),
            Self::FunctionDeclaration(it) => GetSpan::span(it),
            Self::ClassDeclaration(it) => GetSpan::span(it),
            Self::TSTypeAliasDeclaration(it) => GetSpan::span(it),
            Self::TSInterfaceDeclaration(it) => GetSpan::span(it),
            Self::TSEnumDeclaration(it) => GetSpan::span(it),
            Self::TSModuleDeclaration(it) => GetSpan::span(it),
            Self::TSImportEqualsDeclaration(it) => GetSpan::span(it),
            Self::ImportDeclaration(it) => GetSpan::span(it),
            Self::ExportAllDeclaration(it) => GetSpan::span(it),
            Self::ExportDefaultDeclaration(it) => GetSpan::span(it),
            Self::ExportNamedDeclaration(it) => GetSpan::span(it),
            Self::TSExportAssignment(it) => GetSpan::span(it),
            Self::TSNamespaceExportDeclaration(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for Directive<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for Hashbang<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for BlockStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Declaration<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::VariableDeclaration(it) => GetSpan::span(it),
            Self::FunctionDeclaration(it) => GetSpan::span(it),
            Self::ClassDeclaration(it) => GetSpan::span(it),
            Self::TSTypeAliasDeclaration(it) => GetSpan::span(it),
            Self::TSInterfaceDeclaration(it) => GetSpan::span(it),
            Self::TSEnumDeclaration(it) => GetSpan::span(it),
            Self::TSModuleDeclaration(it) => GetSpan::span(it),
            Self::TSImportEqualsDeclaration(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for VariableDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for VariableDeclarator<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for EmptyStatement {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ExpressionStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for IfStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for DoWhileStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for WhileStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ForStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ForStatementInit<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::VariableDeclaration(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ForInStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ForStatementLeft<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::VariableDeclaration(it) => GetSpan::span(it),
            Self::AssignmentTargetIdentifier(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
            Self::ArrayAssignmentTarget(it) => GetSpan::span(it),
            Self::ObjectAssignmentTarget(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ForOfStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ContinueStatement<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for BreakStatement<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ReturnStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for WithStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for SwitchStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for SwitchCase<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for LabeledStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ThrowStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TryStatement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CatchClause<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for CatchParameter<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for DebuggerStatement {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for BindingPattern<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        GetSpan::span(&self.kind)
    }
}

impl<'a, A: AstAllocator> GetSpan for BindingPatternKind<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::BindingIdentifier(it) => GetSpan::span(it),
            Self::ObjectPattern(it) => GetSpan::span(it),
            Self::ArrayPattern(it) => GetSpan::span(it),
            Self::AssignmentPattern(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for AssignmentPattern<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ObjectPattern<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for BindingProperty<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ArrayPattern<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for BindingRestElement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Function<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for FormalParameters<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for FormalParameter<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for FunctionBody<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ArrowFunctionExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for YieldExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassModifiers {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Class<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ClassBody<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ClassElement<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::StaticBlock(it) => GetSpan::span(it),
            Self::MethodDefinition(it) => GetSpan::span(it),
            Self::PropertyDefinition(it) => GetSpan::span(it),
            Self::AccessorProperty(it) => GetSpan::span(it),
            Self::TSIndexSignature(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for MethodDefinition<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for ClassElementModifiers {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for PropertyDefinition<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for PrivateIdentifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for StaticBlock<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ModuleDeclaration<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ImportDeclaration(it) => GetSpan::span(it),
            Self::ExportAllDeclaration(it) => GetSpan::span(it),
            Self::ExportDefaultDeclaration(it) => GetSpan::span(it),
            Self::ExportNamedDeclaration(it) => GetSpan::span(it),
            Self::TSExportAssignment(it) => GetSpan::span(it),
            Self::TSNamespaceExportDeclaration(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for AccessorProperty<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ImportExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ImportDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ImportDeclarationSpecifier<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ImportSpecifier(it) => GetSpan::span(it),
            Self::ImportDefaultSpecifier(it) => GetSpan::span(it),
            Self::ImportNamespaceSpecifier(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for ImportSpecifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ImportDefaultSpecifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ImportNamespaceSpecifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for WithClause<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ImportAttribute<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ImportAttributeKey<'a> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for ExportNamedDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ExportDefaultDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ExportAllDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for ExportSpecifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for ExportDefaultDeclarationKind<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::FunctionDeclaration(it) => GetSpan::span(it),
            Self::ClassDeclaration(it) => GetSpan::span(it),
            Self::TSInterfaceDeclaration(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for ModuleExportName<'a> {
    fn span(&self) -> Span {
        match self {
            Self::IdentifierName(it) => GetSpan::span(it),
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSThisParameter<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSEnumDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSEnumMember<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSEnumMemberName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::StaticIdentifier(it) => GetSpan::span(it),
            Self::StaticStringLiteral(it) => GetSpan::span(it),
            Self::StaticTemplateLiteral(it) => GetSpan::span(it),
            Self::StaticNumericLiteral(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeAnnotation<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSLiteralType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSLiteral<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSType<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::TSAnyKeyword(it) => GetSpan::span(it),
            Self::TSBigIntKeyword(it) => GetSpan::span(it),
            Self::TSBooleanKeyword(it) => GetSpan::span(it),
            Self::TSIntrinsicKeyword(it) => GetSpan::span(it),
            Self::TSNeverKeyword(it) => GetSpan::span(it),
            Self::TSNullKeyword(it) => GetSpan::span(it),
            Self::TSNumberKeyword(it) => GetSpan::span(it),
            Self::TSObjectKeyword(it) => GetSpan::span(it),
            Self::TSStringKeyword(it) => GetSpan::span(it),
            Self::TSSymbolKeyword(it) => GetSpan::span(it),
            Self::TSUndefinedKeyword(it) => GetSpan::span(it),
            Self::TSUnknownKeyword(it) => GetSpan::span(it),
            Self::TSVoidKeyword(it) => GetSpan::span(it),
            Self::TSArrayType(it) => GetSpan::span(it),
            Self::TSConditionalType(it) => GetSpan::span(it),
            Self::TSConstructorType(it) => GetSpan::span(it),
            Self::TSFunctionType(it) => GetSpan::span(it),
            Self::TSImportType(it) => GetSpan::span(it),
            Self::TSIndexedAccessType(it) => GetSpan::span(it),
            Self::TSInferType(it) => GetSpan::span(it),
            Self::TSIntersectionType(it) => GetSpan::span(it),
            Self::TSLiteralType(it) => GetSpan::span(it),
            Self::TSMappedType(it) => GetSpan::span(it),
            Self::TSNamedTupleMember(it) => GetSpan::span(it),
            Self::TSQualifiedName(it) => GetSpan::span(it),
            Self::TSTemplateLiteralType(it) => GetSpan::span(it),
            Self::TSThisType(it) => GetSpan::span(it),
            Self::TSTupleType(it) => GetSpan::span(it),
            Self::TSTypeLiteral(it) => GetSpan::span(it),
            Self::TSTypeOperatorType(it) => GetSpan::span(it),
            Self::TSTypePredicate(it) => GetSpan::span(it),
            Self::TSTypeQuery(it) => GetSpan::span(it),
            Self::TSTypeReference(it) => GetSpan::span(it),
            Self::TSUnionType(it) => GetSpan::span(it),
            Self::TSParenthesizedType(it) => GetSpan::span(it),
            Self::JSDocNullableType(it) => GetSpan::span(it),
            Self::JSDocNonNullableType(it) => GetSpan::span(it),
            Self::JSDocUnknownType(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSConditionalType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSUnionType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSIntersectionType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSParenthesizedType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeOperator<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSArrayType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSIndexedAccessType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTupleType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSNamedTupleMember<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSOptionalType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSRestType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTupleElement<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::TSOptionalType(it) => GetSpan::span(it),
            Self::TSRestType(it) => GetSpan::span(it),
            Self::TSAnyKeyword(it) => GetSpan::span(it),
            Self::TSBigIntKeyword(it) => GetSpan::span(it),
            Self::TSBooleanKeyword(it) => GetSpan::span(it),
            Self::TSIntrinsicKeyword(it) => GetSpan::span(it),
            Self::TSNeverKeyword(it) => GetSpan::span(it),
            Self::TSNullKeyword(it) => GetSpan::span(it),
            Self::TSNumberKeyword(it) => GetSpan::span(it),
            Self::TSObjectKeyword(it) => GetSpan::span(it),
            Self::TSStringKeyword(it) => GetSpan::span(it),
            Self::TSSymbolKeyword(it) => GetSpan::span(it),
            Self::TSUndefinedKeyword(it) => GetSpan::span(it),
            Self::TSUnknownKeyword(it) => GetSpan::span(it),
            Self::TSVoidKeyword(it) => GetSpan::span(it),
            Self::TSArrayType(it) => GetSpan::span(it),
            Self::TSConditionalType(it) => GetSpan::span(it),
            Self::TSConstructorType(it) => GetSpan::span(it),
            Self::TSFunctionType(it) => GetSpan::span(it),
            Self::TSImportType(it) => GetSpan::span(it),
            Self::TSIndexedAccessType(it) => GetSpan::span(it),
            Self::TSInferType(it) => GetSpan::span(it),
            Self::TSIntersectionType(it) => GetSpan::span(it),
            Self::TSLiteralType(it) => GetSpan::span(it),
            Self::TSMappedType(it) => GetSpan::span(it),
            Self::TSNamedTupleMember(it) => GetSpan::span(it),
            Self::TSQualifiedName(it) => GetSpan::span(it),
            Self::TSTemplateLiteralType(it) => GetSpan::span(it),
            Self::TSThisType(it) => GetSpan::span(it),
            Self::TSTupleType(it) => GetSpan::span(it),
            Self::TSTypeLiteral(it) => GetSpan::span(it),
            Self::TSTypeOperatorType(it) => GetSpan::span(it),
            Self::TSTypePredicate(it) => GetSpan::span(it),
            Self::TSTypeQuery(it) => GetSpan::span(it),
            Self::TSTypeReference(it) => GetSpan::span(it),
            Self::TSUnionType(it) => GetSpan::span(it),
            Self::TSParenthesizedType(it) => GetSpan::span(it),
            Self::JSDocNullableType(it) => GetSpan::span(it),
            Self::JSDocNonNullableType(it) => GetSpan::span(it),
            Self::JSDocUnknownType(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for TSAnyKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSStringKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSBooleanKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSNumberKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSNeverKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSIntrinsicKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSUnknownKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSNullKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSUndefinedKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSVoidKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSSymbolKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSThisType {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSObjectKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSBigIntKeyword {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeReference<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::QualifiedName(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSQualifiedName<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeParameterInstantiation<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeParameter<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeParameterDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeAliasDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSClassImplements<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSInterfaceDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSInterfaceBody<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSPropertySignature<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSSignature<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::TSIndexSignature(it) => GetSpan::span(it),
            Self::TSPropertySignature(it) => GetSpan::span(it),
            Self::TSCallSignatureDeclaration(it) => GetSpan::span(it),
            Self::TSConstructSignatureDeclaration(it) => GetSpan::span(it),
            Self::TSMethodSignature(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSIndexSignature<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSCallSignatureDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSMethodSignature<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSConstructSignatureDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSIndexSignatureName<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSInterfaceHeritage<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypePredicate<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypePredicateName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::This(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSModuleDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for TSModuleDeclarationName<'a> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSModuleDeclarationBody<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::TSModuleDeclaration(it) => GetSpan::span(it),
            Self::TSModuleBlock(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSModuleBlock<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeLiteral<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSInferType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeQuery<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeQueryExprName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::TSImportType(it) => GetSpan::span(it),
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::QualifiedName(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSImportType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSImportAttributes<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSImportAttribute<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for TSImportAttributeName<'a> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for TSFunctionType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSConstructorType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSMappedType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTemplateLiteralType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSAsExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSSatisfiesExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeAssertionAnnotation<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSTypeAssertion<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSImportEqualsDeclaration<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSModuleReference<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::ExternalModuleReference(it) => GetSpan::span(it),
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::QualifiedName(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for TSExternalModuleReference<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSNonNullExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for Decorator<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSExportAssignment<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for TSNamespaceExportDeclaration<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for TSInstantiationExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSOptionalMark {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for TSDefiniteMark {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSDocNullableType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSDocNonNullableType<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for JSDocUnknownType {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXElement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXOpeningElement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXClosingElement<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXFragment<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for JSXOpeningFragment {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for JSXClosingFragment {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXElementName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::NamespacedName(it) => GetSpan::span(it),
            Self::MemberExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for JSXNamespacedName<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXMemberExpression<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXMemberExpressionObject<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::IdentifierReference(it) => GetSpan::span(it),
            Self::MemberExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXExpressionContainer<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXExpression<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::EmptyExpression(it) => GetSpan::span(it),
            Self::BooleanLiteral(it) => GetSpan::span(it),
            Self::NullLiteral(it) => GetSpan::span(it),
            Self::NumericLiteral(it) => GetSpan::span(it),
            Self::BigIntLiteral(it) => GetSpan::span(it),
            Self::RegExpLiteral(it) => GetSpan::span(it),
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::TemplateLiteral(it) => GetSpan::span(it),
            Self::Identifier(it) => GetSpan::span(it),
            Self::MetaProperty(it) => GetSpan::span(it),
            Self::Super(it) => GetSpan::span(it),
            Self::ArrayExpression(it) => GetSpan::span(it),
            Self::ArrowFunctionExpression(it) => GetSpan::span(it),
            Self::AssignmentExpression(it) => GetSpan::span(it),
            Self::AwaitExpression(it) => GetSpan::span(it),
            Self::BinaryExpression(it) => GetSpan::span(it),
            Self::CallExpression(it) => GetSpan::span(it),
            Self::ChainExpression(it) => GetSpan::span(it),
            Self::ClassExpression(it) => GetSpan::span(it),
            Self::ConditionalExpression(it) => GetSpan::span(it),
            Self::FunctionExpression(it) => GetSpan::span(it),
            Self::ImportExpression(it) => GetSpan::span(it),
            Self::LogicalExpression(it) => GetSpan::span(it),
            Self::NewExpression(it) => GetSpan::span(it),
            Self::ObjectExpression(it) => GetSpan::span(it),
            Self::ParenthesizedExpression(it) => GetSpan::span(it),
            Self::SequenceExpression(it) => GetSpan::span(it),
            Self::TaggedTemplateExpression(it) => GetSpan::span(it),
            Self::ThisExpression(it) => GetSpan::span(it),
            Self::UnaryExpression(it) => GetSpan::span(it),
            Self::UpdateExpression(it) => GetSpan::span(it),
            Self::YieldExpression(it) => GetSpan::span(it),
            Self::PrivateInExpression(it) => GetSpan::span(it),
            Self::JSXElement(it) => GetSpan::span(it),
            Self::JSXFragment(it) => GetSpan::span(it),
            Self::TSAsExpression(it) => GetSpan::span(it),
            Self::TSSatisfiesExpression(it) => GetSpan::span(it),
            Self::TSTypeAssertion(it) => GetSpan::span(it),
            Self::TSNonNullExpression(it) => GetSpan::span(it),
            Self::TSInstantiationExpression(it) => GetSpan::span(it),
            Self::ComputedMemberExpression(it) => GetSpan::span(it),
            Self::StaticMemberExpression(it) => GetSpan::span(it),
            Self::PrivateFieldExpression(it) => GetSpan::span(it),
        }
    }
}

impl GetSpan for JSXEmptyExpression {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXAttributeItem<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::Attribute(it) => GetSpan::span(it),
            Self::SpreadAttribute(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXAttribute<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXSpreadAttribute<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXAttributeName<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::Identifier(it) => GetSpan::span(it),
            Self::NamespacedName(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXAttributeValue<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::StringLiteral(it) => GetSpan::span(it),
            Self::ExpressionContainer(it) => GetSpan::span(it),
            Self::Element(it) => GetSpan::span(it),
            Self::Fragment(it) => GetSpan::span(it),
        }
    }
}

impl<'a> GetSpan for JSXIdentifier<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXChild<'a, A> {
    fn span(&self) -> Span {
        match self {
            Self::Text(it) => GetSpan::span(it),
            Self::Element(it) => GetSpan::span(it),
            Self::Fragment(it) => GetSpan::span(it),
            Self::ExpressionContainer(it) => GetSpan::span(it),
            Self::Spread(it) => GetSpan::span(it),
        }
    }
}

impl<'a, A: AstAllocator> GetSpan for JSXSpreadChild<'a, A> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl<'a> GetSpan for JSXText<'a> {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}

impl GetSpan for Comment {
    #[inline]
    fn span(&self) -> Span {
        self.span
    }
}
