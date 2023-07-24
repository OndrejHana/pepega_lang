#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Identifier(String),
    Int(String),
    Illegal,
    EoF,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Comma,
    LT,
    GT,
    Eq,
    NotEq,
    Semicolon,
    LParen,
    RParen,
    LSquigly,
    RSquigly,
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
}

const KEYWORDS: [(&str, Token); 7] = [
    ("fn", Token::Function),
    ("let", Token::Let),
    ("if", Token::If),
    ("else", Token::Else),
    ("return", Token::Return),
    ("true", Token::True),
    ("false", Token::False),
];

pub struct Lexer {
    input: Vec<u8>,
    char: u8,
    position: usize,
    read_position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Self {
            input: input.into(),
            position: 0,
            read_position: 0,
            char: 0,
        };

        l.read_char();

        return l;
    }

    fn is_letter(char: char) -> bool {
        return char.is_ascii_alphabetic() || char == '_';
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input[self.read_position].into();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.char = 0;
        } else {
            self.char = self.input[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let starting_position = self.position;

        while Lexer::is_letter(self.char.into()) {
            self.read_char();
        }

        let identifier_slice = &self.input[starting_position..self.position];
        return String::from_utf8_lossy(identifier_slice).into();
    }

    fn read_int(&mut self) -> String {
        let starting_position = self.position;

        while self.char.is_ascii_digit() {
            self.read_char();
        }

        let identifier_slice = &self.input[starting_position..self.position];
        return String::from_utf8_lossy(identifier_slice).into();
    }

    fn lookup_identifier(identifier: &str) -> Option<Token> {
        for (keyword, token) in KEYWORDS {
            if identifier == keyword {
                return Some(token);
            }
        }

        return None;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let char: char = self.char.into();

        let token = match char {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Slash,
            '*' => Token::Asterisk,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::Comma,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LSquigly,
            '}' => Token::RSquigly,
            char if Lexer::is_letter(char) => {
                let identifier = self.read_identifier();
                return Lexer::lookup_identifier(&identifier)
                    .unwrap_or(Token::Identifier(identifier));
            }
            char if char.is_ascii_digit() => {
                let value = self.read_int();
                return Token::Int(value);
            }
            '\0' => Token::EoF,
            _ => Token::Illegal,
        };

        self.read_char();
        return token;
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    pub fn test_tokens() {
        let input = "(){},;=";

        let correct = vec![
            Token::LParen,
            Token::RParen,
            Token::LSquigly,
            Token::RSquigly,
            Token::Comma,
            Token::Semicolon,
            Token::Assign,
        ];

        let mut l = Lexer::new(input.into());

        for correct_token in correct {
            let token = l.next_token();

            assert_eq!(token, correct_token);
        }
    }

    #[test]
    pub fn test_tokens2() {
        let input = "let five = 5;
let ten = 10;
    let add = fn(x, y) {
     x + y;
    };
let result = add(five, ten);";

        let correct = vec![
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RParen,
            Token::LSquigly,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RSquigly,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::EoF,
        ];
        let mut l = Lexer::new(input.into());

        for correct_token in correct {
            let token = l.next_token();

            assert_eq!(token, correct_token);
        }
    }

    #[test]
    fn test_tokens3() {
        let input = "let five = 5;
let ten = 10;
    let add = fn(x, y) {
        x + y;
    };
let result = add(five, ten);
!-/*5;
5 < 10 > 5;";

        let correct = vec![
            Token::Let,
            Token::Identifier("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Let,
            Token::Identifier("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Identifier("x".into()),
            Token::Comma,
            Token::Identifier("y".into()),
            Token::RParen,
            Token::LSquigly,
            Token::Identifier("x".into()),
            Token::Plus,
            Token::Identifier("y".into()),
            Token::Semicolon,
            Token::RSquigly,
            Token::Semicolon,
            Token::Let,
            Token::Identifier("result".into()),
            Token::Assign,
            Token::Identifier("add".into()),
            Token::LParen,
            Token::Identifier("five".into()),
            Token::Comma,
            Token::Identifier("ten".into()),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LT,
            Token::Int("10".into()),
            Token::GT,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::EoF,
        ];

        let mut l = Lexer::new(input.into());

        for correct_token in correct {
            let token = l.next_token();

            assert_eq!(token, correct_token);
        }
    }

    #[test]
    fn test_tokens4() {
        let input = "if (5 < 10) {
    return true;
} else {
    return false;
}";

        let correct = vec![
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::LT,
            Token::Int("10".into()),
            Token::RParen,
            Token::LSquigly,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RSquigly,
            Token::Else,
            Token::LSquigly,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RSquigly,
            Token::EoF,
        ];

        let mut l = Lexer::new(input.into());
        for correct_token in correct {
            let token = l.next_token();

            assert_eq!(token, correct_token);
        }
    }

    #[test]
    fn test_tokens5() {
        let input = "10 == 10; 
10 != 9;";

        let correct = vec![
            Token::Int("10".into()),
            Token::Eq,
            Token::Int("10".into()),
            Token::Semicolon,
            Token::Int("10".into()),
            Token::NotEq,
            Token::Int("9".into()),
            Token::Semicolon,
            Token::EoF,
        ];

        let mut l = Lexer::new(input.into());
        for correct_token in correct {
            let token = l.next_token();

            assert_eq!(token, correct_token);
        }
    }
}
