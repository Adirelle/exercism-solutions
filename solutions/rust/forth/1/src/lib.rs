use std::{collections::HashMap, rc::Rc, str::FromStr};

pub type Value = i32;
pub type Name = String;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub struct Forth {
    stack: Stack,
    definitions: HashMap<Name, Rc<Vec<Node>>>,
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            stack: Stack::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_ref()
    }

    pub fn eval(&mut self, input: &str) -> Result<()> {
        let tokens = input.split_whitespace().map(|w| w.parse()).collect::<Result<Vec<Token>>>()?;
        let nodes = self.parse(&tokens)?;
        self.exec(&nodes)
    }

    fn parse(&mut self, tokens: &[Token]) -> Result<Vec<Node>> {
        let mut tokens = tokens.into_iter();
        let mut nodes = Vec::new();
        while let Some(token) = tokens.next() {
            if let Some(def) = self.definitions.get(token.as_ref()) {
                nodes.push(Node::Call(def.clone()));
            } else {
                use Token::*;
                match token {
                    Colon => {
                        let name = match tokens.next() {
                            Some(Token::Value(_)) => return Err(Error::InvalidWord),
                            Some(token) => token.as_ref().to_string(),
                            None => return Err(Error::InvalidWord),
                        };
                        let mut def_tokens = Vec::new();
                        loop {
                            match tokens.next() {
                                Some(Semicolon) => break,
                                Some(token) => def_tokens.push(token.clone()),
                                None => return Err(Error::UnknownWord),
                            }
                        }
                        let def_nodes = self.parse(&def_tokens)?;
                        self.definitions.insert(name, Rc::new(def_nodes));
                    },
                    Op(operation) => nodes.push(Node::Op(*operation)),
                    Value(value) => nodes.push(Node::Push(*value)),
                    _ => return Err(Error::UnknownWord),
                }
            }
        }
        Ok(nodes)
    }

    fn exec(&mut self, nodes: &[Node]) -> Result<()> {
        for node in nodes.iter() {
            println!("stack={:?}", self.stack);
            println!("node={:?}", node);
            match node {
                Node::Op(operation) => operation.apply(&mut self.stack)?,
                Node::Push(value) => self.stack.push(*value),
                Node::Call(nodes) => {
                    println!("entering def");
                    self.exec(&nodes)?;
                    println!("returned from def");
                },
            }
        }
        println!("stack={:?}", self.stack);
        Ok(())
    }
}

#[derive(Debug, Clone)]
enum Node {
    Op(Operation),
    Push(Value),
    Call(Rc<Vec<Node>>)
}

#[derive(Debug, Clone)]
enum Token {
    Op(Operation),
    Colon,
    Semicolon,
    Value(Value),
    Word(String),
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self> {
        use Token::*;
        Ok(match value {
            "+" => Op(Operation::Plus),
            "-" => Op(Operation::Minus),
            "*" => Op(Operation::Star),
            "/" => Op(Operation::Slash),
            ":" => Colon,
            ";" => Semicolon,
            _ => match value.parse() {
                Ok(number) => Value(number),
                _ => {
                    let word = value.to_uppercase();
                    match word.as_ref() {
                        "DUP" => Op(Operation::Dup),
                        "DROP" => Op(Operation::Drop),
                        "SWAP" => Op(Operation::Swap),
                        "OVER" => Op(Operation::Over),
                        _ => Word(word),
                    }
                }
            },
        })
    }
}

impl AsRef<str> for Token {
    fn as_ref(&self) -> &str {
        use Token::*;
        match self {
            Op(op) => op.as_ref(),
            Colon => ":",
            Semicolon => ";",
            Value(_) => "<value>",
            Word(ref word) => word,
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum Operation {
    Plus,
    Minus,
    Star,
    Slash,
    Dup,
    Drop,
    Swap,
    Over,
}

impl Operation {
    fn apply(&self, stack: &mut Stack) -> Result<()> {
        if let Some(value) = self.eval(stack)? {
            stack.push(value);
        }
        Ok(())
    }

    fn eval(&self, stack: &mut Stack) -> Result<Option<Value>> {
        use Operation::*;
        Ok(match self {
            Plus => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                Some(a + b)
            }
            Minus => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                Some(a - b)
            }
            Star => {
                let b = stack.pop()?;
                let a = stack.pop()?;
                Some(a * b)
            }
            Slash =>  {
                let b = stack.pop()?;
                if b == 0 {
                    return Err(Error::DivisionByZero);
                }
                let a = stack.pop()?;
                Some(a / b)
            }
            Dup => Some(*stack.peek(0)?),
            Drop => {
                _ = stack.pop()?;
                None
            }
            Swap => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a);
                Some(b)
            },
            Over => Some(*stack.peek(1)?),
        })
    }
}

impl AsRef<str> for Operation {
    fn as_ref(&self) -> &str {
        use Operation::*;
        match self {
            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Dup => "DUP",
            Drop => "DROP",
            Swap => "SWAP",
            Over => "OVER",
        }
    }
}


#[derive(Debug)]
struct Stack(Vec<Value>);

impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn as_ref(&self) -> &[Value] {
        &self.0[..]
    }

    fn pop(&mut self) -> Result<Value> {
        self.0.pop().ok_or(Error::StackUnderflow)
    }

    fn push(&mut self, value: Value) {
        self.0.push(value)
    }

    fn peek(&self, i: usize) -> Result<&Value> {
        let last = self.0.len();
        if i + 1 <= last {
            Ok(self.0.get(last - 1 - i).expect("a value"))
        } else {
            Err(Error::StackUnderflow)
        }
    }
}
