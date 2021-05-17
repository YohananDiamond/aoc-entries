#[macro_use]
extern crate pest_derive;

// https://createlang.rs/01_calculator/ast.html

mod aoc;
use aoc::parse_number;

use pest::iterators::Pairs;
use pest::Parser;

#[derive(Parser)]
#[grammar = "day18_old_grammar.pest"]
struct ExprParser;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    Add,
    Mul,
}

#[derive(Clone, Debug, PartialEq)]
struct Operation {
    operator: Op,
    operands: [Expr; 2],
}

#[derive(Clone, Debug, PartialEq)]
enum Expr {
    Number(i32),
    Operation(Box<Operation>),
}

fn main() {
    aoc::start_with_file("day18_example.txt", part1, part2);
    // aoc::start_with_file("day18.txt", part1, part2);
}

fn part1(input: &str) -> Result<String, String> {
    // let homework: Vec<Expr> = input
    //     .split('\n')
    //     .filter(|line| !line.is_empty())
    //     .map(Expr::parse)
    //     .collect::<Result<_, _>>()?;

    let homework: Vec<Expr> = ["1 + 2 * 3"]
        .iter()
        .cloned()
        .filter(|line| !line.is_empty())
        .map(Expr::parse)
        .collect::<Result<_, _>>()?;

    for line in &homework {
        println!("{:?}", line);
    }

    Ok(format!(
        "{}",
        homework.iter().map(Expr::eval).sum::<isize>()
    ))
}

fn part2(input: &str) -> Result<String, String> {
    Err(format!("INCOMPLETE"))
}

impl From<Operation> for Expr {
    fn from(op: Operation) -> Self {
        Self::Operation(Box::new(op))
    }
}

impl Operation {
    pub fn eval(&self) -> isize {
        let (lhs, rhs) = (&self.operands[0], &self.operands[1]);

        match self.op {
            Op::Add => lhs.eval() + 
        }
    }
}

impl Expr {
    pub fn eval(&self) -> isize {
        match self {
            Self::Number(n) => *n as isize,
            Self::Operation(o) => o.eval(),
        }
    }

    pub fn parse(input: &str) -> Result<Self, String> {
        match ExprParser::parse(Rule::line, input) {
            Ok(mut line) => {
                let main_expr = line
                    .next() // get the only element on the main iterator
                    .unwrap()
                    .into_inner() // the inner of that element is the main expr and the EOI token
                    .next()
                    .unwrap() // the actual expr itself
                    .into_inner();

                Self::parse_pairs(main_expr)
            }
            Err(err) => Err(format!("Parsing error:\n{}", err)),
        }
    }

    fn parse_pairs(mut pairs: Pairs<Rule>) -> Result<Self, String> {
        match pairs.clone().count() {
            1 => {
                let expr1 = pairs.next().unwrap();

                match expr1.as_rule() {
                    Rule::number => Ok(Self::Number(parse_number(expr1.as_str())?)),
                    other => Err(format!(
                        "Invalid rule in context (expected number, found {:?})",
                        other
                    )),
                }
            }
            3 => {
                let expr1 = {
                    let pair = pairs.next().unwrap();

                    match pair.as_rule() {
                        Rule::number => Self::Number(parse_number(pair.as_str())?),
                        other => {
                            return Err(format!(
                                "Invalid rule in context (expected number, found {:?})",
                                other
                            ))
                        }
                    }
                };

                let operator = {
                    let pair = pairs.next().unwrap();

                    match pair.as_rule() {
                        Rule::operator => match pair.as_str() {
                            "+" => Op::Add,
                            "*" => Op::Mul,
                            other => return Err(format!("Invalid operator: {:?}", other)),
                        },
                        other => {
                            return Err(format!(
                                "Invalid rule in context (expected operator, found {:?})",
                                other
                            ))
                        }
                    }
                };

                let expr2 = {
                    let pair = pairs.next().unwrap();

                    match pair.as_rule() {
                        Rule::number => Self::Number(parse_number(pair.as_str())?),
                        Rule::expr => Self::parse_pairs(pair.into_inner())?,
                        other => {
                            return Err(format!(
                                "Invalid rule in context (expected number, found {:?})",
                                other
                            ))
                        }
                    }
                };

                Ok(Self::from(Operation {
                    operator: operator,
                    operands: [expr1, expr2],
                }))
            }
            other => Err(format!(
                "Invalid length for inner pairs (expected either 1 or 3), found {}",
                other
            )),
        }
    }
}
