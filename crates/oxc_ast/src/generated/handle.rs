// Auto-generated code, DO NOT EDIT DIRECTLY!
// To edit this generated file you have to edit `tasks/ast_tools/src/generators/visit.rs`

use crate::ast::*;
use oxc_span::ast_alloc::AstAllocator;
pub trait Handler<'a, A: AstAllocator> {
    #[inline]
    fn enter_scope(&mut self) {}
    #[inline]
    fn leave_scope(&mut self) {}

    #[inline]
    fn handle_program(&mut self, _: &Program<'a, A>) {}

    #[inline]
    fn handle_hashbang(&mut self, _: &Hashbang<'a>) {}

    #[inline]
    fn handle_directive(&mut self, _: &Directive<'a>) {}

    #[inline]
    fn handle_string_literal(&mut self, _: &StringLiteral<'a>) {}

    #[inline]
    fn handle_statement(&mut self, _: &Statement<'a, A>) {}

    #[inline]
    fn handle_block_statement(&mut self, _: &BlockStatement<'a, A>) {}

    #[inline]
    fn handle_break_statement(&mut self, _: &BreakStatement<'a>) {}

    #[inline]
    fn handle_label_identifier(&mut self, _: &LabelIdentifier<'a>) {}

    #[inline]
    fn handle_continue_statement(&mut self, _: &ContinueStatement<'a>) {}

    #[inline]
    fn handle_debugger_statement(&mut self, _: &DebuggerStatement) {}

    #[inline]
    fn handle_do_while_statement(&mut self, _: &DoWhileStatement<'a, A>) {}

    #[inline]
    fn handle_expression(&mut self, _: &Expression<'a, A>) {}

    #[inline]
    fn handle_boolean_literal(&mut self, _: &BooleanLiteral) {}

    #[inline]
    fn handle_null_literal(&mut self, _: &NullLiteral) {}

    #[inline]
    fn handle_numeric_literal(&mut self, _: &NumericLiteral<'a>) {}

    #[inline]
    fn handle_big_int_literal(&mut self, _: &BigIntLiteral<'a>) {}

    #[inline]
    fn handle_reg_exp_literal(&mut self, _: &RegExpLiteral<'a, A>) {}

    #[inline]
    fn handle_template_literal(&mut self, _: &TemplateLiteral<'a, A>) {}

    #[inline]
    fn handle_template_element(&mut self, _: &TemplateElement<'a>) {}

    #[inline]
    fn handle_identifier_reference(&mut self, _: &IdentifierReference<'a>) {}

    #[inline]
    fn handle_meta_property(&mut self, _: &MetaProperty<'a>) {}

    #[inline]
    fn handle_identifier_name(&mut self, _: &IdentifierName<'a>) {}

    #[inline]
    fn handle_super(&mut self, _: &Super) {}

    #[inline]
    fn handle_array_expression(&mut self, _: &ArrayExpression<'a, A>) {}

    #[inline]
    fn handle_array_expression_element(&mut self, _: &ArrayExpressionElement<'a, A>) {}

    #[inline]
    fn handle_spread_element(&mut self, _: &SpreadElement<'a, A>) {}

    #[inline]
    fn handle_elision(&mut self, _: &Elision) {}

    #[inline]
    fn handle_expression_array_element(&mut self, _: &Expression<'a, A>) {}

    #[inline]
    fn handle_arrow_function_expression(&mut self, _: &ArrowFunctionExpression<'a, A>) {}

    #[inline]
    fn handle_ts_type_parameter_declaration(&mut self, _: &TSTypeParameterDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_type_parameter(&mut self, _: &TSTypeParameter<'a, A>) {}

    #[inline]
    fn handle_binding_identifier(&mut self, _: &BindingIdentifier<'a>) {}

    #[inline]
    fn handle_ts_type(&mut self, _: &TSType<'a, A>) {}

    #[inline]
    fn handle_ts_any_keyword(&mut self, _: &TSAnyKeyword) {}

    #[inline]
    fn handle_ts_big_int_keyword(&mut self, _: &TSBigIntKeyword) {}

    #[inline]
    fn handle_ts_boolean_keyword(&mut self, _: &TSBooleanKeyword) {}

    #[inline]
    fn handle_ts_intrinsic_keyword(&mut self, _: &TSIntrinsicKeyword) {}

    #[inline]
    fn handle_ts_never_keyword(&mut self, _: &TSNeverKeyword) {}

    #[inline]
    fn handle_ts_null_keyword(&mut self, _: &TSNullKeyword) {}

    #[inline]
    fn handle_ts_number_keyword(&mut self, _: &TSNumberKeyword) {}

    #[inline]
    fn handle_ts_object_keyword(&mut self, _: &TSObjectKeyword) {}

    #[inline]
    fn handle_ts_string_keyword(&mut self, _: &TSStringKeyword) {}

    #[inline]
    fn handle_ts_symbol_keyword(&mut self, _: &TSSymbolKeyword) {}

    #[inline]
    fn handle_ts_undefined_keyword(&mut self, _: &TSUndefinedKeyword) {}

    #[inline]
    fn handle_ts_unknown_keyword(&mut self, _: &TSUnknownKeyword) {}

    #[inline]
    fn handle_ts_void_keyword(&mut self, _: &TSVoidKeyword) {}

    #[inline]
    fn handle_ts_array_type(&mut self, _: &TSArrayType<'a, A>) {}

    #[inline]
    fn handle_ts_conditional_type(&mut self, _: &TSConditionalType<'a, A>) {}

    #[inline]
    fn handle_ts_constructor_type(&mut self, _: &TSConstructorType<'a, A>) {}

    #[inline]
    fn handle_formal_parameters(&mut self, _: &FormalParameters<'a, A>) {}

    #[inline]
    fn handle_formal_parameter(&mut self, _: &FormalParameter<'a, A>) {}

    #[inline]
    fn handle_decorator(&mut self, _: &Decorator<'a, A>) {}

    #[inline]
    fn handle_binding_pattern(&mut self, _: &BindingPattern<'a, A>) {}

    #[inline]
    fn handle_binding_pattern_kind(&mut self, _: &BindingPatternKind<'a, A>) {}

    #[inline]
    fn handle_object_pattern(&mut self, _: &ObjectPattern<'a, A>) {}

    #[inline]
    fn handle_binding_property(&mut self, _: &BindingProperty<'a, A>) {}

    #[inline]
    fn handle_property_key(&mut self, _: &PropertyKey<'a, A>) {}

    #[inline]
    fn handle_private_identifier(&mut self, _: &PrivateIdentifier<'a>) {}

    #[inline]
    fn handle_binding_rest_element(&mut self, _: &BindingRestElement<'a, A>) {}

    #[inline]
    fn handle_array_pattern(&mut self, _: &ArrayPattern<'a, A>) {}

    #[inline]
    fn handle_assignment_pattern(&mut self, _: &AssignmentPattern<'a, A>) {}

    #[inline]
    fn handle_ts_type_annotation(&mut self, _: &TSTypeAnnotation<'a, A>) {}

    #[inline]
    fn handle_ts_function_type(&mut self, _: &TSFunctionType<'a, A>) {}

    #[inline]
    fn handle_ts_this_parameter(&mut self, _: &TSThisParameter<'a, A>) {}

    #[inline]
    fn handle_ts_import_type(&mut self, _: &TSImportType<'a, A>) {}

    #[inline]
    fn handle_ts_type_name(&mut self, _: &TSTypeName<'a, A>) {}

    #[inline]
    fn handle_ts_qualified_name(&mut self, _: &TSQualifiedName<'a, A>) {}

    #[inline]
    fn handle_ts_import_attributes(&mut self, _: &TSImportAttributes<'a, A>) {}

    #[inline]
    fn handle_ts_import_attribute(&mut self, _: &TSImportAttribute<'a, A>) {}

    #[inline]
    fn handle_ts_import_attribute_name(&mut self, _: &TSImportAttributeName<'a>) {}

    #[inline]
    fn handle_ts_type_parameter_instantiation(&mut self, _: &TSTypeParameterInstantiation<'a, A>) {}

    #[inline]
    fn handle_ts_indexed_access_type(&mut self, _: &TSIndexedAccessType<'a, A>) {}

    #[inline]
    fn handle_ts_infer_type(&mut self, _: &TSInferType<'a, A>) {}

    #[inline]
    fn handle_ts_intersection_type(&mut self, _: &TSIntersectionType<'a, A>) {}

    #[inline]
    fn handle_ts_literal_type(&mut self, _: &TSLiteralType<'a, A>) {}

    #[inline]
    fn handle_ts_literal(&mut self, _: &TSLiteral<'a, A>) {}

    #[inline]
    fn handle_unary_expression(&mut self, _: &UnaryExpression<'a, A>) {}

    #[inline]
    fn handle_ts_mapped_type(&mut self, _: &TSMappedType<'a, A>) {}

    #[inline]
    fn handle_ts_named_tuple_member(&mut self, _: &TSNamedTupleMember<'a, A>) {}

    #[inline]
    fn handle_ts_tuple_element(&mut self, _: &TSTupleElement<'a, A>) {}

    #[inline]
    fn handle_ts_optional_type(&mut self, _: &TSOptionalType<'a, A>) {}

    #[inline]
    fn handle_ts_rest_type(&mut self, _: &TSRestType<'a, A>) {}

    #[inline]
    fn handle_ts_template_literal_type(&mut self, _: &TSTemplateLiteralType<'a, A>) {}

    #[inline]
    fn handle_ts_this_type(&mut self, _: &TSThisType) {}

    #[inline]
    fn handle_ts_tuple_type(&mut self, _: &TSTupleType<'a, A>) {}

    #[inline]
    fn handle_ts_type_literal(&mut self, _: &TSTypeLiteral<'a, A>) {}

    #[inline]
    fn handle_ts_signature(&mut self, _: &TSSignature<'a, A>) {}

    #[inline]
    fn handle_ts_index_signature(&mut self, _: &TSIndexSignature<'a, A>) {}

    #[inline]
    fn handle_ts_index_signature_name(&mut self, _: &TSIndexSignatureName<'a, A>) {}

    #[inline]
    fn handle_ts_property_signature(&mut self, _: &TSPropertySignature<'a, A>) {}

    #[inline]
    fn handle_ts_call_signature_declaration(&mut self, _: &TSCallSignatureDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_construct_signature_declaration(
        &mut self,
        _: &TSConstructSignatureDeclaration<'a, A>,
    ) {
    }

    #[inline]
    fn handle_ts_method_signature(&mut self, _: &TSMethodSignature<'a, A>) {}

    #[inline]
    fn handle_ts_type_operator(&mut self, _: &TSTypeOperator<'a, A>) {}

    #[inline]
    fn handle_ts_type_predicate(&mut self, _: &TSTypePredicate<'a, A>) {}

    #[inline]
    fn handle_ts_type_predicate_name(&mut self, _: &TSTypePredicateName<'a, A>) {}

    #[inline]
    fn handle_ts_type_query(&mut self, _: &TSTypeQuery<'a, A>) {}

    #[inline]
    fn handle_ts_type_query_expr_name(&mut self, _: &TSTypeQueryExprName<'a, A>) {}

    #[inline]
    fn handle_ts_type_reference(&mut self, _: &TSTypeReference<'a, A>) {}

    #[inline]
    fn handle_ts_union_type(&mut self, _: &TSUnionType<'a, A>) {}

    #[inline]
    fn handle_ts_parenthesized_type(&mut self, _: &TSParenthesizedType<'a, A>) {}

    #[inline]
    fn handle_js_doc_nullable_type(&mut self, _: &JSDocNullableType<'a, A>) {}

    #[inline]
    fn handle_js_doc_non_nullable_type(&mut self, _: &JSDocNonNullableType<'a, A>) {}

    #[inline]
    fn handle_js_doc_unknown_type(&mut self, _: &JSDocUnknownType) {}

    #[inline]
    fn handle_function_body(&mut self, _: &FunctionBody<'a, A>) {}

    #[inline]
    fn handle_assignment_expression(&mut self, _: &AssignmentExpression<'a, A>) {}

    #[inline]
    fn handle_assignment_target(&mut self, _: &AssignmentTarget<'a, A>) {}

    #[inline]
    fn handle_simple_assignment_target(&mut self, _: &SimpleAssignmentTarget<'a, A>) {}

    #[inline]
    fn handle_ts_as_expression(&mut self, _: &TSAsExpression<'a, A>) {}

    #[inline]
    fn handle_ts_satisfies_expression(&mut self, _: &TSSatisfiesExpression<'a, A>) {}

    #[inline]
    fn handle_ts_non_null_expression(&mut self, _: &TSNonNullExpression<'a, A>) {}

    #[inline]
    fn handle_ts_type_assertion(&mut self, _: &TSTypeAssertion<'a, A>) {}

    #[inline]
    fn handle_ts_instantiation_expression(&mut self, _: &TSInstantiationExpression<'a, A>) {}

    #[inline]
    fn handle_member_expression(&mut self, _: &MemberExpression<'a, A>) {}

    #[inline]
    fn handle_computed_member_expression(&mut self, _: &ComputedMemberExpression<'a, A>) {}

    #[inline]
    fn handle_static_member_expression(&mut self, _: &StaticMemberExpression<'a, A>) {}

    #[inline]
    fn handle_private_field_expression(&mut self, _: &PrivateFieldExpression<'a, A>) {}

    #[inline]
    fn handle_assignment_target_pattern(&mut self, _: &AssignmentTargetPattern<'a, A>) {}

    #[inline]
    fn handle_array_assignment_target(&mut self, _: &ArrayAssignmentTarget<'a, A>) {}

    #[inline]
    fn handle_assignment_target_maybe_default(&mut self, _: &AssignmentTargetMaybeDefault<'a, A>) {}

    #[inline]
    fn handle_assignment_target_with_default(&mut self, _: &AssignmentTargetWithDefault<'a, A>) {}

    #[inline]
    fn handle_assignment_target_rest(&mut self, _: &AssignmentTargetRest<'a, A>) {}

    #[inline]
    fn handle_object_assignment_target(&mut self, _: &ObjectAssignmentTarget<'a, A>) {}

    #[inline]
    fn handle_assignment_target_property(&mut self, _: &AssignmentTargetProperty<'a, A>) {}

    #[inline]
    fn handle_assignment_target_property_identifier(
        &mut self,
        _: &AssignmentTargetPropertyIdentifier<'a, A>,
    ) {
    }

    #[inline]
    fn handle_assignment_target_property_property(
        &mut self,
        _: &AssignmentTargetPropertyProperty<'a, A>,
    ) {
    }

    #[inline]
    fn handle_await_expression(&mut self, _: &AwaitExpression<'a, A>) {}

    #[inline]
    fn handle_binary_expression(&mut self, _: &BinaryExpression<'a, A>) {}

    #[inline]
    fn handle_call_expression(&mut self, _: &CallExpression<'a, A>) {}

    #[inline]
    fn handle_argument(&mut self, _: &Argument<'a, A>) {}

    #[inline]
    fn handle_chain_expression(&mut self, _: &ChainExpression<'a, A>) {}

    #[inline]
    fn handle_chain_element(&mut self, _: &ChainElement<'a, A>) {}

    #[inline]
    fn handle_class(&mut self, _: &Class<'a, A>) {}

    #[inline]
    fn handle_class_heritage(&mut self, _: &Expression<'a, A>) {}

    #[inline]
    fn handle_ts_class_implements(&mut self, _: &TSClassImplements<'a, A>) {}

    #[inline]
    fn handle_class_body(&mut self, _: &ClassBody<'a, A>) {}

    #[inline]
    fn handle_class_element(&mut self, _: &ClassElement<'a, A>) {}

    #[inline]
    fn handle_static_block(&mut self, _: &StaticBlock<'a, A>) {}

    #[inline]
    fn handle_method_definition(&mut self, _: &MethodDefinition<'a, A>) {}

    #[inline]
    fn handle_function(&mut self, _: &Function<'a, A>) {}

    #[inline]
    fn handle_property_definition(&mut self, _: &PropertyDefinition<'a, A>) {}

    #[inline]
    fn handle_accessor_property(&mut self, _: &AccessorProperty<'a, A>) {}

    #[inline]
    fn handle_conditional_expression(&mut self, _: &ConditionalExpression<'a, A>) {}

    #[inline]
    fn handle_import_expression(&mut self, _: &ImportExpression<'a, A>) {}

    #[inline]
    fn handle_logical_expression(&mut self, _: &LogicalExpression<'a, A>) {}

    #[inline]
    fn handle_new_expression(&mut self, _: &NewExpression<'a, A>) {}

    #[inline]
    fn handle_object_expression(&mut self, _: &ObjectExpression<'a, A>) {}

    #[inline]
    fn handle_object_property_kind(&mut self, _: &ObjectPropertyKind<'a, A>) {}

    #[inline]
    fn handle_object_property(&mut self, _: &ObjectProperty<'a, A>) {}

    #[inline]
    fn handle_parenthesized_expression(&mut self, _: &ParenthesizedExpression<'a, A>) {}

    #[inline]
    fn handle_sequence_expression(&mut self, _: &SequenceExpression<'a, A>) {}

    #[inline]
    fn handle_tagged_template_expression(&mut self, _: &TaggedTemplateExpression<'a, A>) {}

    #[inline]
    fn handle_this_expression(&mut self, _: &ThisExpression) {}

    #[inline]
    fn handle_update_expression(&mut self, _: &UpdateExpression<'a, A>) {}

    #[inline]
    fn handle_yield_expression(&mut self, _: &YieldExpression<'a, A>) {}

    #[inline]
    fn handle_private_in_expression(&mut self, _: &PrivateInExpression<'a, A>) {}

    #[inline]
    fn handle_jsx_element(&mut self, _: &JSXElement<'a, A>) {}

    #[inline]
    fn handle_jsx_opening_element(&mut self, _: &JSXOpeningElement<'a, A>) {}

    #[inline]
    fn handle_jsx_element_name(&mut self, _: &JSXElementName<'a, A>) {}

    #[inline]
    fn handle_jsx_identifier(&mut self, _: &JSXIdentifier<'a>) {}

    #[inline]
    fn handle_jsx_namespaced_name(&mut self, _: &JSXNamespacedName<'a>) {}

    #[inline]
    fn handle_jsx_member_expression(&mut self, _: &JSXMemberExpression<'a, A>) {}

    #[inline]
    fn handle_jsx_member_expression_object(&mut self, _: &JSXMemberExpressionObject<'a, A>) {}

    #[inline]
    fn handle_jsx_attribute_item(&mut self, _: &JSXAttributeItem<'a, A>) {}

    #[inline]
    fn handle_jsx_attribute(&mut self, _: &JSXAttribute<'a, A>) {}

    #[inline]
    fn handle_jsx_attribute_name(&mut self, _: &JSXAttributeName<'a, A>) {}

    #[inline]
    fn handle_jsx_attribute_value(&mut self, _: &JSXAttributeValue<'a, A>) {}

    #[inline]
    fn handle_jsx_expression_container(&mut self, _: &JSXExpressionContainer<'a, A>) {}

    #[inline]
    fn handle_jsx_expression(&mut self, _: &JSXExpression<'a, A>) {}

    #[inline]
    fn handle_jsx_empty_expression(&mut self, _: &JSXEmptyExpression) {}

    #[inline]
    fn handle_jsx_fragment(&mut self, _: &JSXFragment<'a, A>) {}

    #[inline]
    fn handle_jsx_child(&mut self, _: &JSXChild<'a, A>) {}

    #[inline]
    fn handle_jsx_text(&mut self, _: &JSXText<'a>) {}

    #[inline]
    fn handle_jsx_spread_child(&mut self, _: &JSXSpreadChild<'a, A>) {}

    #[inline]
    fn handle_jsx_spread_attribute(&mut self, _: &JSXSpreadAttribute<'a, A>) {}

    #[inline]
    fn handle_jsx_closing_element(&mut self, _: &JSXClosingElement<'a, A>) {}

    #[inline]
    fn handle_empty_statement(&mut self, _: &EmptyStatement) {}

    #[inline]
    fn handle_expression_statement(&mut self, _: &ExpressionStatement<'a, A>) {}

    #[inline]
    fn handle_for_in_statement(&mut self, _: &ForInStatement<'a, A>) {}

    #[inline]
    fn handle_for_statement_left(&mut self, _: &ForStatementLeft<'a, A>) {}

    #[inline]
    fn handle_variable_declaration(&mut self, _: &VariableDeclaration<'a, A>) {}

    #[inline]
    fn handle_variable_declarator(&mut self, _: &VariableDeclarator<'a, A>) {}

    #[inline]
    fn handle_for_of_statement(&mut self, _: &ForOfStatement<'a, A>) {}

    #[inline]
    fn handle_for_statement(&mut self, _: &ForStatement<'a, A>) {}

    #[inline]
    fn handle_for_statement_init(&mut self, _: &ForStatementInit<'a, A>) {}

    #[inline]
    fn handle_if_statement(&mut self, _: &IfStatement<'a, A>) {}

    #[inline]
    fn handle_labeled_statement(&mut self, _: &LabeledStatement<'a, A>) {}

    #[inline]
    fn handle_return_statement(&mut self, _: &ReturnStatement<'a, A>) {}

    #[inline]
    fn handle_switch_statement(&mut self, _: &SwitchStatement<'a, A>) {}

    #[inline]
    fn handle_switch_case(&mut self, _: &SwitchCase<'a, A>) {}

    #[inline]
    fn handle_throw_statement(&mut self, _: &ThrowStatement<'a, A>) {}

    #[inline]
    fn handle_try_statement(&mut self, _: &TryStatement<'a, A>) {}

    #[inline]
    fn handle_catch_clause(&mut self, _: &CatchClause<'a, A>) {}

    #[inline]
    fn handle_catch_parameter(&mut self, _: &CatchParameter<'a, A>) {}

    #[inline]
    fn handle_finally_clause(&mut self, _: &BlockStatement<'a, A>) {}

    #[inline]
    fn handle_while_statement(&mut self, _: &WhileStatement<'a, A>) {}

    #[inline]
    fn handle_with_statement(&mut self, _: &WithStatement<'a, A>) {}

    #[inline]
    fn handle_declaration(&mut self, _: &Declaration<'a, A>) {}

    #[inline]
    fn handle_ts_type_alias_declaration(&mut self, _: &TSTypeAliasDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_interface_declaration(&mut self, _: &TSInterfaceDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_interface_heritage(&mut self, _: &TSInterfaceHeritage<'a, A>) {}

    #[inline]
    fn handle_ts_interface_body(&mut self, _: &TSInterfaceBody<'a, A>) {}

    #[inline]
    fn handle_ts_enum_declaration(&mut self, _: &TSEnumDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_enum_member(&mut self, _: &TSEnumMember<'a, A>) {}

    #[inline]
    fn handle_ts_enum_member_name(&mut self, _: &TSEnumMemberName<'a, A>) {}

    #[inline]
    fn handle_ts_module_declaration(&mut self, _: &TSModuleDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_module_declaration_name(&mut self, _: &TSModuleDeclarationName<'a>) {}

    #[inline]
    fn handle_ts_module_declaration_body(&mut self, _: &TSModuleDeclarationBody<'a, A>) {}

    #[inline]
    fn handle_ts_module_block(&mut self, _: &TSModuleBlock<'a, A>) {}

    #[inline]
    fn handle_ts_import_equals_declaration(&mut self, _: &TSImportEqualsDeclaration<'a, A>) {}

    #[inline]
    fn handle_ts_module_reference(&mut self, _: &TSModuleReference<'a, A>) {}

    #[inline]
    fn handle_ts_external_module_reference(&mut self, _: &TSExternalModuleReference<'a>) {}

    #[inline]
    fn handle_module_declaration(&mut self, _: &ModuleDeclaration<'a, A>) {}

    #[inline]
    fn handle_import_declaration(&mut self, _: &ImportDeclaration<'a, A>) {}

    #[inline]
    fn handle_import_declaration_specifier(&mut self, _: &ImportDeclarationSpecifier<'a, A>) {}

    #[inline]
    fn handle_import_specifier(&mut self, _: &ImportSpecifier<'a>) {}

    #[inline]
    fn handle_module_export_name(&mut self, _: &ModuleExportName<'a>) {}

    #[inline]
    fn handle_import_default_specifier(&mut self, _: &ImportDefaultSpecifier<'a>) {}

    #[inline]
    fn handle_import_namespace_specifier(&mut self, _: &ImportNamespaceSpecifier<'a>) {}

    #[inline]
    fn handle_with_clause(&mut self, _: &WithClause<'a, A>) {}

    #[inline]
    fn handle_import_attribute(&mut self, _: &ImportAttribute<'a>) {}

    #[inline]
    fn handle_import_attribute_key(&mut self, _: &ImportAttributeKey<'a>) {}

    #[inline]
    fn handle_export_all_declaration(&mut self, _: &ExportAllDeclaration<'a, A>) {}

    #[inline]
    fn handle_export_default_declaration(&mut self, _: &ExportDefaultDeclaration<'a, A>) {}

    #[inline]
    fn handle_export_default_declaration_kind(&mut self, _: &ExportDefaultDeclarationKind<'a, A>) {}

    #[inline]
    fn handle_export_named_declaration(&mut self, _: &ExportNamedDeclaration<'a, A>) {}

    #[inline]
    fn handle_export_specifier(&mut self, _: &ExportSpecifier<'a>) {}

    #[inline]
    fn handle_ts_export_assignment(&mut self, _: &TSExportAssignment<'a, A>) {}

    #[inline]
    fn handle_ts_namespace_export_declaration(&mut self, _: &TSNamespaceExportDeclaration<'a>) {}
}
