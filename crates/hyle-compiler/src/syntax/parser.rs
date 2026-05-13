use thiserror::Error;

use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::source::SourceFile;
use crate::syntax::ast::{
    BoundsAst, ExprAst, FieldAst, InputAst, LiteralAst, ModelAst, NeighborhoodAst, RuleAst,
    RuleSourceAst, RuleStatementAst, SamplingAst, ScriptAst, TypeAst,
};
use crate::syntax::lexer::{lex, LexError, Token, TokenKind};

/// Parses a full `.hyle` script.
///
/// # Errors
///
/// Returns diagnostics for lexical or syntactic failures.
pub fn parse_script(source: &SourceFile) -> Result<ScriptAst, DiagnosticReport> {
    if source.contents.trim().is_empty() {
        return Err(single_error(source, "source is empty"));
    }

    let tokens = lex(&source.contents).map_err(|error| lex_report(source, error))?;
    Parser::new(source, tokens)
        .parse_script()
        .map_err(|error| single_error(source, error.to_string()))
}

#[derive(Debug, Error, PartialEq, Eq)]
enum ParseError {
    #[error("{0}")]
    Message(String),
}

type ParseResult<T> = Result<T, ParseError>;

struct Parser<'a> {
    source: &'a SourceFile,
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Parser<'a> {
    fn new(source: &'a SourceFile, tokens: Vec<Token>) -> Self {
        Self {
            source,
            tokens,
            current: 0,
        }
    }

    fn parse_script(&mut self) -> ParseResult<ScriptAst> {
        self.expect_directive("hyle")?;
        let version = self.expect_value_text("version")?;

        let mut dimensions = None;
        let mut cell = None;
        let mut neighborhoods = Vec::new();
        let mut models = Vec::new();
        let mut inputs = Vec::new();
        let mut rules = Vec::new();

        while !self.at(TokenKind::Eof) {
            if self.at_directive("dimensions") {
                self.advance();
                dimensions = Some(self.expect_u8("dimensions")?);
            } else if self.at_directive("cell") {
                self.advance();
                cell = Some(self.expect_identifier("cell shape")?);
            } else if self.at_keyword("neighborhood") {
                neighborhoods.push(self.parse_neighborhood()?);
            } else if self.at_keyword("model") {
                models.push(self.parse_model()?);
            } else if self.at_keyword("in") {
                inputs.push(self.parse_input()?);
            } else if self.at_kind(TokenKind::Identifier) {
                rules.push(self.parse_rule()?);
            } else {
                return self.unexpected("top-level item");
            }
        }

        Ok(ScriptAst {
            source_path: self.source.path.clone(),
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
        self.expect_keyword("neighborhood")?;
        let name = self.expect_identifier("neighborhood name")?;
        self.expect_symbol('{')?;

        let mut radius = None;
        let mut center = None;
        let mut metric = None;

        while !self.at_symbol('}') {
            if self.at_keyword("radius") {
                self.advance();
                radius = Some(self.expect_value_text("radius")?);
            } else if self.at_keyword("center") {
                self.advance();
                center = Some(self.expect_bool("center")?);
            } else if self.at_keyword("metric") {
                self.advance();
                metric = Some(self.expect_identifier("metric")?);
            } else {
                return self.unexpected("neighborhood item");
            }
        }
        self.expect_symbol('}')?;

        Ok(NeighborhoodAst {
            name,
            radius: radius.unwrap_or_else(|| "1".to_owned()),
            center: center.unwrap_or(false),
            metric: metric.unwrap_or_else(|| "Manhattan".to_owned()),
        })
    }

    fn parse_model(&mut self) -> ParseResult<ModelAst> {
        self.expect_keyword("model")?;
        let name = self.expect_identifier("model name")?;
        self.expect_symbol('{')?;

        let mut resolution = None;
        let mut range = None;
        let mut fields = Vec::new();

        while !self.at_symbol('}') {
            if self.at_keyword("resolution") {
                self.advance();
                resolution = Some(self.expect_u32("resolution")?);
            } else if self.at_keyword("range") {
                self.advance();
                range = Some(self.expect_identifier("model range")?);
            } else if self.at_keyword("fields") {
                fields = self.parse_fields()?;
            } else {
                return self.unexpected("model item");
            }
        }
        self.expect_symbol('}')?;

        Ok(ModelAst {
            name,
            resolution,
            range,
            fields,
        })
    }

    fn parse_fields(&mut self) -> ParseResult<Vec<FieldAst>> {
        self.expect_keyword("fields")?;
        self.expect_symbol('{')?;

        let mut fields = Vec::new();
        while !self.at_symbol('}') {
            let name = self.expect_identifier("field name")?;
            self.expect_symbol(':')?;
            let ty = self.parse_type()?;
            let default = if self.match_symbol('<') {
                let value = self.parse_literal()?;
                self.expect_symbol('>')?;
                Some(value)
            } else {
                None
            };
            let bounds = if self.at_symbol('[') || self.at_symbol('(') {
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

        self.expect_symbol('}')?;
        Ok(fields)
    }

    fn parse_input(&mut self) -> ParseResult<InputAst> {
        self.expect_keyword("in")?;
        let name = self.expect_identifier("input name")?;
        self.expect_symbol(':')?;
        let ty = self.parse_type()?;
        let default = if self.match_symbol('<') {
            let value = self.parse_literal()?;
            self.expect_symbol('>')?;
            Some(value)
        } else {
            None
        };
        self.expect_symbol(';')?;
        Ok(InputAst { name, ty, default })
    }

    fn parse_rule(&mut self) -> ParseResult<RuleAst> {
        let mut sources = vec![RuleSourceAst {
            model: self.expect_identifier("rule source model")?,
            sampling: None,
        }];

        while self.match_symbol('+') {
            self.expect_symbol('(')?;
            let sampling = self.parse_sampling()?;
            self.expect_symbol(')')?;
            let model = self.expect_identifier("sampled rule source model")?;
            sources.push(RuleSourceAst {
                model,
                sampling: Some(sampling),
            });
        }

        self.expect_kind(TokenKind::Arrow, "`->`")?;
        let output = self.expect_identifier("rule output model")?;

        let mut range = None;
        let mut condition = None;

        if self.at_keyword("range") {
            self.advance();
            range = Some(self.expect_identifier("rule range")?);
        }

        if self.at_keyword("when") {
            self.advance();
            condition = Some(self.collect_expression_until_block()?);
        }

        let statements = self.parse_rule_body()?;

        Ok(RuleAst {
            sources,
            output,
            range,
            condition,
            statements,
        })
    }

    fn parse_rule_body(&mut self) -> ParseResult<Vec<RuleStatementAst>> {
        self.expect_symbol('{')?;
        let mut statements = Vec::new();

        while !self.at_symbol('}') {
            if self.at_keyword("let") {
                self.advance();
                let name = self.expect_identifier("binding name")?;
                self.expect_symbol('=')?;
                let expression = self.collect_expression_until_semicolon()?;
                self.expect_symbol(';')?;
                statements.push(RuleStatementAst::Let { name, expression });
            } else if self.at_keyword("next") {
                self.advance();
                let model = self.expect_identifier("next model")?;
                self.expect_symbol('.')?;
                let field = self.expect_identifier("next field")?;
                self.expect_symbol('=')?;
                let expression = self.collect_expression_until_semicolon()?;
                self.expect_symbol(';')?;
                statements.push(RuleStatementAst::Next {
                    model,
                    field,
                    expression,
                });
            } else {
                return self.unexpected("rule statement");
            }
        }

        self.expect_symbol('}')?;
        Ok(statements)
    }

    fn parse_bounds(&mut self) -> ParseResult<BoundsAst> {
        let lower_inclusive = if self.match_symbol('[') {
            true
        } else {
            self.expect_symbol('(')?;
            false
        };
        let lower = self.expect_value_text("lower bound")?;
        let upper = self.expect_value_text("upper bound")?;
        let upper_inclusive = if self.match_symbol(']') {
            true
        } else {
            self.expect_symbol(')')?;
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
        let name = self.expect_identifier("type")?;
        Ok(match name.as_str() {
            "Int" => TypeAst::Int,
            "Float" => TypeAst::Float,
            "Bool" => TypeAst::Bool,
            _ => TypeAst::Custom(name),
        })
    }

    fn parse_literal(&mut self) -> ParseResult<LiteralAst> {
        if self.at_kind(TokenKind::Number) {
            return Ok(LiteralAst::Number(self.advance().text));
        }

        if self.at_keyword("true") {
            self.advance();
            return Ok(LiteralAst::Bool(true));
        }

        if self.at_keyword("false") {
            self.advance();
            return Ok(LiteralAst::Bool(false));
        }

        self.unexpected("literal")
    }

    fn parse_sampling(&mut self) -> ParseResult<SamplingAst> {
        let name = self.expect_identifier("sampling algorithm")?;
        Ok(match name.as_str() {
            "Average" => SamplingAst::Average,
            "Nearest" => SamplingAst::Nearest,
            "Sum" => SamplingAst::Sum,
            "All" => SamplingAst::All,
            _ => SamplingAst::Custom(name),
        })
    }

    fn collect_expression_until_block(&mut self) -> ParseResult<ExprAst> {
        self.collect_expression(|parser, depth| depth == 0 && parser.at_symbol('{'))
    }

    fn collect_expression_until_semicolon(&mut self) -> ParseResult<ExprAst> {
        self.collect_expression(|parser, depth| depth == 0 && parser.at_symbol(';'))
    }

    fn collect_expression(
        &mut self,
        stop: impl Fn(&Parser<'a>, usize) -> bool,
    ) -> ParseResult<ExprAst> {
        let start = self.peek().span.start;
        let mut end = start;
        let mut depth = 0usize;

        while !self.at(TokenKind::Eof) && !stop(self, depth) {
            match &self.peek().kind {
                TokenKind::Symbol('(') | TokenKind::Symbol('[') | TokenKind::Symbol('{') => {
                    depth += 1;
                }
                TokenKind::Symbol(')') | TokenKind::Symbol(']') | TokenKind::Symbol('}') => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
            end = self.advance().span.end;
        }

        if start == end {
            return Err(ParseError::Message("expected expression".to_owned()));
        }

        Ok(ExprAst {
            text: self.source.contents[start..end].trim().to_owned(),
        })
    }

    fn expect_directive(&mut self, name: &str) -> ParseResult<()> {
        if self.at_directive(name) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected `#{name}` directive")))
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> ParseResult<()> {
        if self.at_keyword(keyword) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected `{keyword}`")))
        }
    }

    fn expect_identifier(&mut self, context: &str) -> ParseResult<String> {
        if self.at_kind(TokenKind::Identifier) {
            Ok(self.advance().text)
        } else {
            Err(ParseError::Message(format!("expected {context}")))
        }
    }

    fn expect_value_text(&mut self, context: &str) -> ParseResult<String> {
        if matches!(
            self.peek().kind,
            TokenKind::Identifier | TokenKind::Number | TokenKind::String
        ) {
            Ok(self.advance().text)
        } else {
            Err(ParseError::Message(format!("expected {context}")))
        }
    }

    fn expect_bool(&mut self, context: &str) -> ParseResult<bool> {
        if self.at_keyword("true") {
            self.advance();
            Ok(true)
        } else if self.at_keyword("false") {
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

    fn expect_symbol(&mut self, symbol: char) -> ParseResult<()> {
        if self.match_symbol(symbol) {
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected `{symbol}`")))
        }
    }

    fn expect_kind(&mut self, kind: TokenKind, label: &str) -> ParseResult<()> {
        if self.at(kind) {
            self.advance();
            Ok(())
        } else {
            Err(ParseError::Message(format!("expected {label}")))
        }
    }

    fn match_symbol(&mut self, symbol: char) -> bool {
        if self.at_symbol(symbol) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn at_directive(&self, name: &str) -> bool {
        self.at_kind(TokenKind::Directive) && self.peek().text == format!("#{name}")
    }

    fn at_keyword(&self, keyword: &str) -> bool {
        self.at_kind(TokenKind::Identifier) && self.peek().text == keyword
    }

    fn at_symbol(&self, symbol: char) -> bool {
        self.at_kind(TokenKind::Symbol(symbol))
    }

    fn at(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn at_kind(&self, kind: TokenKind) -> bool {
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
            "expected {expected}, found `{}`",
            self.peek().text
        )))
    }
}

fn lex_report(source: &SourceFile, error: LexError) -> DiagnosticReport {
    single_error(source, error.to_string())
}

fn single_error(source: &SourceFile, message: impl Into<String>) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(Diagnostic::error(Some(source.path.clone()), message));
    report
}
