/// Byte span in a source file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    /// Inclusive start byte.
    pub start: usize,
    /// Exclusive end byte.
    pub end: usize,
}

impl Span {
    /// Creates a byte span.
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

/// Token emitted by the lexer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    /// Token kind and lexical payload.
    pub kind: TokenKind,
    /// Source span.
    pub span: Span,
}

impl Token {
    /// Creates a token from a kind and span.
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Creates an identifier or keyword token from source text.
    pub fn identifier_from_source(source: &str, start: usize, end: usize) -> Self {
        let text = &source[start..end];
        Self::new(TokenKind::from_identifier_text(text), Span::new(start, end))
    }

    /// Creates a numeric literal token from source text.
    pub fn number_from_source(source: &str, start: usize, end: usize) -> Option<Self> {
        let text = &source[start..end];
        TokenKind::from_number_text(text).map(|kind| Self::new(kind, Span::new(start, end)))
    }

    /// Creates a directive token from source text.
    pub fn directive_from_source(source: &str, start: usize, end: usize) -> Self {
        let name_start = start + DIRECTIVE_PREFIX.len_utf8();
        let name = &source[name_start..end];
        Self::new(
            TokenKind::Directive(Directive::from_name(name)),
            Span::new(start, end),
        )
    }

    /// Creates a string token whose text excludes delimiters.
    pub fn string_from_source(source: &str, start: usize, end: usize) -> Self {
        let content_start = start + STRING_DELIMITER.len_utf8();
        let content_end = end - STRING_DELIMITER.len_utf8();
        Self::new(
            TokenKind::StringLiteral(source[content_start..content_end].to_owned()),
            Span::new(start, end),
        )
    }

    /// Creates a fixed-spelling token.
    pub fn fixed(kind: TokenKind, start: usize, end: usize) -> Self {
        Self::new(kind, Span::new(start, end))
    }

    /// Creates an end-of-file token at the provided byte offset.
    pub fn eof(offset: usize) -> Self {
        Self::new(TokenKind::Eof, Span::new(offset, offset))
    }

    /// Returns parser-facing value text when this token carries one.
    pub fn value_text(&self) -> Option<String> {
        match &self.kind {
            TokenKind::Identifier(value)
            | TokenKind::Integer(value)
            | TokenKind::Float(value)
            | TokenKind::StringLiteral(value) => Some(value.clone()),
            TokenKind::Keyword(keyword) => Some(keyword.text().to_owned()),
            _ => None,
        }
    }
}

/// Lexical token kind.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    /// Identifier.
    Identifier(String),
    /// Reserved language keyword.
    Keyword(Keyword),
    /// Integer literal text.
    Integer(String),
    /// Floating-point literal text.
    Float(String),
    /// String literal without quotes.
    StringLiteral(String),
    /// `#name` directive.
    Directive(Directive),
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
    Symbol(Symbol),
    /// End of input.
    Eof,
}

impl TokenKind {
    /// Returns the token kind for identifier-like source text.
    pub fn from_identifier_text(text: &str) -> Self {
        Keyword::from_text(text).map_or_else(|| Self::Identifier(text.to_owned()), Self::Keyword)
    }

    /// Returns the token kind for numeric source text.
    pub fn from_number_text(text: &str) -> Option<Self> {
        if text.contains(DECIMAL_SEPARATOR) {
            text.parse::<f64>().ok()?;
            Some(Self::Float(text.to_owned()))
        } else {
            text.parse::<i64>().ok()?;
            Some(Self::Integer(text.to_owned()))
        }
    }

    /// Returns the fixed token kind and text at the start of `source`.
    pub fn from_fixed_prefix(source: &str) -> Option<FixedToken> {
        FIXED_TOKENS
            .iter()
            .find(|token| source.starts_with(token.text))
            .cloned()
    }

    /// Returns the single-character symbol token kind for `character`.
    pub fn from_symbol_char(character: char) -> Option<Self> {
        Symbol::from_char(character).map(Self::Symbol)
    }
}

/// `#name` directive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Directive {
    /// `#hyle`.
    Hyle,
    /// `#dimensions`.
    Dimensions,
    /// `#cell`.
    Cell,
    /// Forward-compatible custom directive.
    Custom(String),
}

impl Directive {
    /// Returns the directive matching a source name without the leading prefix.
    pub fn from_name(name: &str) -> Self {
        match name {
            HYLE_DIRECTIVE => Self::Hyle,
            DIMENSIONS_DIRECTIVE => Self::Dimensions,
            CELL_DIRECTIVE => Self::Cell,
            _ => Self::Custom(name.to_owned()),
        }
    }
}

/// Reserved language keyword.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    /// `neighborhood`.
    Neighborhood,
    /// `radius`.
    Radius,
    /// `center`.
    Center,
    /// `metric`.
    Metric,
    /// `model`.
    Model,
    /// `resolution`.
    Resolution,
    /// `range`.
    Range,
    /// `fields`.
    Fields,
    /// `in`.
    In,
    /// `when`.
    When,
    /// `let`.
    Let,
    /// `next`.
    Next,
    /// `true`.
    True,
    /// `false`.
    False,
    /// Expression `sum` reduction keyword.
    Sum,
    /// `Int`.
    Int,
    /// `Float`.
    Float,
    /// `Bool`.
    Bool,
    /// `Average`.
    Average,
    /// `Nearest`.
    Nearest,
    /// Sampling `Sum`.
    SamplingSum,
    /// `All`.
    All,
}

impl Keyword {
    /// Returns the source spelling for this keyword.
    pub const fn text(self) -> &'static str {
        match self {
            Self::Neighborhood => "neighborhood",
            Self::Radius => "radius",
            Self::Center => "center",
            Self::Metric => "metric",
            Self::Model => "model",
            Self::Resolution => "resolution",
            Self::Range => "range",
            Self::Fields => "fields",
            Self::In => "in",
            Self::When => "when",
            Self::Let => "let",
            Self::Next => "next",
            Self::True => "true",
            Self::False => "false",
            Self::Sum => "sum",
            Self::Int => "Int",
            Self::Float => "Float",
            Self::Bool => "Bool",
            Self::Average => "Average",
            Self::Nearest => "Nearest",
            Self::SamplingSum => "Sum",
            Self::All => "All",
        }
    }

    /// Returns the keyword matching identifier text.
    pub fn from_text(text: &str) -> Option<Self> {
        KEYWORDS
            .iter()
            .copied()
            .find(|keyword| keyword.text() == text)
    }
}

/// Reserved language keywords recognized by the lexer.
pub const KEYWORDS: &[Keyword] = &[
    Keyword::Neighborhood,
    Keyword::Radius,
    Keyword::Center,
    Keyword::Metric,
    Keyword::Model,
    Keyword::Resolution,
    Keyword::Range,
    Keyword::Fields,
    Keyword::In,
    Keyword::When,
    Keyword::Let,
    Keyword::Next,
    Keyword::True,
    Keyword::False,
    Keyword::Sum,
    Keyword::Int,
    Keyword::Float,
    Keyword::Bool,
    Keyword::Average,
    Keyword::Nearest,
    Keyword::SamplingSum,
    Keyword::All,
];

/// Single-character symbol.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {
    /// `{`.
    LeftBrace,
    /// `}`.
    RightBrace,
    /// `(`.
    LeftParen,
    /// `)`.
    RightParen,
    /// `[`.
    LeftBracket,
    /// `]`.
    RightBracket,
    /// `<`.
    Less,
    /// `>`.
    Greater,
    /// `+`.
    Plus,
    /// `-`.
    Minus,
    /// `*`.
    Star,
    /// `/`.
    Slash,
    /// `=`.
    Equal,
    /// `:`.
    Colon,
    /// `.`.
    Dot,
    /// `;`.
    Semicolon,
    /// `,`.
    Comma,
    /// `!`.
    Bang,
}

impl Symbol {
    /// Returns the symbol for a source character.
    pub fn from_char(character: char) -> Option<Self> {
        SYMBOL_TOKENS
            .iter()
            .copied()
            .find(|symbol| symbol.char() == character)
    }

    /// Returns the source spelling for this symbol.
    pub const fn char(self) -> char {
        match self {
            Self::LeftBrace => '{',
            Self::RightBrace => '}',
            Self::LeftParen => '(',
            Self::RightParen => ')',
            Self::LeftBracket => '[',
            Self::RightBracket => ']',
            Self::Less => '<',
            Self::Greater => '>',
            Self::Plus => '+',
            Self::Minus => '-',
            Self::Star => '*',
            Self::Slash => '/',
            Self::Equal => '=',
            Self::Colon => ':',
            Self::Dot => '.',
            Self::Semicolon => ';',
            Self::Comma => ',',
            Self::Bang => '!',
        }
    }
}

/// A fixed multi-character token spelling.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FixedToken {
    /// Source spelling.
    pub text: &'static str,
    /// Token kind.
    pub kind: TokenKind,
}

/// Fixed multi-character tokens recognized by the lexer.
pub const FIXED_TOKENS: &[FixedToken] = &[
    FixedToken {
        text: "->",
        kind: TokenKind::Arrow,
    },
    FixedToken {
        text: "==",
        kind: TokenKind::EqEq,
    },
    FixedToken {
        text: "!=",
        kind: TokenKind::BangEq,
    },
    FixedToken {
        text: "<=",
        kind: TokenKind::LessEq,
    },
    FixedToken {
        text: ">=",
        kind: TokenKind::GreaterEq,
    },
    FixedToken {
        text: "&&",
        kind: TokenKind::AndAnd,
    },
    FixedToken {
        text: "||",
        kind: TokenKind::OrOr,
    },
];

/// Single-character symbol tokens recognized by the lexer.
pub const SYMBOL_TOKENS: &[Symbol] = &[
    Symbol::LeftBrace,
    Symbol::RightBrace,
    Symbol::LeftParen,
    Symbol::RightParen,
    Symbol::LeftBracket,
    Symbol::RightBracket,
    Symbol::Less,
    Symbol::Greater,
    Symbol::Plus,
    Symbol::Minus,
    Symbol::Star,
    Symbol::Slash,
    Symbol::Equal,
    Symbol::Colon,
    Symbol::Dot,
    Symbol::Semicolon,
    Symbol::Comma,
    Symbol::Bang,
];

/// `hyle` directive name.
pub const HYLE_DIRECTIVE: &str = "hyle";

/// `dimensions` directive name.
pub const DIMENSIONS_DIRECTIVE: &str = "dimensions";

/// `cell` directive name.
pub const CELL_DIRECTIVE: &str = "cell";

/// Character that starts a directive token.
pub const DIRECTIVE_PREFIX: char = '#';

/// Character that separates integer and fractional parts in numeric literals.
pub const DECIMAL_SEPARATOR: char = '.';

/// Character that delimits string literals.
pub const STRING_DELIMITER: char = '"';

/// Character that escapes the following character inside string literals.
pub const STRING_ESCAPE: char = '\\';

/// Prefix that starts a line comment.
pub const LINE_COMMENT_PREFIX: &str = "//";

/// Character that terminates a line comment.
pub const LINE_TERMINATOR: char = '\n';
