use thiserror::Error;

/// Byte span in a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Inclusive start byte.
    pub start: usize,
    /// Exclusive end byte.
    pub end: usize,
}

/// Token emitted by the lexer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// Token kind.
    pub kind: TokenKind,
    /// Original token text.
    pub text: String,
    /// Source span.
    pub span: Span,
}

/// Lexical token kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Identifier or keyword.
    Identifier,
    /// Number literal.
    Number,
    /// String literal without quotes.
    String,
    /// `#name` directive.
    Directive,
    /// `->`.
    Arrow,
    /// `==`.
    EqEq,
    /// `!=`.
    BangEq,
    /// `<=`.
    LessEq,
    /// `>=`.
    GreaterEq,
    /// `&&`.
    AndAnd,
    /// `||`.
    OrOr,
    /// Single-character symbol.
    Symbol(char),
    /// End of input.
    Eof,
}

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
            return Ok(Token {
                kind: TokenKind::Eof,
                text: String::new(),
                span: Span {
                    start: self.offset,
                    end: self.offset,
                },
            });
        };

        if is_ident_start(character) {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if is_ident_part(next)) {
                self.bump_char();
            }
            return Ok(self.token(TokenKind::Identifier, start, self.offset));
        }

        if character.is_ascii_digit()
            || (character == '.'
                && matches!(self.peek_next_char(), Some(next) if next.is_ascii_digit()))
        {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if next.is_ascii_digit() || next == '.')
            {
                self.bump_char();
            }
            return Ok(self.token(TokenKind::Number, start, self.offset));
        }

        if character == '#' {
            self.bump_char();
            while matches!(self.peek_char(), Some((_, next)) if is_ident_part(next)) {
                self.bump_char();
            }
            return Ok(self.token(TokenKind::Directive, start, self.offset));
        }

        if character == '"' {
            self.bump_char();
            let mut escaped = false;
            while let Some((_, next)) = self.peek_char() {
                self.bump_char();
                if escaped {
                    escaped = false;
                } else if next == '\\' {
                    escaped = true;
                } else if next == '"' {
                    let mut token = self.token(TokenKind::String, start, self.offset);
                    token.text = token.text[1..token.text.len() - 1].to_owned();
                    return Ok(token);
                }
            }
            return Err(LexError::UnterminatedString);
        }

        let two = self.source[start..].chars().take(2).collect::<String>();
        let two_kind = match two.as_str() {
            "->" => Some(TokenKind::Arrow),
            "==" => Some(TokenKind::EqEq),
            "!=" => Some(TokenKind::BangEq),
            "<=" => Some(TokenKind::LessEq),
            ">=" => Some(TokenKind::GreaterEq),
            "&&" => Some(TokenKind::AndAnd),
            "||" => Some(TokenKind::OrOr),
            _ => None,
        };
        if let Some(kind) = two_kind {
            self.bump_char();
            self.bump_char();
            return Ok(self.token(kind, start, self.offset));
        }

        if "{}()[]<>+-*/=:.;,.".contains(character) {
            self.bump_char();
            return Ok(self.token(TokenKind::Symbol(character), start, self.offset));
        }

        Err(LexError::UnexpectedCharacter {
            character,
            span: Span {
                start,
                end: start + character.len_utf8(),
            },
        })
    }

    fn skip_trivia(&mut self) {
        loop {
            while matches!(self.peek_char(), Some((_, character)) if character.is_whitespace()) {
                self.bump_char();
            }

            if self.source[self.offset..].starts_with("//") {
                while let Some((_, character)) = self.peek_char() {
                    self.bump_char();
                    if character == '\n' {
                        break;
                    }
                }
                continue;
            }

            break;
        }
    }

    fn token(&self, kind: TokenKind, start: usize, end: usize) -> Token {
        Token {
            kind,
            text: self.source[start..end].to_owned(),
            span: Span { start, end },
        }
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
