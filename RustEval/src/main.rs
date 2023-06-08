use std::fmt::write;

fn main() {
    // Constructing an expression: (One + Zero) * ETrue
    let expr = E::Mult(
        Box::new(E::Plus(Box::new(E::One), Box::new(E::Zero))),
        Box::new(E::ETrue),
    );

    // Evaluating the expression
    let result = expr.evaluate();

    println!("Display: {}", expr);
    println!("Result {}", result);

    return;
}

enum Maybe<T> {
    Just(T),
    Nothing,
}

enum Either {
    Right(bool),
    Left(i32),
}

enum E {
    One,
    Zero,
    ETrue,
    EFalse,
    Plus(Box<E>, Box<E>),
    Mult(Box<E>, Box<E>),
    Or(Box<E>, Box<E>),
}

impl E {
    fn evaluate(&self) -> Maybe<Either> {
        match self {
            E::One => Maybe::Just(Either::Left(1)),
            E::Zero => Maybe::Just(Either::Left(0)),
            E::ETrue => Maybe::Just(Either::Right(true)),
            E::EFalse => Maybe::Just(Either::Right(false)),
            E::Plus(e1, e2) => {
                let val1 = e1.evaluate();
                let val2 = e2.evaluate();
                match (val1, val2) {
                    (Maybe::Just(Either::Left(i1)), Maybe::Just(Either::Left(i2))) => {
                        Maybe::Just(Either::Left(i1 + i2))
                    }
                    _ => Maybe::Nothing,
                }
            }
            E::Mult(e1, e2) => {
                let val1 = e1.evaluate();
                let val2 = e2.evaluate();
                match (val1, val2) {
                    (Maybe::Just(Either::Left(i1)), Maybe::Just(Either::Left(i2))) => {
                        Maybe::Just(Either::Left(i1 * i2))
                    }
                    _ => Maybe::Nothing,
                }
            }
            E::Or(e1, e2) => {
                let val1 = e1.evaluate();
                match val1 {
                    Maybe::Nothing => Maybe::Nothing,
                    Maybe::Just(Either::Left(..)) => Maybe::Nothing,
                    Maybe::Just(Either::Right(true)) => Maybe::Just(Either::Right(true)),
                    _ => {
                        let val2 = e2.evaluate();
                        match val2 {
                            Maybe::Nothing => Maybe::Nothing,
                            Maybe::Just(Either::Left(..)) => Maybe::Nothing,
                            Maybe::Just(Either::Right(true)) => Maybe::Just(Either::Right(true)),
                            _ => Maybe::Just(Either::Right(false))
                        }
                    }
                }
            }
                /*let val1 = e1.evaluate();
                let val2 = e2.evaluate();
                match (val1, val2) {
                    (Maybe::Just(Either::Right(b1)), Maybe::Just(Either::Right(b2))) => {
                        Maybe::Just(Either::Right(b1 || b2))
                    }
                    _ => Maybe::Nothing,
                }
            }*/
        }
    }
}

impl std::fmt::Display for E {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            E::One => write!(f, "1"),
            E::Zero => write!(f, "0"),
            E::ETrue => write!(f, "true"),
            E::EFalse => write!(f, "false"),
            E::Plus(e1, e2) => write!(f, "({} + {})", e1, e2),
            E::Mult(e1, e2) => write!(f, "({} * {})", e1, e2),
            E::Or(e1, e2) => write!(f, "({} || {})", e1, e2),
        }
    }
}

impl std::fmt::Display for Maybe<Either> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Maybe::Nothing => write!(f, "nothing"),
            Maybe::Just(Either::Left(0)) => write!(f, "0"),
            Maybe::Just(Either::Left(1)) => write!(f, "0"),
            Maybe::Just(Either::Right(true)) => write!(f, "true"),
            Maybe::Just(Either::Right(false)) => write!(f, "false"),
            _ => write!(f, "error"),
        }
    }
}


