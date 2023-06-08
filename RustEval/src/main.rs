fn main() {
    // Constructing an expression: (One + Zero) * ETrue
    let expr = E::Mult(
        Box::new(E::Plus(Box::new(E::One), Box::new(E::Zero))),
        Box::new(E::ETrue),
    );

    // Evaluating the expression
    let result = expr.evaluate();

    println!("Result: {}", result); // Output: Result: true
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
    fn evaluate(&self) -> bool {
        match self {
            E::One => true,
            E::Zero => false,
            E::ETrue => true,
            E::EFalse => false,
            E::Plus(e1, e2) => e1.evaluate() || e2.evaluate(),
            E::Mult(e1, e2) => e1.evaluate() && e2.evaluate(),
            E::Or(e1, e2) => e1.evaluate() || e2.evaluate(),
        }
    }
}

