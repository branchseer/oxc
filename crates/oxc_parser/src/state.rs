use derive_where::derive_where;
use oxc_ast::ast::Decorator;
use oxc_span::ast_alloc::AstAllocator;
use rustc_hash::FxHashSet;

#[derive_where(Default)]
pub struct ParserState<'a, A: AstAllocator> {
    pub not_parenthesized_arrow: FxHashSet<u32>,

    pub decorators: Vec<Decorator<'a, A>>,
}
