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

fn eval(token : &Token) -> i32 {
    match token {
        Token::Int { val } => return *val,
        Token::Plus { left, right } => return eval(left) + eval(right),
        Token::Mult { left, right } => return eval(left) * eval(right),
    }
}

fn parse(eval_string : &str) -> Token {
    return if eval_string.contains("+") {
        parse_plus(eval_string)
    } else if eval_string.contains("*") {
        parse_mult(eval_string)
    } else {
        parse_int(eval_string)
    }
}

fn parse_mult(eval_string : &str) -> Token {
    let parts = eval_string.split("*").collect::<Vec<_>>();
    return Token::Mult { left: Box::new(parse(parts[0]))
        , right: Box::new(parse(parts[1]))};
}

fn parse_plus(eval_string : &str) -> Token {
    let parts = eval_string.split("+").collect::<Vec<_>>();
    return Token::Plus { left: Box::new(parse(parts[0]))
        , right: Box::new(parse(parts[1]))};
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
        let e = Token::Int { val: 1 };
        println!("{}", eval(&e));
    }

    {
        let e = Token::Plus{ left: Box::new(Token::Int { val: 1 }), right: Box::new(Token::Int { val: 2})};
        println!("{}", eval(&e));
    }

    {
        let str = "2 + 10 * 3";
        let t = parse(&str);
        println!("{}", eval(&t));
    }
}
