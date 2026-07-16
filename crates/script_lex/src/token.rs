//! Lexical token kinds for Phase 1 language surface.
//!
//! Spec: `PHASE-1-LANGUAGE-SPEC.md` §5, §6 (lexical portion)

use crate::span::Span;

/// Reserved keywords (defined + reserved-for-future).
/// Contextual words (`case`, `test`, …) are lexed as identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    And,
    As,
    Assert,
    Async,
    Await,
    Break,
    Catch,
    Class,
    Const,
    Continue,
    Def,
    Defer,
    Elif,
    Else,
    Enum,
    Export,
    False,
    Field,
    Finally,
    For,
    From,
    Global,
    If,
    Import,
    In,
    Is,
    Lambda,
    Let,
    Match,
    Mutable,
    Nil,
    Nonlocal,
    Not,
    Or,
    Raise,
    Readonly,
    Record,
    Return,
    Static,
    Trait,
    True,
    Try,
    Type,
    Use,
    Where,
    While,
    Yield,
}

impl Keyword {
    #[must_use]
    pub fn from_ident(s: &str) -> Option<Self> {
        Some(match s {
            "and" => Self::And,
            "as" => Self::As,
            "assert" => Self::Assert,
            "async" => Self::Async,
            "await" => Self::Await,
            "break" => Self::Break,
            "catch" => Self::Catch,
            "class" => Self::Class,
            "const" => Self::Const,
            "continue" => Self::Continue,
            "def" => Self::Def,
            "defer" => Self::Defer,
            "elif" => Self::Elif,
            "else" => Self::Else,
            "enum" => Self::Enum,
            "export" => Self::Export,
            "false" => Self::False,
            "field" => Self::Field,
            "finally" => Self::Finally,
            "for" => Self::For,
            "from" => Self::From,
            "global" => Self::Global,
            "if" => Self::If,
            "import" => Self::Import,
            "in" => Self::In,
            "is" => Self::Is,
            "lambda" => Self::Lambda,
            "let" => Self::Let,
            "match" => Self::Match,
            "mutable" => Self::Mutable,
            "nil" => Self::Nil,
            "nonlocal" => Self::Nonlocal,
            "not" => Self::Not,
            "or" => Self::Or,
            "raise" => Self::Raise,
            "readonly" => Self::Readonly,
            "record" => Self::Record,
            "return" => Self::Return,
            "static" => Self::Static,
            "trait" => Self::Trait,
            "true" => Self::True,
            "try" => Self::Try,
            "type" => Self::Type,
            "use" => Self::Use,
            "where" => Self::Where,
            "while" => Self::While,
            "yield" => Self::Yield,
            _ => return None,
        })
    }

    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::And => "and",
            Self::As => "as",
            Self::Assert => "assert",
            Self::Async => "async",
            Self::Await => "await",
            Self::Break => "break",
            Self::Catch => "catch",
            Self::Class => "class",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Def => "def",
            Self::Defer => "defer",
            Self::Elif => "elif",
            Self::Else => "else",
            Self::Enum => "enum",
            Self::Export => "export",
            Self::False => "false",
            Self::Field => "field",
            Self::Finally => "finally",
            Self::For => "for",
            Self::From => "from",
            Self::Global => "global",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Is => "is",
            Self::Lambda => "lambda",
            Self::Let => "let",
            Self::Match => "match",
            Self::Mutable => "mutable",
            Self::Nil => "nil",
            Self::Nonlocal => "nonlocal",
            Self::Not => "not",
            Self::Or => "or",
            Self::Raise => "raise",
            Self::Readonly => "readonly",
            Self::Record => "record",
            Self::Return => "return",
            Self::Static => "static",
            Self::Trait => "trait",
            Self::True => "true",
            Self::Try => "try",
            Self::Type => "type",
            Self::Use => "use",
            Self::Where => "where",
            Self::While => "while",
            Self::Yield => "yield",
        }
    }
}

/// Token kind produced by the Phase 1 lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident {
        /// NFC-normalized spelling used for comparison.
        nfc: String,
        /// Original source spelling.
        raw: String,
    },
    Keyword(Keyword),
    /// Decimal integer digits without underscores.
    Int(String),
    /// Float lexeme as written (normalized underscore-free digits).
    Float(String),
    /// String content after escape processing.
    String(String),

    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Assign,
    PlusAssign,
    MinusAssign,
    StarAssign,
    SlashAssign,
    PercentAssign,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Colon,
    Dot,
    DotDot,
    Arrow,
    Pipe,
    Question,

    Newline,
    Indent,
    Dedent,
    Eof,
}

/// A single token with source span (byte offsets into UTF-8 source).
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[must_use]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
