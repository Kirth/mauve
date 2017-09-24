use regex::Regex;


pub type LineNumber = u32;
pub type ColumnNumber = u32;

// Todo: implement compile time macroes for this.
static KEYWORDS: &'static [&'static str] = &["add", "sub", "mul", "div", "rem", "shl", "shr",
    "or", "and", "xor", "br", "bge", "bgt", "blt", "ble", "jmp", "ret", "ceq", "cgt", "clt",
    "call", "cp", "dup", "ldc", "nop", "push", "pop"];

#[derive(Debug, Clone)]
pub enum TokenType
{
    Keyword,
    LeftBrace,
    RightBrace,
    Integer,
    Numeric,
    Identifier,
    LeftCurly,
    RightCurly,
    Dot,
    Colon,
    NewLine,
    Unknown
}

#[derive(Debug, Clone)]
pub struct Token
{
    line: LineNumber,
    column: ColumnNumber,
    token_type: TokenType,
    str_value: Option<String>,
}

#[derive(Debug)]
pub struct Scanner
{
    input: String,
    position: usize,
    current_column: ColumnNumber,
    current_line: LineNumber,
    current_token: Option<Token>
}

impl Scanner
{
    pub fn is_eof(&self) -> bool
    {
        return self.position >= self.input.len()
    }

    pub fn peek(&self) -> Option<char>
    {
        if self.is_eof() {
            return None;
        }

        // chars is an iterator starting from the slice[pos..]
        // so calling it on every peek is relatively cheap.
        return self.input[self.position ..].chars().next()
    }

    pub fn consume_char(&mut self) -> Option<char>
    {
        let mut iter = self.input[self.position ..].char_indices();
        if let Some((_, c)) = iter.next() {
            let (n, _) = iter.next().unwrap_or((1, ' ')); // add that last position to
            self.position += n;                           // clock in is_eof

            if c != '\n' {
                self.current_column += 1;
            } else {
                self.current_column = 1;
                self.current_line += 1;
            }

            return Some(c);
        }

        return None;
    }

    pub fn consume_while<F>(&mut self, test: F) -> String
        where F: Fn(char) -> bool
    {
        let mut result = String::new();

        while !self.is_eof() && test(self.peek().unwrap()) {
            if let Some(c) = self.consume_char() {
                result.push(c);
            }
        }

        return result;
    }

    pub fn consume_whitespace(&mut self) {
        self.consume_while(|c| c == ' ' || c == '\t' || c == '\r');
    }

    pub fn consume_word(&mut self) -> String {
        self.consume_while(|c| c != ' ' && c != '\t' && c != '\r'
            && c != '\n' && c != '[' && c != ']' && c != '{' && c != '}'
            && c != '.' && c != ':')
    }

    pub fn str_tokentype(str: &str) -> TokenType
    {
        if str.len() == 1 {
            if str.chars().next().unwrap() == '\n' { return TokenType::NewLine; }
            if str.chars().next().unwrap() == '[' { return TokenType::LeftBrace; }
            if str.chars().next().unwrap() == ']' { return TokenType::RightBrace; }
            if str.chars().next().unwrap() == '{' { return TokenType::LeftCurly; }
            if str.chars().next().unwrap() == '}' { return TokenType::RightCurly; }
            if str.chars().next().unwrap() == '.' { return TokenType::Dot; }
            if str.chars().next().unwrap() == ':' { return TokenType::Colon; }
        }

        if KEYWORDS.iter().any(|v| v == &str.to_lowercase()) {
            return TokenType::Keyword;
        }

        let ident_regex = Regex::new(r"[a-zA-Z][a-zA-Z0-9]*").unwrap();
        if ident_regex.is_match(str) {
            return TokenType::Identifier;
        }

        let num_regex = Regex::new(r"[0-9]+").unwrap();
        if num_regex.is_match(str) {
            return TokenType::Integer;
        }

        return TokenType::Unknown;
    }

    pub fn consume_token(&mut self) -> Option<Token>
    {
        if !self.current_token.is_none() {
            let token = self.current_token.clone();
            self.current_token = None;

            return token;
        }

        self.consume_whitespace();

        if self.is_eof() {
            return None;
        }

        let line = self.current_line;
        let col = self.current_column;
        //let pos = self.position;

        let value = match self.peek().unwrap() {
            '[' | ']' | '{' | '}' | ':' | '.' | '\n' => self.consume_char().unwrap().to_string(),
            _ => self.consume_word()
        };  /* && c != '\n'
            && c != ']' && c != '}' && c != '[' && c != '{' && c != '.' && c != ':' */

        let token_type = Scanner::str_tokentype(&value);

        return Some(Token {
            line: line,
            column: col,
            token_type: token_type,
            str_value: Some(value),
        });
    }

    pub fn peek_token(&mut self) -> Option<Token> {
        if self.current_token.is_none() {
            self.current_token = self.consume_token();
        }

        return self.current_token.clone();
    }

    pub fn from(input: String) -> Scanner {
        Scanner {
            input: input,
            position: 0,
            current_column: 1,
            current_line: 1,
            current_token: None,
        }
    }
}

enum Argument
{
    Register(String),
    Label(String),
    Int(i32)
}

enum Operation
{

}

struct Instruction
{
    op: Operation,
    arg1: Option<Box<Argument>>,
    arg2: Option<Box<Argument>>,
}

struct AST(Vec<Instruction>);

struct Parser
{
    scanner: Scanner,
}

impl Parser
{
    pub fn parse_instruction(&mut self) -> Option<Instruction> // ParseError!
    {
        let kw = self.scanner.consume_token();
        let mut instr = Instruction {
            op: ,
            arg1: None,
            arg2: None,
        }
        // if not kw, return error
        let arg1 = self.scanner.consume_token().unwrap();

        if arg1.token_type == token

    }
}