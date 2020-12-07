use std::collections::VecDeque;

#[derive(Debug)]
pub enum Node {
    Command(String, Vec<Node>),
    Int(usize),
    Str(String),
    URL(String),
    Name(String),
    Flag(String),
    FlagVal(String, Box<Node>),
    Inline(String),
}

#[derive(Clone, Debug)]
pub enum ParserError {
    BadToken(usize),
    ExpectedWhitespace(usize),
    ExpectedName(usize),
    ExpectedFlag(usize),
    ExpectedInt(usize),
    ExpectedStr(usize),
    EOF(usize),
}

#[derive(PartialEq)]
enum Quote {
    DoubleQuote,
    SingleQuote,
    NoQuote,
}

struct Lexer {
    pub pos: usize,
    code: VecDeque<char>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            pos: 0,
            code: code.chars().collect(),
        }
    }

    fn is_whitespace(c: &char) -> bool {
        match *c {
            ' ' | '\n' | '\t' | '\r' => true,
            _ => false,
        }
    }

    fn is_name(c: &char, index: usize) -> bool {
        match *c {
            x if Lexer::is_whitespace(&x) => false,
            '$' | '`' => false,
            '-' if index == 0 => false,
            _ => true,
        }
    }

    fn is_int(c: &char) -> bool {
        match *c {
            '0'..='9' => true,
            _ => false,
        }
    }

    fn is_flag(c: &char) -> bool {
        match *c {
            x if Lexer::is_whitespace(&x) => false,
            '$' | '`' | '=' => false,
            _ => true,
        }
    }

    fn is_str(c: &char, quote: &Quote) -> bool {
        match *c {
            '"' if *quote == Quote::DoubleQuote => false,
            '\'' if *quote == Quote::SingleQuote => false,
            ' ' if *quote == Quote::NoQuote => false,
            _ => true,
        }
    }

    fn advance(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn try_whitespace(&mut self) -> Option<usize> {
        let mut count = 0;
        while self.code.front().map_or(false, Lexer::is_whitespace) {
            self.code.pop_front();
            count += 1;
        }
        self.advance(count);

        match count {
            0 => None,
            _ => Some(count),
        }
    }

    pub fn expect_whitespace(&mut self) -> Result<usize, ParserError> {
        match self.code.front() {
            None => {
                return Err(ParserError::EOF(self.pos));
            }
            _ => {}
        }

        match self.try_whitespace() {
            None => {
                return Err(ParserError::ExpectedWhitespace(self.pos));
            }
            Some(x) => {
                return Ok(x);
            }
        };
    }

    pub fn try_name(&mut self) -> Option<String> {
        let mut out = String::new();

        match self.code.front() {
            Some('$') => {}
            _ => {
                return None;
            }
        }
        self.advance(1);
        self.code.pop_front();

        let mut count = 0;
        while self
            .code
            .front()
            .map_or(false, |x| Lexer::is_name(x, count))
        {
            out.push(self.code.pop_front().unwrap());
            count += 1;
        }
        self.advance(count);

        match out.len() {
            0 => None,
            _ => Some(out),
        }
    }

    pub fn expect_name(&mut self) -> Result<String, ParserError> {
        match self.code.front() {
            None => {
                return Err(ParserError::EOF(self.pos));
            }
            _ => {}
        }

        match self.try_name() {
            None => {
                return Err(ParserError::ExpectedName(self.pos));
            }
            Some(x) => {
                return Ok(x);
            }
        };
    }

    pub fn try_flag(&mut self) -> Option<String> {
        match self.code.front() {
            Some('-') => {}
            _ => {
                return None;
            }
        }

        let mut out = String::new();
        let mut count = 0;
        while self.code.front().map_or(false, Lexer::is_flag) {
            out.push(self.code.pop_front().unwrap());
            count += 1;
        }
        self.advance(count);

        match self.code.front() {
            Some('=') => {
                self.code.pop_front();
                self.advance(1);
                out += "=";

                match self.try_str() {
                    Some(x) => {
                        out += &x;
                    }

                    _ => {}
                }
            }

            _ => {}
        }

        match out.len() {
            0 => None,
            _ => Some(out),
        }
    }

    pub fn expect_flag(&mut self) -> Result<String, ParserError> {
        match self.code.front() {
            None => {
                return Err(ParserError::EOF(self.pos));
            }
            _ => {}
        }

        match self.try_flag() {
            None => {
                return Err(ParserError::ExpectedFlag(self.pos));
            }
            Some(x) => {
                return Ok(x);
            }
        };
    }

    pub fn try_int(&mut self) -> Option<usize> {
        let mut out = String::new();
        let mut count = 0;
        while self.code.front().map_or(false, Lexer::is_int) {
            out.push(self.code.pop_front().unwrap());
            count += 1;
        }
        self.advance(count);

        match out.len() {
            0 => None,
            _ => Some(out.parse::<usize>().unwrap()),
        }
    }

    pub fn expect_int(&mut self) -> Result<usize, ParserError> {
        match self.code.front() {
            None => {
                return Err(ParserError::EOF(self.pos));
            }
            _ => {}
        }

        match self.try_int() {
            None => {
                return Err(ParserError::ExpectedInt(self.pos));
            }
            Some(x) => {
                return Ok(x);
            }
        };
    }

    pub fn try_str(&mut self) -> Option<String> {
        let mut out = String::new();

        let quote = match self.code.front() {
            Some('"') => Quote::DoubleQuote,
            Some('\'') => Quote::SingleQuote,
            _ => Quote::NoQuote,
        };

        match quote {
            Quote::DoubleQuote | Quote::SingleQuote => {
                self.code.pop_front();
                self.advance(1);
            }
            _ => {}
        }

        let mut count = 0;
        while self
            .code
            .front()
            .map_or(false, |x| Lexer::is_str(x, &quote))
        {
            out.push(self.code.pop_front().unwrap());
            count += 1;
        }
        self.advance(count);

        match quote {
            Quote::NoQuote => {}
            Quote::DoubleQuote if self.code.front() == Some(&'"') => {
                self.code.pop_front();
                self.advance(1);
            }
            Quote::SingleQuote if self.code.front() == Some(&'\'') => {
                self.code.pop_front();
                self.advance(1);
            }
            _ => {
                return None;
            }
        }

        match out.len() {
            0 => None,
            _ => Some(out),
        }
    }

    pub fn expect_str(&mut self) -> Result<String, ParserError> {
        match self.code.front() {
            None => {
                return Err(ParserError::EOF(self.pos));
            }
            _ => {}
        }

        match self.try_str() {
            None => {
                return Err(ParserError::ExpectedStr(self.pos));
            }
            Some(x) => {
                return Ok(x);
            }
        };
    }
}

fn parse_name(lexer: &mut Lexer) -> Result<Node, ParserError> {
    return Ok(Node::Name(lexer.expect_name()?));
}

fn parse_flag(lexer: &mut Lexer) -> Result<Node, ParserError> {
    return Ok(Node::Flag(lexer.expect_flag()?));
}

fn parse_int(lexer: &mut Lexer) -> Result<Node, ParserError> {
    return Ok(Node::Int(lexer.expect_int()?));
}

fn parse_str(lexer: &mut Lexer) -> Result<Node, ParserError> {
    return Ok(Node::Str(lexer.expect_str()?));
}

fn parse_arg(lexer: &mut Lexer) -> Result<Node, ParserError> {
    parse_int(lexer)
        .or(parse_name(lexer))
        .or(parse_flag(lexer))
        .or(parse_str(lexer))
}

fn parse_command(lexer: &mut Lexer) -> Result<Node, ParserError> {
    let name = lexer.expect_str()?;

    let mut args = Vec::new();

    loop {
        lexer.try_whitespace();

        match parse_arg(lexer) {
            Err(ParserError::EOF(_)) => {
                return Ok(Node::Command(name, args));
            }
            Err(e) => {
                return Err(e);
            }
            Ok(x) => {
                args.push(x);
            }
        };
    }
}

pub fn parse(command: String) -> Result<Node, ParserError> {
    let mut lexer = Lexer::new(command);

    parse_command(&mut lexer)
}
