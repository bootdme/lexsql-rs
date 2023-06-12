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

#[derive(Debug)]
enum State {
    Start,
    Identifier,
    Literal,
    StringLiteral,
}

#[derive(Debug)]
struct Lexer<'a> {
    input: &'a str,
    chars: CharIndices<'a>,
    current_char: Option<(usize, char)>,
    state: State,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.char_indices();
        let current_char = chars.next();
        Lexer {
            input,
            chars,
            current_char,
            state: State::Start,
        }
    }

    fn bump(&mut self) {
        self.current_char = self.chars.next();
    }
}

fn main() {
    let input = "test";
    let mut lexer = Lexer::new(input);

    println!("{:?}", lexer);
}
