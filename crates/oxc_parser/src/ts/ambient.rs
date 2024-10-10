use crate::{lexer::Kind, Handler, ParserImpl};
use oxc_ast::ast::{TSType, TSTypeParameterDeclaration};
use oxc_diagnostics::Result;
use oxc_span::ast_alloc::AstAllocator;

impl<'a, H: Handler<'a, A>, A: AstAllocator> ParserImpl<'a, H, A> {
    fn try_skip_ambient(
        &mut self,
        begin: Kind,
        end: Kind,
        mut re_lex_begin: impl FnMut(&mut Self) + Copy,
        mut inner_skip: impl (FnMut(&mut Self) -> Result<bool>) + Copy,
        mut re_lex_end: impl FnMut(&mut Self) + Copy,
    ) -> Result<bool> {
        re_lex_begin(self);
        if !self.eat(begin) {
            return Ok(false);
        }
        loop {
            if self.at(Kind::Eof) {
                return Err(self.unexpected());
            }
            re_lex_end(self);
            if inner_skip(self)? {
                continue;
            }
            if self.try_skip_ambient(begin, end, re_lex_begin, inner_skip, re_lex_end)? {
                continue;
            }
            if self.eat(end) {
                return Ok(true);
            }
            self.bump_any();
        }
    }

    fn try_skip_ambient_template_literal(&mut self) -> Result<bool> {
        self.try_skip_ambient(
            Kind::TemplateHead,
            Kind::TemplateTail,
            |_| (),
            Self::try_skip_ambient_curly,
            Self::re_lex_template_substitution_tail,
        )
    }

    pub(crate) fn try_skip_ambient_curly(&mut self) -> Result<bool> {
        self.try_skip_ambient(
            Kind::LCurly,
            Kind::RCurly,
            |_| (),
            Self::try_skip_ambient_template_literal,
            |_| (),
        )
    }
    pub(crate) fn skip_ambient_curly(&mut self) -> Result<()> {
        if !self.try_skip_ambient_curly()? {
            return Err(self.unexpected());
        }
        Ok(())
    }

    /// Call this where a ts type is surely expected (not in try_parse)
    pub(crate) fn parse_ts_type_skipping_ambient(&mut self) -> Result<TSType<'a, A>> {
        let ctx = self.ctx;
        if self.options.allow_skip_ambient {
            self.ctx = ctx.and_skip_ambient(true);
        }
        let ret = self.parse_ts_type();
        if self.options.allow_skip_ambient {
            self.ctx = ctx;
        }
        ret
    }

    #[inline(always)]
    pub(crate) fn can_skip_ambient(&self) -> bool {
        self.options.allow_skip_ambient && self.ctx.has_skip_ambient()
    }

    pub(crate) fn skip_ambient_curly_as_type(&mut self) -> Result<TSType<'a, A>> {
        let start_span = self.start_span();
        if !self.try_skip_ambient_curly()? {
            return Err(self.unexpected());
        };
        Ok(self.ast.ts_type_type_literal(self.end_span(start_span), self.ast.vec()))
    }
    pub(crate) fn skip_ambient_brack_as_type(&mut self) -> Result<TSType<'a, A>> {
        let start_span = self.start_span();
        if !self.try_skip_ambient_brack()? {
            return Err(self.unexpected());
        };
        Ok(self.ast.ts_type_tuple_type(self.end_span(start_span), self.ast.vec()))
    }
    pub(crate) fn skip_ambient_paren_as_type(&mut self) -> Result<TSType<'a, A>> {
        let start_span = self.start_span();
        if !self.try_skip_ambient_paren()? {
            return Err(self.unexpected());
        };
        let span = self.end_span(start_span);
        let dummy_any = self.ast.ts_type_any_keyword(span);
        Ok(self.ast.ts_type_parenthesized_type(span, dummy_any))
    }
    pub(crate) fn skip_ambient_template_literal_as_type(&mut self) -> Result<TSType<'a, A>> {
        let start_span = self.start_span();
        if !self.try_skip_ambient_template_literal()? {
            return Err(self.unexpected());
        };
        Ok(self.ast.ts_type_template_literal_type(
            self.end_span(start_span),
            self.ast.vec(),
            self.ast.vec(),
        ))
    }
    pub(crate) fn skip_ambient_angle_as_type_params(
        &mut self,
    ) -> Result<TSTypeParameterDeclaration<'a, A>> {
        let start_span = self.start_span();
        if !self.try_skip_ambient_angle()? {
            return Err(self.unexpected());
        };
        Ok(self.ast.ts_type_parameter_declaration(self.end_span(start_span), self.ast.vec()))
    }

    fn try_skip_ambient_brack(&mut self) -> Result<bool> {
        self.try_skip_ambient(
            Kind::LBrack,
            Kind::RBrack,
            |_| (),
            Self::try_skip_ambient_template_literal,
            |_| (),
        )
    }

    fn try_skip_ambient_paren(&mut self) -> Result<bool> {
        self.try_skip_ambient(
            Kind::LParen,
            Kind::RParen,
            |_| (),
            Self::try_skip_ambient_template_literal,
            |_| (),
        )
    }

    pub(crate) fn try_skip_ambient_angle(&mut self) -> Result<bool> {
        self.try_skip_ambient(
            Kind::LAngle,
            Kind::RAngle,
            |me| {
                me.re_lex_l_angle();
            },
            Self::try_skip_ambient_template_literal,
            |me| {
                me.re_lex_ts_r_angle();
            },
        )
    }
}
