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
}

fn eval(e : &Token) -> i32 {
    match e {
        Token::Int { val } => return *val,
        Token::Plus { left, right } => return eval(left) + eval(right),
        Token::Mult { left, right } => return eval(left) * eval(right),
    }
}


fn main() {
    {
        let e = Token::Int { val: 1 };
        println!("{}", eval(&e));
    }

    {
        let e = Token::Plus{ left: Box::new(Token::Int { val: 1 }), right: Box::new(Token::Int { val: 2})};
        println!("{}", eval(&e));
    }
}
