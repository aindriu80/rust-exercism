use std::collections::HashMap;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    dictionary: HashMap<String, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

#[derive(PartialEq)]
enum Token {
    Number(i32),
    Word(String),
    Symbol(char),
}

struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.position >= self.input.len() {
            return None;
        }
        let c = self.input.chars().nth(self.position).unwrap();

        // Handle negative numbers
        if c == '-' && self.position + 1 < self.input.len() {
            let next_char = self.input.chars().nth(self.position + 1).unwrap();
            if next_char.is_digit(10) {
                self.position += 1; // Skip the '-'
                let mut num_str = String::from("-");
                while self.position < self.input.len()
                    && self.input.chars().nth(self.position).unwrap().is_digit(10)
                {
                    num_str.push(self.input.chars().nth(self.position).unwrap());
                    self.position += 1;
                }
                let num = num_str.parse::<i32>().unwrap();
                return Some(Token::Number(num));
            }
        }

        if c.is_digit(10) {
            let mut num_str = String::new();
            while self.position < self.input.len()
                && self.input.chars().nth(self.position).unwrap().is_digit(10)
            {
                num_str.push(self.input.chars().nth(self.position).unwrap());
                self.position += 1;
            }
            let num = num_str.parse::<i32>().unwrap();
            Some(Token::Number(num))
        } else if c.is_alphabetic() || c == '-' || c == '+' || c == '*' || c == '/' {
            let mut word = String::new();
            while self.position < self.input.len() {
                let ch = self.input.chars().nth(self.position).unwrap();
                if ch.is_alphanumeric() || ch == '-' || ch == '+' || ch == '*' || ch == '/' {
                    word.push(ch);
                    self.position += 1;
                } else {
                    break;
                }
            }
            Some(Token::Word(word.to_lowercase()))
        } else {
            self.position += 1;
            Some(Token::Symbol(c))
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_whitespace()
        {
            self.position += 1;
        }
    }
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            dictionary: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn pop_two(&mut self) -> std::result::Result<(Value, Value), Error> {
        let b = self.pop()?;
        let a = self.pop()?;
        Ok((a, b))
    }

    fn is_builtin(word: &str) -> bool {
        matches!(
            word,
            "+" | "-" | "*" | "/" | "dup" | "drop" | "swap" | "over"
        )
    }

    fn execute_builtin(&mut self, word: &str) -> Result {
        match word {
            "+" => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a + b);
            }
            "-" => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a - b);
            }
            "*" => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a * b);
            }
            "/" => {
                let (a, b) = self.pop_two()?;
                if b == 0 {
                    return Err(Error::DivisionByZero);
                }
                self.stack.push(a / b);
            }
            "dup" => {
                let a = self.pop()?;
                self.stack.push(a);
                self.stack.push(a);
            }
            "drop" => {
                self.pop()?;
            }
            "swap" => {
                let (a, b) = self.pop_two()?;
                self.stack.push(b);
                self.stack.push(a);
            }
            "over" => {
                let (a, b) = self.pop_two()?;
                self.stack.push(a);
                self.stack.push(b);
                self.stack.push(a);
            }
            _ => return Err(Error::UnknownWord),
        }
        Ok(())
    }

    fn execute_word(&mut self, word: &str) -> Result {
        let word_lower = word.to_lowercase();

        if let Some(definition) = self.dictionary.get(&word_lower).cloned() {
            // Join tokens into a string and evaluate as a new input
            let input = definition.join(" ");
            self.eval(&input)?;
        } else if Self::is_builtin(&word_lower) {
            self.execute_builtin(&word_lower)?;
        } else {
            return Err(Error::UnknownWord);
        }
        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();
        while let Some(token) = lexer.next_token() {
            tokens.push(token);
        }

        let mut i = 0;
        while i < tokens.len() {
            match &tokens[i] {
                Token::Symbol(':') => {
                    i += 1; // Skip ':'
                    if i >= tokens.len() {
                        return Err(Error::InvalidWord);
                    }

                    // Get the word name
                    let word = match &tokens[i] {
                        Token::Word(w) => w.clone(),
                        Token::Number(_) => return Err(Error::InvalidWord),
                        Token::Symbol(_) => return Err(Error::InvalidWord),
                    };
                    i += 1;

                    // Check if word is a number (including negative numbers)
                    if word.parse::<i32>().is_ok() {
                        return Err(Error::InvalidWord);
                    }

                    // Collect definition until ';'
                    // let mut definition = Vec::new();
                    // while i < tokens.len() && tokens[i] != Token::Symbol(';') {
                    //     match &tokens[i] {
                    //         Token::Number(n) => definition.push(n.to_string()),
                    //         Token::Word(w) => definition.push(w.clone()), // Store original word case
                    //         Token::Symbol(c) => {
                    //             if *c == ':' {
                    //                 return Err(Error::InvalidWord); // Nested definitions not allowed
                    //             }
                    //             definition.push(c.to_string());
                    //         }
                    //     }
                    //     i += 1;
                    // }
                    //
                    // Collect definition until ';'
                    let mut definition = Vec::new();
                    while i < tokens.len() && tokens[i] != Token::Symbol(';') {
                        match &tokens[i] {
                            Token::Number(n) => definition.push(n.to_string()),
                            Token::Word(w) => {
                                if let Some(inner_def) = self.dictionary.get(&w.to_lowercase()) {
                                    definition.extend(inner_def.clone());
                                } else {
                                    definition.push(w.clone());
                                }
                            }
                            Token::Symbol(c) => {
                                if *c == ':' {
                                    return Err(Error::InvalidWord); // Nested definitions not allowed
                                }
                                definition.push(c.to_string());
                            }
                        }
                        i += 1;
                    }

                    if i >= tokens.len() || tokens[i] != Token::Symbol(';') {
                        return Err(Error::InvalidWord);
                    }
                    i += 1; // Skip ';'

                    // Store the definition with lowercase key
                    self.dictionary.insert(word.to_lowercase(), definition);
                }
                Token::Number(num) => {
                    self.stack.push(*num);
                    i += 1;
                }
                Token::Word(word) => {
                    self.execute_word(word)?;
                    i += 1;
                }
                Token::Symbol(c) => {
                    self.execute_word(&c.to_string())?;
                    i += 1;
                }
            }
        }
        Ok(())
    }
}
