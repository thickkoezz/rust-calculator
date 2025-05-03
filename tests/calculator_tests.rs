//! Tests for the rust-calculator library

#[cfg(test)]
mod tests {
  use rust_calculator::evaluate_expression;

  #[test]
  fn test_basic_operations() {
    // Addition
    assert_eq!(evaluate_expression("5 + 3").unwrap(), 8.0);

    // Subtraction
    assert_eq!(evaluate_expression("10 - 4").unwrap(), 6.0);

    // Multiplication
    assert_eq!(evaluate_expression("6 * 7").unwrap(), 42.0);

    // Division
    assert_eq!(evaluate_expression("15 / 3").unwrap(), 5.0);

    // Modulo
    assert_eq!(evaluate_expression("7 % 3").unwrap(), 1.0);

    // Exponentiation
    assert_eq!(evaluate_expression("2 ^ 3").unwrap(), 8.0);
  }

  #[test]
  fn test_mathematical_functions() {
    // Square root
    assert_eq!(evaluate_expression("sqrt 16").unwrap(), 4.0);

    // Trigonometric functions (using approximately equal due to floating point)
    assert!((evaluate_expression("sin 30").unwrap() - 0.5).abs() < 0.0001);
    assert!((evaluate_expression("cos 0").unwrap() - 1.0).abs() < 0.0001);
    assert!((evaluate_expression("tan 45").unwrap() - 1.0).abs() < 0.0001);

    // Logarithms
    assert_eq!(evaluate_expression("log 100").unwrap(), 2.0);
    assert_eq!(evaluate_expression("ln 1").unwrap(), 0.0);

    // Exponential
    assert!((evaluate_expression("exp 1").unwrap() - std::f64::consts::E).abs() < 0.0001);

    // Absolute value
    assert_eq!(evaluate_expression("abs -5").unwrap(), 5.0);

    // Floor and Ceiling
    assert_eq!(evaluate_expression("floor 3.7").unwrap(), 3.0);
    assert_eq!(evaluate_expression("ceil 3.2").unwrap(), 4.0);

    // Factorial
    assert_eq!(evaluate_expression("fact 0").unwrap(), 1.0);
    assert_eq!(evaluate_expression("fact 5").unwrap(), 120.0);
    assert_eq!(evaluate_expression("fact 10").unwrap(), 3628800.0);
  }

  #[test]
  fn test_mathematical_constants() {
    // Test pi constant
    assert!((evaluate_expression("pi").unwrap() - std::f64::consts::PI).abs() < 0.0001);

    // Test e constant
    assert!((evaluate_expression("e").unwrap() - std::f64::consts::E).abs() < 0.0001);

    // Test using constants in expressions
    assert!((evaluate_expression("2 * pi").unwrap() - (2.0 * std::f64::consts::PI)).abs() < 0.0001);
    assert!((evaluate_expression("e^2").unwrap() - std::f64::consts::E.powf(2.0)).abs() < 0.0001);
  }

  #[test]
  fn test_inverse_trigonometric_functions() {
    // Test inverse trigonometric functions
    assert!((evaluate_expression("asin 0.5").unwrap() - 30.0).abs() < 0.0001);
    assert!((evaluate_expression("acos 0").unwrap() - 90.0).abs() < 0.0001);
    assert!((evaluate_expression("atan 1").unwrap() - 45.0).abs() < 0.0001);

    // Test error handling for inverse trigonometric functions
    assert!(evaluate_expression("asin 2").is_err());
    assert!(evaluate_expression("acos -2").is_err());
  }

  #[test]
  fn test_complex_expressions() {
    // Test expressions with multiple operations
    assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), 14.0);
    assert_eq!(evaluate_expression("(2 + 3) * 4").unwrap(), 20.0);
    assert_eq!(evaluate_expression("3 * 4 + 2").unwrap(), 14.0);

    // Test expressions with functions and operators
    assert_eq!(evaluate_expression("sqrt 16 + 2").unwrap(), 6.0);
    assert_eq!(evaluate_expression("5 * sqrt 4").unwrap(), 10.0);

    // Test nested parentheses
    assert_eq!(evaluate_expression("2 * (3 + (4 - 1))").unwrap(), 12.0);

    // Test expressions with functions and parentheses
    assert_eq!(evaluate_expression("sqrt(16) + 2").unwrap(), 6.0);

    // Test operator precedence
    assert_eq!(evaluate_expression("2 + 3 * 4 ^ 2").unwrap(), 50.0);
    assert_eq!(evaluate_expression("(2 + 3 * 4) ^ 2").unwrap(), 196.0);
  }

  #[test]
  fn test_error_handling() {
    // Division by zero
    assert!(evaluate_expression("5 / 0").is_err());

    // Modulo by zero
    assert!(evaluate_expression("10 % 0").is_err());

    // Invalid format
    assert!(evaluate_expression("1 + + 2").is_err());

    // Invalid number
    assert!(evaluate_expression("abc + 3").is_err());

    // Square root of negative number
    assert!(evaluate_expression("sqrt -4").is_err());

    // Log of negative/zero
    assert!(evaluate_expression("log -1").is_err());
    assert!(evaluate_expression("ln 0").is_err());

    // Unknown function
    assert!(evaluate_expression("xyz 5").is_err());

    // Unknown operator
    assert!(evaluate_expression("5 $ 3").is_err());

    // Mismatched parentheses
    assert!(evaluate_expression("(2 + 3 * (4 - 1)").is_err());
    assert!(evaluate_expression("2 + 3) * 4").is_err());

    // Factorial errors
    assert!(evaluate_expression("fact -1").is_err());
    assert!(evaluate_expression("fact 1.5").is_err());
  }

  #[test]
  fn test_memory_functions() {
    // Reset memory to start with a clean state
    assert_eq!(evaluate_expression("mc").unwrap(), 0.0);

    // Add to memory
    assert_eq!(evaluate_expression("5 m+").unwrap(), 5.0);

    // Verify memory value
    assert_eq!(evaluate_expression("mr").unwrap(), 5.0);

    // Add more to memory
    assert_eq!(evaluate_expression("3 m+").unwrap(), 8.0);

    // Subtract from memory
    assert_eq!(evaluate_expression("2 m-").unwrap(), 6.0);

    // Verify final memory value
    assert_eq!(evaluate_expression("mr").unwrap(), 6.0);

    // Clear memory and verify
    assert_eq!(evaluate_expression("mc").unwrap(), 0.0);
    assert_eq!(evaluate_expression("mr").unwrap(), 0.0);
  }

  #[test]
  fn test_unit_conversions() {
    // Length conversions
    assert!((evaluate_expression("10 km_to_mi").unwrap() - 6.21371).abs() < 0.0001);
    assert!((evaluate_expression("5 mi_to_km").unwrap() - 8.0467).abs() < 0.0001);
    assert!((evaluate_expression("12 in_to_cm").unwrap() - 30.48).abs() < 0.0001);
    assert!((evaluate_expression("25.4 cm_to_in").unwrap() - 10.0).abs() < 0.0001);

    // Weight conversions
    assert!((evaluate_expression("100 kg_to_lb").unwrap() - 220.462).abs() < 0.0001);
    assert!((evaluate_expression("50 lb_to_kg").unwrap() - 22.6796).abs() < 0.0001);

    // Temperature conversions
    assert!((evaluate_expression("32 f_to_c").unwrap() - 0.0).abs() < 0.0001);
    assert!((evaluate_expression("100 c_to_f").unwrap() - 212.0).abs() < 0.0001);

    // Volume conversions
    assert!((evaluate_expression("1 gal_to_l").unwrap() - 3.78541).abs() < 0.0001);
    assert!((evaluate_expression("3.78541 l_to_gal").unwrap() - 1.0).abs() < 0.0001);

    // Angular measure conversions
    assert!((evaluate_expression("180 deg_to_rad").unwrap() - std::f64::consts::PI).abs() < 0.0001);
    assert!((evaluate_expression("pi rad_to_deg").unwrap() - 180.0).abs() < 0.0001);
  }

  #[test]
  fn test_variables() {
    // Reset any existing variables
    match rust_calculator::set_variable("test_var", 0.0) {
      Ok(_) => {}
      Err(_) => panic!("Failed to reset variables"),
    }

    // Set a variable
    match rust_calculator::set_variable("test_var", 42.0) {
      Ok(_) => {}
      Err(_) => panic!("Failed to set variable"),
    }

    // Use variable in an expression
    assert_eq!(evaluate_expression("test_var").unwrap(), 42.0);
    assert_eq!(evaluate_expression("test_var * 2").unwrap(), 84.0);

    // Use variable with other operations
    assert_eq!(evaluate_expression("test_var + 8").unwrap(), 50.0);
    assert_eq!(
      evaluate_expression("sqrt test_var").unwrap(),
      6.48074069840786
    );

    // Complex expressions with variables
    match rust_calculator::set_variable("x", 10.0) {
      Ok(_) => {}
      Err(_) => panic!("Failed to set variable x"),
    }
    match rust_calculator::set_variable("y", 5.0) {
      Ok(_) => {}
      Err(_) => panic!("Failed to set variable y"),
    }

    assert_eq!(evaluate_expression("x + y").unwrap(), 15.0);
    assert_eq!(evaluate_expression("x * y + 2").unwrap(), 52.0);
    assert_eq!(evaluate_expression("(x + y) * 2").unwrap(), 30.0);
  }
}
