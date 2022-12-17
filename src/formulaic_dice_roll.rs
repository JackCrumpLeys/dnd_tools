// token stream for parsing streams of equations with dice-roles like 2d6+3d8+4d10 or more complex equations such as (3d100*40d4)/(2^d8)

use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::str::FromStr;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq, Copy)]
pub enum DiceRollEquationToken {
    Number(i64),
    DiceRoll(i64, i64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LeftParenthesis,
    RightParenthesis,
    Power,
}

pub fn tokenize_equation(equation: &str) -> Result<Vec<DiceRollEquationToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = equation.chars().peekable();
    let mut first_dice_number_buffer: Option<i64> = None;
    while let Some(c) = chars.next() {
        match c {
            '0'..='9' => {
                let mut number = String::new();
                let mut is_prefix = false;
                number.push(c);
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        chars.next();
                    } else if c == 'd' {
                        first_dice_number_buffer = Some(number.parse().unwrap());
                        is_prefix = true;
                        break;
                    } else {
                        break;
                    }
                }

                if !is_prefix {
                    tokens.push(DiceRollEquationToken::Number(number.parse().unwrap()));
                }
            }
            'd' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) {
                        number.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if number.is_empty() {
                    return Err("Expected number after d".to_string());
                }
                if let Some(first_dice_number) = first_dice_number_buffer {
                    tokens.push(DiceRollEquationToken::DiceRoll(
                        first_dice_number,
                        number.parse().unwrap(),
                    ));
                    first_dice_number_buffer = None;
                } else {
                    tokens.push(DiceRollEquationToken::DiceRoll(1, number.parse().unwrap()));
                }
            }
            '+' => tokens.push(DiceRollEquationToken::Plus),
            '-' => tokens.push(DiceRollEquationToken::Minus),
            '*' => tokens.push(DiceRollEquationToken::Multiply),
            '/' => tokens.push(DiceRollEquationToken::Divide),
            '(' => tokens.push(DiceRollEquationToken::LeftParenthesis),
            ')' => tokens.push(DiceRollEquationToken::RightParenthesis),
            '^' => tokens.push(DiceRollEquationToken::Power),
            ' ' => {}
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }
    Ok(tokens)
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone, Ord, PartialEq, PartialOrd, Eq)]
pub enum DiceRollEquationNode {
    Number(i64),
    DiceRoll(i64, i64),
    Plus(Box<DiceRollEquationNode>, Box<DiceRollEquationNode>),
    Minus(Box<DiceRollEquationNode>, Box<DiceRollEquationNode>),
    Multiply(Box<DiceRollEquationNode>, Box<DiceRollEquationNode>),
    Divide(Box<DiceRollEquationNode>, Box<DiceRollEquationNode>),
    Power(Box<DiceRollEquationNode>, Box<DiceRollEquationNode>),
}

impl DiceRollEquationNode {
    pub fn evaluate(&self) -> i64 {
        match self {
            DiceRollEquationNode::Number(n) => *n,
            DiceRollEquationNode::DiceRoll(num_dice, dice_sides) => {
                let mut rng = rand::thread_rng();
                let mut total = 0;
                for _ in 0..*num_dice {
                    total += rng.gen_range(1..=*dice_sides);
                }
                total
            }
            DiceRollEquationNode::Plus(a, b) => a.evaluate() + b.evaluate(),
            DiceRollEquationNode::Minus(a, b) => a.evaluate() - b.evaluate(),
            DiceRollEquationNode::Multiply(a, b) => a.evaluate() * b.evaluate(),
            DiceRollEquationNode::Divide(a, b) => a.evaluate() / b.evaluate(),
            DiceRollEquationNode::Power(a, b) => a.evaluate().pow(b.evaluate() as u32),
        }
    }
}

impl Display for DiceRollEquationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DiceRollEquationNode::Number(n) => write!(f, "{}", n),
            DiceRollEquationNode::DiceRoll(num_dice, dice_sides) => {
                write!(f, "{}d{}", num_dice, dice_sides)
            }
            DiceRollEquationNode::Plus(a, b) => write!(f, "{} + {}", a, b),
            DiceRollEquationNode::Minus(a, b) => write!(f, "{} - {}", a, b),
            DiceRollEquationNode::Multiply(a, b) => write!(f, "{} * {}", a, b),
            DiceRollEquationNode::Divide(a, b) => write!(f, "{} / {}", a, b),
            DiceRollEquationNode::Power(a, b) => write!(f, "{} ^ {}", a, b),
        }
    }
}

pub fn parse_equation(tokens: &[DiceRollEquationToken]) -> Result<DiceRollEquationNode, String> {
    let mut tokens = tokens.to_vec();

    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == DiceRollEquationToken::Minus {
            if i == 0 || tokens[i - 1] == DiceRollEquationToken::LeftParenthesis {
                tokens.remove(i);
                let mut num = String::new();
                while let Some(&DiceRollEquationToken::Number(n)) = tokens.get(i) {
                    num.push_str(&n.to_string());
                    tokens.remove(i);
                }
                if num.is_empty() {
                    return Err("Expected number after minus".to_string());
                }
                tokens.insert(
                    i,
                    DiceRollEquationToken::Number(num.parse::<i64>().unwrap() * -1),
                );
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    let mut tokens = tokens.into_iter().peekable();
    let mut nodes = Vec::new();
    let mut operators = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            DiceRollEquationToken::Number(n) => nodes.push(DiceRollEquationNode::Number(n)),
            DiceRollEquationToken::DiceRoll(num_dice, dice_sides) => {
                nodes.push(DiceRollEquationNode::DiceRoll(num_dice, dice_sides))
            }
            DiceRollEquationToken::LeftParenthesis => {
                let mut paren_tokens = Vec::new();
                let mut paren_depth = 1;
                while let Some(&token) = tokens.peek() {
                    if token == DiceRollEquationToken::LeftParenthesis {
                        paren_depth += 1;
                    } else if token == DiceRollEquationToken::RightParenthesis {
                        paren_depth -= 1;
                        if paren_depth == 0 {
                            break;
                        }
                    }
                    paren_tokens.push(token);
                    tokens.next();
                }
                if paren_depth != 0 {
                    return Err("Mismatched parenthesis".to_string());
                }
                tokens.next();
                nodes.push(parse_equation(&paren_tokens)?);
            }
            DiceRollEquationToken::RightParenthesis => {
                return Err("Mismatched parenthesis".to_string())
            }
            DiceRollEquationToken::Plus => operators.push(DiceRollEquationToken::Plus),
            DiceRollEquationToken::Minus => operators.push(DiceRollEquationToken::Minus),
            DiceRollEquationToken::Multiply => operators.push(DiceRollEquationToken::Multiply),
            DiceRollEquationToken::Divide => operators.push(DiceRollEquationToken::Divide),
            DiceRollEquationToken::Power => operators.push(DiceRollEquationToken::Power),
        };
    }

    while let Some(operator) = operators.pop() {
        let b = nodes.pop().ok_or(format!("no number to the left of operator number."))?;
        let a = nodes.pop().ok_or(format!("no number to the right of operator number."))?;
        nodes.push(match operator {
            DiceRollEquationToken::Plus => DiceRollEquationNode::Plus(Box::new(a), Box::new(b)),
            DiceRollEquationToken::Minus => DiceRollEquationNode::Minus(Box::new(a), Box::new(b)),
            DiceRollEquationToken::Multiply => {
                DiceRollEquationNode::Multiply(Box::new(a), Box::new(b))
            }
            DiceRollEquationToken::Divide => DiceRollEquationNode::Divide(Box::new(a), Box::new(b)),
            DiceRollEquationToken::Power => DiceRollEquationNode::Power(Box::new(a), Box::new(b)),
            _ => unreachable!(),
        });
    }

    Ok(nodes.pop().ok_or(format!("No input."))?)
}

#[test]
fn test_tokenize_equation() {
    assert_eq!(
        tokenize_equation("2d6+3d8+4d10"),
        Ok(vec![
            DiceRollEquationToken::DiceRoll(2, 6),
            DiceRollEquationToken::Plus,
            DiceRollEquationToken::DiceRoll(3, 8),
            DiceRollEquationToken::Plus,
            DiceRollEquationToken::DiceRoll(4, 10),
        ])
    );
    assert_eq!(
        tokenize_equation("(3d100*40d4)/(2^d8)"),
        Ok(vec![
            DiceRollEquationToken::LeftParenthesis,
            DiceRollEquationToken::DiceRoll(3, 100),
            DiceRollEquationToken::Multiply,
            DiceRollEquationToken::DiceRoll(40, 4),
            DiceRollEquationToken::RightParenthesis,
            DiceRollEquationToken::Divide,
            DiceRollEquationToken::LeftParenthesis,
            DiceRollEquationToken::Number(2),
            DiceRollEquationToken::Power,
            DiceRollEquationToken::DiceRoll(1, 8),
            DiceRollEquationToken::RightParenthesis,
        ])
    );

    println!("{:?}", tokenize_equation("2d6+3d8+4d10"));
    println!("{:?}", tokenize_equation("(3d100*40d4)/(2^d8)"));
}

#[test]
fn test_parse_equation() {
    assert_eq!(
        parse_equation(&tokenize_equation("2d6+3d8+4d10").unwrap()),
        Ok(DiceRollEquationNode::Plus(
            Box::new(DiceRollEquationNode::DiceRoll(2, 6)),
            Box::new(DiceRollEquationNode::Plus(
                Box::new(DiceRollEquationNode::DiceRoll(3, 8)),
                Box::new(DiceRollEquationNode::DiceRoll(4, 10))
            )),
        )),
    );
    assert_eq!(
        parse_equation(&tokenize_equation("(3d100*40d4)/(2^d8)").unwrap()),
        Ok(DiceRollEquationNode::Divide(
            Box::new(DiceRollEquationNode::Multiply(
                Box::new(DiceRollEquationNode::DiceRoll(3, 100)),
                Box::new(DiceRollEquationNode::DiceRoll(40, 4)),
            )),
            Box::new(DiceRollEquationNode::Power(
                Box::new(DiceRollEquationNode::Number(2)),
                Box::new(DiceRollEquationNode::DiceRoll(1, 8)),
            )),
        ))
    );

    println!("{:?}", parse_equation(&tokenize_equation("d20").unwrap()).unwrap());
}
