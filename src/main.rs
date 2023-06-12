use std::str::CharIndices;

#[derive(Debug, PartialEq)]
enum Token {
    Select,
    From,
    Where,
    And,
    As,
    In,
    Dot,
    Equal,
    Insert,
    Into,
    Values,
    Inner,
    Left,
    Right,
    Join,
    On,
    LParen,
    RParen,
    Create,
    Database,
    Table,
    Identifier(String),
    Literal(String),
    StringLiteral(String),
    Unknown(char),
}

enum State {
    Start,
    Identifier,
    Literal,
    StringLiteral,
}

struct Lexer<'a> {
    input: &'a str,
    chars: CharIndices<'a>,
    current_char: Option<(usize, char)>,
    state: State,
}

fn main() {}
