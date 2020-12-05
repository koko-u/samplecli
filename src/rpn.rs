pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> i32 {
        let mut tokens = formula.split_whitespace().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> i32 {
        let mut stack = Vec::<i32>::new();

        for token in tokens.iter() {
            match token.parse::<i32>() {
                Ok(num) => stack.push(num),
                Err(_) => {
                    match *token {
                        "+" => self.add(&mut stack),
                        "-" => self.sub(&mut stack),
                        "*" => self.mul(&mut stack),
                        "/" => self.div(&mut stack),
                        "%" => self.quot(&mut stack),
                        _ => (),
                    }
                }
            }

            if (self.0) {
                // verbose
                println!("current token: {}, stack: {:?}", token, stack);
            }
        }

        stack.pop().unwrap_or(0)
    }

    fn add(&self, stack: &mut Vec<i32>) {
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
             stack.push(lhs + rhs);
        }
    }
    fn sub(&self, stack: &mut Vec<i32>) {
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
            stack.push(lhs - rhs);
        }
    }
    fn mul(&self, stack: &mut Vec<i32>) {
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
            stack.push(lhs * rhs);
        }
    }
    fn div(&self, stack: &mut Vec<i32>) {
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
            stack.push(lhs / rhs);
        }
    }
    fn quot(&self, stack: &mut Vec<i32>) {
        if let (Some(rhs), Some(lhs)) = (stack.pop(), stack.pop()) {
            stack.push(lhs % rhs);
        }
    }
}