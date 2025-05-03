//! Property-based tests for the rust-calculator
use proptest::prelude::*;
use rust_calculator::evaluate_expression;

// Helper function to test that a given binary operation follows the expected behavior
fn test_binary_op(
  a: f64,
  b: f64,
  op: &str,
  expected_fn: impl Fn(f64, f64) -> f64,
) -> Result<(), TestCaseError> {
  // Skip NaN, Infinity, and other special values that might cause issues
  if a.is_nan() || a.is_infinite() || b.is_nan() || b.is_infinite() {
    return Ok(());
  }

  // Skip division by zero cases
  if (op == "/" || op == "%") && b == 0.0 {
    return Ok(());
  }

  let expr = format!("{} {} {}", a, op, b);

  match evaluate_expression(&expr) {
    Ok(result) => {
      let expected = expected_fn(a, b);
      // Use approximate equality for floating point numbers
      prop_assert!(
        (result - expected).abs() < 0.0001,
        "Expression: {}, Got: {}, Expected: {}",
        expr,
        result,
        expected
      );
      Ok(())
    }
    Err(e) => {
      // We should handle known error cases like division by very small numbers
      // that might result in overflow
      if (op == "/" || op == "%") && b.abs() < 1e-10 {
        Ok(())
      } else {
        Err(TestCaseError::fail(format!(
          "Expression: {} failed with error: {}",
          expr, e
        )))
      }
    }
  }
}

proptest! {
    // Test basic operations with a wide range of numbers
    #[test]
    fn test_addition(a in -1000.0..1000.0, b in -1000.0..1000.0) {
        test_binary_op(a, b, "+", |x, y| x + y)?;
    }

    #[test]
    fn test_subtraction(a in -1000.0..1000.0, b in -1000.0..1000.0) {
        test_binary_op(a, b, "-", |x, y| x - y)?;
    }

    #[test]
    fn test_multiplication(a in -100.0..100.0, b in -100.0..100.0) {
        test_binary_op(a, b, "*", |x, y| x * y)?;
    }

    #[test]
    fn test_division(a in -100.0..100.0, b in -100.0..100.0) {
        if b == 0.0 { return Ok(()); }
        test_binary_op(a, b, "/", |x, y| x / y)?;
    }

    #[test]
    fn test_modulo(a in -100.0..100.0, b in -100.0..100.0) {
        if b == 0.0 { return Ok(()); }
        test_binary_op(a, b, "%", |x, y| x % y)?;
    }

    #[test]
    fn test_exponentiation(a in -10.0..10.0, b in 0.0..5.0) {
        // Skip when raising negative numbers to non-integer powers
        // as this would produce complex numbers, represented as NaN in Rust
        let b_typed: f64 = b;
        if a < 0.0 && !b_typed.fract().eq(&0.0) {
            return Ok(());
        }
        // Limit the range of powers to avoid excessive computation
        test_binary_op(a, b, "^", |x, y| x.powf(y))?;
    }

    // Test mathematical functions
    #[test]
    fn test_sqrt(x in 0.0..1000.0) {
        let expr = format!("sqrt {}", x);
        match evaluate_expression(&expr) {
            Ok(result) => {
                let x_typed: f64 = x;
                let expected: f64 = x_typed.sqrt();
                prop_assert!((result - expected).abs() < 0.0001);
            }
            Err(e) => {
                panic!("Expression: {} failed with error: {}", expr, e);
            }
        }
    }

    #[test]
    fn test_abs(x in -1000.0..1000.0) {
        let expr = format!("abs {}", x);
        match evaluate_expression(&expr) {
            Ok(result) => {
                let x_typed: f64 = x;
                let expected: f64 = x_typed.abs();
                prop_assert!((result - expected).abs() < 0.0001);
            }
            Err(e) => {
                panic!("Expression: {} failed with error: {}", expr, e);
            }
        }
    }

    // Test complex expressions
    #[test]
    fn test_complex_expressions(
        a in -10.0..10.0,
        b in -10.0..10.0,
        c in -10.0..10.0
    ) {
        if b == 0.0 || c == 0.0 { return Ok(()); }

        // Test an expression with multiple operations
        let expr = format!("{} + {} * {}", a, b, c);
        match evaluate_expression(&expr) {
            Ok(result) => {
                let result_typed: f64 = result;
                let expected: f64 = a + (b * c);
                prop_assert!((result_typed - expected).abs() < 0.0001);
            }
            Err(e) => {
                panic!("Expression: {} failed with error: {}", expr, e);
            }
        }

        // Test with parentheses to change order of operations
        let expr = format!("({} + {}) * {}", a, b, c);
        match evaluate_expression(&expr) {
            Ok(result) => {
                let result_typed: f64 = result;
                let expected: f64 = (a + b) * c;
                prop_assert!((result_typed - expected).abs() < 0.0001);
            }
            Err(e) => {
                panic!("Expression: {} failed with error: {}", expr, e);
            }
        }
    }
}
