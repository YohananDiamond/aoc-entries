mod aoc;
use aoc::parse_number;

// TODO: remove "expect" and "unwrap" from here?

fn main() {
    aoc::start_with_file("day18.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    let homework = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| Tokenizer::new(line).collect::<Result<Vec<Token>, _>>())
        .collect::<Result<Vec<Vec<Token>>, _>>()?;

    Ok(format!(
        "{}",
        homework
            .iter()
            .try_fold::<isize, _, Result<isize, String>>(0isize, |acc, tokens| Ok(
                acc + eval_left_to_right(tokens.iter().cloned().peekable())?
            ))?
    ))
}

fn part2(input: &str) -> Result<String, String> {
    Err(format!("INCOMPLETE"))
}

#[derive(Debug, Clone, Copy)]
enum Context {
    Number,
}

#[derive(Clone, Debug)]
enum Token {
    Number(isize),
    Operator(Op),
    ParenOpen,
    ParenClose,
}

#[derive(Debug)]
struct Tokenizer<'a> {
    input: &'a str,
    context_stack: Vec<Context>,
    iterator: std::iter::Peekable<std::str::Chars<'a>>,
    byte_pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input,
            context_stack: Vec::new(),
            iterator: input.chars().peekable(),
            byte_pos: 0,
        }
    }

    pub fn get_token(&mut self) -> Option<Result<Token, String>> {
        let mut end_pos = self.byte_pos;

        loop {
            match self.context_stack.last() {
                Some(Context::Number) => match self.iterator.peek() {
                    Some(&c @ '0'..='9') => {
                        end_pos += c.len_utf8();
                        self.iterator.next();
                    }
                    Some(_) | None => {
                        let token = Token::Number(
                            match aoc::parse_number(unsafe {
                                std::str::from_utf8_unchecked(
                                    &self.input.as_bytes()[self.byte_pos..end_pos],
                                )
                            }) {
                                Ok(n) => n,
                                Err(e) => return Some(Err(e)),
                            },
                        );

                        self.byte_pos = end_pos;
                        self.context_stack.pop();

                        return Some(Ok(token));
                    }
                },
                None => match self.iterator.peek() {
                    Some('0'..='9') => self.context_stack.push(Context::Number),
                    Some(&c @ '+') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();

                        return Some(Ok(Token::Operator(Op::Add)));
                    }
                    Some(&c @ '-') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();

                        return Some(Ok(Token::Operator(Op::Sub)));
                    }
                    Some(&c @ '*') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();

                        return Some(Ok(Token::Operator(Op::Mul)));
                    }
                    Some(&c @ '/') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();
                        return Some(Ok(Token::Operator(Op::Div)));
                    }
                    Some(&c @ '(') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();

                        return Some(Ok(Token::ParenOpen));
                    }
                    Some(&c @ ')') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();

                        return Some(Ok(Token::ParenClose));
                    }
                    Some(&c @ ' ') => {
                        end_pos += c.len_utf8();
                        self.byte_pos = end_pos;
                        self.iterator.next();
                    }
                    Some(ch) => return Some(Err(format!("unexpected character {:?}", ch))),
                    None => return None,
                },
            }
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.get_token()
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn apply(self, lhs: isize, rhs: isize) -> isize {
        match self {
            Op::Add => lhs + rhs,
            Op::Sub => lhs - rhs,
            Op::Mul => lhs * rhs,
            Op::Div => lhs / rhs,
        }
    }
}

#[derive(Clone, Debug)]
enum CalcState {
    WaitingWithNumber { number: isize },
    WaitingWithOperand { number: isize, operand: Op },
    Waiting,
}

struct InputTokenRepr(Option<Token>);

impl std::fmt::Display for InputTokenRepr {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref thing) = self.0 {
            write!(fmt, "{:?}", thing)
        } else {
            write!(fmt, "<End of Input>")
        }
    }
}

/// Evaluates an expression without operator precedence.
fn eval_left_to_right<I>(mut tokens: std::iter::Peekable<I>) -> Result<isize, String>
where
    I: Iterator<Item = Token>,
{
    let mut calc_stack: Vec<CalcState> = Vec::with_capacity(32);
    match tokens.next() {
        Some(Token::Number(number)) => {
            calc_stack.push(CalcState::WaitingWithNumber { number });
        }
        Some(Token::ParenOpen) => {
            calc_stack.push(CalcState::Waiting);

            loop {
                match tokens.peek() {
                    Some(Token::ParenOpen) => {
                        calc_stack.push(CalcState::Waiting);
                        tokens.next();
                    }
                    Some(&Token::Number(number)) => {
                        calc_stack.push(CalcState::WaitingWithNumber { number });
                        tokens.next();
                        break;
                    }
                    other_tk => {
                        return Err(format!(
                            "unexpected token: {} (expected number or opening paren)",
                            InputTokenRepr(other_tk.map(|i| i.clone()))
                        ));
                    }
                }
            }
        }
        other_tk => return Err(format!("unexpected token: {}", InputTokenRepr(other_tk))),
    }

    loop {
        match tokens.next() {
            Some(Token::Operator(op)) => match tokens.next() {
                Some(Token::Number(number)) => match calc_stack.last_mut().unwrap() {
                    CalcState::WaitingWithNumber { number: ref mut n } => {
                        *n = op.apply(*n, number);
                    }
                    _ => unreachable!(),
                },

                Some(Token::ParenOpen) => {
                    {
                        let last = calc_stack.last_mut().unwrap();

                        // WaitingWithNumber { number: isize },
                        // WaitingWithOperand { number: isize, operand: Op },
                        // Waiting,

                        match last {
                            CalcState::WaitingWithNumber { number } => {
                                *last = CalcState::WaitingWithOperand {
                                    number: *number,
                                    operand: op,
                                };
                            }
                            _ => unreachable!(),
                        }
                    }

                    loop {
                        match tokens.next() {
                            Some(Token::Number(number)) => {
                                calc_stack.push(CalcState::WaitingWithNumber { number });
                                break;
                            }
                            Some(Token::ParenOpen) => {
                                calc_stack.push(CalcState::Waiting);
                            }
                            Some(Token::ParenClose) => {
                                return Err(format!(
                                    "Empty parens (expected number or more parens)"
                                ));
                            }
                            other_tk => {
                                return Err(format!(
                                    "Unexpected token: {}",
                                    InputTokenRepr(other_tk)
                                ))
                            }
                        }
                    }
                }

                other_tk => {
                    return Err(format!(
                        "Expected number or open paren, found {}",
                        InputTokenRepr(other_tk)
                    ))
                }
            },
            Some(Token::ParenClose) => {
                let popped_value = match calc_stack.pop().expect("No values to pop on the stack") {
                    CalcState::WaitingWithNumber { number } => number,
                    _ => unreachable!(),
                };

                let previous_value = calc_stack.last_mut().expect("Too many closing brackets");

                *previous_value = match previous_value {
                    CalcState::WaitingWithNumber { .. } => unreachable!("No operator to apply"),
                    CalcState::WaitingWithOperand { number, operand } => {
                        CalcState::WaitingWithNumber {
                            number: operand.apply(*number, popped_value),
                        }
                    }
                    CalcState::Waiting => CalcState::WaitingWithNumber {
                        number: popped_value,
                    },
                };
            }
            Some(other_tk) => {
                return Err(format!(
                    "Expected operator or closing paren, found {:?}",
                    other_tk
                ))
            }
            None => break,
        }
    }

    if calc_stack.len() == 1 {
        match calc_stack.get(0).unwrap() {
            CalcState::WaitingWithNumber { number } => Ok(*number),
            _ => unreachable!(),
        }
    } else {
        Err(format!(
            "At least one unclosed parenthesis on the token list"
        ))
    }
}

#[cfg(test)]
mod tests {
    fn eval(expr: &str) -> isize {
        super::eval_left_to_right(super::Tokenizer::new(expr).peekable())
    }

    #[test]
    fn tests_here() {
        assert_eq!(eval("1 + 2 * 3"), 9);
        assert_eq!(eval("1 + (2 * 3)"), 7);
        assert_eq!(eval("1 + 2 / 3"), 1);
    }
}
