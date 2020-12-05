use anyhow::{Result, Context, bail};

pub struct RpnCalculator(bool);

impl RpnCalculator {
    pub fn new(verbose: bool) -> Self {
        Self(verbose)
    }

    pub fn eval(&self, formula: &str) -> Result<i32> {
        let mut tokens = formula.split_whitespace().collect::<Vec<_>>();
        self.eval_inner(&mut tokens)
    }

    fn eval_inner(&self, tokens: &mut Vec<&str>) -> Result<i32> {
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
                        _ => bail!("invalid token: {}", token),
                    }
                }
            }

            if self.0 {
                // verbose
                println!("current token: {}, stack: {:?}", token, stack);
            }
        }

        if stack.len() != 1 {
            bail!("failed to calculate: {:?}", stack)
        }
        let result = stack.pop().context("empty stack!")?;

        Ok(result)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value_evaluation() {
        let calculator = RpnCalculator::new(false);
        assert_eq!(20, calculator.eval("20").unwrap());
        assert_eq!(-40, calculator.eval("-40").unwrap());
    }

    #[test]
    fn single_operation() {
        let calculator = RpnCalculator::new(false);
        assert_eq!(20, calculator.eval("5 15 +").unwrap());
        assert_eq!(40, calculator.eval("62 22 -").unwrap());
        assert_eq!(60, calculator.eval("10 6 *").unwrap());
        assert_eq!(80, calculator.eval("240 3 /").unwrap());
        assert_eq!(3, calculator.eval("18 5 %").unwrap());
    }
}