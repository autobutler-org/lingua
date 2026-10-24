use super::location::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
    // Position just past the token's last character
    pub end: Location,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Eof,
    Newline,
    Wildcard,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
    Equals,
    Arrow,
    PlusEquals,
    EqualsEquals,
    BangEquals,
    LessThan,
    GreaterThan,
    LessEquals,
    GreaterEquals,
    Identifier(String),
    Integer(i64),
    String(Vec<StringPart>),
    Keyword(Keyword),
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringPart {
    Text(String),
    // Raw hole source and where it starts; the parser parses it as `interp-expr`
    Hole { source: String, location: Location },
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Keyword {
    Assert,
    At,
    Catalog,
    Clock,
    Comptime,
    Const,
    Else,
    End,
    Example,
    Exception,
    Fail,
    False,
    Fn,
    For,
    From,
    Fs,
    If,
    Import,
    In,
    IntAtLeast,
    IntIn,
    Is,
    Least,
    Let,
    Match,
    Net,
    Nothing,
    Or,
    Provide,
    Pure,
    Quote,
    Reads,
    Resource,
    Return,
    Set,
    Through,
    True,
    Type,
    Unquote,
    Until,
    Var,
    While,
    Writes,
    Ok,
    Err,
}

const KEYWORDS: [(&str, Keyword); 45] = [
    ("assert", Keyword::Assert),
    ("at", Keyword::At),
    ("catalog", Keyword::Catalog),
    ("clock", Keyword::Clock),
    ("comptime", Keyword::Comptime),
    ("const", Keyword::Const),
    ("else", Keyword::Else),
    ("end", Keyword::End),
    ("example", Keyword::Example),
    ("exception", Keyword::Exception),
    ("fail", Keyword::Fail),
    ("false", Keyword::False),
    ("fn", Keyword::Fn),
    ("for", Keyword::For),
    ("from", Keyword::From),
    ("fs", Keyword::Fs),
    ("if", Keyword::If),
    ("import", Keyword::Import),
    ("in", Keyword::In),
    ("int_at_least", Keyword::IntAtLeast),
    ("int_in", Keyword::IntIn),
    ("is", Keyword::Is),
    ("least", Keyword::Least),
    ("let", Keyword::Let),
    ("match", Keyword::Match),
    ("net", Keyword::Net),
    ("nothing", Keyword::Nothing),
    ("or", Keyword::Or),
    ("provide", Keyword::Provide),
    ("pure", Keyword::Pure),
    ("quote", Keyword::Quote),
    ("reads", Keyword::Reads),
    ("resource", Keyword::Resource),
    ("return", Keyword::Return),
    ("set", Keyword::Set),
    ("through", Keyword::Through),
    ("true", Keyword::True),
    ("type", Keyword::Type),
    ("unquote", Keyword::Unquote),
    ("until", Keyword::Until),
    ("var", Keyword::Var),
    ("while", Keyword::While),
    ("writes", Keyword::Writes),
    ("Ok", Keyword::Ok),
    ("Err", Keyword::Err),
];

impl Keyword {
    #[must_use]
    pub fn from_spelling(spelling: &str) -> Option<Self> {
        KEYWORDS
            .iter()
            .find(|(candidate, _)| *candidate == spelling)
            .map(|(_, keyword)| *keyword)
    }

    #[must_use]
    pub fn spelling(self) -> &'static str {
        KEYWORDS
            .iter()
            .find(|(_, keyword)| *keyword == self)
            .map_or("", |(spelling, _)| spelling)
    }
}

// One CLI and golden-file line: start position, then the kind
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{} {:?}",
            self.location.line, self.location.column, self.kind
        )
    }
}
