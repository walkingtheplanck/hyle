//! Lexical and syntactic analysis for `.hyle` scripts.

use thiserror::Error;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod token;

pub use ast::{
    BinaryOpAst, BoundsAst, ExprAst, ExprKindAst, FieldAst, InputAst, LiteralAst, ModelAst,
    NeighborhoodAst, ReductionOpAst, RuleAst, RuleSourceAst, RuleStatementAst, SamplingAst,
    ScriptAst, TypeAst, UnaryOpAst,
};
pub use lexer::{lex, LexError};
pub use parser::ParseError;
pub use token::{Directive, Keyword, Span, Symbol, Token, TokenKind};

/// Parses source text into a full `.hyle` AST.
///
/// # Errors
///
/// Returns lexical or syntactic failures.
pub fn parse(source: &str) -> Result<ScriptAst, SyntaxError> {
    if source.trim().is_empty() {
        return Err(SyntaxError::EmptySource);
    }

    let tokens = lex(source)?;
    parse_tokens(tokens)
}

/// Parses pre-tokenized source into a full `.hyle` AST.
///
/// # Errors
///
/// Returns syntactic failures.
pub fn parse_tokens(tokens: Vec<Token>) -> Result<ScriptAst, SyntaxError> {
    parser::parse_tokens(tokens).map_err(SyntaxError::Parse)
}

/// Syntax pipeline failure.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SyntaxError {
    /// Source text was blank.
    #[error("source is empty")]
    EmptySource,
    /// Lexical failure.
    #[error(transparent)]
    Lex(#[from] LexError),
    /// Parse failure.
    #[error(transparent)]
    Parse(#[from] ParseError),
}
