#[derive(Debug)]
enum token {
    TkTrue,
    TkFalse,
    TkAnd,
    TkOr,
    TkLParen,
    TkRParen,
    TkId(String),
    TkEnd,
}

// struct state then impl functions below
struct State<'a> {
    idx: usize,
    input: &'a str,
    input_len: usize,
}

impl<'a> State<'a> {
    fn init(s: &'a str) -> Self {
        State {
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

    fn lex_kw_or_id(&mut self) -> token {
        let mut lexeme = String::new();

        while self.is_more() && State::is_id_char(self.peek()) {
            let c = self.peek();
            self.eat(c);
            lexeme.push(c);
        }

        let token = match lexeme.as_str() {
            "true" => token::TkTrue,
            "false" => token::TkFalse,
            _ => token::TkId(lexeme),
        };

        token
    }

}

fn main() {
    let s = "foo && true || (false && bar)"; 
    let mut state = State::init(s);
    let mut tokens: Vec<token> = Vec::new();

    while state.is_more() {
        match state.peek() {
            '(' => {
                state.eat('(');
                tokens.push(token::TkLParen);
            }
            ')' => {
                state.eat(')');
                tokens.push(token::TkRParen);
            }
            '&' => {
                state.eat('&');
                state.eat('&');
                tokens.push(token::TkAnd);
            }
            '|' => {
                state.eat('|');
                state.eat('|');
                tokens.push(token::TkOr);
            }

            c => {
                if c.is_lowercase() {
                    tokens.push(state.lex_kw_or_id()); 
                }
                if c.is_whitespace() {
                    state.eat(c);
                } else {
                    eprintln!("Did not exect to find {}", c)
                }
            }
        }
    }
    tokens.push(token::TkEnd);
    // tokens.reverse(); // don't need because push appends to end of list

    println!("{:?}", tokens);

}