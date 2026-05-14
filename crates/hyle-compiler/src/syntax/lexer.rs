use thiserror::Error;

use crate::syntax::token::{
    Span, Token, TokenKind, DECIMAL_SEPARATOR, DIRECTIVE_PREFIX, LINE_COMMENT_PREFIX,
    LINE_TERMINATOR, STRING_DELIMITER, STRING_ESCAPE,
};

/// Lexing failure.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum LexError {
    /// A string literal was not closed.
    #[error("unterminated string literal")]
    UnterminatedString,
    /// A character is not part of the language.
    #[error("unexpected character `{character}`")]
    UnexpectedCharacter {
        /// Unexpected character.
        character: char,
        /// Byte span.
        span: Span,
    },
    /// A numeric literal was malformed.
    #[error("invalid numeric literal")]
    InvalidNumber {
        /// Byte span.
        span: Span,
    },
}

/// Lexes a `.hyle` source string.
///
/// Comments and whitespace are skipped. Comments before `#hyle` are therefore
/// naturally accepted by the parser.
pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer { source, offset: 0 };
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token()?;
        let eof = token.kind == TokenKind::Eof;
        tokens.push(token);
        if eof {
            return Ok(tokens);
        }
    }
}

struct Lexer<'a> {
    source: &'a str,
    offset: usize,
}

impl Lexer<'_> {
    fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_trivia();

        let Some((start, character)) = self.peek_char() else {
            return Ok(Token::eof(self.offset));
        };

        if is_ident_start(character) {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if is_ident_part(next)) {
                self.bump_char();
            }
            return Ok(Token::identifier_from_source(
                self.source,
                start,
                self.offset,
            ));
        }

        if character.is_ascii_digit()
            || (character == DECIMAL_SEPARATOR
                && matches!(self.peek_next_char(), Some(next) if next.is_ascii_digit()))
        {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if next.is_ascii_digit() || next == DECIMAL_SEPARATOR)
            {
                self.bump_char();
            }
            return Token::number_from_source(self.source, start, self.offset).ok_or(
                LexError::InvalidNumber {
                    span: Span::new(start, self.offset),
                },
            );
        }

        if character == DIRECTIVE_PREFIX {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if is_ident_part(next)) {
                self.bump_char();
            }
            return Ok(Token::directive_from_source(
                self.source,
                start,
                self.offset,
            ));
        }

        if character == STRING_DELIMITER {
            self.bump_char();
            let mut escaped = false;
            while let Some((_, next)) = self.peek_char() {
                self.bump_char();
                if escaped {
                    escaped = false;
                } else if next == STRING_ESCAPE {
                    escaped = true;
                } else if next == STRING_DELIMITER {
                    return Ok(Token::string_from_source(self.source, start, self.offset));
                }
            }
            return Err(LexError::UnterminatedString);
        }

        if let Some(fixed) = TokenKind::from_fixed_prefix(&self.source[start..]) {
            for _ in fixed.text.chars() {
                self.bump_char();
            }
            return Ok(self.token(fixed.kind, start, self.offset));
        }

        if let Some(kind) = TokenKind::from_symbol_char(character) {
            self.bump_char();
            return Ok(self.token(kind, start, self.offset));
        }

        Err(LexError::UnexpectedCharacter {
            character,
            span: Span::new(start, start + character.len_utf8()),
        })
    }

    fn skip_trivia(&mut self) {
        loop {
            while matches!(self.peek_char(), Some((_, character)) if character.is_whitespace()) {
                self.bump_char();
            }

            if self.source[self.offset..].starts_with(LINE_COMMENT_PREFIX) {
                while let Some((_, character)) = self.peek_char() {
                    self.bump_char();
                    if character == LINE_TERMINATOR {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn token(&self, kind: TokenKind, start: usize, end: usize) -> Token {
        Token::fixed(kind, start, end)
    }

    fn peek_char(&self) -> Option<(usize, char)> {
        self.source[self.offset..]
            .char_indices()
            .next()
            .map(|(relative, character)| (self.offset + relative, character))
    }

    fn peek_next_char(&self) -> Option<char> {
        let mut chars = self.source[self.offset..].chars();
        chars.next()?;
        chars.next()
    }

    fn bump_char(&mut self) -> Option<char> {
        let (_, character) = self.peek_char()?;
        self.offset += character.len_utf8();
        Some(character)
    }
}

fn is_ident_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_ident_part(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}
