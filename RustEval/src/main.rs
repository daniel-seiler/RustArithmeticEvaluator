pub enum E {
    Int {
        val: i32
    },
    Plus {
        left: Box<E>,
        right: Box<E>
    },
    Mult {
        left: Box<E>,
        right: Box<E>
    },
}

fn eval(e : &E) -> i32 {
    match e {
        E::Int { val } => return *val,
        E::Plus { left, right } => return eval(left) + eval(right),
        E::Mult { left, right } => return eval(left) * eval(right),
    }
}


fn main() {
    {
        let e = E::Int { val: 1 };
        println!("{}", eval(&e));
    }

    {
        let e = E::Plus{ left: Box::new(E::Int { val: 1 }), right: Box::new(E::Int { val: 2})};
        println!("{}", eval(&e));
    }
}
