#[derive(Debug, PartialEq)]
enum Token {
    TkTrue,
    TkFalse,
    TkAnd,
    TkOr,
    TkLParen,
    TkRParen,
    TkId(String),
    TkEnd,
}



// struct LexState then impl functions below
struct LexState<'a> {
    idx: usize,
    input: &'a str,
    input_len: usize,
}

impl<'a> LexState<'a> {
    fn init(s: &'a str) -> Self {
        LexState {
            idx: 0,
            input: s,
            input_len: s.len(),
        }
    }

    fn is_more(&self) -> bool {
        self.idx < self.input_len
    }

    fn peek(&self) -> char {
        self.input.chars().nth(self.idx).unwrap() //could use .next()
    }

    fn eat(&mut self, c: char) {
        if self.peek() == c {
            self.idx += 1;
        } else {
            eprintln!("Expected {}", c);
        }
    }

    fn is_id_char(c: char) -> bool {
        c.is_lowercase() || c.is_uppercase() || c == '\''
    }

    fn lex_kw_or_id(&mut self) -> Token {
        let mut lexeme = String::new();

        while self.is_more() && LexState::is_id_char(self.peek()) {
            let c = self.peek();
            self.eat(c);
            lexeme.push(c);
        }

        let token = match lexeme.as_str() {
            "true" => Token::TkTrue,
            "false" => Token::TkFalse,
            _ => Token::TkId(lexeme),
        };

        token
    }
}

fn lex() -> Vec<Token> {
    let s = "foo && true || (false && bar)";
    let mut lex_state = LexState::init(s);
    let mut tokens: Vec<Token> = Vec::new();

    while lex_state.is_more() {
        match lex_state.peek() {
            '(' => {
                lex_state.eat('(');
                tokens.push(Token::TkLParen);
            }
            ')' => {
                lex_state.eat(')');
                tokens.push(Token::TkRParen);
            }
            '&' => {
                lex_state.eat('&');
                lex_state.eat('&');
                tokens.push(Token::TkAnd);
            }
            '|' => {
                lex_state.eat('|');
                lex_state.eat('|');
                tokens.push(Token::TkOr);
            }

            c => {
                if c.is_lowercase() {
                    let token = lex_state.lex_kw_or_id();
                    if let Token::TkId(ref id) = token {
                        println!("Identifier found: {}", id);
                    }
                    tokens.push(token);
                } else if c.is_whitespace() {
                    lex_state.eat(c);
                } else {
                    eprintln!("Did not exect to find {}", c);
                    break;
                }
            }
        }
    }
    tokens.push(Token::TkEnd);
    // tokens.reverse(); // don't need because push appends to end of list

    println!("{:?}", tokens);
    tokens
}


struct ParseState {
    input: Vec<Token>,
    index: usize
}

impl ParseState {
    fn init(tokens: Vec<Token>) -> Self{
        ParseState{
            input: tokens,
            index: 0
        }
    }

    // borrowed as should not be 'taken out' and owned by calling scope
    fn peek(&self) -> &Token {
        let token = &self.input[self.index];
        token
    }

    fn eat(&mut self, t: &Token) {
        if self.peek() == t {
            self.index += 1;
        } else {
            eprintln!("Expected {:?}", t);
        }
    }
}

fn parse(t: Vec<Token>) {
    let parser = ParseState::init(t);

    
}




pub fn main() {
    lex();

}