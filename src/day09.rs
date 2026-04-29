// Day 9: Enums and Pattern Matching
//
// Build a recursive expression evaluator that uses enums for expression trees
// and pattern matching for evaluation, printing, and simplification.
//
// Learning goals:
//   - Defining enums with data (recursive via Box)
//   - Exhaustive `match` with destructuring
//   - `if let` syntax for concise single-pattern matches
//   - Recursive tree processing

/// A simple arithmetic expression.
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
}

/// Evaluates the expression, returning a Result.
/// Division by zero returns an error message.
pub fn evaluate(expr: &Expr) -> Result<f64, &'static str> {
    todo!("Implement evaluate using exhaustive match")
}

/// Converts the expression to a human-readable infix string.
/// Use parentheses to make operator precedence unambiguous.
/// Example: Add(Mul(Num(1), Num(2)), Num(3)) → "((1 * 2) + 3)"
pub fn to_string(expr: &Expr) -> String {
    todo!("Implement to_string")
}

/// Returns the total number of arithmetic operations in the expression.
/// Negate counts as one operation. Number does not count.
pub fn count_operations(expr: &Expr) -> usize {
    todo!("Implement count_operations")
}

/// Simplifies the expression where possible using these rewrite rules:
///   - Add(Number(0), x) → x
///   - Add(x, Number(0)) → x
///   - Subtract(x, Number(0)) → x
///   - Multiply(Number(1), x) → x
///   - Multiply(x, Number(1)) → x
///   - Multiply(Number(0), _) → Number(0)
///   - Multiply(_, Number(0)) → Number(0)
///   - Divide(x, Number(1)) → x
///   - Negate(Negate(x)) → x
///
/// Recursively simplifies sub-expressions first.
pub fn simplify(expr: &Expr) -> Expr {
    todo!("Implement simplify recursively")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a boxed expression
    fn num(n: f64) -> Box<Expr> {
        Box::new(Expr::Number(n))
    }
    fn add(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Add(l, r))
    }
    fn sub(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Subtract(l, r))
    }
    fn mul(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Multiply(l, r))
    }
    fn div(l: Box<Expr>, r: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Divide(l, r))
    }
    fn neg(e: Box<Expr>) -> Box<Expr> {
        Box::new(Expr::Negate(e))
    }

    #[test]
    fn evaluate_simple_number() {
        let expr = Expr::Number(42.0);
        assert_eq!(evaluate(&expr).unwrap(), 42.0);
    }

    #[test]
    fn evaluate_addition() {
        let expr = add(num(3.0), num(4.0));
        assert_eq!(evaluate(&expr).unwrap(), 7.0);
    }

    #[test]
    fn evaluate_all_operations() {
        // (10 + 2) * (8 - 3) / 6 = 12 * 5 / 6 = 10
        let expr = div(mul(add(num(10.0), num(2.0)), sub(num(8.0), num(3.0))), num(6.0));
        assert_eq!(evaluate(&expr).unwrap(), 10.0);
    }

    #[test]
    fn evaluate_negate() {
        let expr = neg(num(7.0));
        assert_eq!(evaluate(&expr).unwrap(), -7.0);
    }

    #[test]
    fn evaluate_double_negate() {
        let expr = neg(neg(num(5.0)));
        assert_eq!(evaluate(&expr).unwrap(), 5.0);
    }

    #[test]
    fn evaluate_division_by_zero_returns_error() {
        let expr = div(num(5.0), num(0.0));
        assert!(evaluate(&expr).is_err());
    }

    #[test]
    fn evaluate_nested_complex() {
        // ((1 + 2) * (4 - Negate(1))) = 3 * (4 - (-1)) = 3 * 5 = 15
        let expr = mul(add(num(1.0), num(2.0)), sub(num(4.0), neg(num(1.0))));
        assert_eq!(evaluate(&expr).unwrap(), 15.0);
    }

    #[test]
    fn to_string_simple_number() {
        assert_eq!(to_string(&Expr::Number(5.0)), "5");
    }

    #[test]
    fn to_string_addition() {
        let expr = add(num(1.0), num(2.0));
        assert_eq!(to_string(&expr), "(1 + 2)");
    }

    #[test]
    fn to_string_nested_with_precedence() {
        let expr = mul(add(num(1.0), num(2.0)), num(3.0));
        assert_eq!(to_string(&expr), "((1 + 2) * 3)");
    }

    #[test]
    fn to_string_negate() {
        let expr = neg(num(4.0));
        assert_eq!(to_string(&expr), "(-4)");
    }

    #[test]
    fn count_operations_number_is_zero() {
        assert_eq!(count_operations(&Expr::Number(42.0)), 0);
    }

    #[test]
    fn count_operations_complex_expression() {
        // Add(Add, Sub) = 1 add + 1 add + 1 sub = 3
        let expr = add(add(num(1.0), num(2.0)), sub(num(3.0), num(4.0)));
        assert_eq!(count_operations(&expr), 3);
    }

    #[test]
    fn count_operations_includes_negate() {
        let expr = neg(add(num(1.0), num(2.0)));
        assert_eq!(count_operations(&expr), 2); // Add + Negate
    }

    #[test]
    fn simplify_add_zero_left() {
        // 0 + 5 → 5
        let expr = add(num(0.0), num(5.0));
        assert_eq!(simplify(&expr), Expr::Number(5.0));
    }

    #[test]
    fn simplify_add_zero_right() {
        // 5 + 0 → 5
        let expr = add(num(5.0), num(0.0));
        assert_eq!(simplify(&expr), Expr::Number(5.0));
    }

    #[test]
    fn simplify_subtract_zero() {
        // 5 - 0 → 5
        let expr = sub(num(5.0), num(0.0));
        assert_eq!(simplify(&expr), Expr::Number(5.0));
    }

    #[test]
    fn simplify_multiply_by_one() {
        // 1 * 7 → 7
        let expr = mul(num(1.0), num(7.0));
        assert_eq!(simplify(&expr), Expr::Number(7.0));
    }

    #[test]
    fn simplify_multiply_by_zero() {
        // 6 * 0 → 0
        let expr = mul(num(6.0), num(0.0));
        assert_eq!(simplify(&expr), Expr::Number(0.0));
    }

    #[test]
    fn simplify_divide_by_one() {
        // 9 / 1 → 9
        let expr = div(num(9.0), num(1.0));
        assert_eq!(simplify(&expr), Expr::Number(9.0));
    }

    #[test]
    fn simplify_negate_negate() {
        // -(-42) → 42
        let expr = neg(neg(num(42.0)));
        assert_eq!(simplify(&expr), Expr::Number(42.0));
    }

    #[test]
    fn simplify_recursive() {
        // 0 + (1 * (5 - 0)) → 0 + (1 * 5) → 0 + 5 → 5
        let expr = add(num(0.0), mul(num(1.0), sub(num(5.0), num(0.0))));
        assert_eq!(simplify(&expr), Expr::Number(5.0));
    }
}
