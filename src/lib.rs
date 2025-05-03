//! # Rust Calculator Library
//!
//! Core functionality for a simple calculator written in Rust.
//! This library provides functions for evaluating mathematical expressions.
//!
//! ## Features
//!
//! - Basic operations: addition, subtraction, multiplication, division, modulo, and exponentiation
//! - Mathematical functions: sqrt, sin, cos, tan, log, ln, exp, abs, floor, ceil
//! - Mathematical constants: pi, e, tau, phi
//! - Memory functions: M+, M-, MR, MC
//! - History persistence between sessions
//! - User-defined variables
//! - Error handling for invalid inputs

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};

// Global memory state for calculator
static MEMORY: RwLock<f64> = RwLock::new(0.0);

// Global variables storage
lazy_static::lazy_static! {
    static ref VARIABLES: Arc<Mutex<HashMap<String, f64>>> = Arc::new(Mutex::new(HashMap::new()));
}

/// Adds an expression and its result to the history file
pub fn add_to_history(expression: &str, result: f64) -> io::Result<()> {
  let history_path = get_history_path();
  let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(history_path)?;

  writeln!(file, "{} = {}", expression, result)?;
  Ok(())
}

/// Gets the last n entries from the history file
pub fn get_history(n: usize) -> io::Result<Vec<String>> {
  let history_path = get_history_path();
  if !Path::new(&history_path).exists() {
    return Ok(Vec::new());
  }

  let file = File::open(history_path)?;
  let reader = BufReader::new(file);

  // Read all lines, but keep only the last n
  let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

  let start = if lines.len() > n { lines.len() - n } else { 0 };

  Ok(lines[start..].to_vec())
}

/// Clear history file
pub fn clear_history() -> io::Result<()> {
  let history_path = get_history_path();
  if Path::new(&history_path).exists() {
    // Truncate file to zero length
    let _file = OpenOptions::new()
      .write(true)
      .truncate(true)
      .open(history_path)?;
  }
  Ok(())
}

/// Get the path to the history file
pub fn get_history_path() -> String {
  if let Some(proj_dirs) = directories::ProjectDirs::from("com", "thickkoezz", "rust-calculator") {
    let data_dir = proj_dirs.data_dir();
    // Create directory if it doesn't exist
    std::fs::create_dir_all(data_dir).unwrap_or_default();
    format!("{}/calculator_history.txt", data_dir.display())
  } else {
    // Fallback to current directory
    "calculator_history.txt".to_string()
  }
}

/// Sets a user-defined variable to a specific value
pub fn set_variable(name: &str, value: f64) -> io::Result<()> {
  let mut vars = VARIABLES.lock().unwrap();
  vars.insert(name.to_lowercase(), value);

  // Optionally persist variables to a file
  let var_path = get_variables_path();
  let mut file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(var_path)?;

  for (key, val) in vars.iter() {
    writeln!(file, "{}={}", key, val)?;
  }

  Ok(())
}

/// Gets a user-defined variable's value
pub fn get_variable(name: &str) -> Option<f64> {
  let vars = VARIABLES.lock().unwrap();
  vars.get(&name.to_lowercase()).cloned()
}

/// Gets all user-defined variables
pub fn get_all_variables() -> io::Result<Vec<(String, f64)>> {
  let vars = VARIABLES.lock().unwrap();
  let result: Vec<(String, f64)> = vars.iter().map(|(k, v)| (k.clone(), *v)).collect();
  Ok(result)
}

/// Loads user-defined variables from persistence
pub fn load_variables() -> io::Result<()> {
  let var_path = get_variables_path();
  if !Path::new(&var_path).exists() {
    return Ok(());
  }

  let file = File::open(var_path)?;
  let reader = BufReader::new(file);
  let mut vars = VARIABLES.lock().unwrap();

  for line in reader.lines() {
    let line = line?;
    if let Some((name, value_str)) = line.split_once('=') {
      if let Ok(value) = value_str.parse::<f64>() {
        vars.insert(name.to_lowercase(), value);
      }
    }
  }

  Ok(())
}

/// Get the path to the variables file
fn get_variables_path() -> String {
  if let Some(proj_dirs) = directories::ProjectDirs::from("com", "thickkoezz", "rust-calculator") {
    let data_dir = proj_dirs.data_dir();
    // Create directory if it doesn't exist
    std::fs::create_dir_all(data_dir).unwrap_or_default();
    format!("{}/calculator_variables.txt", data_dir.display())
  } else {
    // Fallback to current directory
    "calculator_variables.txt".to_string()
  }
}

/// Custom error type for calculator operations
#[derive(Debug)]
pub enum CalculatorError {
  /// Error when parsing input
  ParseError(String),
  /// Error in mathematical operations (division by zero, etc.)
  MathError(String),
  /// Error with syntax of the expression
  SyntaxError(String),
  /// Error with function arguments (out of bounds, etc.)
  ArgumentError(String),
  /// Other errors
  Other(String),
}

impl std::fmt::Display for CalculatorError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      CalculatorError::ParseError(msg) => write!(f, "Parse error: {}", msg),
      CalculatorError::MathError(msg) => write!(f, "Math error: {}", msg),
      CalculatorError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
      CalculatorError::ArgumentError(msg) => write!(f, "Argument error: {}", msg),
      CalculatorError::Other(msg) => write!(f, "{}", msg),
    }
  }
}

impl std::error::Error for CalculatorError {}

/// Evaluates a mathematical expression and returns the result.
///
/// # Arguments
///
/// * `expression` - A string slice containing the mathematical expression to evaluate
///
/// # Returns
///
/// * `Result<f64, CalculatorError>` - Either the result of the evaluation or an error
///
/// # Examples
///
/// ```
/// let result = rust_calculator::evaluate_expression("5 + 3");
/// assert_eq!(result.unwrap(), 8.0);
///
/// let result = rust_calculator::evaluate_expression("sqrt 16");
/// assert_eq!(result.unwrap(), 4.0);
/// ```
pub fn evaluate_expression(expression: &str) -> Result<f64, CalculatorError> {
  // Handle special cases first
  let trimmed = expression.trim();

  // Memory commands without arguments
  if trimmed == "mr" {
    let memory = MEMORY.read().unwrap();
    return Ok(*memory);
  }
  if trimmed == "mc" {
    let mut memory = MEMORY.write().unwrap();
    *memory = 0.0;
    return Ok(0.0);
  }

  // Memory commands with arguments
  if let Some(rest) = trimmed.strip_suffix("m+") {
    let num = rest.trim();
    if let Ok(value) = num.parse::<f64>() {
      let mut memory = MEMORY.write().unwrap();
      *memory += value;
      return Ok(*memory);
    }
  }

  if let Some(rest) = trimmed.strip_suffix("m-") {
    let num = rest.trim();
    if let Ok(value) = num.parse::<f64>() {
      let mut memory = MEMORY.write().unwrap();
      *memory -= value;
      return Ok(*memory);
    }
  }

  // Unit conversions with various formats
  let conversions = [
    "km_to_mi",
    "mi_to_km",
    "kg_to_lb",
    "lb_to_kg",
    "c_to_f",
    "f_to_c",
    "rad_to_deg",
    "deg_to_rad",
    "in_to_cm",
    "cm_to_in",
    "gal_to_l",
    "l_to_gal",
  ];

  // Case 1: Format like "10 km_to_mi" (with space)
  let parts: Vec<&str> = trimmed.split_whitespace().collect();
  if parts.len() == 2 {
    // Try parsing first part as number
    if let Ok(value) = parts[0].parse::<f64>() {
      if conversions.contains(&parts[1]) {
        return evaluate_function(parts[1], value);
      }
    }
    // Special case for "pi rad_to_deg" and similar
    else if parts[0].to_lowercase() == "pi" && conversions.contains(&parts[1]) {
      return evaluate_function(parts[1], std::f64::consts::PI);
    }
    // Check for other constants
    else if parts[0].to_lowercase() == "e" && conversions.contains(&parts[1]) {
      return evaluate_function(parts[1], std::f64::consts::E);
    }
  }

  // Case 2: Format like "10km_to_mi" (without space)
  for conv in &conversions {
    if let Some(rest) = trimmed.strip_suffix(conv) {
      if let Ok(value) = rest.trim().parse::<f64>() {
        return evaluate_function(conv, value);
      }
    }
  }

  // Continue with normal tokenization for other expressions
  let tokens = tokenize(expression)?;

  // If there are no tokens, return an error
  if tokens.is_empty() {
    return Err(CalculatorError::SyntaxError("Empty expression".to_string()));
  }

  // Handle unary operations (functions)
  if tokens.len() == 2 && tokens[0].is_function() {
    let function = tokens[0].get_function()?;
    let value = tokens[1].get_number()?;
    return evaluate_function(function, value);
  }

  // Handle simple binary operations
  if tokens.len() == 3 && tokens[1].is_operator() {
    let left = tokens[0].get_number()?;
    let operator = tokens[1].get_operator()?;
    let right = tokens[2].get_number()?;
    return evaluate_binary_operation(left, operator, right);
  }

  // Handle complex expressions with operator precedence
  if tokens.len() > 3 {
    return evaluate_complex_expression(tokens);
  }

  // If we reach here with exactly one token, it must be a number
  if tokens.len() == 1 {
    return tokens[0].get_number();
  }

  Err(CalculatorError::SyntaxError(
    "Invalid expression format".to_string(),
  ))
}

/// Token enum to represent different parts of an expression
#[derive(Debug, Clone)]
enum Token {
  Number(f64),
  Operator(String),
  Function(String),
  LeftParen,
  RightParen,
}

impl Token {
  fn is_function(&self) -> bool {
    match self {
      Token::Function(_) => true,
      _ => false,
    }
  }

  fn is_operator(&self) -> bool {
    match self {
      Token::Operator(_) => true,
      _ => false,
    }
  }

  fn get_number(&self) -> Result<f64, CalculatorError> {
    match self {
      Token::Number(n) => Ok(*n),
      _ => Err(CalculatorError::ParseError("Expected a number".to_string())),
    }
  }

  fn get_operator(&self) -> Result<&str, CalculatorError> {
    match self {
      Token::Operator(op) => Ok(op),
      _ => Err(CalculatorError::ParseError(
        "Expected an operator".to_string(),
      )),
    }
  }

  fn get_function(&self) -> Result<&str, CalculatorError> {
    match self {
      Token::Function(fun) => Ok(fun),
      _ => Err(CalculatorError::ParseError(
        "Expected a function".to_string(),
      )),
    }
  }
}

/// Tokenize an expression string into a vector of tokens
fn tokenize(expression: &str) -> Result<Vec<Token>, CalculatorError> {
  let mut tokens = Vec::new();
  let mut current_token = String::new();
  let mut chars = expression.chars().peekable();

  // Special case for memory operations and unit conversions that don't require parameters
  if expression.trim() == "mr" || expression.trim() == "mc" {
    return Ok(vec![
      Token::Function(expression.trim().to_string()),
      Token::Number(0.0),
    ]);
  }

  while let Some(ch) = chars.next() {
    if ch.is_whitespace() {
      if !current_token.is_empty() {
        add_token(&mut tokens, &current_token)?;
        current_token.clear();
      }
    } else if ch.is_digit(10) || ch == '.' {
      current_token.push(ch);
    } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' || ch == '^' {
      // Handle special case for memory addition and subtraction
      if (ch == '+' || ch == '-') && !current_token.is_empty() && current_token == "m" {
        current_token.push(ch);
        // Add the m+ or m- token
        add_token(&mut tokens, &current_token)?;
        current_token.clear();
        continue;
      }

      // If we have a pending token, add it first
      if !current_token.is_empty() {
        add_token(&mut tokens, &current_token)?;
        current_token.clear();
      }

      // Special handling for negative numbers
      // If the token is '-' and it's either at the start or after another operator or left parenthesis
      if ch == '-'
        && (tokens.is_empty()
          || matches!(
            tokens.last(),
            Some(Token::Operator(_)) | Some(Token::LeftParen) | Some(Token::Function(_))
          ))
      {
        current_token.push(ch); // Start a negative number
      } else {
        tokens.push(Token::Operator(ch.to_string()));
      }
    } else if ch == '(' {
      if !current_token.is_empty() {
        // If we have a token before left paren, it's a function
        tokens.push(Token::Function(current_token.clone()));
        current_token.clear();
      }
      tokens.push(Token::LeftParen);
    } else if ch == ')' {
      if !current_token.is_empty() {
        add_token(&mut tokens, &current_token)?;
        current_token.clear();
      }
      tokens.push(Token::RightParen);
    } else {
      // Must be part of a function name or invalid character
      current_token.push(ch);
    }
  }

  // Don't forget to add the last token if any
  if !current_token.is_empty() {
    add_token(&mut tokens, &current_token)?;
  }

  // Special case handling for memory operations and unit conversions
  if tokens.len() >= 1 {
    // Handle memory operations: m+ and m-
    if tokens.len() == 1 && (tokens[0].is_function()) {
      let func = tokens[0].get_function()?;
      if func == "mr" || func == "mc" {
        tokens.push(Token::Number(0.0));
      }
    }

    // Handle unit conversions directly
    if tokens.len() == 2 && tokens[1].is_function() {
      let function = tokens[1].get_function()?;
      if function.contains("_to_") {
        let value = tokens[0].get_number()?;
        return Ok(vec![
          Token::Number(value),
          Token::Function(function.to_string()),
        ]);
      }
    }
  }

  // Special case handling for function calls without parentheses
  if tokens.len() >= 2 && tokens[0].is_function() {
    let next_token = &tokens[1];
    if let Token::Number(_) = next_token {
      // It's a function call without parentheses, like "sin 30"
      return Ok(tokens);
    }
  }

  Ok(tokens)
}

/// Helper function to add a token to the tokens vector
fn add_token(tokens: &mut Vec<Token>, token_str: &str) -> Result<(), CalculatorError> {
  // Try to parse as a number first
  if let Ok(num) = token_str.parse::<f64>() {
    tokens.push(Token::Number(num));
    return Ok(());
  }

  // Check for memory operations and unit conversions
  if token_str == "m+" || token_str == "m-" || token_str == "mr" || token_str == "mc" {
    tokens.push(Token::Function(token_str.to_string()));
    return Ok(());
  }

  // Check for unit conversion functions
  if token_str.contains("_to_") && is_valid_conversion(token_str) {
    tokens.push(Token::Function(token_str.to_string()));
    return Ok(());
  }

  // Check for mathematical constants
  match token_str.to_lowercase().as_str() {
    "pi" => {
      tokens.push(Token::Number(std::f64::consts::PI));
      return Ok(());
    }
    "e" => {
      tokens.push(Token::Number(std::f64::consts::E));
      return Ok(());
    }
    "tau" => {
      tokens.push(Token::Number(std::f64::consts::TAU)); // 2π
      return Ok(());
    }
    "phi" => {
      tokens.push(Token::Number(1.618033988749895)); // Golden ratio
      return Ok(());
    }
    "inf" | "infinity" => {
      tokens.push(Token::Number(f64::INFINITY));
      return Ok(());
    }
    _ => {}
  }

  // Check if it's a recognized function
  match token_str {
    "sqrt" | "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "log" | "ln" | "exp" | "abs"
    | "floor" | "ceil" | "fact" => {
      tokens.push(Token::Function(token_str.to_string()));
      Ok(())
    }
    _ => {
      // Check if it's a user-defined variable
      if let Some(value) = get_variable(token_str) {
        tokens.push(Token::Number(value));
        Ok(())
      } else {
        Err(CalculatorError::ParseError(format!(
          "Unknown token: {}",
          token_str
        )))
      }
    }
  }
}

/// Check if a string is a valid unit conversion function
fn is_valid_conversion(conversion: &str) -> bool {
  matches!(
    conversion,
    "km_to_mi"
      | "mi_to_km"
      | "kg_to_lb"
      | "lb_to_kg"
      | "c_to_f"
      | "f_to_c"
      | "rad_to_deg"
      | "deg_to_rad"
      | "in_to_cm"
      | "cm_to_in"
      | "gal_to_l"
      | "l_to_gal"
  )
}

/// Evaluate a complex expression with multiple operations, respecting operator precedence
fn evaluate_complex_expression(tokens: Vec<Token>) -> Result<f64, CalculatorError> {
  // Special case handling for memory operations and unit conversions
  if tokens.len() == 2 {
    if let Token::Function(func) = &tokens[0] {
      if func == "m+" || func == "m-" {
        if let Token::Number(num) = tokens[1] {
          return evaluate_function(func, num);
        }
      }
    }

    if let Token::Number(num) = tokens[0] {
      if let Token::Function(func) = &tokens[1] {
        if func.contains("_to_") {
          return evaluate_function(func, num);
        }
      }
    }
  }

  // Implementation of the Shunting Yard algorithm for expression evaluation
  // with proper operator precedence

  let mut output_queue: Vec<Token> = Vec::new();
  let mut operator_stack: Vec<Token> = Vec::new();

  for token in tokens {
    match &token {
      Token::Number(_) => output_queue.push(token),
      Token::Function(_) => operator_stack.push(token),
      Token::LeftParen => operator_stack.push(token),
      Token::RightParen => {
        // Pop operators until we find a left parenthesis
        let mut found_left_paren = false;
        while let Some(top) = operator_stack.last() {
          match top {
            Token::LeftParen => {
              operator_stack.pop(); // Remove the left parenthesis
              found_left_paren = true;
              break;
            }
            _ => {
              output_queue.push(operator_stack.pop().unwrap());
            }
          }
        }

        if !found_left_paren {
          return Err(CalculatorError::SyntaxError(
            "Mismatched parentheses: missing '('".to_string(),
          ));
        }

        // If we have a function at the top of the stack, pop it too
        if let Some(Token::Function(_)) = operator_stack.last() {
          output_queue.push(operator_stack.pop().unwrap());
        }
      }
      Token::Operator(op) => {
        // Handle operator precedence
        while let Some(top) = operator_stack.last() {
          if let Token::Operator(top_op) = top {
            // If the top operator has higher precedence, or equal precedence
            // and current operator is left-associative, pop it to the output queue
            if (get_precedence(op) <= get_precedence(top_op))
              && (op != "^" || get_precedence(op) < get_precedence(top_op))
            {
              output_queue.push(operator_stack.pop().unwrap());
            } else {
              break;
            }
          } else if let Token::Function(_) = top {
            output_queue.push(operator_stack.pop().unwrap());
          } else {
            break;
          }
        }
        operator_stack.push(token);
      }
    }
  }

  // Pop any remaining operators to the output queue
  while let Some(op) = operator_stack.pop() {
    match op {
      Token::LeftParen => {
        return Err(CalculatorError::SyntaxError(
          "Mismatched parentheses: missing ')'".to_string(),
        ));
      }
      Token::RightParen => {
        return Err(CalculatorError::SyntaxError(
          "Mismatched parentheses: extra ')'".to_string(),
        ));
      }
      _ => output_queue.push(op),
    }
  }

  // Evaluate the Reverse Polish Notation expression
  evaluate_rpn(output_queue)
}

/// Get the precedence level of an operator
fn get_precedence(op: &str) -> u8 {
  match op {
    "+" | "-" => 1,
    "*" | "/" | "%" => 2,
    "^" => 3,
    _ => 0,
  }
}

/// Evaluate a Reverse Polish Notation expression
fn evaluate_rpn(tokens: Vec<Token>) -> Result<f64, CalculatorError> {
  let mut stack: Vec<f64> = Vec::new();

  for token in tokens {
    match token {
      Token::Number(n) => stack.push(n),
      Token::Operator(op) => {
        if stack.len() < 2 {
          return Err(CalculatorError::SyntaxError(
            "Invalid expression: not enough operands".to_string(),
          ));
        }

        let right = stack.pop().unwrap();
        let left = stack.pop().unwrap();

        let result = evaluate_binary_operation(left, &op, right)?;
        stack.push(result);
      }
      Token::Function(func) => {
        if stack.is_empty() {
          return Err(CalculatorError::SyntaxError(
            "Invalid expression: function without argument".to_string(),
          ));
        }

        let arg = stack.pop().unwrap();
        let result = evaluate_function(&func, arg)?;
        stack.push(result);
      }
      _ => {
        return Err(CalculatorError::SyntaxError(
          "Unexpected token in RPN evaluation".to_string(),
        ));
      }
    }
  }

  if stack.len() != 1 {
    return Err(CalculatorError::SyntaxError(
      "Invalid expression: too many operands".to_string(),
    ));
  }

  Ok(stack.pop().unwrap())
}

/// Evaluates a binary operation with specified operands and operator
fn evaluate_binary_operation(
  left: f64,
  operator: &str,
  right: f64,
) -> Result<f64, CalculatorError> {
  match operator {
    "+" => Ok(left + right),
    "-" => Ok(left - right),
    "*" => Ok(left * right),
    "/" => {
      if right == 0.0 {
        Err(CalculatorError::MathError("Division by zero".to_string()))
      } else {
        Ok(left / right)
      }
    }
    "%" => {
      if right == 0.0 {
        Err(CalculatorError::MathError("Modulo by zero".to_string()))
      } else {
        Ok(left % right)
      }
    }
    "^" => Ok(left.powf(right)),
    _ => Err(CalculatorError::SyntaxError(format!(
      "Unknown operator: {}",
      operator
    ))),
  }
}

/// Evaluates a mathematical function (unary operation)
fn evaluate_function(function: &str, value: f64) -> Result<f64, CalculatorError> {
  match function {
    "sqrt" => {
      if value < 0.0 {
        Err(CalculatorError::ArgumentError(
          "Cannot calculate square root of negative number".to_string(),
        ))
      } else {
        Ok(value.sqrt())
      }
    }
    "sin" => Ok(value.to_radians().sin()),
    "cos" => Ok(value.to_radians().cos()),
    "tan" => Ok(value.to_radians().tan()),
    "asin" => {
      if value < -1.0 || value > 1.0 {
        Err(CalculatorError::ArgumentError(
          "Inverse sine argument must be between -1 and 1".to_string(),
        ))
      } else {
        Ok(value.asin().to_degrees())
      }
    }
    "acos" => {
      if value < -1.0 || value > 1.0 {
        Err(CalculatorError::ArgumentError(
          "Inverse cosine argument must be between -1 and 1".to_string(),
        ))
      } else {
        Ok(value.acos().to_degrees())
      }
    }
    "atan" => Ok(value.atan().to_degrees()),
    "log" => {
      if value <= 0.0 {
        Err(CalculatorError::ArgumentError(
          "Cannot calculate logarithm of non-positive number".to_string(),
        ))
      } else {
        Ok(value.log10())
      }
    }
    "ln" => {
      if value <= 0.0 {
        Err(CalculatorError::ArgumentError(
          "Cannot calculate natural logarithm of non-positive number".to_string(),
        ))
      } else {
        Ok(value.ln())
      }
    }
    "exp" => Ok(value.exp()),
    "abs" => Ok(value.abs()),
    "floor" => Ok(value.floor()),
    "ceil" => Ok(value.ceil()),
    "fact" => {
      // Factorial only works on non-negative integers
      if value < 0.0 {
        return Err(CalculatorError::ArgumentError(
          "Cannot calculate factorial of negative number".to_string(),
        ));
      }

      // Check if the value is an integer
      if value.fract() != 0.0 {
        return Err(CalculatorError::ArgumentError(
          "Factorial requires an integer value".to_string(),
        ));
      }

      // Calculate factorial
      let mut result = 1.0;
      for i in 2..=value as u64 {
        result *= i as f64;
      }
      Ok(result)
    }
    "m+" => {
      let mut memory = MEMORY.write().unwrap();
      *memory += value;
      Ok(*memory)
    }
    "m-" => {
      let mut memory = MEMORY.write().unwrap();
      *memory -= value;
      Ok(*memory)
    }
    "mr" => {
      let memory = MEMORY.read().unwrap();
      Ok(*memory)
    }
    "mc" => {
      let mut memory = MEMORY.write().unwrap();
      *memory = 0.0;
      Ok(*memory)
    }
    // Unit conversion functions
    "km_to_mi" => Ok(value * 0.621371), // Kilometers to miles
    "mi_to_km" => Ok(value * 1.60934),  // Miles to kilometers
    "kg_to_lb" => Ok(value * 2.20462),  // Kilograms to pounds
    "lb_to_kg" => Ok(value * 0.453592), // Pounds to kilograms
    "c_to_f" => Ok(value * 9.0 / 5.0 + 32.0), // Celsius to Fahrenheit
    "f_to_c" => Ok((value - 32.0) * 5.0 / 9.0), // Fahrenheit to Celsius
    "rad_to_deg" => Ok(value * 180.0 / std::f64::consts::PI), // Radians to degrees
    "deg_to_rad" => Ok(value * std::f64::consts::PI / 180.0), // Degrees to radians
    "in_to_cm" => Ok(value * 2.54),     // Inches to centimeters
    "cm_to_in" => Ok(value / 2.54),     // Centimeters to inches
    "gal_to_l" => Ok(value * 3.78541),  // US gallons to liters
    "l_to_gal" => Ok(value / 3.78541),  // Liters to US gallons
    _ => Err(CalculatorError::SyntaxError(format!(
      "Unknown function: {}",
      function
    ))),
  }
}
