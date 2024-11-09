/// Macro to inherit enum variants from another enum.
///
/// (for further details see <https://github.com/oxc-project/oxc/pull/3115, A>)
///
/// # Types which can be inherited
///
/// The following types' variants can be inherited:
///
/// * `Expression`
/// * `MemberExpression`
/// * `AssignmentTarget`
/// * `SimpleAssignmentTarget`
/// * `AssignmentTargetPattern`
/// * `Declaration`
/// * `ModuleDeclaration`
/// * `TSType`
/// * `TSTypeName`
///
/// # Expansion
///
/// ```
/// inherit_variants! {
///     #[ast]
///     enum Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///         pub enum Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///             BlockStatement(Box<'a, BlockStatement<'a, A>, A>) = 0,
///             BreakStatement(Box<'a, BreakStatement<'a>, A>) = 1,
///             @inherit Declaration
///             @inherit ModuleDeclaration
///         }
///     }
/// }
/// ```
///
/// expands to:
///
/// ```
/// #[ast]
/// enum Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///     pub enum Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///         BlockStatement(Box<'a, BlockStatement<'a, A>, A>) = 0,
///         BreakStatement(Box<'a, BreakStatement<'a>, A>) = 1,
///
///         // Inherited from `Declaration`
///         VariableDeclaration(Box<'a, VariableDeclaration<'a, A>, A>) = 32,
///         FunctionDeclaration(Box<'a, Function<'a, A>, A>) = 33,
///         // ...and many more
///
///         // Inherited from `ModuleDeclaration`
///         ImportDeclaration(Box<'a, ImportDeclaration<'a, A>, A>) = 64,
///         ExportAllDeclaration(Box<'a, ExportAllDeclaration<'a, A>, A>) = 65,
///         // ...and many more
///     }
/// }
///
/// shared_enum_variants!(
///     Statement, Declaration,
///     is_declaration,
///     into_declaration,
///     as_declaration, as_declaration_mut,
///     to_declaration, to_declaration_mut,
///     [VariableDeclaration, FunctionDeclaration, ...more]
/// )
///
/// shared_enum_variants!(
///     Statement, ModuleDeclaration,
///     is_module_declaration,
///     into_module_declaration,
///     as_module_declaration, as_module_declaration_mut,
///     to_module_declaration, to_module_declaration_mut,
///     [ImportDeclaration, ExportAllDeclaration, ...more]
/// )
/// ```
macro_rules! inherit_variants {
    // Inherit `Expression`'s variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit Expression
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                // `Expression`'s own variants

                /// Inherited from [`Expression`]
                BooleanLiteral(Box<'a, BooleanLiteral, A>) = 0,
                /// Inherited from [`Expression`]
                NullLiteral(Box<'a, NullLiteral, A>) = 1,
                /// Inherited from [`Expression`]
                NumericLiteral(Box<'a, NumericLiteral<'a>, A>) = 2,
                /// Inherited from [`Expression`]
                BigIntLiteral(Box<'a, BigIntLiteral<'a>, A>) = 3,
                /// Inherited from [`Expression`]
                RegExpLiteral(Box<'a, RegExpLiteral<'a, A>, A>) = 4,
                /// Inherited from [`Expression`]
                StringLiteral(Box<'a, StringLiteral<'a>, A>) = 5,
                /// Inherited from [`Expression`]
                TemplateLiteral(Box<'a, TemplateLiteral<'a, A>, A>) = 6,

                /// Inherited from [`Expression`]
                Identifier(Box<'a, IdentifierReference<'a>, A>) = 7,

                /// Inherited from [`Expression`]
                MetaProperty(Box<'a, MetaProperty<'a>, A>) = 8,
                /// Inherited from [`Expression`]
                Super(Box<'a, Super, A>) = 9,

                /// Inherited from [`Expression`]
                ArrayExpression(Box<'a, ArrayExpression<'a, A>, A>) = 10,
                /// Inherited from [`Expression`]
                ArrowFunctionExpression(Box<'a, ArrowFunctionExpression<'a, A>, A>) = 11,
                /// Inherited from [`Expression`]
                AssignmentExpression(Box<'a, AssignmentExpression<'a, A>, A>) = 12,
                /// Inherited from [`Expression`]
                AwaitExpression(Box<'a, AwaitExpression<'a, A>, A>) = 13,
                /// Inherited from [`Expression`]
                BinaryExpression(Box<'a, BinaryExpression<'a, A>, A>) = 14,
                /// Inherited from [`Expression`]
                CallExpression(Box<'a, CallExpression<'a, A>, A>) = 15,
                /// Inherited from [`Expression`]
                ChainExpression(Box<'a, ChainExpression<'a, A>, A>) = 16,
                /// Inherited from [`Expression`]
                ClassExpression(Box<'a, Class<'a, A>, A>) = 17,
                /// Inherited from [`Expression`]
                ConditionalExpression(Box<'a, ConditionalExpression<'a, A>, A>) = 18,
                /// Inherited from [`Expression`]
                FunctionExpression(Box<'a, Function<'a, A>, A>) = 19,
                /// Inherited from [`Expression`]
                ImportExpression(Box<'a, ImportExpression<'a, A>, A>) = 20,
                /// Inherited from [`Expression`]
                LogicalExpression(Box<'a, LogicalExpression<'a, A>, A>) = 21,
                /// Inherited from [`Expression`]
                NewExpression(Box<'a, NewExpression<'a, A>, A>) = 22,
                /// Inherited from [`Expression`]
                ObjectExpression(Box<'a, ObjectExpression<'a, A>, A>) = 23,
                /// Inherited from [`Expression`]
                ParenthesizedExpression(Box<'a, ParenthesizedExpression<'a, A>, A>) = 24,
                /// Inherited from [`Expression`]
                SequenceExpression(Box<'a, SequenceExpression<'a, A>, A>) = 25,
                /// Inherited from [`Expression`]
                TaggedTemplateExpression(Box<'a, TaggedTemplateExpression<'a, A>, A>) = 26,
                /// Inherited from [`Expression`]
                ThisExpression(Box<'a, ThisExpression, A>) = 27,
                /// Inherited from [`Expression`]
                UnaryExpression(Box<'a, UnaryExpression<'a, A>, A>) = 28,
                /// Inherited from [`Expression`]
                UpdateExpression(Box<'a, UpdateExpression<'a, A>, A>) = 29,
                /// Inherited from [`Expression`]
                YieldExpression(Box<'a, YieldExpression<'a, A>, A>) = 30,
                /// Inherited from [`Expression`]
                PrivateInExpression(Box<'a, PrivateInExpression<'a, A>, A>) = 31,

                /// Inherited from [`Expression`]
                JSXElement(Box<'a, JSXElement<'a, A>, A>) = 32,
                /// Inherited from [`Expression`]
                JSXFragment(Box<'a, JSXFragment<'a, A>, A>) = 33,

                /// Inherited from [`Expression`]
                TSAsExpression(Box<'a, TSAsExpression<'a, A>, A>) = 34,
                /// Inherited from [`Expression`]
                TSSatisfiesExpression(Box<'a, TSSatisfiesExpression<'a, A>, A>) = 35,
                /// Inherited from [`Expression`]
                TSTypeAssertion(Box<'a, TSTypeAssertion<'a, A>, A>) = 36,
                /// Inherited from [`Expression`]
                TSNonNullExpression(Box<'a, TSNonNullExpression<'a, A>, A>) = 37,
                /// Inherited from [`Expression`]
                TSInstantiationExpression(Box<'a, TSInstantiationExpression<'a, A>, A>) = 38,

                // Inherited from `MemberExpression`
                @inherit MemberExpression

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            Expression,
            is_expression,
            into_expression,
            as_expression,
            as_expression_mut,
            to_expression,
            to_expression_mut,
            [
                BooleanLiteral,
                NullLiteral,
                NumericLiteral,
                BigIntLiteral,
                RegExpLiteral,
                StringLiteral,
                TemplateLiteral,
                Identifier,
                MetaProperty,
                Super,
                ArrayExpression,
                ArrowFunctionExpression,
                AssignmentExpression,
                AwaitExpression,
                BinaryExpression,
                CallExpression,
                ChainExpression,
                ClassExpression,
                ConditionalExpression,
                FunctionExpression,
                ImportExpression,
                LogicalExpression,
                NewExpression,
                ObjectExpression,
                ParenthesizedExpression,
                SequenceExpression,
                TaggedTemplateExpression,
                ThisExpression,
                UnaryExpression,
                UpdateExpression,
                YieldExpression,
                PrivateInExpression,
                JSXElement,
                JSXFragment,
                TSAsExpression,
                TSSatisfiesExpression,
                TSTypeAssertion,
                TSNonNullExpression,
                TSInstantiationExpression,
                ComputedMemberExpression,
                StaticMemberExpression,
                PrivateFieldExpression,
            ]
        );
    };

    // Inherit `MemberExpression`'s variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit MemberExpression
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`MemberExpression`].
                ///
                /// `MemberExpression[?Yield, ?Await] [ Expression[+In, ?Yield, ?Await] ]`
                ComputedMemberExpression(Box<'a, ComputedMemberExpression<'a, A>, A>) = 48,
                /// Inherited from [`MemberExpression`].
                ///
                /// `MemberExpression[?Yield, ?Await] . IdentifierName`
                StaticMemberExpression(Box<'a, StaticMemberExpression<'a, A>, A>) = 49,
                /// Inherited from [`MemberExpression`].
                ///
                /// `MemberExpression[?Yield, ?Await] . PrivateIdentifier`
                PrivateFieldExpression(Box<'a, PrivateFieldExpression<'a, A>, A>) = 50,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            MemberExpression,
            is_member_expression,
            into_member_expression,
            as_member_expression,
            as_member_expression_mut,
            to_member_expression,
            to_member_expression_mut,
            [ComputedMemberExpression, StaticMemberExpression, PrivateFieldExpression]
        );
    };

    // Inherit `AssignmentTarget` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit AssignmentTarget
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                @inherit SimpleAssignmentTarget
                @inherit AssignmentTargetPattern

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            AssignmentTarget,
            is_assignment_target,
            into_assignment_target,
            as_assignment_target,
            as_assignment_target_mut,
            to_assignment_target,
            to_assignment_target_mut,
            [
                AssignmentTargetIdentifier,
                ComputedMemberExpression,
                StaticMemberExpression,
                PrivateFieldExpression,
                TSAsExpression,
                TSSatisfiesExpression,
                TSNonNullExpression,
                TSTypeAssertion,
                TSInstantiationExpression,
                ArrayAssignmentTarget,
                ObjectAssignmentTarget,
            ]
        );
    };

    // Inherit `SimpleAssignmentTarget` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit SimpleAssignmentTarget
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`SimpleAssignmentTarget`]
                AssignmentTargetIdentifier(Box<'a, IdentifierReference<'a>, A>) = 0,

                /// Inherited from [`SimpleAssignmentTarget`]
                TSAsExpression(Box<'a, TSAsExpression<'a, A>, A>) = 1,
                /// Inherited from [`SimpleAssignmentTarget`]
                TSSatisfiesExpression(Box<'a, TSSatisfiesExpression<'a, A>, A>) = 2,
                /// Inherited from [`SimpleAssignmentTarget`]
                TSNonNullExpression(Box<'a, TSNonNullExpression<'a, A>, A>) = 3,
                /// Inherited from [`SimpleAssignmentTarget`]
                TSTypeAssertion(Box<'a, TSTypeAssertion<'a, A>, A>) = 4,
                /// Inherited from [`SimpleAssignmentTarget`]
                TSInstantiationExpression(Box<'a, TSInstantiationExpression<'a, A>, A>) = 5,

                // Inherited from `MemberExpression`
                @inherit MemberExpression

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            SimpleAssignmentTarget,
            is_simple_assignment_target,
            into_simple_assignment_target,
            as_simple_assignment_target,
            as_simple_assignment_target_mut,
            to_simple_assignment_target,
            to_simple_assignment_target_mut,
            [
                AssignmentTargetIdentifier,
                ComputedMemberExpression,
                StaticMemberExpression,
                PrivateFieldExpression,
                TSAsExpression,
                TSSatisfiesExpression,
                TSNonNullExpression,
                TSTypeAssertion,
                TSInstantiationExpression
            ]
        );
    };

    // Inherit `AssignmentTargetPattern` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit AssignmentTargetPattern
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`AssignmentTargetPattern`]
                ArrayAssignmentTarget(Box<'a, ArrayAssignmentTarget<'a, A>, A>) = 8,
                /// Inherited from [`AssignmentTargetPattern`]
                ObjectAssignmentTarget(Box<'a, ObjectAssignmentTarget<'a, A>, A>) = 9,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            AssignmentTargetPattern,
            is_assignment_target_pattern,
            into_assignment_target_pattern,
            as_assignment_target_pattern,
            as_assignment_target_pattern_mut,
            to_assignment_target_pattern,
            to_assignment_target_pattern_mut,
            [ArrayAssignmentTarget, ObjectAssignmentTarget]
        );
    };

    // Inherit `Declaration` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit Declaration
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`Declaration`]
                VariableDeclaration(Box<'a, VariableDeclaration<'a, A>, A>) = 32,
                /// Inherited from [`Declaration`]
                FunctionDeclaration(Box<'a, Function<'a, A>, A>) = 33,
                /// Inherited from [`Declaration`]
                ClassDeclaration(Box<'a, Class<'a, A>, A>) = 34,

                /// Inherited from [`Declaration`]
                TSTypeAliasDeclaration(Box<'a, TSTypeAliasDeclaration<'a, A>, A>) = 35,
                /// Inherited from [`Declaration`]
                TSInterfaceDeclaration(Box<'a, TSInterfaceDeclaration<'a, A>, A>) = 36,
                /// Inherited from [`Declaration`]
                TSEnumDeclaration(Box<'a, TSEnumDeclaration<'a, A>, A>) = 37,
                /// Inherited from [`Declaration`]
                TSModuleDeclaration(Box<'a, TSModuleDeclaration<'a, A>, A>) = 38,
                /// Inherited from [`Declaration`]
                TSImportEqualsDeclaration(Box<'a, TSImportEqualsDeclaration<'a, A>, A>) = 39,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            Declaration,
            is_declaration,
            into_declaration,
            as_declaration,
            as_declaration_mut,
            to_declaration,
            to_declaration_mut,
            [
                VariableDeclaration,
                FunctionDeclaration,
                ClassDeclaration,
                TSTypeAliasDeclaration,
                TSInterfaceDeclaration,
                TSEnumDeclaration,
                TSModuleDeclaration,
                TSImportEqualsDeclaration,
            ]
        );
    };

    // Inherit `ModuleDeclaration` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit ModuleDeclaration
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`ModuleDeclaration`].
                /// `import hello from './world.js';`
                /// `import * as t from './world.js';`
                ImportDeclaration(Box<'a, ImportDeclaration<'a, A>, A>) = 64,
                /// Inherited from [`ModuleDeclaration`].
                /// `export * as numbers from '../numbers.js'`
                ExportAllDeclaration(Box<'a, ExportAllDeclaration<'a, A>, A>) = 65,
                /// Inherited from [`ModuleDeclaration`].
                /// `export default 5;`
                ExportDefaultDeclaration(Box<'a, ExportDefaultDeclaration<'a, A>, A>) = 66,
                /// Inherited from [`ModuleDeclaration`].
                /// `export {five} from './numbers.js';`
                /// `export {six, seven};`
                ExportNamedDeclaration(Box<'a, ExportNamedDeclaration<'a, A>, A>) = 67,

                /// Inherited from [`ModuleDeclaration`].
                /// `export = 5;`
                TSExportAssignment(Box<'a, TSExportAssignment<'a, A>, A>) = 68,
                /// Inherited from [`ModuleDeclaration`].
                /// `export as namespace React;`
                TSNamespaceExportDeclaration(Box<'a, TSNamespaceExportDeclaration<'a>, A>) = 69,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            ModuleDeclaration,
            is_module_declaration,
            into_module_declaration,
            as_module_declaration,
            as_module_declaration_mut,
            to_module_declaration,
            to_module_declaration_mut,
            [
                ImportDeclaration,
                ExportAllDeclaration,
                ExportDefaultDeclaration,
                ExportNamedDeclaration,
                TSExportAssignment,
                TSNamespaceExportDeclaration,
            ]
        );
    };

    // Inherit `TSType` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit TSType
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                // Keyword
                /// Inherited from [`TSType`]
                TSAnyKeyword(Box<'a, TSAnyKeyword, A>) = 0,
                /// Inherited from [`TSType`]
                TSBigIntKeyword(Box<'a, TSBigIntKeyword, A>) = 1,
                /// Inherited from [`TSType`]
                TSBooleanKeyword(Box<'a, TSBooleanKeyword, A>) = 2,
                /// Inherited from [`TSType`]
                TSIntrinsicKeyword(Box<'a, TSIntrinsicKeyword, A>) = 3,
                /// Inherited from [`TSType`]
                TSNeverKeyword(Box<'a, TSNeverKeyword, A>) = 4,
                /// Inherited from [`TSType`]
                TSNullKeyword(Box<'a, TSNullKeyword, A>) = 5,
                /// Inherited from [`TSType`]
                TSNumberKeyword(Box<'a, TSNumberKeyword, A>) = 6,
                /// Inherited from [`TSType`]
                TSObjectKeyword(Box<'a, TSObjectKeyword, A>) = 7,
                /// Inherited from [`TSType`]
                TSStringKeyword(Box<'a, TSStringKeyword, A>) = 8,
                /// Inherited from [`TSType`]
                TSSymbolKeyword(Box<'a, TSSymbolKeyword, A>) = 9,
                /// Inherited from [`TSType`]
                TSThisType(Box<'a, TSThisType, A>) = 10,
                /// Inherited from [`TSType`]
                TSUndefinedKeyword(Box<'a, TSUndefinedKeyword, A>) = 11,
                /// Inherited from [`TSType`]
                TSUnknownKeyword(Box<'a, TSUnknownKeyword, A>) = 12,
                /// Inherited from [`TSType`]
                TSVoidKeyword(Box<'a, TSVoidKeyword, A>) = 13,

                // Compound
                /// Inherited from [`TSType`]
                TSArrayType(Box<'a, TSArrayType<'a, A>, A>) = 14,
                /// Inherited from [`TSType`]
                TSConditionalType(Box<'a, TSConditionalType<'a, A>, A>) = 15,
                /// Inherited from [`TSType`]
                TSConstructorType(Box<'a, TSConstructorType<'a, A>, A>) = 16,
                /// Inherited from [`TSType`]
                TSFunctionType(Box<'a, TSFunctionType<'a, A>, A>) = 17,
                /// Inherited from [`TSType`]
                TSImportType(Box<'a, TSImportType<'a, A>, A>) = 18,
                /// Inherited from [`TSType`]
                TSIndexedAccessType(Box<'a, TSIndexedAccessType<'a, A>, A>) = 19,
                /// Inherited from [`TSType`]
                TSInferType(Box<'a, TSInferType<'a, A>, A>) = 20,
                /// Inherited from [`TSType`]
                TSIntersectionType(Box<'a, TSIntersectionType<'a, A>, A>) = 21,
                /// Inherited from [`TSType`]
                TSLiteralType(Box<'a, TSLiteralType<'a, A>, A>) = 22,
                /// Inherited from [`TSType`]
                TSMappedType(Box<'a, TSMappedType<'a, A>, A>) = 23,
                /// Inherited from [`TSType`]
                TSNamedTupleMember(Box<'a, TSNamedTupleMember<'a, A>, A>) = 24,
                /// Inherited from [`TSType`]
                TSQualifiedName(Box<'a, TSQualifiedName<'a, A>, A>) = 25,
                /// Inherited from [`TSType`]
                TSTemplateLiteralType(Box<'a, TSTemplateLiteralType<'a, A>, A>) = 26,
                /// Inherited from [`TSType`]
                TSTupleType(Box<'a, TSTupleType<'a, A>, A>) = 27,
                /// Inherited from [`TSType`]
                TSTypeLiteral(Box<'a, TSTypeLiteral<'a, A>, A>) = 28,
                /// Inherited from [`TSType`]
                TSTypeOperatorType(Box<'a, TSTypeOperator<'a, A>, A>) = 29,
                /// Inherited from [`TSType`]
                TSTypePredicate(Box<'a, TSTypePredicate<'a, A>, A>) = 30,
                /// Inherited from [`TSType`]
                TSTypeQuery(Box<'a, TSTypeQuery<'a, A>, A>) = 31,
                /// Inherited from [`TSType`]
                TSTypeReference(Box<'a, TSTypeReference<'a, A>, A>) = 32,
                /// Inherited from [`TSType`]
                TSUnionType(Box<'a, TSUnionType<'a, A>, A>) = 33,
                /// Inherited from [`TSType`]
                TSParenthesizedType(Box<'a, TSParenthesizedType<'a, A>, A>) = 34,

                // JSDoc
                /// Inherited from [`TSType`]
                JSDocNullableType(Box<'a, JSDocNullableType<'a, A>, A>) = 35,
                /// Inherited from [`TSType`]
                JSDocNonNullableType(Box<'a, JSDocNonNullableType<'a, A>, A>) = 36,
                /// Inherited from [`TSType`]
                JSDocUnknownType(Box<'a, JSDocUnknownType, A>) = 37,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            TSType,
            is_ts_type,
            into_ts_type,
            as_ts_type,
            as_ts_type_mut,
            to_ts_type,
            to_ts_type_mut,
            [
                TSAnyKeyword,
                TSBigIntKeyword,
                TSBooleanKeyword,
                TSIntrinsicKeyword,
                TSNeverKeyword,
                TSNullKeyword,
                TSNumberKeyword,
                TSObjectKeyword,
                TSStringKeyword,
                TSSymbolKeyword,
                TSThisType,
                TSUndefinedKeyword,
                TSUnknownKeyword,
                TSVoidKeyword,
                TSArrayType,
                TSConditionalType,
                TSConstructorType,
                TSFunctionType,
                TSImportType,
                TSIndexedAccessType,
                TSInferType,
                TSIntersectionType,
                TSLiteralType,
                TSMappedType,
                TSNamedTupleMember,
                TSQualifiedName,
                TSTemplateLiteralType,
                TSTupleType,
                TSTypeLiteral,
                TSTypeOperatorType,
                TSTypePredicate,
                TSTypeQuery,
                TSTypeReference,
                TSUnionType,
                TSParenthesizedType,
                JSDocNullableType,
                JSDocNonNullableType,
                JSDocUnknownType,
            ]
        );
    };

    // Inherit `TSTypeName` variants
    (
        $(#[$attr:meta])*
        pub enum $ty:ident<'a, A: AstAllocator = oxc_allocator::Allocator> {
            $($(#[$variant_attr:meta])* $variant_name:ident($variant_type:ty) = $variant_discrim:literal,)*
            @inherit TSTypeName
            $($rest:tt)*
        }
    ) => {
        $crate::ast::macros::inherit_variants! {
            $(#[$attr])*
            pub enum $ty<'a, A: AstAllocator = oxc_allocator::Allocator> {
                $($(#[$variant_attr])* $variant_name($variant_type) = $variant_discrim,)*

                /// Inherited from [`TSTypeName`]
                IdentifierReference(Box<'a, IdentifierReference<'a>, A>) = 0,
                /// Inherited from [`TSTypeName`]
                QualifiedName(Box<'a, TSQualifiedName<'a, A>, A>) = 1,

                $($rest)*
            }
        }

        $crate::ast::macros::shared_enum_variants!(
            $ty,
            TSTypeName,
            is_ts_type_name,
            into_ts_type_name,
            as_ts_type_name,
            as_ts_type_name_mut,
            to_ts_type_name,
            to_ts_type_name_mut,
            [IdentifierReference, QualifiedName]
        );
    };

    // Passthrough - no further inheritance to handle
    ($($rest:tt)*) => {$($rest)*};
}
pub(crate) use inherit_variants;

/// Macro to allow conversion between 2 enum types where they share some of the same variants.
/// "Parent" enum contains all the "child"'s variants, plus parent contains further other variants.
/// e.g. `Statement` and `Declaration`.
///
/// The discriminants and types of the shared variants must be identical between the 2 enums.
/// All variants must have a `Box<_, A>` payload.
/// Equality of types is guaranteed by `From` and `TryFrom` impls this macro creates.
/// These will fail to compile if the types differ for any variant.
/// Equality of discriminants is checked with a compile-time assertion.
///
/// # SAFETY
/// Both enums must be `#[repr(C, u8)]` or using this macro is unsound.
///
/// # Expansion
///
/// NB: For illustration only - `Statement` and `Declaration` in reality share 9 variants, not 2.
///
/// ```
/// shared_enum_variants!(
///     Statement, Declaration,
///     is_declaration,
///     into_declaration,
///     as_declaration, as_declaration_mut,
///     to_declaration, to_declaration_mut,
///     [VariableDeclaration, FunctionDeclaration]
/// )
/// ```
///
/// expands to:
///
/// ```
/// const _: () = {
///     assert!(discriminant!(Statement::VariableDeclaration) == discriminant!(Declaration::VariableDeclaration));
///     assert!(discriminant!(Statement::FunctionDeclaration) == discriminant!(Declaration::FunctionDeclaration));
/// };
///
/// impl<'a> Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///     /// Return if a `Statement` is a `Declaration`.
///     #[inline]
///     pub fn is_declaration(&self) -> bool {
///         match self {
///             Self::VariableDeclaration(_) | Self::FunctionDeclaration(_) => true,
///             _ => false,
///         }
///     }
///
///     /// Convert `Statement` to `Declaration`.
///     /// # Panic
///     /// Panics if not convertible.
///     #[inline]
///     pub fn into_declaration(self) -> Declaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
///         Declaration::try_from(self).unwrap()
///     }
///
///     /// Convert `&Statement` to `&Declaration`.
///     #[inline]
///     pub fn as_declaration(&self) -> Option<&Declaration<'a>> {
///         if self.is_declaration() {
///             Some(unsafe { &*(self as *const _ as *const Declaration) })
///         } else {
///             None
///         }
///     }
///
///     /// Convert `&mut Statement` to `&mut Declaration`.
///     #[inline]
///     pub fn as_declaration_mut(&mut self) -> Option<&mut Declaration<'a>> {
///         if self.is_declaration() {
///             Some(unsafe { &mut *(self as *mut _ as *mut Declaration) })
///         } else {
///             None
///         }
///     }
///
///     /// Convert `&Statement` to `&Declaration`.
///     /// # Panic
///     /// Panics if not convertible.
///     #[inline]
///     pub fn to_declaration(&self) -> &Declaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
///         self.as_declaration().unwrap()
///     }
///
///     /// Convert `&mut Statement` to `&mut Declaration`.
///     /// # Panic
///     /// Panics if not convertible.
///     #[inline]
///     pub fn to_declaration_mut(&mut self) -> &mut Declaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
///         self.as_declaration_mut().unwrap()
///     }
/// }
///
/// impl<'a> TryFrom<Statement<'a>> for Declaration<'a, A: AstAllocator = oxc_allocator::Allocator> {
///     type Error = ();
///
///     /// "Convert `Statement` to `Declaration`.
///     #[inline]
///     fn try_from(value: Statement<'a, A>) -> Result<Self, Self::Error> {
///         match value {
///             Statement::VariableDeclaration(o) => Ok(Declaration::VariableDeclaration(o)),
///             Statement::FunctionDeclaration(o) => Ok(Declaration::FunctionDeclaration(o)),
///             _ => Err(()),
///         }
///     }
/// }
///
/// impl<'a> From<Declaration<'a>> for Statement<'a, A: AstAllocator = oxc_allocator::Allocator> {
///     /// Convert `Declaration` to `Statement`.
///     #[inline]
///     fn from(value: Declaration<'a, A>) -> Self {
///         match value {
///             Declaration::VariableDeclaration(o) => Statement::VariableDeclaration(o),
///             Declaration::FunctionDeclaration(o) => Statement::FunctionDeclaration(o),
///         }
///     }
/// }
/// ```
macro_rules! shared_enum_variants {
    (
        $parent:ident, $child:ident,
        $is_child:ident,
        $into_child:ident,
        $as_child:ident, $as_child_mut:ident,
        $to_child:ident, $to_child_mut:ident,
        [$($variant:ident),+ $(,)?]
    ) => {
        // Ensure discriminants match for all variants between parent and child types
        const _: () = {
            $(
                assert!(
                    $crate::ast::macros::discriminant!($parent::$variant)
                    == $crate::ast::macros::discriminant!($child::$variant),
                    concat!(
                        "Non-matching discriminants for `", stringify!($variant),
                        "` between `", stringify!($parent), "` and `", stringify!($child), "`"
                    )
                );
            )+
        };

        impl<'a, A: AstAllocator> $parent<'a, A> {
            #[doc = concat!("Return if a `", stringify!($parent), "` is a `", stringify!($child), "`.")]
            #[inline]
            pub fn $is_child(&self) -> bool {
                matches!(
                    self,
                    $(Self::$variant(_))|+
                )
            }

            #[doc = concat!("Convert `", stringify!($parent), "` to `", stringify!($child), "`.")]
            #[doc = "# Panic"]
            #[doc = "Panics if not convertible."]
            #[inline]
            pub fn $into_child(self) -> $child<'a, A> {
                $child::try_from(self).unwrap()
            }

            #[doc = concat!("Convert `&", stringify!($parent), "` to `&", stringify!($child), "`.")]
            #[inline]
            pub fn $as_child(&self) -> Option<&$child<'a, A>> {
                if self.$is_child() {
                    #[allow(unsafe_code)]
                    // SAFETY: Transmute is safe because discriminants + types are identical between
                    // `$parent` and `$child` for $child variants
                    Some(unsafe { &*std::ptr::from_ref(self).cast::<$child<'a, A>>() })
                } else {
                    None
                }
            }

            #[doc = concat!("Convert `&mut ", stringify!($parent), "` to `&mut ", stringify!($child), "`.")]
            #[inline]
            pub fn $as_child_mut(&mut self) -> Option<&mut $child<'a, A>> {
                if self.$is_child() {
                    #[allow(unsafe_code)]
                    // SAFETY: Transmute is safe because discriminants + types are identical between
                    // `$parent` and `$child` for $child variants
                    Some(unsafe { &mut *std::ptr::from_mut(self).cast::<$child<'a, A>>() })
                } else {
                    None
                }
            }

            #[doc = concat!("Convert `&", stringify!($parent), "` to `&", stringify!($child), "`.")]
            #[doc = "# Panic"]
            #[doc = "Panics if not convertible."]
            #[inline]
            pub fn $to_child(&self) -> &$child<'a, A> {
                self.$as_child().unwrap()
            }

            #[doc = concat!("Convert `&mut ", stringify!($parent), "` to `&mut ", stringify!($child), "`.")]
            #[doc = "# Panic"]
            #[doc = "Panics if not convertible."]
            #[inline]
            pub fn $to_child_mut(&mut self) -> &mut $child<'a, A> {
                self.$as_child_mut().unwrap()
            }
        }

        impl<'a, A: AstAllocator> TryFrom<$parent<'a, A>> for $child<'a, A> {
            type Error = ();

            #[doc = concat!("Convert `", stringify!($parent), "` to `", stringify!($child), "`.")]
            #[inline]
            fn try_from(value: $parent<'a, A>) -> Result<Self, Self::Error> {
                // Compiler should implement this as a check of discriminant and then zero-cost transmute,
                // as discriminants for `$parent` and `$child` are aligned
                match value {
                    $($parent::$variant(o) => Ok($child::$variant(o)),)+
                    _ => Err(())
                }
            }
        }

        impl<'a, A: AstAllocator> From<$child<'a, A>> for $parent<'a, A> {
            #[doc = concat!("Convert `", stringify!($child), "` to `", stringify!($parent), "`.")]
            #[inline]
            fn from(value: $child<'a, A>) -> Self {
                // Compiler should implement this as zero-cost transmute as discriminants
                // for `$child` and `$parent` are aligned
                match value {
                    $($child::$variant(o) => $parent::$variant(o),)+
                }
            }
        }
    }
}
pub(crate) use shared_enum_variants;

/// Macro to get discriminant of an enum.
/// # SAFETY
/// Enum must be `#[repr(C, u8)]` or using this macro is unsound.
/// <https://doc.rust-lang.org/std/mem/fn.discriminant.html>
macro_rules! discriminant {
    ($ty:ident :: $variant:ident) => {{
        #[allow(unsafe_code, clippy::undocumented_unsafe_blocks)]
        unsafe {
            let t = std::mem::ManuallyDrop::new($ty::<'_, oxc_allocator::Allocator>::$variant(
                ::oxc_span::ast_alloc::Box::dangling(),
            ));
            *(std::ptr::addr_of!(t).cast::<u8>())
        }
    }};
}
pub(crate) use discriminant;
