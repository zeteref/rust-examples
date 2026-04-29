#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

pub fn evaluate(expr: &Expr) -> Result<f64, &'static str> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Add(l, r) => Ok(evaluate(l)? + evaluate(r)?),
        Expr::Subtract(l, r) => Ok(evaluate(l)? - evaluate(r)?),
        Expr::Multiply(l, r) => Ok(evaluate(l)? * evaluate(r)?),
        Expr::Divide(l, r) => {
            let rhs = evaluate(r)?;
            if rhs == 0.0 {
                Err("division by zero")
            } else {
                Ok(evaluate(l)? / rhs)
            }
        }
        Expr::Negate(e) => Ok(-evaluate(e)?),
    }
}

pub fn to_string(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => {
            if *n == n.trunc() && n.is_finite() {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Expr::Negate(e) => format!("(-{})", to_string(e)),
        Expr::Add(l, r) => format!("({} + {})", to_string(l), to_string(r)),
        Expr::Subtract(l, r) => format!("({} - {})", to_string(l), to_string(r)),
        Expr::Multiply(l, r) => format!("({} * {})", to_string(l), to_string(r)),
        Expr::Divide(l, r) => format!("({} / {})", to_string(l), to_string(r)),
    }
}

pub fn count_operations(expr: &Expr) -> usize {
    match expr {
        Expr::Number(_) => 0,
        Expr::Negate(e) => 1 + count_operations(e),
        Expr::Add(l, r)
        | Expr::Subtract(l, r)
        | Expr::Multiply(l, r)
        | Expr::Divide(l, r) => 1 + count_operations(l) + count_operations(r),
    }
}

pub fn simplify(expr: &Expr) -> Expr {
    match expr {
        Expr::Add(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            match (&l, &r) {
                (Expr::Number(0.0), _) => r,
                (_, Expr::Number(0.0)) => l,
                _ => Expr::Add(Box::new(l), Box::new(r)),
            }
        }
        Expr::Subtract(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            match (&l, &r) {
                (_, Expr::Number(0.0)) => l,
                _ => Expr::Subtract(Box::new(l), Box::new(r)),
            }
        }
        Expr::Multiply(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            match (&l, &r) {
                (Expr::Number(1.0), _) => r,
                (_, Expr::Number(1.0)) => l,
                (Expr::Number(0.0), _) | (_, Expr::Number(0.0)) => Expr::Number(0.0),
                _ => Expr::Multiply(Box::new(l), Box::new(r)),
            }
        }
        Expr::Divide(l, r) => {
            let l = simplify(l);
            let r = simplify(r);
            match (&l, &r) {
                (_, Expr::Number(1.0)) => l,
                _ => Expr::Divide(Box::new(l), Box::new(r)),
            }
        }
        Expr::Negate(e) => {
            let e = simplify(e);
            match &e {
                Expr::Negate(inner) => *inner.clone(),
                _ => Expr::Negate(Box::new(e)),
            }
        }
        Expr::Number(_) => expr.clone(),
    }
}
