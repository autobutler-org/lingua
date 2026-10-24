use super::location::Location;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum StringPart {
    Text(String),
    // Raw hole source; the parser parses it as `cmp-expr`
    Hole(String),
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

impl Keyword {
    #[must_use]
    pub fn from_spelling(spelling: &str) -> Option<Self> {
        Some(match spelling {
            "assert" => Self::Assert,
            "at" => Self::At,
            "catalog" => Self::Catalog,
            "clock" => Self::Clock,
            "comptime" => Self::Comptime,
            "const" => Self::Const,
            "else" => Self::Else,
            "end" => Self::End,
            "example" => Self::Example,
            "exception" => Self::Exception,
            "fail" => Self::Fail,
            "false" => Self::False,
            "fn" => Self::Fn,
            "for" => Self::For,
            "from" => Self::From,
            "fs" => Self::Fs,
            "if" => Self::If,
            "import" => Self::Import,
            "in" => Self::In,
            "int_at_least" => Self::IntAtLeast,
            "int_in" => Self::IntIn,
            "is" => Self::Is,
            "least" => Self::Least,
            "let" => Self::Let,
            "match" => Self::Match,
            "net" => Self::Net,
            "nothing" => Self::Nothing,
            "or" => Self::Or,
            "provide" => Self::Provide,
            "pure" => Self::Pure,
            "quote" => Self::Quote,
            "reads" => Self::Reads,
            "resource" => Self::Resource,
            "return" => Self::Return,
            "set" => Self::Set,
            "through" => Self::Through,
            "true" => Self::True,
            "type" => Self::Type,
            "unquote" => Self::Unquote,
            "until" => Self::Until,
            "var" => Self::Var,
            "while" => Self::While,
            "writes" => Self::Writes,
            "Ok" => Self::Ok,
            "Err" => Self::Err,
            _ => return None,
        })
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
