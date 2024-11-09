use oxc_span::{ast_alloc::AstAllocator, cast_ref, Atom, GetSpan, Span};
use serde::{
    ser::{SerializeSeq, Serializer},
    Serialize,
};

use crate::ast::{
    ArrayAssignmentTarget, ArrayPattern, AssignmentTargetMaybeDefault, AssignmentTargetProperty,
    AssignmentTargetRest, BindingPattern, BindingPatternKind, BindingProperty, BindingRestElement,
    Directive, Elision, FormalParameter, FormalParameterKind, FormalParameters, JSXElementName,
    JSXIdentifier, JSXMemberExpressionObject, ObjectAssignmentTarget, ObjectPattern, Program,
    RegExpFlags, Statement, StringLiteral, TSModuleBlock, TSTypeAnnotation,
};
use oxc_allocator::Allocator;
use oxc_span::ast_alloc::{
    traits::{Box as _, Vec as _},
    Box, Vec,
};

pub struct EcmaFormatter;

/// Serialize f64 with `ryu_js`
impl serde_json::ser::Formatter for EcmaFormatter {
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> std::io::Result<()>
    where
        W: ?Sized + std::io::Write,
    {
        use oxc_syntax::number::ToJsString;
        writer.write_all(value.to_js_string().as_bytes())
    }
}

impl<'a> Program<'a> {
    /// # Panics
    pub fn to_json(&self) -> String {
        let ser = self.serializer();
        String::from_utf8(ser.into_inner()).unwrap()
    }

    /// # Panics
    pub fn serializer(&self) -> serde_json::Serializer<std::vec::Vec<u8>, EcmaFormatter> {
        let buf = std::vec::Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(buf, EcmaFormatter);
        self.serialize(&mut ser).unwrap();
        ser
    }
}

impl Serialize for RegExpFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// Serialize `ArrayExpressionElement::Elision` variant as `null` in JSON
impl Serialize for Elision {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

/// Serialize `ArrayAssignmentTarget`, `ObjectAssignmentTarget`, `ObjectPattern`, `ArrayPattern`
/// to be estree compatible, with `elements`/`properties` and `rest` fields combined.

impl<'a, A: AstAllocator> Serialize for ArrayAssignmentTarget<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted = SerArrayAssignmentTarget {
            span: self.span,
            elements: ElementsAndRest::new(self.elements.as_slice_or_empty(), &self.rest),
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "ArrayAssignmentTarget", rename_all = "camelCase", bound = "")]
struct SerArrayAssignmentTarget<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    elements: ElementsAndRest<
        'b,
        Option<AssignmentTargetMaybeDefault<'a, A>>,
        AssignmentTargetRest<'a, A>,
    >,
}

impl<'a, A: AstAllocator> Serialize for ObjectAssignmentTarget<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted = SerObjectAssignmentTarget {
            span: self.span,
            properties: ElementsAndRest::new(&self.properties.as_slice_or_empty(), &self.rest),
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "ObjectAssignmentTarget", bound = "")]
struct SerObjectAssignmentTarget<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    properties: ElementsAndRest<'b, AssignmentTargetProperty<'a, A>, AssignmentTargetRest<'a, A>>,
}

impl<'a, A: AstAllocator> Serialize for ObjectPattern<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted = SerObjectPattern {
            span: self.span,
            properties: ElementsAndRest::new(&self.properties.as_slice_or_empty(), &self.rest),
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "ObjectPattern", bound = "")]
struct SerObjectPattern<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    properties: ElementsAndRest<'b, BindingProperty<'a, A>, Box<'a, BindingRestElement<'a, A>, A>>,
}

impl<'a, A: AstAllocator> Serialize for ArrayPattern<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted = SerArrayPattern {
            span: self.span,
            elements: ElementsAndRest::new(self.elements.as_slice_or_empty(), &self.rest),
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "ArrayPattern", bound = "")]
struct SerArrayPattern<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    elements:
        ElementsAndRest<'b, Option<BindingPattern<'a, A>>, Box<'a, BindingRestElement<'a, A>, A>>,
}

/// Serialize `FormalParameters`, to be estree compatible, with `items` and `rest` fields combined
/// and `argument` field flattened.
impl<'a, A: AstAllocator> Serialize for FormalParameters<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted_rest = self.rest.as_ref().and_then(|rest| {
            let rest = rest.try_deref()?;
            // let rest = cast_ref!(&rest, BindingRestElement<'a, A as Allocator>).unwrap();
            Some(SerFormalParameterRest {
                span: rest.span,
                argument: &rest.argument.kind,
                type_annotation: &rest.argument.type_annotation,
                optional: rest.argument.optional.is_some(),
            })
        });
        let converted = SerFormalParameters {
            span: self.span,
            kind: self.kind,
            items: ElementsAndRest::new(self.items.as_slice_or_empty(), &converted_rest),
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "FormalParameters", bound = "")]
struct SerFormalParameters<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    kind: FormalParameterKind,
    items: ElementsAndRest<'b, FormalParameter<'a, A>, SerFormalParameterRest<'a, 'b, A>>,
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "RestElement", rename_all = "camelCase", bound = "")]
struct SerFormalParameterRest<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    argument: &'b BindingPatternKind<'a, A>,
    type_annotation: &'b Option<Box<'a, TSTypeAnnotation<'a, A>, A>>,
    optional: bool,
}

pub struct ElementsAndRest<'b, E, R> {
    elements: &'b [E],
    rest: &'b Option<R>,
}

impl<'b, E, R> ElementsAndRest<'b, E, R> {
    pub fn new(elements: &'b [E], rest: &'b Option<R>) -> Self {
        Self { elements, rest }
    }
}

impl<'b, E: Serialize, R: Serialize> Serialize for ElementsAndRest<'b, E, R> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(rest) = self.rest {
            let mut seq = serializer.serialize_seq(Some(self.elements.len() + 1))?;
            for element in self.elements {
                seq.serialize_element(element)?;
            }
            seq.serialize_element(rest)?;
            seq.end()
        } else {
            self.elements.serialize(serializer)
        }
    }
}

/// Serialize `TSModuleBlock` to be ESTree compatible, with `body` and `directives` fields combined,
/// and directives output as `StringLiteral` expression statements
impl<'a, A: AstAllocator> Serialize for TSModuleBlock<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let converted = SerTSModuleBlock {
            span: self.span,
            body: DirectivesAndStatements {
                directives: self.directives.as_slice_or_empty(),
                body: self.body.as_slice_or_empty(),
            },
        };
        converted.serialize(serializer)
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "TSModuleBlock", bound = "")]
struct SerTSModuleBlock<'a, 'b, A: AstAllocator> {
    #[serde(flatten)]
    span: Span,
    body: DirectivesAndStatements<'a, 'b, A>,
}

struct DirectivesAndStatements<'a, 'b, A: AstAllocator> {
    directives: &'b [Directive<'a>],
    body: &'b [Statement<'a, A>],
}

impl<'a, 'b, A: AstAllocator> Serialize for DirectivesAndStatements<'a, 'b, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.directives.len() + self.body.len()))?;
        for directive in self.directives {
            seq.serialize_element(&DirectiveAsStatement {
                span: directive.span,
                expression: &directive.expression,
            })?;
        }
        for stmt in self.body {
            seq.serialize_element(stmt)?;
        }
        seq.end()
    }
}

#[derive(Serialize)]
#[serde(tag = "type", rename = "ExpressionStatement")]
struct DirectiveAsStatement<'a, 'b> {
    #[serde(flatten)]
    span: Span,
    expression: &'b StringLiteral<'a>,
}

impl<'a, A: AstAllocator> Serialize for JSXElementName<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Identifier(ident) => ident.serialize(serializer),
            Self::IdentifierReference(ident) => JSXIdentifier {
                span: ident.span(),
                name: ident.try_deref().map_or_else(|| Atom::empty(), |ident| ident.name.clone()),
            }
            .serialize(serializer),
            Self::NamespacedName(name) => name.serialize(serializer),
            Self::MemberExpression(expr) => expr.serialize(serializer),
            Self::ThisExpression(expr) => {
                JSXIdentifier { span: expr.span(), name: "this".into() }.serialize(serializer)
            }
        }
    }
}

impl<'a, A: AstAllocator> Serialize for JSXMemberExpressionObject<'a, A> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::IdentifierReference(ident) => JSXIdentifier {
                span: ident.span(),
                name: ident.try_deref().map_or_else(|| Atom::empty(), |ident| ident.name.clone()),
            }
            .serialize(serializer),
            Self::MemberExpression(expr) => expr.serialize(serializer),
            Self::ThisExpression(expr) => {
                JSXIdentifier { span: expr.span(), name: "this".into() }.serialize(serializer)
            }
        }
    }
}
