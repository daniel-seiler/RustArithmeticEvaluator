use std::fmt;
use std::fmt::write;

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
    Divide {
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
            Token::Divide { left, right } => write!(f, "{} / {}", left, right),
            Token::Mod { left, right } => write!(f, "{} % {}", left, right),
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
    match token {
        Token::Int { val } => return *val,
        Token::Plus { left, right } => return eval(left) + eval(right),
        Token::Minus { left, right } => return eval(left) - eval(right),
        Token::Mult { left, right } => return eval(left) * eval(right),
        Token::Div { left, right } => return eval(left) / eval(right),
        Token::Mod { left, right } => return eval(left) % eval(right),
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
        Token::Divide { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            if let Token::Int { val: 0 } = simplified_left {
                Token::Int { val: 0 }
            } else if let Token::Int { val: 1 } = simplified_right {
                simplified_left
            } else {
                Token::Divide {
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
fn parse_int(eval_string : &str) -> Token {
    return match eval_string.trim().parse::<i32>() {
        Ok(n) => Token::Int { val: n },
        Err(e) =>
            {
                eprintln!("Invalid input {}", eval_string);
                Token::Int { val: 0 }
            }
    }
}

fn main() {
    {
        let e = Token::Plus {
            left: Box::new(Token::Int { val: 3 }),
            right: Box::new(Token::Mod {
                left: Box::new(Token::Int { val: 1 }),
                right: Box::new(Token::Int { val: 2 }),
            }),
        };
        println!("{}", eval(&e)); // Output: 0
        let simplified_e = simplify(e);
        println!("{}", simplified_e); // Output: Int { val: 0 }
    }

    {
        let str = "2 + 10 * 3";
        let t = parse(&str);
        println!("{}", eval(&t));
    }

    {
        let str = "4 * 9 / 3 * 2";
        let t = parse(&str);
        println!("{}", eval(&t));
    }

    {
        let str = "5 % 3";
        let t = parse(&str);
        println!("{}", eval(&t));
    }
}
