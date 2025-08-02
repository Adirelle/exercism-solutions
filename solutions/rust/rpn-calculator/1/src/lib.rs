#[derive(Debug, Clone)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

impl CalculatorInput {
    fn apply(&self, stack: &mut Vec<i32>) -> bool {
        use CalculatorInput::*;
        match self {
            Value(x) => {
                stack.push(*x);
                true
            }
            Add | Subtract | Multiply | Divide => match (stack.pop(), stack.pop()) {
                (Some(b), Some(a)) => {
                    stack.push(self.resolve(a, b));
                    true
                }
                _ => false,
            },
        }
    }

    fn resolve(&self, a: i32, b: i32) -> i32 {
        use CalculatorInput::*;
        match self {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
            Value(x) => *x,
        }
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::new();
    for input in inputs {
        if !input.apply(stack.as_mut()) {
            return None;
        }
    }

    match stack[..] {
        [result] => Some(result),
        _ => None,
    }
}
