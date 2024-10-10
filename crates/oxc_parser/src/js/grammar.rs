//! Cover Grammar for Destructuring Assignment

use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_ast::AstBuilder;
use oxc_diagnostics::{OxcDiagnostic, Result};
use oxc_span::ast_alloc::AstAllocator;
use oxc_span::{cast, GetSpan};

use crate::{diagnostics, Handler, ParserImpl};

pub trait CoverContext<'a> {
    fn ast(&self) -> AstBuilder<'a>;
    fn unexpected(&mut self) -> OxcDiagnostic;
    fn error(&mut self, diagnostic: OxcDiagnostic);
}
impl<'a, A: AstAllocator, H: Handler<'a, A>> CoverContext<'a> for ParserImpl<'a, H, A> {
    fn ast(&self) -> AstBuilder<'a> {
        AstBuilder::new(self.lexer.allocator)
    }
    fn unexpected(&mut self) -> OxcDiagnostic {
        ParserImpl::unexpected(self)
    }
    fn error(&mut self, diagnostic: OxcDiagnostic) {
        ParserImpl::error(self, diagnostic)
    }
}

pub trait CoverGrammar<'a, T>: Sized {
    fn cover<C: CoverContext<'a>>(value: T, p: &mut C) -> Result<Self>;
}

impl<'a, A: AstAllocator> CoverGrammar<'a, Expression<'a, A>> for AssignmentTarget<'a, A> {
    fn cover<C: CoverContext<'a>>(expr: Expression<'a, A>, ctx: &mut C) -> Result<Self> {
        let ast = ctx.ast();
        let expr = match cast!(expr, Expression<'a, A as Allocator>) {
            Ok(ok) => ok,
            Err(expr_void) => {
                return Ok(AssignmentTarget::AssignmentTargetIdentifier(
                    A::box_from_span(expr_void.span()).unwrap(),
                ))
            }
        };
        let assignment_target = match expr {
            Expression::ArrayExpression(array_expr) => {
                ArrayAssignmentTarget::cover(array_expr.unbox(), ctx)
                    .map(|pat| ast.alloc(pat))
                    .map(AssignmentTarget::ArrayAssignmentTarget)
            }
            Expression::ObjectExpression(object_expr) => {
                ObjectAssignmentTarget::cover(object_expr.unbox(), ctx)
                    .map(|pat| ast.alloc(pat))
                    .map(AssignmentTarget::ObjectAssignmentTarget)
            }
            _ => SimpleAssignmentTarget::cover(expr, ctx).map(AssignmentTarget::from),
        }?;
        Ok(cast!(assignment_target, AssignmentTarget<'a, Allocator as A>).unwrap())
    }
}

impl<'a, A: AstAllocator> CoverGrammar<'a, Expression<'a, A>> for SimpleAssignmentTarget<'a, A> {
    #[allow(clippy::only_used_in_recursion)]
    fn cover<C: CoverContext<'a>>(expr: Expression<'a, A>, p: &mut C) -> Result<Self> {
        let expr = match cast!(expr, Expression<'a, A as Allocator>) {
            Ok(ok) => ok,
            Err(expr_void) => return todo!(),
        };
        let assignment_target = match expr {
            Expression::Identifier(ident) => {
                Ok(SimpleAssignmentTarget::AssignmentTargetIdentifier(ident))
            }
            match_member_expression!(Expression) => {
                let member_expr = MemberExpression::try_from(expr).unwrap();
                Ok(SimpleAssignmentTarget::from(member_expr))
            }
            Expression::ParenthesizedExpression(expr) => {
                let span = expr.span;
                match expr.unbox().expression {
                    Expression::ObjectExpression(_) | Expression::ArrayExpression(_) => {
                        Err(diagnostics::invalid_assignment(span))
                    }
                    expr => SimpleAssignmentTarget::cover(expr, p),
                }
            }
            Expression::TSAsExpression(expr) => Ok(SimpleAssignmentTarget::TSAsExpression(expr)),
            Expression::TSSatisfiesExpression(expr) => {
                Ok(SimpleAssignmentTarget::TSSatisfiesExpression(expr))
            }
            Expression::TSNonNullExpression(expr) => {
                Ok(SimpleAssignmentTarget::TSNonNullExpression(expr))
            }
            Expression::TSTypeAssertion(expr) => Ok(SimpleAssignmentTarget::TSTypeAssertion(expr)),
            Expression::TSInstantiationExpression(expr) => {
                Ok(SimpleAssignmentTarget::TSInstantiationExpression(expr))
            }
            expr => Err(diagnostics::invalid_assignment(expr.span())),
        }?;
        Ok(cast!(assignment_target, SimpleAssignmentTarget<'a, Allocator as A>).unwrap())
    }
}

impl<'a> CoverGrammar<'a, ArrayExpression<'a>> for ArrayAssignmentTarget<'a> {
    fn cover<C: CoverContext<'a>>(expr: ArrayExpression<'a>, ctx: &mut C) -> Result<Self> {
        let ast = ctx.ast();
        let mut elements = ast.vec();
        let mut rest = None;

        let len = expr.elements.len();
        for (i, elem) in expr.elements.into_iter().enumerate() {
            match elem {
                match_expression!(ArrayExpressionElement) => {
                    let expr = Expression::try_from(elem).unwrap();
                    let target = AssignmentTargetMaybeDefault::cover(expr, ctx)?;
                    elements.push(Some(target));
                }
                ArrayExpressionElement::SpreadElement(elem) => {
                    if i == len - 1 {
                        rest = Some(ast.assignment_target_rest(
                            elem.span,
                            AssignmentTarget::cover(elem.unbox().argument, ctx)?,
                        ));
                        if let Some(span) = expr.trailing_comma {
                            ctx.error(diagnostics::binding_rest_element_trailing_comma(span));
                        }
                    } else {
                        return Err(diagnostics::spread_last_element(elem.span));
                    }
                }
                ArrayExpressionElement::Elision(_) => elements.push(None),
            }
        }

        Ok(ast.array_assignment_target(expr.span, elements, rest, expr.trailing_comma))
    }
}

impl<'a> CoverGrammar<'a, Expression<'a>> for AssignmentTargetMaybeDefault<'a> {
    fn cover<C: CoverContext<'a>>(expr: Expression<'a>, ctx: &mut C) -> Result<Self> {
        let ast = ctx.ast();
        match expr {
            Expression::AssignmentExpression(assignment_expr) => {
                let target = AssignmentTargetWithDefault::cover(assignment_expr.unbox(), ctx)?;
                Ok(AssignmentTargetMaybeDefault::AssignmentTargetWithDefault(ast.alloc(target)))
            }
            expr => {
                let target = AssignmentTarget::cover(expr, ctx)?;
                Ok(AssignmentTargetMaybeDefault::from(target))
            }
        }
    }
}

impl<'a> CoverGrammar<'a, AssignmentExpression<'a>> for AssignmentTargetWithDefault<'a> {
    fn cover<C: CoverContext<'a>>(expr: AssignmentExpression<'a>, ctx: &mut C) -> Result<Self> {
        Ok(ctx.ast().assignment_target_with_default(expr.span, expr.left, expr.right))
    }
}

impl<'a> CoverGrammar<'a, ObjectExpression<'a>> for ObjectAssignmentTarget<'a> {
    fn cover<C: CoverContext<'a>>(expr: ObjectExpression<'a>, ctx: &mut C) -> Result<Self> {
        let ast = ctx.ast();
        let mut properties = ast.vec();
        let mut rest = None;

        let len = expr.properties.len();
        for (i, elem) in expr.properties.into_iter().enumerate() {
            match elem {
                ObjectPropertyKind::ObjectProperty(property) => {
                    let target = AssignmentTargetProperty::cover(property.unbox(), ctx)?;
                    properties.push(target);
                }
                ObjectPropertyKind::SpreadProperty(spread) => {
                    if i == len - 1 {
                        rest = Some(ast.assignment_target_rest(
                            spread.span,
                            AssignmentTarget::cover(spread.unbox().argument, ctx)?,
                        ));
                    } else {
                        return Err(diagnostics::spread_last_element(spread.span));
                    }
                }
            }
        }

        Ok(ast.object_assignment_target(expr.span, properties, rest))
    }
}

impl<'a> CoverGrammar<'a, ObjectProperty<'a>> for AssignmentTargetProperty<'a> {
    fn cover<C: CoverContext<'a>>(property: ObjectProperty<'a>, ctx: &mut C) -> Result<Self> {
        let ast = ctx.ast();
        if property.shorthand {
            let binding = match property.key {
                PropertyKey::StaticIdentifier(ident) => {
                    let ident = ident.unbox();
                    IdentifierReference::new(ident.span, ident.name)
                }
                _ => return Err(ctx.unexpected()),
            };
            // convert `CoverInitializedName`
            let init = match property.init {
                Some(Expression::AssignmentExpression(assignment_expr)) => {
                    Some(assignment_expr.unbox().right)
                }
                _ => None,
            };
            Ok(ast.assignment_target_property_assignment_target_property_identifier(
                property.span,
                binding,
                init,
            ))
        } else {
            let binding = AssignmentTargetMaybeDefault::cover(property.value, ctx)?;
            Ok(ast.assignment_target_property_assignment_target_property_property(
                property.span,
                property.key,
                binding,
            ))
        }
    }
}
