use std::fmt;

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
    Devide {
        left: Box<Token>,
        right: Box<Token>
    },
    Minus {
        left: Box<Token>,
        right: Box<Token>
    },
    Bracket {
        val: Box<Token>
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Int { val } => write!(f, "{}", val),
            Token::Plus { left, right } => write!(f, "({} + {})", left, right),
            Token::Mult { left, right } => write!(f, "({} * {})", left, right),
        }
    }
}

fn eval(e: &Token) -> i32 {
    match e {
        Token::Int { val } => *val,
        Token::Plus { left, right } => eval(left) + eval(right),
        Token::Mult { left, right } => eval(left) * eval(right),
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
            } else {
                Token::Mult {
                    left: Box::new(simplified_left),
                    right: Box::new(simplified_right),
                }
            }
        }
        Token::Plus { left, right } => {
            let simplified_left = simplify(*left);
            let simplified_right = simplify(*right);
            Token::Plus {
                left: Box::new(simplified_left),
                right: Box::new(simplified_right),
            }
        }
        _ => e,
    }
}

fn main() {
    {
        let e = Token::Int { val: 1 };
        println!("{}", eval(&e)); // Output: 1
        let simplified_e = simplify(e);
        println!("{}", simplified_e); // Output: Int { val: 1 }
    }

    {
        let e = Token::Plus {
            left: Box::new(Token::Int { val: 1 }),
            right: Box::new(Token::Int { val: 2 }),
        };
        println!("{}", eval(&e)); // Output: 3
        let simplified_e = simplify(e);
        println!("{}", simplified_e); // Output: Plus { left: Int { val: 1 }, right: Int { val: 2 } }
    }

    {
        let e = Token::Mult {
            left: Box::new(Token::Int { val: 0 }),
            right: Box::new(Token::Int { val: 5 }),
        };
        println!("{}", eval(&e)); // Output: 0
        let simplified_e = simplify(e);
        println!("{}", simplified_e); // Output: Int { val: 0 }
    }

    {
        let e = Token::Plus {
            left: Box::new(Token::Int { val: 3 }),
            right: Box::new(Token::Mult {
                left: Box::new(Token::Int { val: 0 }),
                right: Box::new(Token::Int { val: 2 }),
            }),
        };
        println!("{}", eval(&e)); // Output: 0
        let simplified_e = simplify(e);
        println!("{}", simplified_e); // Output: Int { val: 0 }
    }
}
