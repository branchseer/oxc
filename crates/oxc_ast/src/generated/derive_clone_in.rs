// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/derives/clone_in.rs`

#![allow(clippy::default_trait_access)]

use oxc_allocator::{Allocator, CloneIn};

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

impl CloneIn for BooleanLiteral {
    type Cloned<'a> = BooleanLiteral;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BooleanLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl CloneIn for NullLiteral {
    type Cloned<'a> = NullLiteral;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        NullLiteral { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for NumericLiteral<'old_alloc> {
    type Cloned<'a> = NumericLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        NumericLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            raw: CloneIn::clone_in(&self.raw, allocator),
            base: CloneIn::clone_in(&self.base, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BigIntLiteral<'old_alloc> {
    type Cloned<'a> = BigIntLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BigIntLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            raw: CloneIn::clone_in(&self.raw, allocator),
            base: CloneIn::clone_in(&self.base, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for RegExpLiteral<'old_alloc> {
    type Cloned<'a> = RegExpLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        RegExpLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            regex: CloneIn::clone_in(&self.regex, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for RegExp<'old_alloc> {
    type Cloned<'a> = RegExp<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        RegExp {
            pattern: CloneIn::clone_in(&self.pattern, allocator),
            flags: CloneIn::clone_in(&self.flags, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for RegExpPattern<'old_alloc> {
    type Cloned<'a> = RegExpPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Raw(it) => RegExpPattern::Raw(CloneIn::clone_in(it, allocator)),
            Self::Invalid(it) => RegExpPattern::Invalid(CloneIn::clone_in(it, allocator)),
            Self::Pattern(it) => RegExpPattern::Pattern(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl CloneIn for EmptyObject {
    type Cloned<'a> = EmptyObject;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        EmptyObject
    }
}

impl<'old_alloc> CloneIn for StringLiteral<'old_alloc> {
    type Cloned<'a> = StringLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        StringLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Program<'old_alloc> {
    type Cloned<'a> = Program<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Program {
            span: CloneIn::clone_in(&self.span, allocator),
            source_type: CloneIn::clone_in(&self.source_type, allocator),
            source_text: CloneIn::clone_in(&self.source_text, allocator),
            comments: CloneIn::clone_in(&self.comments, allocator),
            hashbang: CloneIn::clone_in(&self.hashbang, allocator),
            directives: CloneIn::clone_in(&self.directives, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for Expression<'old_alloc> {
    type Cloned<'a> = Expression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::BooleanLiteral(it) => {
                Expression::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => Expression::NullLiteral(CloneIn::clone_in(it, allocator)),
            Self::NumericLiteral(it) => {
                Expression::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => Expression::BigIntLiteral(CloneIn::clone_in(it, allocator)),
            Self::RegExpLiteral(it) => Expression::RegExpLiteral(CloneIn::clone_in(it, allocator)),
            Self::StringLiteral(it) => Expression::StringLiteral(CloneIn::clone_in(it, allocator)),
            Self::TemplateLiteral(it) => {
                Expression::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => Expression::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => Expression::MetaProperty(CloneIn::clone_in(it, allocator)),
            Self::Super(it) => Expression::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                Expression::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                Expression::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                Expression::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                Expression::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                Expression::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                Expression::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                Expression::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                Expression::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                Expression::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                Expression::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                Expression::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                Expression::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => Expression::NewExpression(CloneIn::clone_in(it, allocator)),
            Self::ObjectExpression(it) => {
                Expression::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                Expression::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                Expression::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                Expression::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                Expression::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                Expression::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                Expression::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                Expression::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                Expression::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => Expression::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => Expression::JSXFragment(CloneIn::clone_in(it, allocator)),
            Self::TSAsExpression(it) => {
                Expression::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                Expression::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                Expression::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                Expression::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                Expression::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                Expression::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                Expression::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                Expression::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for IdentifierName<'old_alloc> {
    type Cloned<'a> = IdentifierName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        IdentifierName {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for IdentifierReference<'old_alloc> {
    type Cloned<'a> = IdentifierReference<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        IdentifierReference {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            reference_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for BindingIdentifier<'old_alloc> {
    type Cloned<'a> = BindingIdentifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BindingIdentifier {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            symbol_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for LabelIdentifier<'old_alloc> {
    type Cloned<'a> = LabelIdentifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        LabelIdentifier {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}

impl CloneIn for ThisExpression {
    type Cloned<'a> = ThisExpression;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ThisExpression { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for ArrayExpression<'old_alloc> {
    type Cloned<'a> = ArrayExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ArrayExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            elements: CloneIn::clone_in(&self.elements, allocator),
            trailing_comma: CloneIn::clone_in(&self.trailing_comma, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ArrayExpressionElement<'old_alloc> {
    type Cloned<'a> = ArrayExpressionElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::SpreadElement(it) => {
                ArrayExpressionElement::SpreadElement(CloneIn::clone_in(it, allocator))
            }
            Self::Elision(it) => ArrayExpressionElement::Elision(CloneIn::clone_in(it, allocator)),
            Self::BooleanLiteral(it) => {
                ArrayExpressionElement::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => {
                ArrayExpressionElement::NullLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NumericLiteral(it) => {
                ArrayExpressionElement::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => {
                ArrayExpressionElement::BigIntLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::RegExpLiteral(it) => {
                ArrayExpressionElement::RegExpLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                ArrayExpressionElement::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TemplateLiteral(it) => {
                ArrayExpressionElement::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => {
                ArrayExpressionElement::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::MetaProperty(it) => {
                ArrayExpressionElement::MetaProperty(CloneIn::clone_in(it, allocator))
            }
            Self::Super(it) => ArrayExpressionElement::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                ArrayExpressionElement::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                ArrayExpressionElement::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                ArrayExpressionElement::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                ArrayExpressionElement::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                ArrayExpressionElement::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                ArrayExpressionElement::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                ArrayExpressionElement::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                ArrayExpressionElement::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                ArrayExpressionElement::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                ArrayExpressionElement::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                ArrayExpressionElement::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                ArrayExpressionElement::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => {
                ArrayExpressionElement::NewExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectExpression(it) => {
                ArrayExpressionElement::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                ArrayExpressionElement::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                ArrayExpressionElement::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                ArrayExpressionElement::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                ArrayExpressionElement::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                ArrayExpressionElement::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                ArrayExpressionElement::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                ArrayExpressionElement::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                ArrayExpressionElement::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => {
                ArrayExpressionElement::JSXElement(CloneIn::clone_in(it, allocator))
            }
            Self::JSXFragment(it) => {
                ArrayExpressionElement::JSXFragment(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                ArrayExpressionElement::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                ArrayExpressionElement::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                ArrayExpressionElement::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                ArrayExpressionElement::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                ArrayExpressionElement::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                ArrayExpressionElement::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                ArrayExpressionElement::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                ArrayExpressionElement::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for Elision {
    type Cloned<'a> = Elision;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Elision { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for ObjectExpression<'old_alloc> {
    type Cloned<'a> = ObjectExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ObjectExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            properties: CloneIn::clone_in(&self.properties, allocator),
            trailing_comma: CloneIn::clone_in(&self.trailing_comma, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ObjectPropertyKind<'old_alloc> {
    type Cloned<'a> = ObjectPropertyKind<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ObjectProperty(it) => {
                ObjectPropertyKind::ObjectProperty(CloneIn::clone_in(it, allocator))
            }
            Self::SpreadProperty(it) => {
                ObjectPropertyKind::SpreadProperty(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ObjectProperty<'old_alloc> {
    type Cloned<'a> = ObjectProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ObjectProperty {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            init: CloneIn::clone_in(&self.init, allocator),
            method: CloneIn::clone_in(&self.method, allocator),
            shorthand: CloneIn::clone_in(&self.shorthand, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for PropertyKey<'old_alloc> {
    type Cloned<'a> = PropertyKey<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::StaticIdentifier(it) => {
                PropertyKey::StaticIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateIdentifier(it) => {
                PropertyKey::PrivateIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::BooleanLiteral(it) => {
                PropertyKey::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => PropertyKey::NullLiteral(CloneIn::clone_in(it, allocator)),
            Self::NumericLiteral(it) => {
                PropertyKey::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => PropertyKey::BigIntLiteral(CloneIn::clone_in(it, allocator)),
            Self::RegExpLiteral(it) => PropertyKey::RegExpLiteral(CloneIn::clone_in(it, allocator)),
            Self::StringLiteral(it) => PropertyKey::StringLiteral(CloneIn::clone_in(it, allocator)),
            Self::TemplateLiteral(it) => {
                PropertyKey::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => PropertyKey::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => PropertyKey::MetaProperty(CloneIn::clone_in(it, allocator)),
            Self::Super(it) => PropertyKey::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                PropertyKey::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                PropertyKey::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                PropertyKey::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                PropertyKey::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                PropertyKey::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                PropertyKey::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                PropertyKey::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                PropertyKey::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                PropertyKey::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                PropertyKey::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                PropertyKey::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                PropertyKey::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => PropertyKey::NewExpression(CloneIn::clone_in(it, allocator)),
            Self::ObjectExpression(it) => {
                PropertyKey::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                PropertyKey::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                PropertyKey::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                PropertyKey::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                PropertyKey::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                PropertyKey::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                PropertyKey::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                PropertyKey::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                PropertyKey::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => PropertyKey::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => PropertyKey::JSXFragment(CloneIn::clone_in(it, allocator)),
            Self::TSAsExpression(it) => {
                PropertyKey::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                PropertyKey::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                PropertyKey::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                PropertyKey::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                PropertyKey::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                PropertyKey::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                PropertyKey::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                PropertyKey::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for PropertyKind {
    type Cloned<'a> = PropertyKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Init => PropertyKind::Init,
            Self::Get => PropertyKind::Get,
            Self::Set => PropertyKind::Set,
        }
    }
}

impl<'old_alloc> CloneIn for TemplateLiteral<'old_alloc> {
    type Cloned<'a> = TemplateLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TemplateLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            quasis: CloneIn::clone_in(&self.quasis, allocator),
            expressions: CloneIn::clone_in(&self.expressions, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TaggedTemplateExpression<'old_alloc> {
    type Cloned<'a> = TaggedTemplateExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TaggedTemplateExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            tag: CloneIn::clone_in(&self.tag, allocator),
            quasi: CloneIn::clone_in(&self.quasi, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TemplateElement<'old_alloc> {
    type Cloned<'a> = TemplateElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TemplateElement {
            span: CloneIn::clone_in(&self.span, allocator),
            tail: CloneIn::clone_in(&self.tail, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TemplateElementValue<'old_alloc> {
    type Cloned<'a> = TemplateElementValue<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TemplateElementValue {
            raw: CloneIn::clone_in(&self.raw, allocator),
            cooked: CloneIn::clone_in(&self.cooked, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for MemberExpression<'old_alloc> {
    type Cloned<'a> = MemberExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ComputedMemberExpression(it) => {
                MemberExpression::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                MemberExpression::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                MemberExpression::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ComputedMemberExpression<'old_alloc> {
    type Cloned<'a> = ComputedMemberExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ComputedMemberExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            object: CloneIn::clone_in(&self.object, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for StaticMemberExpression<'old_alloc> {
    type Cloned<'a> = StaticMemberExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        StaticMemberExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            object: CloneIn::clone_in(&self.object, allocator),
            property: CloneIn::clone_in(&self.property, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for PrivateFieldExpression<'old_alloc> {
    type Cloned<'a> = PrivateFieldExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        PrivateFieldExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            object: CloneIn::clone_in(&self.object, allocator),
            field: CloneIn::clone_in(&self.field, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for CallExpression<'old_alloc> {
    type Cloned<'a> = CallExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CallExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            callee: CloneIn::clone_in(&self.callee, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            arguments: CloneIn::clone_in(&self.arguments, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for NewExpression<'old_alloc> {
    type Cloned<'a> = NewExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        NewExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            callee: CloneIn::clone_in(&self.callee, allocator),
            arguments: CloneIn::clone_in(&self.arguments, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for MetaProperty<'old_alloc> {
    type Cloned<'a> = MetaProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        MetaProperty {
            span: CloneIn::clone_in(&self.span, allocator),
            meta: CloneIn::clone_in(&self.meta, allocator),
            property: CloneIn::clone_in(&self.property, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for SpreadElement<'old_alloc> {
    type Cloned<'a> = SpreadElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        SpreadElement {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Argument<'old_alloc> {
    type Cloned<'a> = Argument<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::SpreadElement(it) => Argument::SpreadElement(CloneIn::clone_in(it, allocator)),
            Self::BooleanLiteral(it) => Argument::BooleanLiteral(CloneIn::clone_in(it, allocator)),
            Self::NullLiteral(it) => Argument::NullLiteral(CloneIn::clone_in(it, allocator)),
            Self::NumericLiteral(it) => Argument::NumericLiteral(CloneIn::clone_in(it, allocator)),
            Self::BigIntLiteral(it) => Argument::BigIntLiteral(CloneIn::clone_in(it, allocator)),
            Self::RegExpLiteral(it) => Argument::RegExpLiteral(CloneIn::clone_in(it, allocator)),
            Self::StringLiteral(it) => Argument::StringLiteral(CloneIn::clone_in(it, allocator)),
            Self::TemplateLiteral(it) => {
                Argument::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => Argument::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => Argument::MetaProperty(CloneIn::clone_in(it, allocator)),
            Self::Super(it) => Argument::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                Argument::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                Argument::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                Argument::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                Argument::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                Argument::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => Argument::CallExpression(CloneIn::clone_in(it, allocator)),
            Self::ChainExpression(it) => {
                Argument::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                Argument::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                Argument::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                Argument::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                Argument::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                Argument::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => Argument::NewExpression(CloneIn::clone_in(it, allocator)),
            Self::ObjectExpression(it) => {
                Argument::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                Argument::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                Argument::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                Argument::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => Argument::ThisExpression(CloneIn::clone_in(it, allocator)),
            Self::UnaryExpression(it) => {
                Argument::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                Argument::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                Argument::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                Argument::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => Argument::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => Argument::JSXFragment(CloneIn::clone_in(it, allocator)),
            Self::TSAsExpression(it) => Argument::TSAsExpression(CloneIn::clone_in(it, allocator)),
            Self::TSSatisfiesExpression(it) => {
                Argument::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                Argument::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                Argument::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                Argument::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                Argument::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                Argument::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                Argument::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for UpdateExpression<'old_alloc> {
    type Cloned<'a> = UpdateExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        UpdateExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            prefix: CloneIn::clone_in(&self.prefix, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for UnaryExpression<'old_alloc> {
    type Cloned<'a> = UnaryExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        UnaryExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BinaryExpression<'old_alloc> {
    type Cloned<'a> = BinaryExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BinaryExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for PrivateInExpression<'old_alloc> {
    type Cloned<'a> = PrivateInExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        PrivateInExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for LogicalExpression<'old_alloc> {
    type Cloned<'a> = LogicalExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        LogicalExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ConditionalExpression<'old_alloc> {
    type Cloned<'a> = ConditionalExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ConditionalExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
            consequent: CloneIn::clone_in(&self.consequent, allocator),
            alternate: CloneIn::clone_in(&self.alternate, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentExpression<'old_alloc> {
    type Cloned<'a> = AssignmentExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTarget<'old_alloc> {
    type Cloned<'a> = AssignmentTarget<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::AssignmentTargetIdentifier(it) => {
                AssignmentTarget::AssignmentTargetIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                AssignmentTarget::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                AssignmentTarget::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                AssignmentTarget::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                AssignmentTarget::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                AssignmentTarget::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                AssignmentTarget::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                AssignmentTarget::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                AssignmentTarget::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrayAssignmentTarget(it) => {
                AssignmentTarget::ArrayAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectAssignmentTarget(it) => {
                AssignmentTarget::ObjectAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for SimpleAssignmentTarget<'old_alloc> {
    type Cloned<'a> = SimpleAssignmentTarget<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::AssignmentTargetIdentifier(it) => {
                SimpleAssignmentTarget::AssignmentTargetIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                SimpleAssignmentTarget::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                SimpleAssignmentTarget::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                SimpleAssignmentTarget::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                SimpleAssignmentTarget::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                SimpleAssignmentTarget::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                SimpleAssignmentTarget::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                SimpleAssignmentTarget::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                SimpleAssignmentTarget::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetPattern<'old_alloc> {
    type Cloned<'a> = AssignmentTargetPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ArrayAssignmentTarget(it) => {
                AssignmentTargetPattern::ArrayAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectAssignmentTarget(it) => {
                AssignmentTargetPattern::ObjectAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ArrayAssignmentTarget<'old_alloc> {
    type Cloned<'a> = ArrayAssignmentTarget<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ArrayAssignmentTarget {
            span: CloneIn::clone_in(&self.span, allocator),
            elements: CloneIn::clone_in(&self.elements, allocator),
            rest: CloneIn::clone_in(&self.rest, allocator),
            trailing_comma: CloneIn::clone_in(&self.trailing_comma, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ObjectAssignmentTarget<'old_alloc> {
    type Cloned<'a> = ObjectAssignmentTarget<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ObjectAssignmentTarget {
            span: CloneIn::clone_in(&self.span, allocator),
            properties: CloneIn::clone_in(&self.properties, allocator),
            rest: CloneIn::clone_in(&self.rest, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetRest<'old_alloc> {
    type Cloned<'a> = AssignmentTargetRest<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentTargetRest {
            span: CloneIn::clone_in(&self.span, allocator),
            target: CloneIn::clone_in(&self.target, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetMaybeDefault<'old_alloc> {
    type Cloned<'a> = AssignmentTargetMaybeDefault<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::AssignmentTargetWithDefault(it) => {
                AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::AssignmentTargetIdentifier(it) => {
                AssignmentTargetMaybeDefault::AssignmentTargetIdentifier(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::TSAsExpression(it) => {
                AssignmentTargetMaybeDefault::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => AssignmentTargetMaybeDefault::TSSatisfiesExpression(
                CloneIn::clone_in(it, allocator),
            ),
            Self::TSNonNullExpression(it) => {
                AssignmentTargetMaybeDefault::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                AssignmentTargetMaybeDefault::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                AssignmentTargetMaybeDefault::TSInstantiationExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::ComputedMemberExpression(it) => {
                AssignmentTargetMaybeDefault::ComputedMemberExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::StaticMemberExpression(it) => {
                AssignmentTargetMaybeDefault::StaticMemberExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::PrivateFieldExpression(it) => {
                AssignmentTargetMaybeDefault::PrivateFieldExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::ArrayAssignmentTarget(it) => AssignmentTargetMaybeDefault::ArrayAssignmentTarget(
                CloneIn::clone_in(it, allocator),
            ),
            Self::ObjectAssignmentTarget(it) => {
                AssignmentTargetMaybeDefault::ObjectAssignmentTarget(CloneIn::clone_in(
                    it, allocator,
                ))
            }
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetWithDefault<'old_alloc> {
    type Cloned<'a> = AssignmentTargetWithDefault<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentTargetWithDefault {
            span: CloneIn::clone_in(&self.span, allocator),
            binding: CloneIn::clone_in(&self.binding, allocator),
            init: CloneIn::clone_in(&self.init, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetProperty<'old_alloc> {
    type Cloned<'a> = AssignmentTargetProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::AssignmentTargetPropertyIdentifier(it) => {
                AssignmentTargetProperty::AssignmentTargetPropertyIdentifier(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::AssignmentTargetPropertyProperty(it) => {
                AssignmentTargetProperty::AssignmentTargetPropertyProperty(CloneIn::clone_in(
                    it, allocator,
                ))
            }
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetPropertyIdentifier<'old_alloc> {
    type Cloned<'a> = AssignmentTargetPropertyIdentifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentTargetPropertyIdentifier {
            span: CloneIn::clone_in(&self.span, allocator),
            binding: CloneIn::clone_in(&self.binding, allocator),
            init: CloneIn::clone_in(&self.init, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentTargetPropertyProperty<'old_alloc> {
    type Cloned<'a> = AssignmentTargetPropertyProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentTargetPropertyProperty {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            binding: CloneIn::clone_in(&self.binding, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for SequenceExpression<'old_alloc> {
    type Cloned<'a> = SequenceExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        SequenceExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expressions: CloneIn::clone_in(&self.expressions, allocator),
        }
    }
}

impl CloneIn for Super {
    type Cloned<'a> = Super;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Super { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for AwaitExpression<'old_alloc> {
    type Cloned<'a> = AwaitExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AwaitExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ChainExpression<'old_alloc> {
    type Cloned<'a> = ChainExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ChainExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ChainElement<'old_alloc> {
    type Cloned<'a> = ChainElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::CallExpression(it) => {
                ChainElement::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                ChainElement::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                ChainElement::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                ChainElement::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ParenthesizedExpression<'old_alloc> {
    type Cloned<'a> = ParenthesizedExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ParenthesizedExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Statement<'old_alloc> {
    type Cloned<'a> = Statement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::BlockStatement(it) => Statement::BlockStatement(CloneIn::clone_in(it, allocator)),
            Self::BreakStatement(it) => Statement::BreakStatement(CloneIn::clone_in(it, allocator)),
            Self::ContinueStatement(it) => {
                Statement::ContinueStatement(CloneIn::clone_in(it, allocator))
            }
            Self::DebuggerStatement(it) => {
                Statement::DebuggerStatement(CloneIn::clone_in(it, allocator))
            }
            Self::DoWhileStatement(it) => {
                Statement::DoWhileStatement(CloneIn::clone_in(it, allocator))
            }
            Self::EmptyStatement(it) => Statement::EmptyStatement(CloneIn::clone_in(it, allocator)),
            Self::ExpressionStatement(it) => {
                Statement::ExpressionStatement(CloneIn::clone_in(it, allocator))
            }
            Self::ForInStatement(it) => Statement::ForInStatement(CloneIn::clone_in(it, allocator)),
            Self::ForOfStatement(it) => Statement::ForOfStatement(CloneIn::clone_in(it, allocator)),
            Self::ForStatement(it) => Statement::ForStatement(CloneIn::clone_in(it, allocator)),
            Self::IfStatement(it) => Statement::IfStatement(CloneIn::clone_in(it, allocator)),
            Self::LabeledStatement(it) => {
                Statement::LabeledStatement(CloneIn::clone_in(it, allocator))
            }
            Self::ReturnStatement(it) => {
                Statement::ReturnStatement(CloneIn::clone_in(it, allocator))
            }
            Self::SwitchStatement(it) => {
                Statement::SwitchStatement(CloneIn::clone_in(it, allocator))
            }
            Self::ThrowStatement(it) => Statement::ThrowStatement(CloneIn::clone_in(it, allocator)),
            Self::TryStatement(it) => Statement::TryStatement(CloneIn::clone_in(it, allocator)),
            Self::WhileStatement(it) => Statement::WhileStatement(CloneIn::clone_in(it, allocator)),
            Self::WithStatement(it) => Statement::WithStatement(CloneIn::clone_in(it, allocator)),
            Self::VariableDeclaration(it) => {
                Statement::VariableDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionDeclaration(it) => {
                Statement::FunctionDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ClassDeclaration(it) => {
                Statement::ClassDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAliasDeclaration(it) => {
                Statement::TSTypeAliasDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSInterfaceDeclaration(it) => {
                Statement::TSInterfaceDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSEnumDeclaration(it) => {
                Statement::TSEnumDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSModuleDeclaration(it) => {
                Statement::TSModuleDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSImportEqualsDeclaration(it) => {
                Statement::TSImportEqualsDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ImportDeclaration(it) => {
                Statement::ImportDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportAllDeclaration(it) => {
                Statement::ExportAllDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportDefaultDeclaration(it) => {
                Statement::ExportDefaultDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportNamedDeclaration(it) => {
                Statement::ExportNamedDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSExportAssignment(it) => {
                Statement::TSExportAssignment(CloneIn::clone_in(it, allocator))
            }
            Self::TSNamespaceExportDeclaration(it) => {
                Statement::TSNamespaceExportDeclaration(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for Directive<'old_alloc> {
    type Cloned<'a> = Directive<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Directive {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            directive: CloneIn::clone_in(&self.directive, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Hashbang<'old_alloc> {
    type Cloned<'a> = Hashbang<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Hashbang {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BlockStatement<'old_alloc> {
    type Cloned<'a> = BlockStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BlockStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for Declaration<'old_alloc> {
    type Cloned<'a> = Declaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::VariableDeclaration(it) => {
                Declaration::VariableDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionDeclaration(it) => {
                Declaration::FunctionDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ClassDeclaration(it) => {
                Declaration::ClassDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAliasDeclaration(it) => {
                Declaration::TSTypeAliasDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSInterfaceDeclaration(it) => {
                Declaration::TSInterfaceDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSEnumDeclaration(it) => {
                Declaration::TSEnumDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSModuleDeclaration(it) => {
                Declaration::TSModuleDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSImportEqualsDeclaration(it) => {
                Declaration::TSImportEqualsDeclaration(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for VariableDeclaration<'old_alloc> {
    type Cloned<'a> = VariableDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        VariableDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            declarations: CloneIn::clone_in(&self.declarations, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
        }
    }
}

impl CloneIn for VariableDeclarationKind {
    type Cloned<'a> = VariableDeclarationKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Var => VariableDeclarationKind::Var,
            Self::Const => VariableDeclarationKind::Const,
            Self::Let => VariableDeclarationKind::Let,
            Self::Using => VariableDeclarationKind::Using,
            Self::AwaitUsing => VariableDeclarationKind::AwaitUsing,
        }
    }
}

impl<'old_alloc> CloneIn for VariableDeclarator<'old_alloc> {
    type Cloned<'a> = VariableDeclarator<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        VariableDeclarator {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            init: CloneIn::clone_in(&self.init, allocator),
            definite: CloneIn::clone_in(&self.definite, allocator),
        }
    }
}

impl CloneIn for EmptyStatement {
    type Cloned<'a> = EmptyStatement;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        EmptyStatement { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for ExpressionStatement<'old_alloc> {
    type Cloned<'a> = ExpressionStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ExpressionStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for IfStatement<'old_alloc> {
    type Cloned<'a> = IfStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        IfStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
            consequent: CloneIn::clone_in(&self.consequent, allocator),
            alternate: CloneIn::clone_in(&self.alternate, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for DoWhileStatement<'old_alloc> {
    type Cloned<'a> = DoWhileStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        DoWhileStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for WhileStatement<'old_alloc> {
    type Cloned<'a> = WhileStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        WhileStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ForStatement<'old_alloc> {
    type Cloned<'a> = ForStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ForStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            init: CloneIn::clone_in(&self.init, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
            update: CloneIn::clone_in(&self.update, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for ForStatementInit<'old_alloc> {
    type Cloned<'a> = ForStatementInit<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::VariableDeclaration(it) => {
                ForStatementInit::VariableDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::BooleanLiteral(it) => {
                ForStatementInit::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => {
                ForStatementInit::NullLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NumericLiteral(it) => {
                ForStatementInit::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => {
                ForStatementInit::BigIntLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::RegExpLiteral(it) => {
                ForStatementInit::RegExpLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                ForStatementInit::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TemplateLiteral(it) => {
                ForStatementInit::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => ForStatementInit::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => {
                ForStatementInit::MetaProperty(CloneIn::clone_in(it, allocator))
            }
            Self::Super(it) => ForStatementInit::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                ForStatementInit::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                ForStatementInit::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                ForStatementInit::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                ForStatementInit::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                ForStatementInit::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                ForStatementInit::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                ForStatementInit::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                ForStatementInit::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                ForStatementInit::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                ForStatementInit::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                ForStatementInit::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                ForStatementInit::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => {
                ForStatementInit::NewExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectExpression(it) => {
                ForStatementInit::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                ForStatementInit::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                ForStatementInit::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                ForStatementInit::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                ForStatementInit::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                ForStatementInit::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                ForStatementInit::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                ForStatementInit::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                ForStatementInit::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => ForStatementInit::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => {
                ForStatementInit::JSXFragment(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                ForStatementInit::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                ForStatementInit::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                ForStatementInit::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                ForStatementInit::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                ForStatementInit::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                ForStatementInit::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                ForStatementInit::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                ForStatementInit::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ForInStatement<'old_alloc> {
    type Cloned<'a> = ForInStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ForInStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for ForStatementLeft<'old_alloc> {
    type Cloned<'a> = ForStatementLeft<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::VariableDeclaration(it) => {
                ForStatementLeft::VariableDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentTargetIdentifier(it) => {
                ForStatementLeft::AssignmentTargetIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                ForStatementLeft::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                ForStatementLeft::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                ForStatementLeft::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                ForStatementLeft::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                ForStatementLeft::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                ForStatementLeft::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                ForStatementLeft::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                ForStatementLeft::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrayAssignmentTarget(it) => {
                ForStatementLeft::ArrayAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectAssignmentTarget(it) => {
                ForStatementLeft::ObjectAssignmentTarget(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ForOfStatement<'old_alloc> {
    type Cloned<'a> = ForOfStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ForOfStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            r#await: CloneIn::clone_in(&self.r#await, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for ContinueStatement<'old_alloc> {
    type Cloned<'a> = ContinueStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ContinueStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            label: CloneIn::clone_in(&self.label, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BreakStatement<'old_alloc> {
    type Cloned<'a> = BreakStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BreakStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            label: CloneIn::clone_in(&self.label, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ReturnStatement<'old_alloc> {
    type Cloned<'a> = ReturnStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ReturnStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for WithStatement<'old_alloc> {
    type Cloned<'a> = WithStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        WithStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            object: CloneIn::clone_in(&self.object, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for SwitchStatement<'old_alloc> {
    type Cloned<'a> = SwitchStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        SwitchStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            discriminant: CloneIn::clone_in(&self.discriminant, allocator),
            cases: CloneIn::clone_in(&self.cases, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for SwitchCase<'old_alloc> {
    type Cloned<'a> = SwitchCase<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        SwitchCase {
            span: CloneIn::clone_in(&self.span, allocator),
            test: CloneIn::clone_in(&self.test, allocator),
            consequent: CloneIn::clone_in(&self.consequent, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for LabeledStatement<'old_alloc> {
    type Cloned<'a> = LabeledStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        LabeledStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            label: CloneIn::clone_in(&self.label, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ThrowStatement<'old_alloc> {
    type Cloned<'a> = ThrowStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ThrowStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TryStatement<'old_alloc> {
    type Cloned<'a> = TryStatement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TryStatement {
            span: CloneIn::clone_in(&self.span, allocator),
            block: CloneIn::clone_in(&self.block, allocator),
            handler: CloneIn::clone_in(&self.handler, allocator),
            finalizer: CloneIn::clone_in(&self.finalizer, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for CatchClause<'old_alloc> {
    type Cloned<'a> = CatchClause<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CatchClause {
            span: CloneIn::clone_in(&self.span, allocator),
            param: CloneIn::clone_in(&self.param, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for CatchParameter<'old_alloc> {
    type Cloned<'a> = CatchParameter<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        CatchParameter {
            span: CloneIn::clone_in(&self.span, allocator),
            pattern: CloneIn::clone_in(&self.pattern, allocator),
        }
    }
}

impl CloneIn for DebuggerStatement {
    type Cloned<'a> = DebuggerStatement;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        DebuggerStatement { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for BindingPattern<'old_alloc> {
    type Cloned<'a> = BindingPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BindingPattern {
            kind: CloneIn::clone_in(&self.kind, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BindingPatternKind<'old_alloc> {
    type Cloned<'a> = BindingPatternKind<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::BindingIdentifier(it) => {
                BindingPatternKind::BindingIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectPattern(it) => {
                BindingPatternKind::ObjectPattern(CloneIn::clone_in(it, allocator))
            }
            Self::ArrayPattern(it) => {
                BindingPatternKind::ArrayPattern(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentPattern(it) => {
                BindingPatternKind::AssignmentPattern(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for AssignmentPattern<'old_alloc> {
    type Cloned<'a> = AssignmentPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AssignmentPattern {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ObjectPattern<'old_alloc> {
    type Cloned<'a> = ObjectPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ObjectPattern {
            span: CloneIn::clone_in(&self.span, allocator),
            properties: CloneIn::clone_in(&self.properties, allocator),
            rest: CloneIn::clone_in(&self.rest, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BindingProperty<'old_alloc> {
    type Cloned<'a> = BindingProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BindingProperty {
            span: CloneIn::clone_in(&self.span, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            shorthand: CloneIn::clone_in(&self.shorthand, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ArrayPattern<'old_alloc> {
    type Cloned<'a> = ArrayPattern<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ArrayPattern {
            span: CloneIn::clone_in(&self.span, allocator),
            elements: CloneIn::clone_in(&self.elements, allocator),
            rest: CloneIn::clone_in(&self.rest, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for BindingRestElement<'old_alloc> {
    type Cloned<'a> = BindingRestElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        BindingRestElement {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Function<'old_alloc> {
    type Cloned<'a> = Function<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Function {
            r#type: CloneIn::clone_in(&self.r#type, allocator),
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            generator: CloneIn::clone_in(&self.generator, allocator),
            r#async: CloneIn::clone_in(&self.r#async, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            this_param: CloneIn::clone_in(&self.this_param, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl CloneIn for FunctionType {
    type Cloned<'a> = FunctionType;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::FunctionDeclaration => FunctionType::FunctionDeclaration,
            Self::FunctionExpression => FunctionType::FunctionExpression,
            Self::TSDeclareFunction => FunctionType::TSDeclareFunction,
            Self::TSEmptyBodyFunctionExpression => FunctionType::TSEmptyBodyFunctionExpression,
        }
    }
}

impl<'old_alloc> CloneIn for FormalParameters<'old_alloc> {
    type Cloned<'a> = FormalParameters<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        FormalParameters {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            items: CloneIn::clone_in(&self.items, allocator),
            rest: CloneIn::clone_in(&self.rest, allocator),
        }
    }
}

impl CloneIn for FormalParameterModifiers {
    type Cloned<'a> = FormalParameterModifiers;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        FormalParameterModifiers {
            span: CloneIn::clone_in(&self.span, allocator),
            accessibility: CloneIn::clone_in(&self.accessibility, allocator),
            readonly: CloneIn::clone_in(&self.readonly, allocator),
            r#override: CloneIn::clone_in(&self.r#override, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for FormalParameter<'old_alloc> {
    type Cloned<'a> = FormalParameter<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        FormalParameter {
            span: CloneIn::clone_in(&self.span, allocator),
            decorators: CloneIn::clone_in(&self.decorators, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            pattern: CloneIn::clone_in(&self.pattern, allocator),
        }
    }
}

impl CloneIn for FormalParameterKind {
    type Cloned<'a> = FormalParameterKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::FormalParameter => FormalParameterKind::FormalParameter,
            Self::UniqueFormalParameters => FormalParameterKind::UniqueFormalParameters,
            Self::ArrowFormalParameters => FormalParameterKind::ArrowFormalParameters,
            Self::Signature => FormalParameterKind::Signature,
        }
    }
}

impl<'old_alloc> CloneIn for FunctionBody<'old_alloc> {
    type Cloned<'a> = FunctionBody<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        FunctionBody {
            span: CloneIn::clone_in(&self.span, allocator),
            directives: CloneIn::clone_in(&self.directives, allocator),
            statements: CloneIn::clone_in(&self.statements, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ArrowFunctionExpression<'old_alloc> {
    type Cloned<'a> = ArrowFunctionExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ArrowFunctionExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            r#async: CloneIn::clone_in(&self.r#async, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for YieldExpression<'old_alloc> {
    type Cloned<'a> = YieldExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        YieldExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            delegate: CloneIn::clone_in(&self.delegate, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl CloneIn for ClassModifiers {
    type Cloned<'a> = ClassModifiers;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ClassModifiers {
            span: CloneIn::clone_in(&self.span, allocator),
            r#abstract: CloneIn::clone_in(&self.r#abstract, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Class<'old_alloc> {
    type Cloned<'a> = Class<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Class {
            r#type: CloneIn::clone_in(&self.r#type, allocator),
            span: CloneIn::clone_in(&self.span, allocator),
            decorators: CloneIn::clone_in(&self.decorators, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            super_class: CloneIn::clone_in(&self.super_class, allocator),
            super_type_parameters: CloneIn::clone_in(&self.super_type_parameters, allocator),
            implements: CloneIn::clone_in(&self.implements, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl CloneIn for ClassType {
    type Cloned<'a> = ClassType;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ClassDeclaration => ClassType::ClassDeclaration,
            Self::ClassExpression => ClassType::ClassExpression,
        }
    }
}

impl<'old_alloc> CloneIn for ClassBody<'old_alloc> {
    type Cloned<'a> = ClassBody<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ClassBody {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ClassElement<'old_alloc> {
    type Cloned<'a> = ClassElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::StaticBlock(it) => ClassElement::StaticBlock(CloneIn::clone_in(it, allocator)),
            Self::MethodDefinition(it) => {
                ClassElement::MethodDefinition(CloneIn::clone_in(it, allocator))
            }
            Self::PropertyDefinition(it) => {
                ClassElement::PropertyDefinition(CloneIn::clone_in(it, allocator))
            }
            Self::AccessorProperty(it) => {
                ClassElement::AccessorProperty(CloneIn::clone_in(it, allocator))
            }
            Self::TSIndexSignature(it) => {
                ClassElement::TSIndexSignature(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for MethodDefinition<'old_alloc> {
    type Cloned<'a> = MethodDefinition<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        MethodDefinition {
            span: CloneIn::clone_in(&self.span, allocator),
            decorators: CloneIn::clone_in(&self.decorators, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl CloneIn for MethodDefinitionType {
    type Cloned<'a> = MethodDefinitionType;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::MethodDefinition => MethodDefinitionType::MethodDefinition,
            Self::TSAbstractMethodDefinition => MethodDefinitionType::TSAbstractMethodDefinition,
        }
    }
}

impl CloneIn for ClassElementModifiers {
    type Cloned<'a> = ClassElementModifiers;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ClassElementModifiers {
            span: CloneIn::clone_in(&self.span, allocator),
            r#async: CloneIn::clone_in(&self.r#async, allocator),
            r#abstract: CloneIn::clone_in(&self.r#abstract, allocator),
            r#static: CloneIn::clone_in(&self.r#static, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            r#override: CloneIn::clone_in(&self.r#override, allocator),
            readonly: CloneIn::clone_in(&self.readonly, allocator),
            accessibility: CloneIn::clone_in(&self.accessibility, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for PropertyDefinition<'old_alloc> {
    type Cloned<'a> = PropertyDefinition<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        PropertyDefinition {
            span: CloneIn::clone_in(&self.span, allocator),
            decorators: CloneIn::clone_in(&self.decorators, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
            definite: CloneIn::clone_in(&self.definite, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl CloneIn for PropertyDefinitionType {
    type Cloned<'a> = PropertyDefinitionType;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::PropertyDefinition => PropertyDefinitionType::PropertyDefinition,
            Self::TSAbstractPropertyDefinition => {
                PropertyDefinitionType::TSAbstractPropertyDefinition
            }
        }
    }
}

impl CloneIn for MethodDefinitionKind {
    type Cloned<'a> = MethodDefinitionKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Constructor => MethodDefinitionKind::Constructor,
            Self::Method => MethodDefinitionKind::Method,
            Self::Get => MethodDefinitionKind::Get,
            Self::Set => MethodDefinitionKind::Set,
        }
    }
}

impl<'old_alloc> CloneIn for PrivateIdentifier<'old_alloc> {
    type Cloned<'a> = PrivateIdentifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        PrivateIdentifier {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for StaticBlock<'old_alloc> {
    type Cloned<'a> = StaticBlock<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        StaticBlock {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for ModuleDeclaration<'old_alloc> {
    type Cloned<'a> = ModuleDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ImportDeclaration(it) => {
                ModuleDeclaration::ImportDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportAllDeclaration(it) => {
                ModuleDeclaration::ExportAllDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportDefaultDeclaration(it) => {
                ModuleDeclaration::ExportDefaultDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ExportNamedDeclaration(it) => {
                ModuleDeclaration::ExportNamedDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSExportAssignment(it) => {
                ModuleDeclaration::TSExportAssignment(CloneIn::clone_in(it, allocator))
            }
            Self::TSNamespaceExportDeclaration(it) => {
                ModuleDeclaration::TSNamespaceExportDeclaration(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for AccessorPropertyType {
    type Cloned<'a> = AccessorPropertyType;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::AccessorProperty => AccessorPropertyType::AccessorProperty,
            Self::TSAbstractAccessorProperty => AccessorPropertyType::TSAbstractAccessorProperty,
        }
    }
}

impl<'old_alloc> CloneIn for AccessorProperty<'old_alloc> {
    type Cloned<'a> = AccessorProperty<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        AccessorProperty {
            span: CloneIn::clone_in(&self.span, allocator),
            decorators: CloneIn::clone_in(&self.decorators, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
            definite: CloneIn::clone_in(&self.definite, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportExpression<'old_alloc> {
    type Cloned<'a> = ImportExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            source: CloneIn::clone_in(&self.source, allocator),
            arguments: CloneIn::clone_in(&self.arguments, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportDeclaration<'old_alloc> {
    type Cloned<'a> = ImportDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            specifiers: CloneIn::clone_in(&self.specifiers, allocator),
            source: CloneIn::clone_in(&self.source, allocator),
            with_clause: CloneIn::clone_in(&self.with_clause, allocator),
            import_kind: CloneIn::clone_in(&self.import_kind, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportDeclarationSpecifier<'old_alloc> {
    type Cloned<'a> = ImportDeclarationSpecifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ImportSpecifier(it) => {
                ImportDeclarationSpecifier::ImportSpecifier(CloneIn::clone_in(it, allocator))
            }
            Self::ImportDefaultSpecifier(it) => {
                ImportDeclarationSpecifier::ImportDefaultSpecifier(CloneIn::clone_in(it, allocator))
            }
            Self::ImportNamespaceSpecifier(it) => {
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(CloneIn::clone_in(
                    it, allocator,
                ))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ImportSpecifier<'old_alloc> {
    type Cloned<'a> = ImportSpecifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportSpecifier {
            span: CloneIn::clone_in(&self.span, allocator),
            imported: CloneIn::clone_in(&self.imported, allocator),
            local: CloneIn::clone_in(&self.local, allocator),
            import_kind: CloneIn::clone_in(&self.import_kind, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportDefaultSpecifier<'old_alloc> {
    type Cloned<'a> = ImportDefaultSpecifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportDefaultSpecifier {
            span: CloneIn::clone_in(&self.span, allocator),
            local: CloneIn::clone_in(&self.local, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportNamespaceSpecifier<'old_alloc> {
    type Cloned<'a> = ImportNamespaceSpecifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportNamespaceSpecifier {
            span: CloneIn::clone_in(&self.span, allocator),
            local: CloneIn::clone_in(&self.local, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for WithClause<'old_alloc> {
    type Cloned<'a> = WithClause<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        WithClause {
            span: CloneIn::clone_in(&self.span, allocator),
            attributes_keyword: CloneIn::clone_in(&self.attributes_keyword, allocator),
            with_entries: CloneIn::clone_in(&self.with_entries, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportAttribute<'old_alloc> {
    type Cloned<'a> = ImportAttribute<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ImportAttribute {
            span: CloneIn::clone_in(&self.span, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ImportAttributeKey<'old_alloc> {
    type Cloned<'a> = ImportAttributeKey<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => {
                ImportAttributeKey::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                ImportAttributeKey::StringLiteral(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ExportNamedDeclaration<'old_alloc> {
    type Cloned<'a> = ExportNamedDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ExportNamedDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            declaration: CloneIn::clone_in(&self.declaration, allocator),
            specifiers: CloneIn::clone_in(&self.specifiers, allocator),
            source: CloneIn::clone_in(&self.source, allocator),
            export_kind: CloneIn::clone_in(&self.export_kind, allocator),
            with_clause: CloneIn::clone_in(&self.with_clause, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ExportDefaultDeclaration<'old_alloc> {
    type Cloned<'a> = ExportDefaultDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ExportDefaultDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            declaration: CloneIn::clone_in(&self.declaration, allocator),
            exported: CloneIn::clone_in(&self.exported, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ExportAllDeclaration<'old_alloc> {
    type Cloned<'a> = ExportAllDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ExportAllDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            exported: CloneIn::clone_in(&self.exported, allocator),
            source: CloneIn::clone_in(&self.source, allocator),
            with_clause: CloneIn::clone_in(&self.with_clause, allocator),
            export_kind: CloneIn::clone_in(&self.export_kind, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ExportSpecifier<'old_alloc> {
    type Cloned<'a> = ExportSpecifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        ExportSpecifier {
            span: CloneIn::clone_in(&self.span, allocator),
            local: CloneIn::clone_in(&self.local, allocator),
            exported: CloneIn::clone_in(&self.exported, allocator),
            export_kind: CloneIn::clone_in(&self.export_kind, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for ExportDefaultDeclarationKind<'old_alloc> {
    type Cloned<'a> = ExportDefaultDeclarationKind<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::FunctionDeclaration(it) => {
                ExportDefaultDeclarationKind::FunctionDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::ClassDeclaration(it) => {
                ExportDefaultDeclarationKind::ClassDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSInterfaceDeclaration(it) => {
                ExportDefaultDeclarationKind::TSInterfaceDeclaration(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::BooleanLiteral(it) => {
                ExportDefaultDeclarationKind::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => {
                ExportDefaultDeclarationKind::NullLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NumericLiteral(it) => {
                ExportDefaultDeclarationKind::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => {
                ExportDefaultDeclarationKind::BigIntLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::RegExpLiteral(it) => {
                ExportDefaultDeclarationKind::RegExpLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                ExportDefaultDeclarationKind::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TemplateLiteral(it) => {
                ExportDefaultDeclarationKind::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => {
                ExportDefaultDeclarationKind::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::MetaProperty(it) => {
                ExportDefaultDeclarationKind::MetaProperty(CloneIn::clone_in(it, allocator))
            }
            Self::Super(it) => {
                ExportDefaultDeclarationKind::Super(CloneIn::clone_in(it, allocator))
            }
            Self::ArrayExpression(it) => {
                ExportDefaultDeclarationKind::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                ExportDefaultDeclarationKind::ArrowFunctionExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::AssignmentExpression(it) => {
                ExportDefaultDeclarationKind::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                ExportDefaultDeclarationKind::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                ExportDefaultDeclarationKind::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                ExportDefaultDeclarationKind::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                ExportDefaultDeclarationKind::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                ExportDefaultDeclarationKind::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => ExportDefaultDeclarationKind::ConditionalExpression(
                CloneIn::clone_in(it, allocator),
            ),
            Self::FunctionExpression(it) => {
                ExportDefaultDeclarationKind::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                ExportDefaultDeclarationKind::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                ExportDefaultDeclarationKind::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => {
                ExportDefaultDeclarationKind::NewExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectExpression(it) => {
                ExportDefaultDeclarationKind::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                ExportDefaultDeclarationKind::ParenthesizedExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::SequenceExpression(it) => {
                ExportDefaultDeclarationKind::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                ExportDefaultDeclarationKind::TaggedTemplateExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::ThisExpression(it) => {
                ExportDefaultDeclarationKind::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                ExportDefaultDeclarationKind::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                ExportDefaultDeclarationKind::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                ExportDefaultDeclarationKind::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                ExportDefaultDeclarationKind::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => {
                ExportDefaultDeclarationKind::JSXElement(CloneIn::clone_in(it, allocator))
            }
            Self::JSXFragment(it) => {
                ExportDefaultDeclarationKind::JSXFragment(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                ExportDefaultDeclarationKind::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => ExportDefaultDeclarationKind::TSSatisfiesExpression(
                CloneIn::clone_in(it, allocator),
            ),
            Self::TSTypeAssertion(it) => {
                ExportDefaultDeclarationKind::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                ExportDefaultDeclarationKind::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                ExportDefaultDeclarationKind::TSInstantiationExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::ComputedMemberExpression(it) => {
                ExportDefaultDeclarationKind::ComputedMemberExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::StaticMemberExpression(it) => {
                ExportDefaultDeclarationKind::StaticMemberExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
            Self::PrivateFieldExpression(it) => {
                ExportDefaultDeclarationKind::PrivateFieldExpression(CloneIn::clone_in(
                    it, allocator,
                ))
            }
        }
    }
}

impl<'old_alloc> CloneIn for ModuleExportName<'old_alloc> {
    type Cloned<'a> = ModuleExportName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::IdentifierName(it) => {
                ModuleExportName::IdentifierName(CloneIn::clone_in(it, allocator))
            }
            Self::IdentifierReference(it) => {
                ModuleExportName::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                ModuleExportName::StringLiteral(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSThisParameter<'old_alloc> {
    type Cloned<'a> = TSThisParameter<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSThisParameter {
            span: CloneIn::clone_in(&self.span, allocator),
            this_span: CloneIn::clone_in(&self.this_span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSEnumHead<'old_alloc> {
    type Cloned<'a> = TSEnumHead<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSEnumHead {
            span: CloneIn::clone_in(&self.span, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            r#const: CloneIn::clone_in(&self.r#const, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSEnumDeclaration<'old_alloc> {
    type Cloned<'a> = TSEnumDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSEnumDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            head: CloneIn::clone_in(&self.head, allocator),
            members: CloneIn::clone_in(&self.members, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for TSEnumMember<'old_alloc> {
    type Cloned<'a> = TSEnumMember<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSEnumMember {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            initializer: CloneIn::clone_in(&self.initializer, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSEnumMemberName<'old_alloc> {
    type Cloned<'a> = TSEnumMemberName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::StaticIdentifier(it) => {
                TSEnumMemberName::StaticIdentifier(CloneIn::clone_in(it, allocator))
            }
            Self::StaticStringLiteral(it) => {
                TSEnumMemberName::StaticStringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StaticTemplateLiteral(it) => {
                TSEnumMemberName::StaticTemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StaticNumericLiteral(it) => {
                TSEnumMemberName::StaticNumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BooleanLiteral(it) => {
                TSEnumMemberName::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => {
                TSEnumMemberName::NullLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NumericLiteral(it) => {
                TSEnumMemberName::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => {
                TSEnumMemberName::BigIntLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::RegExpLiteral(it) => {
                TSEnumMemberName::RegExpLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                TSEnumMemberName::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TemplateLiteral(it) => {
                TSEnumMemberName::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => TSEnumMemberName::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => {
                TSEnumMemberName::MetaProperty(CloneIn::clone_in(it, allocator))
            }
            Self::Super(it) => TSEnumMemberName::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                TSEnumMemberName::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                TSEnumMemberName::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                TSEnumMemberName::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                TSEnumMemberName::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                TSEnumMemberName::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                TSEnumMemberName::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                TSEnumMemberName::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                TSEnumMemberName::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                TSEnumMemberName::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                TSEnumMemberName::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                TSEnumMemberName::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                TSEnumMemberName::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => {
                TSEnumMemberName::NewExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectExpression(it) => {
                TSEnumMemberName::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                TSEnumMemberName::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                TSEnumMemberName::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                TSEnumMemberName::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                TSEnumMemberName::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                TSEnumMemberName::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                TSEnumMemberName::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                TSEnumMemberName::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                TSEnumMemberName::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => TSEnumMemberName::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => {
                TSEnumMemberName::JSXFragment(CloneIn::clone_in(it, allocator))
            }
            Self::TSAsExpression(it) => {
                TSEnumMemberName::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                TSEnumMemberName::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                TSEnumMemberName::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                TSEnumMemberName::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                TSEnumMemberName::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                TSEnumMemberName::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                TSEnumMemberName::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                TSEnumMemberName::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeAnnotation<'old_alloc> {
    type Cloned<'a> = TSTypeAnnotation<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeAnnotation {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSLiteralType<'old_alloc> {
    type Cloned<'a> = TSLiteralType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSLiteralType {
            span: CloneIn::clone_in(&self.span, allocator),
            literal: CloneIn::clone_in(&self.literal, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSLiteral<'old_alloc> {
    type Cloned<'a> = TSLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::BooleanLiteral(it) => TSLiteral::BooleanLiteral(CloneIn::clone_in(it, allocator)),
            Self::NullLiteral(it) => TSLiteral::NullLiteral(CloneIn::clone_in(it, allocator)),
            Self::NumericLiteral(it) => TSLiteral::NumericLiteral(CloneIn::clone_in(it, allocator)),
            Self::BigIntLiteral(it) => TSLiteral::BigIntLiteral(CloneIn::clone_in(it, allocator)),
            Self::RegExpLiteral(it) => TSLiteral::RegExpLiteral(CloneIn::clone_in(it, allocator)),
            Self::StringLiteral(it) => TSLiteral::StringLiteral(CloneIn::clone_in(it, allocator)),
            Self::TemplateLiteral(it) => {
                TSLiteral::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                TSLiteral::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSType<'old_alloc> {
    type Cloned<'a> = TSType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::TSAnyKeyword(it) => TSType::TSAnyKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSBigIntKeyword(it) => TSType::TSBigIntKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSBooleanKeyword(it) => {
                TSType::TSBooleanKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSIntrinsicKeyword(it) => {
                TSType::TSIntrinsicKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSNeverKeyword(it) => TSType::TSNeverKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSNullKeyword(it) => TSType::TSNullKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSNumberKeyword(it) => TSType::TSNumberKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSObjectKeyword(it) => TSType::TSObjectKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSStringKeyword(it) => TSType::TSStringKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSSymbolKeyword(it) => TSType::TSSymbolKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSUndefinedKeyword(it) => {
                TSType::TSUndefinedKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSUnknownKeyword(it) => {
                TSType::TSUnknownKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSVoidKeyword(it) => TSType::TSVoidKeyword(CloneIn::clone_in(it, allocator)),
            Self::TSArrayType(it) => TSType::TSArrayType(CloneIn::clone_in(it, allocator)),
            Self::TSConditionalType(it) => {
                TSType::TSConditionalType(CloneIn::clone_in(it, allocator))
            }
            Self::TSConstructorType(it) => {
                TSType::TSConstructorType(CloneIn::clone_in(it, allocator))
            }
            Self::TSFunctionType(it) => TSType::TSFunctionType(CloneIn::clone_in(it, allocator)),
            Self::TSImportType(it) => TSType::TSImportType(CloneIn::clone_in(it, allocator)),
            Self::TSIndexedAccessType(it) => {
                TSType::TSIndexedAccessType(CloneIn::clone_in(it, allocator))
            }
            Self::TSInferType(it) => TSType::TSInferType(CloneIn::clone_in(it, allocator)),
            Self::TSIntersectionType(it) => {
                TSType::TSIntersectionType(CloneIn::clone_in(it, allocator))
            }
            Self::TSLiteralType(it) => TSType::TSLiteralType(CloneIn::clone_in(it, allocator)),
            Self::TSMappedType(it) => TSType::TSMappedType(CloneIn::clone_in(it, allocator)),
            Self::TSNamedTupleMember(it) => {
                TSType::TSNamedTupleMember(CloneIn::clone_in(it, allocator))
            }
            Self::TSQualifiedName(it) => TSType::TSQualifiedName(CloneIn::clone_in(it, allocator)),
            Self::TSTemplateLiteralType(it) => {
                TSType::TSTemplateLiteralType(CloneIn::clone_in(it, allocator))
            }
            Self::TSThisType(it) => TSType::TSThisType(CloneIn::clone_in(it, allocator)),
            Self::TSTupleType(it) => TSType::TSTupleType(CloneIn::clone_in(it, allocator)),
            Self::TSTypeLiteral(it) => TSType::TSTypeLiteral(CloneIn::clone_in(it, allocator)),
            Self::TSTypeOperatorType(it) => {
                TSType::TSTypeOperatorType(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypePredicate(it) => TSType::TSTypePredicate(CloneIn::clone_in(it, allocator)),
            Self::TSTypeQuery(it) => TSType::TSTypeQuery(CloneIn::clone_in(it, allocator)),
            Self::TSTypeReference(it) => TSType::TSTypeReference(CloneIn::clone_in(it, allocator)),
            Self::TSUnionType(it) => TSType::TSUnionType(CloneIn::clone_in(it, allocator)),
            Self::TSParenthesizedType(it) => {
                TSType::TSParenthesizedType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocNullableType(it) => {
                TSType::JSDocNullableType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocNonNullableType(it) => {
                TSType::JSDocNonNullableType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocUnknownType(it) => {
                TSType::JSDocUnknownType(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSConditionalType<'old_alloc> {
    type Cloned<'a> = TSConditionalType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSConditionalType {
            span: CloneIn::clone_in(&self.span, allocator),
            check_type: CloneIn::clone_in(&self.check_type, allocator),
            extends_type: CloneIn::clone_in(&self.extends_type, allocator),
            true_type: CloneIn::clone_in(&self.true_type, allocator),
            false_type: CloneIn::clone_in(&self.false_type, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for TSUnionType<'old_alloc> {
    type Cloned<'a> = TSUnionType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSUnionType {
            span: CloneIn::clone_in(&self.span, allocator),
            types: CloneIn::clone_in(&self.types, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSIntersectionType<'old_alloc> {
    type Cloned<'a> = TSIntersectionType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSIntersectionType {
            span: CloneIn::clone_in(&self.span, allocator),
            types: CloneIn::clone_in(&self.types, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSParenthesizedType<'old_alloc> {
    type Cloned<'a> = TSParenthesizedType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSParenthesizedType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeOperator<'old_alloc> {
    type Cloned<'a> = TSTypeOperator<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeOperator {
            span: CloneIn::clone_in(&self.span, allocator),
            operator: CloneIn::clone_in(&self.operator, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl CloneIn for TSTypeOperatorOperator {
    type Cloned<'a> = TSTypeOperatorOperator;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Keyof => TSTypeOperatorOperator::Keyof,
            Self::Unique => TSTypeOperatorOperator::Unique,
            Self::Readonly => TSTypeOperatorOperator::Readonly,
        }
    }
}

impl<'old_alloc> CloneIn for TSArrayType<'old_alloc> {
    type Cloned<'a> = TSArrayType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSArrayType {
            span: CloneIn::clone_in(&self.span, allocator),
            element_type: CloneIn::clone_in(&self.element_type, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSIndexedAccessType<'old_alloc> {
    type Cloned<'a> = TSIndexedAccessType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSIndexedAccessType {
            span: CloneIn::clone_in(&self.span, allocator),
            object_type: CloneIn::clone_in(&self.object_type, allocator),
            index_type: CloneIn::clone_in(&self.index_type, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTupleType<'old_alloc> {
    type Cloned<'a> = TSTupleType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTupleType {
            span: CloneIn::clone_in(&self.span, allocator),
            element_types: CloneIn::clone_in(&self.element_types, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSNamedTupleMember<'old_alloc> {
    type Cloned<'a> = TSNamedTupleMember<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNamedTupleMember {
            span: CloneIn::clone_in(&self.span, allocator),
            element_type: CloneIn::clone_in(&self.element_type, allocator),
            label: CloneIn::clone_in(&self.label, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSOptionalType<'old_alloc> {
    type Cloned<'a> = TSOptionalType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSOptionalType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSRestType<'old_alloc> {
    type Cloned<'a> = TSRestType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSRestType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTupleElement<'old_alloc> {
    type Cloned<'a> = TSTupleElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::TSOptionalType(it) => {
                TSTupleElement::TSOptionalType(CloneIn::clone_in(it, allocator))
            }
            Self::TSRestType(it) => TSTupleElement::TSRestType(CloneIn::clone_in(it, allocator)),
            Self::TSAnyKeyword(it) => {
                TSTupleElement::TSAnyKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSBigIntKeyword(it) => {
                TSTupleElement::TSBigIntKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSBooleanKeyword(it) => {
                TSTupleElement::TSBooleanKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSIntrinsicKeyword(it) => {
                TSTupleElement::TSIntrinsicKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSNeverKeyword(it) => {
                TSTupleElement::TSNeverKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSNullKeyword(it) => {
                TSTupleElement::TSNullKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSNumberKeyword(it) => {
                TSTupleElement::TSNumberKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSObjectKeyword(it) => {
                TSTupleElement::TSObjectKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSStringKeyword(it) => {
                TSTupleElement::TSStringKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSSymbolKeyword(it) => {
                TSTupleElement::TSSymbolKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSUndefinedKeyword(it) => {
                TSTupleElement::TSUndefinedKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSUnknownKeyword(it) => {
                TSTupleElement::TSUnknownKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSVoidKeyword(it) => {
                TSTupleElement::TSVoidKeyword(CloneIn::clone_in(it, allocator))
            }
            Self::TSArrayType(it) => TSTupleElement::TSArrayType(CloneIn::clone_in(it, allocator)),
            Self::TSConditionalType(it) => {
                TSTupleElement::TSConditionalType(CloneIn::clone_in(it, allocator))
            }
            Self::TSConstructorType(it) => {
                TSTupleElement::TSConstructorType(CloneIn::clone_in(it, allocator))
            }
            Self::TSFunctionType(it) => {
                TSTupleElement::TSFunctionType(CloneIn::clone_in(it, allocator))
            }
            Self::TSImportType(it) => {
                TSTupleElement::TSImportType(CloneIn::clone_in(it, allocator))
            }
            Self::TSIndexedAccessType(it) => {
                TSTupleElement::TSIndexedAccessType(CloneIn::clone_in(it, allocator))
            }
            Self::TSInferType(it) => TSTupleElement::TSInferType(CloneIn::clone_in(it, allocator)),
            Self::TSIntersectionType(it) => {
                TSTupleElement::TSIntersectionType(CloneIn::clone_in(it, allocator))
            }
            Self::TSLiteralType(it) => {
                TSTupleElement::TSLiteralType(CloneIn::clone_in(it, allocator))
            }
            Self::TSMappedType(it) => {
                TSTupleElement::TSMappedType(CloneIn::clone_in(it, allocator))
            }
            Self::TSNamedTupleMember(it) => {
                TSTupleElement::TSNamedTupleMember(CloneIn::clone_in(it, allocator))
            }
            Self::TSQualifiedName(it) => {
                TSTupleElement::TSQualifiedName(CloneIn::clone_in(it, allocator))
            }
            Self::TSTemplateLiteralType(it) => {
                TSTupleElement::TSTemplateLiteralType(CloneIn::clone_in(it, allocator))
            }
            Self::TSThisType(it) => TSTupleElement::TSThisType(CloneIn::clone_in(it, allocator)),
            Self::TSTupleType(it) => TSTupleElement::TSTupleType(CloneIn::clone_in(it, allocator)),
            Self::TSTypeLiteral(it) => {
                TSTupleElement::TSTypeLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeOperatorType(it) => {
                TSTupleElement::TSTypeOperatorType(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypePredicate(it) => {
                TSTupleElement::TSTypePredicate(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeQuery(it) => TSTupleElement::TSTypeQuery(CloneIn::clone_in(it, allocator)),
            Self::TSTypeReference(it) => {
                TSTupleElement::TSTypeReference(CloneIn::clone_in(it, allocator))
            }
            Self::TSUnionType(it) => TSTupleElement::TSUnionType(CloneIn::clone_in(it, allocator)),
            Self::TSParenthesizedType(it) => {
                TSTupleElement::TSParenthesizedType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocNullableType(it) => {
                TSTupleElement::JSDocNullableType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocNonNullableType(it) => {
                TSTupleElement::JSDocNonNullableType(CloneIn::clone_in(it, allocator))
            }
            Self::JSDocUnknownType(it) => {
                TSTupleElement::JSDocUnknownType(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for TSAnyKeyword {
    type Cloned<'a> = TSAnyKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSAnyKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSStringKeyword {
    type Cloned<'a> = TSStringKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSStringKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSBooleanKeyword {
    type Cloned<'a> = TSBooleanKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSBooleanKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSNumberKeyword {
    type Cloned<'a> = TSNumberKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNumberKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSNeverKeyword {
    type Cloned<'a> = TSNeverKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNeverKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSIntrinsicKeyword {
    type Cloned<'a> = TSIntrinsicKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSIntrinsicKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSUnknownKeyword {
    type Cloned<'a> = TSUnknownKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSUnknownKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSNullKeyword {
    type Cloned<'a> = TSNullKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNullKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSUndefinedKeyword {
    type Cloned<'a> = TSUndefinedKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSUndefinedKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSVoidKeyword {
    type Cloned<'a> = TSVoidKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSVoidKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSSymbolKeyword {
    type Cloned<'a> = TSSymbolKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSSymbolKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSThisType {
    type Cloned<'a> = TSThisType;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSThisType { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSObjectKeyword {
    type Cloned<'a> = TSObjectKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSObjectKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSBigIntKeyword {
    type Cloned<'a> = TSBigIntKeyword;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSBigIntKeyword { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for TSTypeReference<'old_alloc> {
    type Cloned<'a> = TSTypeReference<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeReference {
            span: CloneIn::clone_in(&self.span, allocator),
            type_name: CloneIn::clone_in(&self.type_name, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeName<'old_alloc> {
    type Cloned<'a> = TSTypeName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::IdentifierReference(it) => {
                TSTypeName::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::QualifiedName(it) => TSTypeName::QualifiedName(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl<'old_alloc> CloneIn for TSQualifiedName<'old_alloc> {
    type Cloned<'a> = TSQualifiedName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSQualifiedName {
            span: CloneIn::clone_in(&self.span, allocator),
            left: CloneIn::clone_in(&self.left, allocator),
            right: CloneIn::clone_in(&self.right, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeParameterInstantiation<'old_alloc> {
    type Cloned<'a> = TSTypeParameterInstantiation<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeParameterInstantiation {
            span: CloneIn::clone_in(&self.span, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeParameter<'old_alloc> {
    type Cloned<'a> = TSTypeParameter<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeParameter {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            constraint: CloneIn::clone_in(&self.constraint, allocator),
            default: CloneIn::clone_in(&self.default, allocator),
            r#in: CloneIn::clone_in(&self.r#in, allocator),
            out: CloneIn::clone_in(&self.out, allocator),
            r#const: CloneIn::clone_in(&self.r#const, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeParameterDeclaration<'old_alloc> {
    type Cloned<'a> = TSTypeParameterDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeParameterDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeAliasDeclaration<'old_alloc> {
    type Cloned<'a> = TSTypeAliasDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeAliasDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            scope_id: Default::default(),
        }
    }
}

impl CloneIn for TSAccessibility {
    type Cloned<'a> = TSAccessibility;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Private => TSAccessibility::Private,
            Self::Protected => TSAccessibility::Protected,
            Self::Public => TSAccessibility::Public,
        }
    }
}

impl<'old_alloc> CloneIn for TSClassImplementsItem<'old_alloc> {
    type Cloned<'a> = TSClassImplementsItem<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSClassImplementsItem {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSClassImplements<'old_alloc> {
    type Cloned<'a> = TSClassImplements<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSClassImplements {
            span: CloneIn::clone_in(&self.span, allocator),
            items: CloneIn::clone_in(&self.items, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSInterfaceDeclaration<'old_alloc> {
    type Cloned<'a> = TSInterfaceDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSInterfaceDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            extends: CloneIn::clone_in(&self.extends, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for TSInterfaceBody<'old_alloc> {
    type Cloned<'a> = TSInterfaceBody<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSInterfaceBody {
            span: CloneIn::clone_in(&self.span, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSPropertySignature<'old_alloc> {
    type Cloned<'a> = TSPropertySignature<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSPropertySignature {
            span: CloneIn::clone_in(&self.span, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
            readonly: CloneIn::clone_in(&self.readonly, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSSignature<'old_alloc> {
    type Cloned<'a> = TSSignature<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::TSIndexSignature(it) => {
                TSSignature::TSIndexSignature(CloneIn::clone_in(it, allocator))
            }
            Self::TSPropertySignature(it) => {
                TSSignature::TSPropertySignature(CloneIn::clone_in(it, allocator))
            }
            Self::TSCallSignatureDeclaration(it) => {
                TSSignature::TSCallSignatureDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSConstructSignatureDeclaration(it) => {
                TSSignature::TSConstructSignatureDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSMethodSignature(it) => {
                TSSignature::TSMethodSignature(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSIndexSignature<'old_alloc> {
    type Cloned<'a> = TSIndexSignature<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSIndexSignature {
            span: CloneIn::clone_in(&self.span, allocator),
            modifiers: CloneIn::clone_in(&self.modifiers, allocator),
            parameters: CloneIn::clone_in(&self.parameters, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSCallSignatureDeclaration<'old_alloc> {
    type Cloned<'a> = TSCallSignatureDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSCallSignatureDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            this_param: CloneIn::clone_in(&self.this_param, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
        }
    }
}

impl CloneIn for TSMethodSignatureKind {
    type Cloned<'a> = TSMethodSignatureKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Method => TSMethodSignatureKind::Method,
            Self::Get => TSMethodSignatureKind::Get,
            Self::Set => TSMethodSignatureKind::Set,
        }
    }
}

impl<'old_alloc> CloneIn for TSMethodSignature<'old_alloc> {
    type Cloned<'a> = TSMethodSignature<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSMethodSignature {
            span: CloneIn::clone_in(&self.span, allocator),
            key: CloneIn::clone_in(&self.key, allocator),
            computed: CloneIn::clone_in(&self.computed, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            this_param: CloneIn::clone_in(&self.this_param, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for TSConstructSignatureDeclaration<'old_alloc> {
    type Cloned<'a> = TSConstructSignatureDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSConstructSignatureDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
            scope_id: Default::default(),
        }
    }
}

impl<'old_alloc> CloneIn for TSIndexSignatureName<'old_alloc> {
    type Cloned<'a> = TSIndexSignatureName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSIndexSignatureName {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSInterfaceHeritage<'old_alloc> {
    type Cloned<'a> = TSInterfaceHeritage<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSInterfaceHeritage {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypePredicate<'old_alloc> {
    type Cloned<'a> = TSTypePredicate<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypePredicate {
            span: CloneIn::clone_in(&self.span, allocator),
            parameter_name: CloneIn::clone_in(&self.parameter_name, allocator),
            asserts: CloneIn::clone_in(&self.asserts, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypePredicateName<'old_alloc> {
    type Cloned<'a> = TSTypePredicateName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => {
                TSTypePredicateName::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::This(it) => TSTypePredicateName::This(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl<'old_alloc> CloneIn for TSModuleDeclaration<'old_alloc> {
    type Cloned<'a> = TSModuleDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSModuleDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            declare: CloneIn::clone_in(&self.declare, allocator),
            scope_id: Default::default(),
        }
    }
}

impl CloneIn for TSModuleDeclarationKind {
    type Cloned<'a> = TSModuleDeclarationKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Global => TSModuleDeclarationKind::Global,
            Self::Module => TSModuleDeclarationKind::Module,
            Self::Namespace => TSModuleDeclarationKind::Namespace,
        }
    }
}

impl<'old_alloc> CloneIn for TSModuleDeclarationName<'old_alloc> {
    type Cloned<'a> = TSModuleDeclarationName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => {
                TSModuleDeclarationName::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                TSModuleDeclarationName::StringLiteral(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSModuleDeclarationBody<'old_alloc> {
    type Cloned<'a> = TSModuleDeclarationBody<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::TSModuleDeclaration(it) => {
                TSModuleDeclarationBody::TSModuleDeclaration(CloneIn::clone_in(it, allocator))
            }
            Self::TSModuleBlock(it) => {
                TSModuleDeclarationBody::TSModuleBlock(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSModuleBlock<'old_alloc> {
    type Cloned<'a> = TSModuleBlock<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSModuleBlock {
            span: CloneIn::clone_in(&self.span, allocator),
            directives: CloneIn::clone_in(&self.directives, allocator),
            body: CloneIn::clone_in(&self.body, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeLiteral<'old_alloc> {
    type Cloned<'a> = TSTypeLiteral<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeLiteral {
            span: CloneIn::clone_in(&self.span, allocator),
            members: CloneIn::clone_in(&self.members, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSInferType<'old_alloc> {
    type Cloned<'a> = TSInferType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSInferType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_parameter: CloneIn::clone_in(&self.type_parameter, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeQuery<'old_alloc> {
    type Cloned<'a> = TSTypeQuery<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeQuery {
            span: CloneIn::clone_in(&self.span, allocator),
            expr_name: CloneIn::clone_in(&self.expr_name, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeQueryExprName<'old_alloc> {
    type Cloned<'a> = TSTypeQueryExprName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::TSImportType(it) => {
                TSTypeQueryExprName::TSImportType(CloneIn::clone_in(it, allocator))
            }
            Self::IdentifierReference(it) => {
                TSTypeQueryExprName::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::QualifiedName(it) => {
                TSTypeQueryExprName::QualifiedName(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSImportType<'old_alloc> {
    type Cloned<'a> = TSImportType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSImportType {
            span: CloneIn::clone_in(&self.span, allocator),
            is_type_of: CloneIn::clone_in(&self.is_type_of, allocator),
            parameter: CloneIn::clone_in(&self.parameter, allocator),
            qualifier: CloneIn::clone_in(&self.qualifier, allocator),
            attributes: CloneIn::clone_in(&self.attributes, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSImportAttributes<'old_alloc> {
    type Cloned<'a> = TSImportAttributes<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSImportAttributes {
            span: CloneIn::clone_in(&self.span, allocator),
            attributes_keyword: CloneIn::clone_in(&self.attributes_keyword, allocator),
            elements: CloneIn::clone_in(&self.elements, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSImportAttribute<'old_alloc> {
    type Cloned<'a> = TSImportAttribute<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSImportAttribute {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSImportAttributeName<'old_alloc> {
    type Cloned<'a> = TSImportAttributeName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => {
                TSImportAttributeName::Identifier(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                TSImportAttributeName::StringLiteral(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSFunctionType<'old_alloc> {
    type Cloned<'a> = TSFunctionType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSFunctionType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            this_param: CloneIn::clone_in(&self.this_param, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSConstructorType<'old_alloc> {
    type Cloned<'a> = TSConstructorType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSConstructorType {
            span: CloneIn::clone_in(&self.span, allocator),
            r#abstract: CloneIn::clone_in(&self.r#abstract, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
            params: CloneIn::clone_in(&self.params, allocator),
            return_type: CloneIn::clone_in(&self.return_type, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSMappedType<'old_alloc> {
    type Cloned<'a> = TSMappedType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSMappedType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_parameter: CloneIn::clone_in(&self.type_parameter, allocator),
            name_type: CloneIn::clone_in(&self.name_type, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            optional: CloneIn::clone_in(&self.optional, allocator),
            readonly: CloneIn::clone_in(&self.readonly, allocator),
            scope_id: Default::default(),
        }
    }
}

impl CloneIn for TSMappedTypeModifierOperator {
    type Cloned<'a> = TSMappedTypeModifierOperator;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::True => TSMappedTypeModifierOperator::True,
            Self::Plus => TSMappedTypeModifierOperator::Plus,
            Self::Minus => TSMappedTypeModifierOperator::Minus,
            Self::None => TSMappedTypeModifierOperator::None,
        }
    }
}

impl<'old_alloc> CloneIn for TSTemplateLiteralType<'old_alloc> {
    type Cloned<'a> = TSTemplateLiteralType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTemplateLiteralType {
            span: CloneIn::clone_in(&self.span, allocator),
            quasis: CloneIn::clone_in(&self.quasis, allocator),
            types: CloneIn::clone_in(&self.types, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSAsExpression<'old_alloc> {
    type Cloned<'a> = TSAsExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSAsExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSSatisfiesExpression<'old_alloc> {
    type Cloned<'a> = TSSatisfiesExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSSatisfiesExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeAssertionAnnotation<'old_alloc> {
    type Cloned<'a> = TSTypeAssertionAnnotation<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeAssertionAnnotation {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSTypeAssertion<'old_alloc> {
    type Cloned<'a> = TSTypeAssertion<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSTypeAssertion {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSImportEqualsDeclaration<'old_alloc> {
    type Cloned<'a> = TSImportEqualsDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSImportEqualsDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
            module_reference: CloneIn::clone_in(&self.module_reference, allocator),
            import_kind: CloneIn::clone_in(&self.import_kind, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSModuleReference<'old_alloc> {
    type Cloned<'a> = TSModuleReference<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::ExternalModuleReference(it) => {
                TSModuleReference::ExternalModuleReference(CloneIn::clone_in(it, allocator))
            }
            Self::IdentifierReference(it) => {
                TSModuleReference::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::QualifiedName(it) => {
                TSModuleReference::QualifiedName(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for TSExternalModuleReference<'old_alloc> {
    type Cloned<'a> = TSExternalModuleReference<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSExternalModuleReference {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSNonNullExpression<'old_alloc> {
    type Cloned<'a> = TSNonNullExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNonNullExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            definite_mark: CloneIn::clone_in(&self.definite_mark, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for Decorator<'old_alloc> {
    type Cloned<'a> = Decorator<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Decorator {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSExportAssignment<'old_alloc> {
    type Cloned<'a> = TSExportAssignment<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSExportAssignment {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSNamespaceExportDeclaration<'old_alloc> {
    type Cloned<'a> = TSNamespaceExportDeclaration<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSNamespaceExportDeclaration {
            span: CloneIn::clone_in(&self.span, allocator),
            id: CloneIn::clone_in(&self.id, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for TSInstantiationExpression<'old_alloc> {
    type Cloned<'a> = TSInstantiationExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSInstantiationExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl CloneIn for ImportOrExportKind {
    type Cloned<'a> = ImportOrExportKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Value => ImportOrExportKind::Value,
            Self::Type => ImportOrExportKind::Type,
        }
    }
}

impl CloneIn for TSOptionalMark {
    type Cloned<'a> = TSOptionalMark;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSOptionalMark { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for TSDefiniteMark {
    type Cloned<'a> = TSDefiniteMark;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        TSDefiniteMark { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for JSDocNullableType<'old_alloc> {
    type Cloned<'a> = JSDocNullableType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSDocNullableType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            postfix: CloneIn::clone_in(&self.postfix, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSDocNonNullableType<'old_alloc> {
    type Cloned<'a> = JSDocNonNullableType<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSDocNonNullableType {
            span: CloneIn::clone_in(&self.span, allocator),
            type_annotation: CloneIn::clone_in(&self.type_annotation, allocator),
            postfix: CloneIn::clone_in(&self.postfix, allocator),
        }
    }
}

impl CloneIn for JSDocUnknownType {
    type Cloned<'a> = JSDocUnknownType;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSDocUnknownType { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for JSXElement<'old_alloc> {
    type Cloned<'a> = JSXElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXElement {
            span: CloneIn::clone_in(&self.span, allocator),
            opening_element: CloneIn::clone_in(&self.opening_element, allocator),
            closing_element: CloneIn::clone_in(&self.closing_element, allocator),
            children: CloneIn::clone_in(&self.children, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXOpeningElement<'old_alloc> {
    type Cloned<'a> = JSXOpeningElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXOpeningElement {
            span: CloneIn::clone_in(&self.span, allocator),
            self_closing: CloneIn::clone_in(&self.self_closing, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            attributes: CloneIn::clone_in(&self.attributes, allocator),
            type_parameters: CloneIn::clone_in(&self.type_parameters, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXClosingElement<'old_alloc> {
    type Cloned<'a> = JSXClosingElement<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXClosingElement {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXFragment<'old_alloc> {
    type Cloned<'a> = JSXFragment<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXFragment {
            span: CloneIn::clone_in(&self.span, allocator),
            opening_fragment: CloneIn::clone_in(&self.opening_fragment, allocator),
            closing_fragment: CloneIn::clone_in(&self.closing_fragment, allocator),
            children: CloneIn::clone_in(&self.children, allocator),
        }
    }
}

impl CloneIn for JSXOpeningFragment {
    type Cloned<'a> = JSXOpeningFragment;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXOpeningFragment { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl CloneIn for JSXClosingFragment {
    type Cloned<'a> = JSXClosingFragment;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXClosingFragment { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for JSXElementName<'old_alloc> {
    type Cloned<'a> = JSXElementName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => JSXElementName::Identifier(CloneIn::clone_in(it, allocator)),
            Self::IdentifierReference(it) => {
                JSXElementName::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::NamespacedName(it) => {
                JSXElementName::NamespacedName(CloneIn::clone_in(it, allocator))
            }
            Self::MemberExpression(it) => {
                JSXElementName::MemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                JSXElementName::ThisExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for JSXNamespacedName<'old_alloc> {
    type Cloned<'a> = JSXNamespacedName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXNamespacedName {
            span: CloneIn::clone_in(&self.span, allocator),
            namespace: CloneIn::clone_in(&self.namespace, allocator),
            property: CloneIn::clone_in(&self.property, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXMemberExpression<'old_alloc> {
    type Cloned<'a> = JSXMemberExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXMemberExpression {
            span: CloneIn::clone_in(&self.span, allocator),
            object: CloneIn::clone_in(&self.object, allocator),
            property: CloneIn::clone_in(&self.property, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXMemberExpressionObject<'old_alloc> {
    type Cloned<'a> = JSXMemberExpressionObject<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::IdentifierReference(it) => {
                JSXMemberExpressionObject::IdentifierReference(CloneIn::clone_in(it, allocator))
            }
            Self::MemberExpression(it) => {
                JSXMemberExpressionObject::MemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                JSXMemberExpressionObject::ThisExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for JSXExpressionContainer<'old_alloc> {
    type Cloned<'a> = JSXExpressionContainer<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXExpressionContainer {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXExpression<'old_alloc> {
    type Cloned<'a> = JSXExpression<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::EmptyExpression(it) => {
                JSXExpression::EmptyExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BooleanLiteral(it) => {
                JSXExpression::BooleanLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::NullLiteral(it) => JSXExpression::NullLiteral(CloneIn::clone_in(it, allocator)),
            Self::NumericLiteral(it) => {
                JSXExpression::NumericLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::BigIntLiteral(it) => {
                JSXExpression::BigIntLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::RegExpLiteral(it) => {
                JSXExpression::RegExpLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::StringLiteral(it) => {
                JSXExpression::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::TemplateLiteral(it) => {
                JSXExpression::TemplateLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::Identifier(it) => JSXExpression::Identifier(CloneIn::clone_in(it, allocator)),
            Self::MetaProperty(it) => JSXExpression::MetaProperty(CloneIn::clone_in(it, allocator)),
            Self::Super(it) => JSXExpression::Super(CloneIn::clone_in(it, allocator)),
            Self::ArrayExpression(it) => {
                JSXExpression::ArrayExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ArrowFunctionExpression(it) => {
                JSXExpression::ArrowFunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AssignmentExpression(it) => {
                JSXExpression::AssignmentExpression(CloneIn::clone_in(it, allocator))
            }
            Self::AwaitExpression(it) => {
                JSXExpression::AwaitExpression(CloneIn::clone_in(it, allocator))
            }
            Self::BinaryExpression(it) => {
                JSXExpression::BinaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::CallExpression(it) => {
                JSXExpression::CallExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ChainExpression(it) => {
                JSXExpression::ChainExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ClassExpression(it) => {
                JSXExpression::ClassExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ConditionalExpression(it) => {
                JSXExpression::ConditionalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::FunctionExpression(it) => {
                JSXExpression::FunctionExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ImportExpression(it) => {
                JSXExpression::ImportExpression(CloneIn::clone_in(it, allocator))
            }
            Self::LogicalExpression(it) => {
                JSXExpression::LogicalExpression(CloneIn::clone_in(it, allocator))
            }
            Self::NewExpression(it) => {
                JSXExpression::NewExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ObjectExpression(it) => {
                JSXExpression::ObjectExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ParenthesizedExpression(it) => {
                JSXExpression::ParenthesizedExpression(CloneIn::clone_in(it, allocator))
            }
            Self::SequenceExpression(it) => {
                JSXExpression::SequenceExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TaggedTemplateExpression(it) => {
                JSXExpression::TaggedTemplateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ThisExpression(it) => {
                JSXExpression::ThisExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UnaryExpression(it) => {
                JSXExpression::UnaryExpression(CloneIn::clone_in(it, allocator))
            }
            Self::UpdateExpression(it) => {
                JSXExpression::UpdateExpression(CloneIn::clone_in(it, allocator))
            }
            Self::YieldExpression(it) => {
                JSXExpression::YieldExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateInExpression(it) => {
                JSXExpression::PrivateInExpression(CloneIn::clone_in(it, allocator))
            }
            Self::JSXElement(it) => JSXExpression::JSXElement(CloneIn::clone_in(it, allocator)),
            Self::JSXFragment(it) => JSXExpression::JSXFragment(CloneIn::clone_in(it, allocator)),
            Self::TSAsExpression(it) => {
                JSXExpression::TSAsExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSSatisfiesExpression(it) => {
                JSXExpression::TSSatisfiesExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSTypeAssertion(it) => {
                JSXExpression::TSTypeAssertion(CloneIn::clone_in(it, allocator))
            }
            Self::TSNonNullExpression(it) => {
                JSXExpression::TSNonNullExpression(CloneIn::clone_in(it, allocator))
            }
            Self::TSInstantiationExpression(it) => {
                JSXExpression::TSInstantiationExpression(CloneIn::clone_in(it, allocator))
            }
            Self::ComputedMemberExpression(it) => {
                JSXExpression::ComputedMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::StaticMemberExpression(it) => {
                JSXExpression::StaticMemberExpression(CloneIn::clone_in(it, allocator))
            }
            Self::PrivateFieldExpression(it) => {
                JSXExpression::PrivateFieldExpression(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl CloneIn for JSXEmptyExpression {
    type Cloned<'a> = JSXEmptyExpression;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXEmptyExpression { span: CloneIn::clone_in(&self.span, allocator) }
    }
}

impl<'old_alloc> CloneIn for JSXAttributeItem<'old_alloc> {
    type Cloned<'a> = JSXAttributeItem<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Attribute(it) => JSXAttributeItem::Attribute(CloneIn::clone_in(it, allocator)),
            Self::SpreadAttribute(it) => {
                JSXAttributeItem::SpreadAttribute(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for JSXAttribute<'old_alloc> {
    type Cloned<'a> = JSXAttribute<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXAttribute {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXSpreadAttribute<'old_alloc> {
    type Cloned<'a> = JSXSpreadAttribute<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXSpreadAttribute {
            span: CloneIn::clone_in(&self.span, allocator),
            argument: CloneIn::clone_in(&self.argument, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXAttributeName<'old_alloc> {
    type Cloned<'a> = JSXAttributeName<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Identifier(it) => JSXAttributeName::Identifier(CloneIn::clone_in(it, allocator)),
            Self::NamespacedName(it) => {
                JSXAttributeName::NamespacedName(CloneIn::clone_in(it, allocator))
            }
        }
    }
}

impl<'old_alloc> CloneIn for JSXAttributeValue<'old_alloc> {
    type Cloned<'a> = JSXAttributeValue<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::StringLiteral(it) => {
                JSXAttributeValue::StringLiteral(CloneIn::clone_in(it, allocator))
            }
            Self::ExpressionContainer(it) => {
                JSXAttributeValue::ExpressionContainer(CloneIn::clone_in(it, allocator))
            }
            Self::Element(it) => JSXAttributeValue::Element(CloneIn::clone_in(it, allocator)),
            Self::Fragment(it) => JSXAttributeValue::Fragment(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl<'old_alloc> CloneIn for JSXIdentifier<'old_alloc> {
    type Cloned<'a> = JSXIdentifier<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXIdentifier {
            span: CloneIn::clone_in(&self.span, allocator),
            name: CloneIn::clone_in(&self.name, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXChild<'old_alloc> {
    type Cloned<'a> = JSXChild<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Text(it) => JSXChild::Text(CloneIn::clone_in(it, allocator)),
            Self::Element(it) => JSXChild::Element(CloneIn::clone_in(it, allocator)),
            Self::Fragment(it) => JSXChild::Fragment(CloneIn::clone_in(it, allocator)),
            Self::ExpressionContainer(it) => {
                JSXChild::ExpressionContainer(CloneIn::clone_in(it, allocator))
            }
            Self::Spread(it) => JSXChild::Spread(CloneIn::clone_in(it, allocator)),
        }
    }
}

impl<'old_alloc> CloneIn for JSXSpreadChild<'old_alloc> {
    type Cloned<'a> = JSXSpreadChild<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXSpreadChild {
            span: CloneIn::clone_in(&self.span, allocator),
            expression: CloneIn::clone_in(&self.expression, allocator),
        }
    }
}

impl<'old_alloc> CloneIn for JSXText<'old_alloc> {
    type Cloned<'a> = JSXText<'a>;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        JSXText {
            span: CloneIn::clone_in(&self.span, allocator),
            value: CloneIn::clone_in(&self.value, allocator),
        }
    }
}

impl CloneIn for CommentKind {
    type Cloned<'a> = CommentKind;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Line => CommentKind::Line,
            Self::Block => CommentKind::Block,
        }
    }
}

impl CloneIn for CommentPosition {
    type Cloned<'a> = CommentPosition;
    fn clone_in<'new_alloc>(&self, _: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        match self {
            Self::Leading => CommentPosition::Leading,
            Self::Trailing => CommentPosition::Trailing,
        }
    }
}

impl CloneIn for Comment {
    type Cloned<'a> = Comment;
    fn clone_in<'new_alloc>(&self, allocator: &'new_alloc Allocator) -> Self::Cloned<'new_alloc> {
        Comment {
            span: CloneIn::clone_in(&self.span, allocator),
            kind: CloneIn::clone_in(&self.kind, allocator),
            position: CloneIn::clone_in(&self.position, allocator),
            attached_to: CloneIn::clone_in(&self.attached_to, allocator),
            preceded_by_newline: CloneIn::clone_in(&self.preceded_by_newline, allocator),
            followed_by_newline: CloneIn::clone_in(&self.followed_by_newline, allocator),
        }
    }
}
