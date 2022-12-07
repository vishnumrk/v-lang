#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Equals,
    BinaryOperator,
    Number,
    Identifier,
    Let,
    Const,
    Eof,
    Semicolon,
    Colon,
    Comma,
    Dot,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Equals,
    BinaryOperator(char),
    Number(isize),
    Identifier(String),
    Let,
    Const,
    Eof,
    Semicolon,
    Colon,
    Comma,
    Dot,
}

impl Token {
    pub(crate) fn kind(&self) -> TokenKind {
        match self {
            Token::OpenParen => TokenKind::OpenParen,
            Token::CloseParen => TokenKind::CloseParen,
            Token::Equals => TokenKind::Equals,
            Token::BinaryOperator(_) => TokenKind::BinaryOperator,
            Token::Number(_) => TokenKind::Number,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::Let => TokenKind::Let,
            Token::Const => TokenKind::Const,
            Token::Eof => TokenKind::Eof,
            Token::Semicolon => TokenKind::Semicolon,
            Token::OpenBrace => TokenKind::OpenBrace,
            Token::CloseBrace => TokenKind::CloseBrace,
            Token::Colon => TokenKind::Colon,
            Token::Comma => TokenKind::Comma,
            Token::OpenBracket => TokenKind::OpenBracket,
            Token::CloseBracket => TokenKind::CloseBracket,
            Token::Dot => TokenKind::Dot,
        }
    }
}
