use std::fmt;
use std::fmt::write;
use std::net::Shutdown::Write;

pub enum Token {
    Int {
        val: i32
    },
    Plus {
        left: Box<Token>,
        right: Box<Token>
    },
    Mult {
        left: Box<Token>,
        right: Box<Token>
    },
    Div {
        left: Box<Token>,
        right: Box<Token>
    },
    Minus {
        left: Box<Token>,
        right: Box<Token>
    },
    Mod {
        left: Box<Token>,
        right: Box<Token>
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int { val } => write!(f, "{}", val),
            Token::Plus { left, right } => write!(f, "{} + {}", left, right),
            Token::Mult { left, right } => write!(f, "{} * {}", left, right),
            Token::Div { left, right } => write!(f, "{} / {}", left, right),
            Token::Mod { left, right } => write!(f, "{} % {}", left, right),
            Token::Minus { left, right} => write!(f, "{} - {}", left, right),
            _ => write!(f, "err")
        }
    }
}

fn split_at_last_occurrence(input: &str, delimiter: char) -> (&str, &str) {
    if let Some(index) = input.rfind(delimiter) {
        let (left, right) = input.split_at(index);
        (left, &right[delimiter.len_utf8()..])
    } else {
        ("", input)
    }
}

fn eval(token : &Token) -> i32 {
    return match token {
        Token::Int { val } => *val,
        Token::Plus { left, right } => eval(left) + eval(right),
        Token::Minus { left, right } => eval(left) - eval(right),
        Token::Mult { left, right } => eval(left) * eval(right),
        Token::Div { left, right } => eval(left) / eval(right),
        Token::Mod { left, right } => eval(left) % eval(right),
        _ => {
            println!("error while parsing: {}", token);
            -1
        }
    }
}

fn parse(eval_string : &str) -> Token {
    return if eval_string.contains("+") {
        parse_plus(&eval_string)
    } else if eval_string.contains("-") {
        parse_minus(&eval_string)
    } else if eval_string.contains("*") {
        parse_mult(&eval_string)
    } else if eval_string.contains("/") {
        parse_div(&eval_string)
    } else if eval_string.contains("%") {
        parse_mod(&eval_string)
    } else {
        parse_int(&eval_string)
    }
}

fn parse_mult(eval_string : &str) -> Token {
    let (l, r) = split_at_last_occurrence(&eval_string, '*');
    return Token::Mult { left: Box::new(parse(l)), right: Box::new(parse(r))};
}

fn parse_div(eval_string : &str) -> Token {
    let (l, r) = split_at_last_occurrence(&eval_string, '/');
    return Token::Div { left: Box::new(parse(l)), right: Box::new(parse(r))};
}

fn parse_minus(eval_string : &str) -> Token {
    let (l, r) = split_at_last_occurrence(&eval_string, '-');
    return Token::Minus { left: Box::new(parse(l)), right: Box::new(parse(r))};
}

fn parse_plus(eval_string : &str) -> Token {
    let (l, r) = split_at_last_occurrence(&eval_string, '+');
    return Token::Plus { left: Box::new(parse(l)), right: Box::new(parse(r))};
}

fn parse_mod(eval_string : &str) -> Token {
    let(l, r) = split_at_last_occurrence(&eval_string, '%');
    return Token::Mod { left: Box::new(parse(l)), right: Box::new(parse(r))};
}

fn parse_int(eval_string : &str) -> Token {
    return match eval_string.trim().parse::<i32>() {
        Ok(n) => Token::Int { val: n },
        Err(_e) =>
            {
                eprintln!("Invalid input {}", eval_string);
                Token::Int { val: 0 }
            }
    }
}


fn simplify(e: Token) -> Token {
    match e {
        Token::Mult { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            if let Token::Int { val: 0 } = simplified_left {
                Token::Int { val: 0 }
            } else if let Token::Int { val: 0 } = simplified_right {
                Token::Int { val: 0 }
            } else if let Token::Int { val: 1 } = simplified_right {
               simplified_left
            } else if let Token::Int { val: 1 } = simplified_left {
                simplified_right
            } else {
                Token::Mult {
                    left: Box::new(simplified_left),
                    right: Box::new(simplified_right),
                }
            }
        },
        Token::Div { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            if let Token::Int { val: 0 } = simplified_left {
                Token::Int { val: 0 }
            } else if let Token::Int { val: 1 } = simplified_right {
                simplified_left
            } else {
                Token::Div {
                    left: Box::new(simplified_left),
                    right: Box::new(simplified_right),
                }
            }
        }
        Token::Plus { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            if let Token::Int { val: 0 } = simplified_left {
                simplified_right
            } else if let Token::Int { val: 0 } = simplified_right {
                simplified_left
            } else {
                Token::Plus {
                    left: Box::new(simplified_left),
                    right: Box::new(simplified_right),
                }
            }
        },
        Token::Minus { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            if let Token::Int { val: 0 } = simplified_left {
                simplified_right
            } else if let Token::Int { val: 0 } = simplified_right {
                simplified_left
            } else {
                Token::Minus {
                    left: Box::new(simplified_left),
                    right: Box::new(simplified_right),
                }
            }
        },
        _ => e,
    }
}

fn main() {


    {
        let str = "2 * 1 * 0 + 10 % 2 / 1 * 5 - 10 * 3 + 0";
        let t = parse(&str);
        println!("{}", eval(&t));
        let simplified_e = simplify(t);
        println!("{}", simplified_e);
        println!("{}", eval(&simplified_e));
    }
}
