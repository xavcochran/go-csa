#[derive(Debug, PartialEq, Clone)]
enum Terminals {
    If,
    Then,
    Else,
    While,
    Do,
    Skip,
    Id(String),
    Plus,
    Minus,
    Times,
    True,
    False,
    LessThan,
    Equals,
    And,
    Or,
    Bang,
    Num(u32),
    LParen,
    RParen,
    LBracket,
    RBracket,
    SemiColon,
    LArrow,
    // Empty,
    EndSymbol,
}

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

    fn lex_kw_or_id(&mut self) -> Terminals {
        let mut lexeme = String::new();

        while self.is_more() && LexState::is_id_char(self.peek()) {
            let c = self.peek();
            self.eat(c);
            lexeme.push(c);
        }

        let token = match lexeme.as_str() {
            "true" => Terminals::True,
            "false" => Terminals::False,
            "if" => Terminals::If,
            "then" => Terminals::Then,
            "else" => Terminals::Else,
            "while" => Terminals::While,
            "do" => Terminals::Do,
            "skip" => Terminals::Skip,
            _ => Terminals::Id(lexeme),
        };

        token
    }

    fn lex_num(&mut self) -> Terminals {
        let mut lexeme = String::new();

        while self.is_more() && self.peek().is_numeric() {
            let c = self.peek();
            self.eat(c);
            lexeme.push(c);
        }
        let number = lexeme.parse::<u32>().unwrap();
        Terminals::Num(number)
    }
}

fn lex() -> Vec<Terminals> {
    let s = "foo && true || (false && bar)";
    let mut lex_state = LexState::init(s);
    let mut tokens: Vec<Terminals> = Vec::new();

    while lex_state.is_more() {
        match lex_state.peek() {
            '(' => {
                lex_state.eat('(');
                tokens.push(Terminals::LParen);
            }
            ')' => {
                lex_state.eat(')');
                tokens.push(Terminals::RParen);
            }
            '&' => {
                lex_state.eat('&');
                lex_state.eat('&');
                tokens.push(Terminals::And);
            }
            '|' => {
                lex_state.eat('|');
                lex_state.eat('|');
                tokens.push(Terminals::Or);
            }
            '+' => {
                lex_state.eat('+');
                tokens.push(Terminals::Plus);
            }
            '-' => {
                lex_state.eat('-');
                tokens.push(Terminals::Minus);
            }
            '*' => {
                lex_state.eat('*');
                tokens.push(Terminals::Times);
            }
            '<' => {
                if lex_state.peek() == '-' {
                    lex_state.eat('<');
                    lex_state.eat('-');
                    tokens.push(Terminals::LArrow);
                }
                lex_state.eat('<');
                tokens.push(Terminals::LessThan);
            }
            '=' => {
                lex_state.eat('=');
                tokens.push(Terminals::Equals);
            }
            '!' => {
                lex_state.eat('!');
                tokens.push(Terminals::Bang);
            }
            ';' => {
                lex_state.eat(';');
                tokens.push(Terminals::SemiColon);
            }
            '{' => {
                lex_state.eat('{');
                tokens.push(Terminals::LBracket);
            }
            '}' => {
                lex_state.eat('}');
                tokens.push(Terminals::RBracket);
            }

            c => {
                if c.is_numeric() {
                    let token = lex_state.lex_num();
                    if let Terminals::Num(ref num) = token {
                        println!("Number found: {}", num);
                    }
                    tokens.push(token);
                } else if c.is_lowercase() {
                    let token = lex_state.lex_kw_or_id();
                    if let Terminals::Id(ref id) = token {
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
    tokens.push(Terminals::EndSymbol);

    println!("{:?}", tokens);
    tokens
}

struct ParseState {
    input: Vec<Terminals>,
    index: usize,
}

impl ParseState {
    fn init(tokens: Vec<Terminals>) -> Self {
        ParseState {
            input: tokens,
            index: 0,
        }
    }

    fn peek(&self) -> Terminals {
        self.input[self.index].clone() // cloned as to not transfer ownership
    }

    fn eat(&mut self, t: Terminals) {
        if self.peek() == t {
            self.index += 1;
        } else {
            eprintln!("Expected {:?}", t);
        }
    }

    fn parse_prog(&mut self) {
        match self.peek() {
            _ => {
                self.parse_stmt();
                self.parse_stmts();
                self.eat(Terminals::EndSymbol);
            }
        }
    }

    fn parse_stmt(&mut self) {
        match self.peek() {
            Terminals::If => {
                self.eat(Terminals::If);
                self.parse_b_exp();
                self.eat(Terminals::Then);
                self.parse_stmt();
                self.eat(Terminals::Else);
                self.parse_stmt();
            }
            Terminals::While => {
                self.eat(Terminals::While);
                self.parse_b_exp();
                self.eat(Terminals::Do);
                self.parse_stmt();
            }
            Terminals::Skip => {
                self.eat(Terminals::Skip);
            }
            Terminals::Id(c) => {
                self.eat(Terminals::Id(c));
                self.eat(Terminals::LArrow);
                self.parse_a_exp();
            }
            Terminals::LBracket => {
                self.eat(Terminals::LBracket);
                self.parse_stmt();
                self.parse_stmts();
                self.eat(Terminals::RBracket);
            }
            _ => {
                eprintln!("Expected statement");
            }
        }
    }

    fn parse_stmts(&mut self) {
        match self.peek() {
            Terminals::SemiColon => {
                self.eat(Terminals::SemiColon);
                self.parse_stmt();
                self.parse_stmts();
            }
            Terminals::EndSymbol => {
                self.eat(Terminals::EndSymbol);
            }
            Terminals::RBracket => {
                self.eat(Terminals::RBracket);
            }
            _ => {
                eprintln!("Expected statement");
            }
        }
    }

    fn parse_b_exp(&mut self) {
        match self.peek() {
            Terminals::Id(c) => {
                self.eat(Terminals::Id(c));
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::Bang => {
                self.eat(Terminals::Bang);
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::True => {
                self.eat(Terminals::True);
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::False => {
                self.eat(Terminals::False);
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::Num(n) => {
                self.eat(Terminals::Num(n));
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::LParen => {
                self.eat(Terminals::LParen);
                self.parse_b_fac();
                self.parse_b_exps();
            }
            _ => {
                eprintln!("Expected boolean expression");
            }
        }
    }

    fn parse_b_exps(&mut self) {
        match self.peek() {
            Terminals::Then => {
                self.eat(Terminals::Then);
            }
            Terminals::Do => {
                self.eat(Terminals::Do);
            }
            Terminals::Or => {
                self.eat(Terminals::Or);
                self.parse_b_fac();
                self.parse_b_exps();
            }
            Terminals::RParen => {
                self.eat(Terminals::RParen);
            }
            _ => {
                eprintln!("Expected boolean expressions");
            }
        }
    }

    fn parse_b_fac(&mut self) {
        match self.peek() {
            Terminals::Id(c) => {
                self.eat(Terminals::Id(c));
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::Bang => {
                self.eat(Terminals::Bang);
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::True => {
                self.eat(Terminals::True);
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::False => {
                self.eat(Terminals::False);
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::Num(n) => {
                self.eat(Terminals::Num(n));
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::LParen => {
                self.eat(Terminals::LParen);
                self.parse_b_neg();
                self.parse_b_facs();
            }
            _ => {
                eprintln!("Expected boolean factor");
            }
        }
    }

    fn parse_b_facs(&mut self) {
        match self.peek() {
            Terminals::Then => {
                self.eat(Terminals::Then);
            }
            Terminals::Do => {
                self.eat(Terminals::Do);
            }
            Terminals::Or => {
                self.eat(Terminals::Or);
            }
            Terminals::And => {
                self.eat(Terminals::And);
                self.parse_b_neg();
                self.parse_b_facs();
            }
            Terminals::RParen => {
                self.eat(Terminals::RParen);
            }
            _ => {
                eprintln!("Expected boolean factors");
            }
        }
    }

    fn parse_b_neg(&mut self) {
        match self.peek() {
            Terminals::Id(c) => {
                self.eat(Terminals::Id(c));
                self.parse_b_rel();
            }
            Terminals::Bang => {
                self.eat(Terminals::Bang);
                self.parse_b_neg();
            }
            Terminals::True => {
                self.eat(Terminals::True);
                self.parse_b_rel();
            }
            Terminals::False => {
                self.eat(Terminals::False);
                self.parse_b_rel();
            }
            Terminals::Num(n) => {
                self.eat(Terminals::Num(n));
                self.parse_b_rel();
            }
            Terminals::LParen => {
                self.eat(Terminals::LParen);
                self.parse_b_rel();
            }
            _ => {
                eprintln!("Expected boolean negation");
            }
        }
    }

    fn parse_b_rel(&mut self) {
        match self.peek() {
            Terminals::Id(c) => {
                self.eat(Terminals::Id(c));
                self.parse_a_exp();
                self.parse_b_rels();
            }
            Terminals::True => {
                self.eat(Terminals::True);
                self.parse_a_exp();
                self.parse_b_rels();
            }
            Terminals::False => {
                self.eat(Terminals::False);
                self.parse_a_exp();
                self.parse_b_rels();
            }
            Terminals::Num(n) => {
                self.eat(Terminals::Num(n));
                self.parse_a_exp();
                self.parse_b_rels();
            }
            Terminals::LParen => {
                self.eat(Terminals::LParen);
                self.parse_a_exp();
                self.parse_b_rels();
            }
            _ => {
                eprintln!("Expected boolean relation");
            }
        }
    }

    fn parse_b_rels(&mut self) {
        match self.peek() {
            Terminals::Then => {
                self.eat(Terminals::Then);
            }
            Terminals::Do => {
                self.eat(Terminals::Do);
            }
            Terminals::Or => {
                self.eat(Terminals::Or);
            }
            Terminals::And => {
                self.eat(Terminals::And);
            }
            Terminals::LessThan => {
                self.eat(Terminals::LessThan);
                self.parse_a_exp();
            }
            Terminals::Equals => {
                self.eat(Terminals::Equals);
                self.parse_a_exp();
            }
            Terminals::RParen => {
                self.eat(Terminals::RParen);
            }
            _ => {
                eprintln!("Expected boolean relations");
            }
        }
    }

    fn parse_a_exp(&mut self) {}

    fn parse_a_exps(&mut self) {}

    fn parse_a_fac(&mut self) {}

    fn parse_a_facs(&mut self) {}

    fn parse_atom(&mut self) {}
}

fn main() {
    println!("Hello, world!");
}
