use crate::{
    diagnostics,
    lexer::Kind,
    modifiers::{ModifierFlags, ModifierKind, Modifiers},
    Context, ParserImpl, StatementContext,
};
use oxc_allocator::Allocator;
use oxc_ast::ast::*;
use oxc_diagnostics::Result;
use oxc_ecmascript::PropName;
use oxc_span::ast_alloc::{cast, cast_ref, AstAllocator, Box, Vec as _};
use oxc_span::{GetSpan, GetSpanMut, Span};

type Extends<'a, A> = <A as AstAllocator>::Vec<
    'a,
    (
        Expression<'a, A>,
        Option<<A as AstAllocator>::Box<'a, TSTypeParameterInstantiation<'a, A>>>,
        Span,
    ),
>;

type Implements<'a, A> = <A as AstAllocator>::Vec<'a, TSClassImplements<'a, A>>;

/// Section 15.7 Class Definitions
impl<'a, A: AstAllocator, H: crate::Handler<'a, A>> ParserImpl<'a, H, A> {
    // `start_span` points at the start of all decoractors and `class` keyword.
    pub(crate) fn parse_class_statement(
        &mut self,
        stmt_ctx: StatementContext,
        start_span: Span,
    ) -> Result<Statement<'a, A>> {
        let modifiers = self.parse_modifiers(
            /* allow_decorators */ true, /* permit_const_as_modifier */ false,
            /* stop_on_start_of_class_static_block */ true,
        );
        let decl = self.parse_class_declaration(start_span, &modifiers)?;

        if stmt_ctx.is_single_statement() {
            self.error(diagnostics::class_declaration(Span::new(
                decl.span().start,
                if let Some(decl) = decl.try_deref() {
                    decl.body.span().start
                } else {
                    decl.span().end
                },
            )));
        }

        let decl = self.ast.declaration_from_class(decl);
        Ok(self.ast.statement_declaration(decl))
    }

    /// Section 15.7 Class Definitions
    pub(crate) fn parse_class_declaration(
        &mut self,
        start_span: Span,
        modifiers: &Modifiers<'a>,
    ) -> Result<A::Box<'a, Class<'a, A>>> {
        self.parse_class(start_span, ClassType::ClassDeclaration, modifiers)
    }

    /// Section [Class Definitions](https://tc39.es/ecma262/#prod-ClassExpression)
    /// `ClassExpression`[Yield, Await] :
    ///     class `BindingIdentifier`[?Yield, ?Await]opt `ClassTail`[?Yield, ?Await]
    pub(crate) fn parse_class_expression(&mut self) -> Result<Expression<'a, A>> {
        let class =
            self.parse_class(self.start_span(), ClassType::ClassExpression, &Modifiers::empty())?;
        Ok(self.ast.expression_from_class(class))
    }

    fn parse_class(
        &mut self,
        start_span_after_decorators: Span,
        r#type: ClassType,
        modifiers: &Modifiers<'a>,
    ) -> Result<A::Box<'a, Class<'a, A>>> {
        self.bump_any(); // advance `class`

        let decorators = self.take_decorators();
        let start_span = decorators.iter().next().map_or(start_span_after_decorators, |d| d.span);
        let decorators = self.ast.vec_from_iter(decorators);

        let id = if self.cur_kind().is_binding_identifier() && !self.at(Kind::Implements) {
            Some(self.parse_binding_identifier()?)
        } else {
            None
        };
        let declare = modifiers.contains_declare();
        let head = self.ast.class_head(
            self.end_span(start_span_after_decorators),
            modifiers.contains_abstract(),
            declare,
            id,
        );

        let type_parameters = if self.is_ts { self.parse_ts_type_parameters()? } else { None };
        let (extends, implements) = self.parse_heritage_clause()?;
        let mut super_class = None;
        let mut super_type_parameters = None;

        if let Some(extends) = extends {
            if let Ok(mut extends) = extends.try_into() {
                if !extends.is_empty() {
                    let first_extends = extends.remove(0);
                    super_class = Some(first_extends.0);
                    super_type_parameters = first_extends.1;
                }
            }
        }
        let scope_token = self.ast.enter_scope();
        let body = self.parse_class_body(declare)?;

        self.verify_modifiers(
            modifiers,
            ModifierFlags::DECLARE | ModifierFlags::ABSTRACT,
            diagnostics::modifier_cannot_be_used_here,
        );

        Ok(self.ast.alloc_class(
            scope_token,
            r#type,
            self.end_span(start_span),
            decorators,
            head,
            type_parameters,
            super_class,
            super_type_parameters,
            implements,
            body,
        ))
    }

    pub(crate) fn parse_heritage_clause(
        &mut self,
    ) -> Result<(Option<Extends<'a, A>>, Option<Implements<'a, A>>)> {
        let mut extends = None;
        let mut implements = None;

        loop {
            match self.cur_kind() {
                Kind::Extends => {
                    extends = Some(self.parse_extends_clause()?);
                }
                Kind::Implements => {
                    implements = Some(self.parse_ts_implements_clause()?);
                }
                _ => break,
            }
        }

        Ok((extends, implements))
    }

    /// `ClassHeritage`
    /// extends `LeftHandSideExpression`[?Yield, ?Await]
    fn parse_extends_clause(&mut self) -> Result<Extends<'a, A>> {
        self.bump_any(); // bump `extends`
        let mut extends = self.ast.vec();

        let span = self.start_span();
        let mut first_extends = self.parse_lhs_expression_or_higher()?;
        let first_type_argument;

        if let Expression::TSInstantiationExpression(expr) = first_extends {
            match expr.try_unbox() {
                Ok(expr) => {
                    first_extends = expr.expression;
                    first_type_argument = Some(expr.type_parameters);
                }
                Err(expr) => {
                    first_extends = Expression::TSInstantiationExpression(expr);
                    first_type_argument = None;
                }
            }
        } else {
            first_type_argument = self.try_parse_type_arguments()?;
        }

        extends.push((first_extends, first_type_argument, self.end_span(span)));

        while self.eat(Kind::Comma) {
            let span = self.start_span();
            let mut extend = self.parse_lhs_expression_or_higher()?;
            let type_argument;
            if let Expression::TSInstantiationExpression(expr) = extend {
                match expr.try_unbox() {
                    Ok(expr) => {
                        extend = expr.expression;
                        type_argument = Some(expr.type_parameters);
                    }
                    Err(expr) => {
                        extend = Expression::TSInstantiationExpression(expr);
                        type_argument = None;
                    }
                }
            } else {
                type_argument = self.try_parse_type_arguments()?;
            }

            extends.push((extend, type_argument, self.end_span(span)));
        }

        Ok(extends)
    }

    fn parse_class_body(&mut self, declare: bool) -> Result<A::Box<'a, ClassBody<'a, A>>> {
        let span = self.start_span();
        let class_elements = if self.options.allow_skip_ambient && declare {
            self.skip_ambient_curly()?;
            self.ast.vec()
        } else {
            self.parse_normal_list(Kind::LCurly, Kind::RCurly, Self::parse_class_element)?
        };
        Ok(self.ast.alloc_class_body(self.end_span(span), class_elements))
    }

    pub(crate) fn parse_class_element(&mut self) -> Result<Option<ClassElement<'a, A>>> {
        // skip empty class element `;`
        while self.at(Kind::Semicolon) {
            self.bump_any();
        }
        if self.at(Kind::RCurly) {
            return Ok(None);
        }

        let span = self.start_span();

        let modifiers = self.parse_modifiers(true, true, true);

        let mut kind = MethodDefinitionKind::Method;
        let mut generator = false;

        let mut key_name = None;

        let accessibility = modifiers.accessibility();
        let accessor = modifiers.contains(ModifierKind::Accessor);
        let declare = modifiers.contains(ModifierKind::Declare);
        let readonly = modifiers.contains(ModifierKind::Readonly);
        let r#override = modifiers.contains(ModifierKind::Override);
        let r#abstract = modifiers.contains(ModifierKind::Abstract);
        let mut r#static = modifiers.contains(ModifierKind::Static);
        let mut r#async = modifiers.contains(ModifierKind::Async);

        if self.at(Kind::Static) {
            // static { block }
            if self.peek_at(Kind::LCurly) {
                self.bump(Kind::Static);
                return self.parse_class_static_block(span).map(Some);
            }

            // static ...
            if self.peek_kind().is_class_element_name_start() || self.peek_at(Kind::Star) {
                self.bump(Kind::Static);
                r#static = true;
            } else {
                key_name = Some(self.parse_class_element_name()?);
            }
        }

        // async ...
        if key_name.is_none() && self.at(Kind::Async) && !self.peek_at(Kind::Question) {
            if !self.peek_token().is_on_new_line
                && (self.peek_kind().is_class_element_name_start() || self.peek_at(Kind::Star))
            {
                self.bump(Kind::Async);
                r#async = true;
            } else {
                key_name = Some(self.parse_class_element_name()?);
            }
        }

        if self.is_at_ts_index_signature_member() {
            if let TSSignature::TSIndexSignature(sig) = self.parse_ts_index_signature_member()? {
                return Ok(Some(ClassElement::TSIndexSignature(sig)));
            }
        }

        // * ...
        if key_name.is_none() && self.eat(Kind::Star) {
            generator = true;
        }

        if key_name.is_none() && !r#async && !generator {
            // get ... / set ...
            let peeked_class_element = self.peek_kind().is_class_element_name_start();
            key_name = match self.cur_kind() {
                Kind::Get if peeked_class_element => {
                    self.bump(Kind::Get);
                    kind = MethodDefinitionKind::Get;
                    Some(self.parse_class_element_name()?)
                }
                Kind::Set if peeked_class_element => {
                    self.bump(Kind::Set);
                    kind = MethodDefinitionKind::Set;
                    Some(self.parse_class_element_name()?)
                }
                kind if kind.is_class_element_name_start() => {
                    Some(self.parse_class_element_name()?)
                }
                _ => return Err(self.unexpected()),
            }
        }

        let (key, computed) =
            if let Some(result) = key_name { result } else { self.parse_class_element_name()? };

        let (optional, optional_span) = if self.at(Kind::Question) {
            let span = self.start_span();
            self.bump_any();
            (true, self.end_span(span))
        } else {
            (false, oxc_span::SPAN)
        };
        let definite = self.eat(Kind::Bang);

        if optional && definite {
            self.error(diagnostics::optional_definite_property(optional_span.expand_right(1)));
        }

        if modifiers.contains(ModifierKind::Const) {
            self.error(diagnostics::const_class_member(key.span()));
        }

        if let PropertyKey::PrivateIdentifier(private_ident) = &key {
            // `private #foo`, etc. is illegal
            if self.is_ts {
                self.verify_modifiers(
                    &modifiers,
                    ModifierFlags::all() - ModifierFlags::ACCESSIBILITY,
                    diagnostics::accessibility_modifier_on_private_property,
                );
            }
            if private_ident
                .try_deref()
                .is_some_and(|private_ident| private_ident.name == "constructor")
            {
                self.error(diagnostics::private_name_constructor(private_ident.span()));
            }
        }

        if accessor {
            if optional {
                self.error(diagnostics::optional_accessor_property(optional_span));
            }
            self.parse_class_accessor_property(
                span,
                key,
                computed,
                r#static,
                r#abstract,
                definite,
                accessibility,
            )
            .map(Some)
        } else if self.at(Kind::LParen) || self.at(Kind::LAngle) || r#async || generator {
            // LAngle for start of type parameters `foo<T>`
            //                                         ^
            let definition = self.parse_class_method_definition(
                span,
                kind,
                key,
                computed,
                r#static,
                r#async,
                generator,
                r#override,
                r#abstract,
                accessibility,
                optional,
            )?;
            if let Some(definition) =
                cast_ref!(&definition, ClassElement<'a, A as oxc_allocator::Allocator>)
            {
                if let Some((name, span)) = definition.prop_name() {
                    if r#static && name == "prototype" && !self.ctx.has_ambient() {
                        self.error(diagnostics::static_prototype(span));
                    }
                    if !r#static && name == "constructor" {
                        if kind == MethodDefinitionKind::Get || kind == MethodDefinitionKind::Set {
                            self.error(diagnostics::constructor_getter_setter(span));
                        }
                        if r#async {
                            self.error(diagnostics::constructor_async(span));
                        }
                        if generator {
                            self.error(diagnostics::constructor_generator(span));
                        }
                    }
                }
            }
            Ok(Some(definition))
        } else {
            // getter and setter has no ts type annotation
            if kind != MethodDefinitionKind::Method {
                return Err(self.unexpected());
            }
            let mut definition = self.parse_class_property_definition(
                span,
                key,
                computed,
                r#static,
                declare,
                r#override,
                readonly,
                r#abstract,
                accessibility,
                optional,
                definite,
            )?;
            if let Some(definition) =
                cast_ref!(&definition, ClassElement<'a, A as oxc_allocator::Allocator>)
            {
                if let Some((name, span)) = definition.prop_name() {
                    if name == "constructor" {
                        self.error(diagnostics::field_constructor(span));
                    }
                    if r#static && name == "prototype" && !self.ctx.has_ambient() {
                        self.error(diagnostics::static_prototype(span));
                    }
                }
            }
            Ok(Some(definition))
        }
    }

    fn parse_class_element_name(&mut self) -> Result<(PropertyKey<'a, A>, bool)> {
        match self.cur_kind() {
            Kind::PrivateIdentifier => {
                let private_ident = self.parse_private_identifier();
                Ok((PropertyKey::PrivateIdentifier(self.ast.alloc(private_ident)), false))
            }
            _ => self.parse_property_name(),
        }
    }

    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_method_definition(
        &mut self,
        span: Span,
        mut kind: MethodDefinitionKind,
        key: PropertyKey<'a, A>,
        computed: bool,
        r#static: bool,
        r#async: bool,
        generator: bool,
        r#override: bool,
        r#abstract: bool,
        accessibility: Option<TSAccessibility>,
        optional: bool,
    ) -> Result<ClassElement<'a, A>> {
        if !r#static && !computed {
            let is_constructor = if let Some(key) = cast_ref!(&key, PropertyKey<'a, A as Allocator>)
            {
                key.prop_name().map_or(false, |(name, _)| name == "constructor")
            } else {
                &self.source_text[key.span()] == "constructor"
                    || self.lexer.escaped_strings.get(&key.span().start).copied()
                        == Some("constructor")
            };

            if is_constructor {
                kind = MethodDefinitionKind::Constructor
            }
        };

        let decorators = self.consume_decorators();

        let value = self.parse_method(r#async, generator)?;

        if kind == MethodDefinitionKind::Constructor {
            if let Some(value) = value.try_deref() {
                if let Some(this_param) = &value.this_param {
                    // class Foo { constructor(this: number) {} }
                    self.error(diagnostics::ts_constructor_this_parameter(this_param.span()));
                }
            }

            if r#static {
                self.error(diagnostics::static_constructor(key.span()));
            }
        }

        let r#type = if r#abstract {
            MethodDefinitionType::TSAbstractMethodDefinition
        } else {
            MethodDefinitionType::MethodDefinition
        };
        Ok(self.ast.class_element_method_definition(
            r#type,
            self.end_span(span),
            decorators,
            key,
            value,
            kind,
            computed,
            r#static,
            r#override,
            optional,
            accessibility,
        ))
    }

    /// `FieldDefinition`[?Yield, ?Await] ;
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_property_definition(
        &mut self,
        span: Span,
        key: PropertyKey<'a, A>,
        computed: bool,
        r#static: bool,
        declare: bool,
        r#override: bool,
        readonly: bool,
        r#abstract: bool,
        accessibility: Option<TSAccessibility>,
        optional: bool,
        definite: bool,
    ) -> Result<ClassElement<'a, A>> {
        let type_annotation = if self.is_ts { self.parse_ts_type_annotation()? } else { None };
        let decorators = self.consume_decorators();
        let value = if self.eat(Kind::Eq) { Some(self.parse_expr()?) } else { None };
        self.asi()?;

        let r#type = if r#abstract {
            PropertyDefinitionType::TSAbstractPropertyDefinition
        } else {
            PropertyDefinitionType::PropertyDefinition
        };
        Ok(self.ast.class_element_property_definition(
            r#type,
            self.end_span(span),
            decorators,
            key,
            value,
            computed,
            r#static,
            declare,
            r#override,
            optional,
            definite,
            readonly,
            type_annotation,
            accessibility,
        ))
    }

    /// `ClassStaticBlockStatementList` :
    ///    `StatementList`[~Yield, +Await, ~Return]
    fn parse_class_static_block(&mut self, span: Span) -> Result<ClassElement<'a, A>> {
        let scope_token = self.ast.enter_scope();
        let block =
            self.context(Context::Await, Context::Yield | Context::Return, Self::parse_block)?;
        let body = match block.try_unbox() {
            Ok(block) => block.body,
            Err(_) => self.ast.vec(),
        };
        Ok(self.ast.class_element_static_block(scope_token, self.end_span(span), body))
    }

    /// <https://github.com/tc39/proposal-decorators>
    #[allow(clippy::too_many_arguments, clippy::fn_params_excessive_bools)]
    fn parse_class_accessor_property(
        &mut self,
        span: Span,
        key: PropertyKey<'a, A>,
        computed: bool,
        r#static: bool,
        r#abstract: bool,
        definite: bool,
        accessibility: Option<TSAccessibility>,
    ) -> Result<ClassElement<'a, A>> {
        let type_annotation = if self.is_ts { self.parse_ts_type_annotation()? } else { None };
        let value =
            self.eat(Kind::Eq).then(|| self.parse_assignment_expression_or_higher()).transpose()?;
        let r#type = if r#abstract {
            AccessorPropertyType::TSAbstractAccessorProperty
        } else {
            AccessorPropertyType::AccessorProperty
        };

        let decorators = self.consume_decorators();
        Ok(self.ast.class_element_accessor_property(
            r#type,
            self.end_span(span),
            decorators,
            key,
            value,
            computed,
            r#static,
            definite,
            type_annotation,
            accessibility,
        ))
    }
}
