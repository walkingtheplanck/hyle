use thiserror::Error;

use crate::syntax::ast::{
    BinaryOpAst, BoundsAst, ExprAst, ExprKindAst, FieldAst, InputAst, LiteralAst, ModelAst,
    NeighborhoodAst, ReductionOpAst, RuleAst, RuleSourceAst, RuleStatementAst, SamplingAst,
    ScriptAst, TypeAst, UnaryOpAst,
};
use crate::syntax::token::{Directive, Keyword, Span, Symbol, Token, TokenKind};

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("{0}")]
    Message(String),
}

pub(super) type ParseResult<T> = Result<T, ParseError>;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub(super) fn parse_tokens(tokens: Vec<Token>) -> ParseResult<ScriptAst> {
    Parser::new(tokens).parse_ast()
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse_ast(&mut self) -> ParseResult<ScriptAst> {
        self.expect_directive(Directive::Hyle)?;
        let version = self.expect_value_text("version")?;

        let mut dimensions = None;
        let mut cell = None;
        let mut neighborhoods = Vec::new();
        let mut models = Vec::new();
        let mut inputs = Vec::new();
        let mut rules = Vec::new();

        while !self.at(&TokenKind::Eof) {
            if self.at_directive(&Directive::Dimensions) {
                self.advance();
                dimensions = Some(self.expect_u8("dimensions")?);
            } else if self.at_directive(&Directive::Cell) {
                self.advance();
                cell = Some(self.expect_identifier("cell shape")?);
            } else if self.at_keyword(Keyword::Neighborhood) {
                neighborhoods.push(self.parse_neighborhood()?);
            } else if self.at_keyword(Keyword::Model) {
                models.push(self.parse_model()?);
            } else if self.at_keyword(Keyword::In) {
                inputs.push(self.parse_input()?);
            } else if self.at_identifier() {
                rules.push(self.parse_rule()?);
            } else {
                return self.unexpected("top-level item");
            }
        }

        Ok(ScriptAst {
            source_path: String::new(),
            version,
            dimensions: dimensions.ok_or_else(|| {
                ParseError::Message("missing required `#dimensions` directive".to_owned())
            })?,
            cell: cell.ok_or_else(|| {
                ParseError::Message("missing required `#cell` directive".to_owned())
            })?,
            neighborhoods,
            models,
            inputs,
            rules,
        })
    }

    fn parse_neighborhood(&mut self) -> ParseResult<NeighborhoodAst> {
        self.expect_keyword(Keyword::Neighborhood)?;
        let name = self.expect_identifier("neighborhood name")?;
        self.expect_symbol(Symbol::LeftBrace)?;

        let mut radius = None;
        let mut center = None;
        let mut metric = None;

        while !self.at_symbol(Symbol::RightBrace) {
            if self.at_keyword(Keyword::Radius) {
                self.advance();
                radius = Some(self.parse_number_literal("radius")?);
            } else if self.at_keyword(Keyword::Center) {
                self.advance();
                center = Some(self.expect_bool("center")?);
            } else if self.at_keyword(Keyword::Metric) {
                self.advance();
                metric = Some(self.expect_identifier("metric")?);
            } else {
                return self.unexpected("neighborhood item");
            }
        }
        self.expect_symbol(Symbol::RightBrace)?;

        Ok(NeighborhoodAst {
            name,
            radius: radius.unwrap_or_else(|| LiteralAst::Integer("1".to_owned())),
            center: center.unwrap_or(false),
            metric: metric.unwrap_or_else(|| "Manhattan".to_owned()),
        })
    }

    fn parse_model(&mut self) -> ParseResult<ModelAst> {
        self.expect_keyword(Keyword::Model)?;
        let name = self.expect_identifier("model name")?;
        self.expect_symbol(Symbol::LeftBrace)?;

        let mut resolution = None;
        let mut range = None;
        let mut fields = Vec::new();

        while !self.at_symbol(Symbol::RightBrace) {
            if self.at_keyword(Keyword::Resolution) {
                self.advance();
                resolution = Some(self.expect_u32("resolution")?);
            } else if self.at_keyword(Keyword::Range) {
                self.advance();
                range = Some(self.expect_identifier("model range")?);
            } else if self.at_keyword(Keyword::Fields) {
                fields = self.parse_fields()?;
            } else {
                return self.unexpected("model item");
            }
        }
        self.expect_symbol(Symbol::RightBrace)?;

        Ok(ModelAst {
            name,
            resolution,
            range,
            fields,
        })
    }

    fn parse_fields(&mut self) -> ParseResult<Vec<FieldAst>> {
        self.expect_keyword(Keyword::Fields)?;
        self.expect_symbol(Symbol::LeftBrace)?;

        let mut fields = Vec::new();
        while !self.at_symbol(Symbol::RightBrace) {
            let name = self.expect_identifier("field name")?;
            self.expect_symbol(Symbol::Colon)?;
            let ty = self.parse_type()?;
            let default = if self.match_symbol(Symbol::Less) {
                let value = self.parse_literal()?;
                self.expect_symbol(Symbol::Greater)?;
                Some(value)
            } else {
                None
            };
            let bounds = if self.at_symbol(Symbol::LeftBracket) || self.at_symbol(Symbol::LeftParen)
            {
                Some(self.parse_bounds()?)
            } else {
                None
            };
            fields.push(FieldAst {
                name,
                ty,
                default,
                bounds,
            });
        }

        self.expect_symbol(Symbol::RightBrace)?;
        Ok(fields)
    }

    fn parse_input(&mut self) -> ParseResult<InputAst> {
        self.expect_keyword(Keyword::In)?;
        let name = self.expect_identifier("input name")?;
        self.expect_symbol(Symbol::Colon)?;
        let ty = self.parse_type()?;
        let default = if self.match_symbol(Symbol::Less) {
            let value = self.parse_literal()?;
            self.expect_symbol(Symbol::Greater)?;
            Some(value)
        } else {
            None
        };
        self.expect_symbol(Symbol::Semicolon)?;
        Ok(InputAst { name, ty, default })
    }

    fn parse_rule(&mut self) -> ParseResult<RuleAst> {
        let anchor = self.expect_identifier("rule anchor model")?;

        let sampled = if self.match_symbol(Symbol::Plus) {
            self.expect_symbol(Symbol::LeftParen)?;
            let sampling = self.parse_sampling()?;
            self.expect_symbol(Symbol::RightParen)?;
            let model = self.expect_identifier("sampled rule source model")?;
            Some(RuleSourceAst {
                model,
                sampling: Some(sampling),
            })
        } else {
            None
        };

        if self.at_symbol(Symbol::Plus) {
            return Err(ParseError::Message(
                "rules currently support at most one sampled source".to_owned(),
            ));
        }

        self.expect_kind(TokenKind::Arrow, "`->`")?;
        let output = self.expect_identifier("rule output model")?;

        let mut range = None;
        let mut condition = None;

        if self.at_keyword(Keyword::Range) {
            self.advance();
            range = Some(self.expect_identifier("rule range")?);
        }

        if self.at_keyword(Keyword::When) {
            self.advance();
            condition = Some(self.collect_expression_until_block()?);
        }

        let statements = self.parse_rule_body()?;

        Ok(RuleAst {
            anchor,
            sampled,
            output,
            range,
            condition,
            statements,
        })
    }

    fn parse_rule_body(&mut self) -> ParseResult<Vec<RuleStatementAst>> {
        self.expect_symbol(Symbol::LeftBrace)?;
        let mut statements = Vec::new();

        while !self.at_symbol(Symbol::RightBrace) {
            if self.at_keyword(Keyword::Let) {
                self.advance();
                let name = self.expect_identifier("binding name")?;
                self.expect_symbol(Symbol::Equal)?;
                let expression = self.collect_expression_until_semicolon()?;
                self.expect_symbol(Symbol::Semicolon)?;
                statements.push(RuleStatementAst::Let { name, expression });
            } else if self.at_keyword(Keyword::Next) {
                self.advance();
                let model = self.expect_identifier("next model")?;
                self.expect_symbol(Symbol::Dot)?;
                let field = self.expect_identifier("next field")?;
                self.expect_symbol(Symbol::Equal)?;
                let expression = self.collect_expression_until_semicolon()?;
                self.expect_symbol(Symbol::Semicolon)?;
                statements.push(RuleStatementAst::Next {
                    model,
                    field,
                    expression,
                });
            } else {
                return self.unexpected("rule statement");
            }
        }

        self.expect_symbol(Symbol::RightBrace)?;
        Ok(statements)
    }

    fn parse_bounds(&mut self) -> ParseResult<BoundsAst> {
        let lower_inclusive = if self.match_symbol(Symbol::LeftBracket) {
            true
        } else {
            self.expect_symbol(Symbol::LeftParen)?;
            false
        };
        let lower = self.parse_number_literal("lower bound")?;
        let upper = self.parse_number_literal("upper bound")?;
        let upper_inclusive = if self.match_symbol(Symbol::RightBracket) {
            true
        } else {
            self.expect_symbol(Symbol::RightParen)?;
            false
        };

        Ok(BoundsAst {
            lower,
            lower_inclusive,
            upper,
            upper_inclusive,
        })
    }

    fn parse_type(&mut self) -> ParseResult<TypeAst> {
        if self.match_keyword(Keyword::Int) {
            return Ok(TypeAst::Int);
        }
        if self.match_keyword(Keyword::Float) {
            return Ok(TypeAst::Float);
        }
        if self.match_keyword(Keyword::Bool) {
            return Ok(TypeAst::Bool);
        }

        Ok(TypeAst::Custom(self.expect_identifier("type")?))
    }

    fn parse_literal(&mut self) -> ParseResult<LiteralAst> {
        if self.at_number() {
            return self.parse_number_literal("literal");
        }

        if self.at_keyword(Keyword::True) {
            self.advance();
            return Ok(LiteralAst::Bool(true));
        }

        if self.at_keyword(Keyword::False) {
            self.advance();
            return Ok(LiteralAst::Bool(false));
        }

        self.unexpected("literal")
    }

    fn parse_number_literal(&mut self, context: &str) -> ParseResult<LiteralAst> {
        match self.advance().kind {
            TokenKind::Integer(value) => Ok(LiteralAst::Integer(value)),
            TokenKind::Float(value) => Ok(LiteralAst::Float(value)),
            _ => Err(ParseError::Message(format!("expected numeric {context}"))),
        }
    }

    fn parse_sampling(&mut self) -> ParseResult<SamplingAst> {
        if self.match_keyword(Keyword::Average) {
            return Ok(SamplingAst::Average);
        }
        if self.match_keyword(Keyword::Nearest) {
            return Ok(SamplingAst::Nearest);
        }
        if self.match_keyword(Keyword::SamplingSum) {
            return Ok(SamplingAst::Sum);
        }
        if self.match_keyword(Keyword::All) {
            return Ok(SamplingAst::All);
        }

        Ok(SamplingAst::Custom(
            self.expect_identifier("sampling algorithm")?,
        ))
    }

    fn collect_expression_until_block(&mut self) -> ParseResult<ExprAst> {
        self.collect_expression(|parser, depth| depth == 0 && parser.at_symbol(Symbol::LeftBrace))
    }

    fn collect_expression_until_semicolon(&mut self) -> ParseResult<ExprAst> {
        self.collect_expression(|parser, depth| depth == 0 && parser.at_symbol(Symbol::Semicolon))
    }

    fn collect_expression(
        &mut self,
        stop: impl Fn(&Parser, usize) -> bool,
    ) -> ParseResult<ExprAst> {
        let start_token = self.current;
        let mut depth = 0usize;

        while !self.at(&TokenKind::Eof) && !stop(self, depth) {
            match &self.peek().kind {
                TokenKind::Symbol(Symbol::LeftParen)
                | TokenKind::Symbol(Symbol::LeftBracket)
                | TokenKind::Symbol(Symbol::LeftBrace) => depth += 1,
                TokenKind::Symbol(Symbol::RightParen)
                | TokenKind::Symbol(Symbol::RightBracket)
                | TokenKind::Symbol(Symbol::RightBrace) => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            self.advance();
        }

        if start_token == self.current {
            return Err(ParseError::Message("expected expression".to_owned()));
        }

        parse_expression_tokens(self.tokens[start_token..self.current].to_vec())
    }

    fn expect_directive(&mut self, directive: Directive) -> ParseResult<()> {
        if self.at_directive(&directive) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!(
                "expected `{directive:?}` directive"
            )))
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> ParseResult<()> {
        if self.at_keyword(keyword) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!(
                "expected `{}`",
                keyword.text()
            )))
        }
    }

    fn expect_identifier(&mut self, context: &str) -> ParseResult<String> {
        match self.advance().kind {
            TokenKind::Identifier(value) => Ok(value),
            _ => Err(ParseError::Message(format!("expected {context}"))),
        }
    }

    fn expect_value_text(&mut self, context: &str) -> ParseResult<String> {
        self.advance()
            .value_text()
            .ok_or_else(|| ParseError::Message(format!("expected {context}")))
    }

    fn expect_bool(&mut self, context: &str) -> ParseResult<bool> {
        if self.at_keyword(Keyword::True) {
            self.advance();
            Ok(true)
        } else if self.at_keyword(Keyword::False) {
            self.advance();
            Ok(false)
        } else {
            Err(ParseError::Message(format!("expected boolean {context}")))
        }
    }

    fn expect_u8(&mut self, context: &str) -> ParseResult<u8> {
        let text = self.expect_value_text(context)?;
        text.parse::<u8>()
            .map_err(|_| ParseError::Message(format!("expected integer {context}")))
    }

    fn expect_u32(&mut self, context: &str) -> ParseResult<u32> {
        let text = self.expect_value_text(context)?;
        text.parse::<u32>()
            .map_err(|_| ParseError::Message(format!("expected integer {context}")))
    }

    fn expect_symbol(&mut self, symbol: Symbol) -> ParseResult<()> {
        if self.match_symbol(symbol) {
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected `{}`", symbol.char())))
        }
    }

    fn expect_kind(&mut self, kind: TokenKind, label: &str) -> ParseResult<()> {
        if self.at(&kind) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected {label}")))
        }
    }

    fn match_symbol(&mut self, symbol: Symbol) -> bool {
        if self.at_symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_keyword(&mut self, keyword: Keyword) -> bool {
        if self.at_keyword(keyword) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn at_directive(&self, directive: &Directive) -> bool {
        matches!(&self.peek().kind, TokenKind::Directive(candidate) if candidate == directive)
    }

    fn at_keyword(&self, keyword: Keyword) -> bool {
        self.at_kind(&TokenKind::Keyword(keyword))
    }

    fn at_symbol(&self, symbol: Symbol) -> bool {
        self.at_kind(&TokenKind::Symbol(symbol))
    }

    fn at_identifier(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Identifier(_))
    }

    fn at_number(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Integer(_) | TokenKind::Float(_)
        )
    }

    fn at(&self, kind: &TokenKind) -> bool {
        &self.peek().kind == kind
    }

    fn at_kind(&self, kind: &TokenKind) -> bool {
        self.at(kind)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if token.kind != TokenKind::Eof {
            self.current += 1;
        }
        token
    }

    fn unexpected<T>(&self, expected: &str) -> ParseResult<T> {
        Err(ParseError::Message(format!(
            "expected {expected}, found {:?}",
            self.peek().kind
        )))
    }
}

fn parse_expression_tokens(mut tokens: Vec<Token>) -> ParseResult<ExprAst> {
    let eof_start = tokens.last().map_or(0, |token| token.span.end);
    tokens.push(Token::eof(eof_start));
    ExprParser::new(tokens).parse()
}

struct ParsedExpr {
    ast: ExprAst,
    span: Span,
}

struct ExprParser {
    tokens: Vec<Token>,
    current: usize,
}

impl ExprParser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse(&mut self) -> ParseResult<ExprAst> {
        let expression = self.parse_expression()?;
        if !self.at(&TokenKind::Eof) {
            return self.unexpected("end of expression");
        }
        Ok(expression.ast)
    }

    fn parse_expression(&mut self) -> ParseResult<ParsedExpr> {
        self.parse_binary(0)
    }

    fn parse_binary(&mut self, min_precedence: u8) -> ParseResult<ParsedExpr> {
        let mut left = self.parse_unary()?;

        while let Some((op, precedence)) = self.current_binary_op() {
            if precedence < min_precedence {
                break;
            }

            self.advance();
            let right = self.parse_binary(precedence + 1)?;
            let span = Span {
                start: left.span.start,
                end: right.span.end,
            };
            left = self.expr(
                ExprKindAst::Binary {
                    left: Box::new(left.ast),
                    op,
                    right: Box::new(right.ast),
                },
                span,
            );
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> ParseResult<ParsedExpr> {
        if self.at_symbol(Symbol::Minus) {
            let start = self.advance().span.start;
            let expression = self.parse_unary()?;
            return Ok(self.expr(
                ExprKindAst::Unary {
                    op: UnaryOpAst::Neg,
                    expression: Box::new(expression.ast),
                },
                Span {
                    start,
                    end: expression.span.end,
                },
            ));
        }

        if self.at_symbol(Symbol::Bang) {
            let start = self.advance().span.start;
            let expression = self.parse_unary()?;
            return Ok(self.expr(
                ExprKindAst::Unary {
                    op: UnaryOpAst::Not,
                    expression: Box::new(expression.ast),
                },
                Span {
                    start,
                    end: expression.span.end,
                },
            ));
        }

        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> ParseResult<ParsedExpr> {
        let mut expression = self.parse_primary()?;

        loop {
            if self.match_symbol(Symbol::Dot) {
                let field = self.expect_identifier("field name")?;
                let span = Span {
                    start: expression.span.start,
                    end: self.previous().span.end,
                };
                expression = self.expr(
                    ExprKindAst::Field {
                        base: Box::new(expression.ast),
                        field,
                    },
                    span,
                );
            } else if self.match_symbol(Symbol::LeftParen) {
                let mut arguments = Vec::new();
                if !self.at_symbol(Symbol::RightParen) {
                    loop {
                        arguments.push(self.parse_expression()?.ast);
                        if !self.match_symbol(Symbol::Comma) {
                            break;
                        }
                    }
                }
                self.expect_symbol(Symbol::RightParen)?;
                let span = Span {
                    start: expression.span.start,
                    end: self.previous().span.end,
                };
                expression = self.expr(
                    ExprKindAst::Call {
                        callee: Box::new(expression.ast),
                        arguments,
                    },
                    span,
                );
            } else {
                return Ok(expression);
            }
        }
    }

    fn parse_primary(&mut self) -> ParseResult<ParsedExpr> {
        if self.at_keyword(Keyword::Sum) {
            return self.parse_reduction();
        }

        if self.at_number() {
            let token = self.advance();
            let span = token.span;
            let literal = match token.kind {
                TokenKind::Integer(value) => LiteralAst::Integer(value),
                TokenKind::Float(value) => LiteralAst::Float(value),
                _ => unreachable!(),
            };
            return Ok(self.expr(ExprKindAst::Literal(literal), span));
        }

        if self.at_keyword(Keyword::True) {
            let token = self.advance();
            return Ok(self.expr(ExprKindAst::Literal(LiteralAst::Bool(true)), token.span));
        }

        if self.at_keyword(Keyword::False) {
            let token = self.advance();
            return Ok(self.expr(ExprKindAst::Literal(LiteralAst::Bool(false)), token.span));
        }

        if self.at_identifier() {
            let token = self.advance();
            let span = token.span;
            let TokenKind::Identifier(name) = token.kind else {
                unreachable!();
            };
            return Ok(self.expr(ExprKindAst::Name(name), span));
        }

        if self.match_symbol(Symbol::LeftParen) {
            let start = self.previous().span.start;
            let expression = self.parse_expression()?;
            self.expect_symbol(Symbol::RightParen)?;
            return Ok(self.expr(
                expression.ast.kind,
                Span {
                    start,
                    end: self.previous().span.end,
                },
            ));
        }

        self.unexpected("expression")
    }

    fn parse_reduction(&mut self) -> ParseResult<ParsedExpr> {
        let start = self.expect_keyword(Keyword::Sum)?.start;
        let binding = self.expect_identifier("reduction binding")?;
        self.expect_keyword(Keyword::In)?;
        let iterable = self.parse_expression()?;
        self.expect_symbol(Symbol::LeftBrace)?;
        let body = self.parse_expression()?;
        self.expect_symbol(Symbol::RightBrace)?;
        let end = self.previous().span.end;

        Ok(self.expr(
            ExprKindAst::Reduction {
                op: ReductionOpAst::Sum,
                binding,
                iterable: Box::new(iterable.ast),
                body: Box::new(body.ast),
            },
            Span { start, end },
        ))
    }

    fn current_binary_op(&self) -> Option<(BinaryOpAst, u8)> {
        match &self.peek().kind {
            TokenKind::OrOr => Some((BinaryOpAst::Or, 1)),
            TokenKind::AndAnd => Some((BinaryOpAst::And, 2)),
            TokenKind::EqEq => Some((BinaryOpAst::Eq, 3)),
            TokenKind::BangEq => Some((BinaryOpAst::NotEq, 3)),
            TokenKind::LessEq => Some((BinaryOpAst::LessEq, 4)),
            TokenKind::GreaterEq => Some((BinaryOpAst::GreaterEq, 4)),
            TokenKind::Symbol(Symbol::Less) => Some((BinaryOpAst::Less, 4)),
            TokenKind::Symbol(Symbol::Greater) => Some((BinaryOpAst::Greater, 4)),
            TokenKind::Symbol(Symbol::Plus) => Some((BinaryOpAst::Add, 5)),
            TokenKind::Symbol(Symbol::Minus) => Some((BinaryOpAst::Sub, 5)),
            TokenKind::Symbol(Symbol::Star) => Some((BinaryOpAst::Mul, 6)),
            TokenKind::Symbol(Symbol::Slash) => Some((BinaryOpAst::Div, 6)),
            _ => None,
        }
    }

    fn expect_keyword(&mut self, keyword: Keyword) -> ParseResult<Span> {
        if self.at_keyword(keyword) {
            Ok(self.advance().span)
        } else {
            Err(ParseError::Message(format!(
                "expected `{}`",
                keyword.text()
            )))
        }
    }

    fn expect_identifier(&mut self, context: &str) -> ParseResult<String> {
        match self.advance().kind {
            TokenKind::Identifier(value) => Ok(value),
            _ => Err(ParseError::Message(format!("expected {context}"))),
        }
    }

    fn expect_symbol(&mut self, symbol: Symbol) -> ParseResult<()> {
        if self.match_symbol(symbol) {
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected `{}`", symbol.char())))
        }
    }

    fn match_symbol(&mut self, symbol: Symbol) -> bool {
        if self.at_symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn at_keyword(&self, keyword: Keyword) -> bool {
        self.at_kind(&TokenKind::Keyword(keyword))
    }

    fn at_symbol(&self, symbol: Symbol) -> bool {
        self.at_kind(&TokenKind::Symbol(symbol))
    }

    fn at_identifier(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Identifier(_))
    }

    fn at_number(&self) -> bool {
        matches!(
            self.peek().kind,
            TokenKind::Integer(_) | TokenKind::Float(_)
        )
    }

    fn at_kind(&self, kind: &TokenKind) -> bool {
        self.at(kind)
    }

    fn at(&self, kind: &TokenKind) -> bool {
        &self.peek().kind == kind
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> Token {
        let token = self.tokens[self.current].clone();
        if token.kind != TokenKind::Eof {
            self.current += 1;
        }
        token
    }

    fn expr(&self, kind: ExprKindAst, span: Span) -> ParsedExpr {
        ParsedExpr {
            ast: ExprAst { kind },
            span,
        }
    }

    fn unexpected<T>(&self, expected: &str) -> ParseResult<T> {
        Err(ParseError::Message(format!(
            "expected {expected}, found {:?}",
            self.peek().kind
        )))
    }
}
