pub fn answer(command: &str) -> Option<i32> {
    if !command.starts_with("What is ") || !command.ends_with("?") {
        return None;
    }
    let mut tokenizer = Tokenizer { command: &command[8..command.len()-1] };
    let mut result = match tokenizer.next() {
        Some(Ok(Token::Number(result))) => result,
        any => {
            println!("expected number, not {any:?}");
            return None;
        }
    };
    while let Some(next) = tokenizer.next() {
        let op = match next {
            Ok(Token::Operation(op)) => op,
            any => {
                println!("expected operation, not {any:?}");
                return None;
            } 
        };
        let operand = match tokenizer.next() {
            Some(Ok(Token::Number(operand))) => operand,
            any => {
                println!("expected operand, not {any:?}");
                return None;
            } 
        };
        result = op.apply(result, operand);
    }
    Some(result)
} 

#[derive(Debug)]
struct Tokenizer<'a> {
    command: &'a str
}

#[derive(Debug)]
enum Token {
    Number(i32),
    Operation(Operation),
}

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div
}

impl Operation {
    fn apply(&self, lhs: i32, rhs: i32) -> i32 {
        use Operation::*;
        match self {
            Add => lhs + rhs,
            Sub => lhs - rhs,
            Mul => lhs * rhs,
            Div => lhs / rhs,
        }
    }
}

#[derive(Debug)]
enum Error {
    InvalidCommand(String),
    InvalidNumber(String, std::num::ParseIntError),
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.command.is_empty() {
            return None;
        }
        println!("{}", self.command);
        Some(
            if self.command.starts_with("plus ") {
                self.command = &self.command[5..];
                Ok(Token::Operation(Operation::Add))
            } else if self.command.starts_with("minus ") {
                self.command = &self.command[6..];
                Ok(Token::Operation(Operation::Sub))
            } else if self.command.starts_with("multiplied by ") {
                self.command = &self.command[14..];
                Ok(Token::Operation(Operation::Mul))
            } else if self.command.starts_with("divided by ") {
                self.command = &self.command[11..];
                Ok(Token::Operation(Operation::Div))
            } else {
                let (word, tail) = self.command.split_once(|c: char| c != '-' && !c.is_digit(10)).unwrap_or((self.command, ""));
                match word.parse() {
                    Ok(number) => {
                        self.command = tail;
                        Ok(Token::Number(number))
                    }
                    Err(err) => Err(Error::InvalidNumber(
                        word.to_string(),
                        err
                    ))
                }
            }
        )
    }
}