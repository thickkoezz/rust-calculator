//! # Rust Calculator
//!
//! A simple command-line calculator written in Rust.
//! This calculator provides a CLI interface to the calculator library.

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::process;

/// The main function that runs the calculator application.
///
/// Provides a command-line interface where users can enter mathematical expressions
/// and receive immediate results. The program runs until the user types "exit".
fn main() -> Result<()> {
  println!("Rust Calculator");
  println!("Enter expressions like '2 + 3 * 4' or 'sin(30) + 5', 'exit' to quit");
  println!("Supported operators: +, -, *, /, %, ^ (in order of precedence)");
  println!(
    "Functions: sqrt, sin, cos, tan, asin, acos, atan, log, ln, exp, abs, floor, ceil, fact"
  );
  println!("Constants: pi, e, tau, phi");
  println!("Type 'help' for more information");

  // Initialize the rustyline editor
  let mut rl = DefaultEditor::new()?;

  // Get history path from our cross-platform implementation
  if let Ok(history) = rust_calculator::get_history(0) {
    // If we've previously saved history, initialize the editor with it
    for entry in history {
      rl.add_history_entry(&entry)?;
    }
    println!("Loaded previous history.");
  } else {
    println!("No previous history.");
  }

  // Load saved variables
  if let Err(err) = rust_calculator::load_variables() {
    eprintln!("Error loading variables: {}", err);
  }

  loop {
    let readline = rl.readline("> ");
    match readline {
      Ok(line) => {
        let input = line.trim();

        // Skip empty lines
        if input.is_empty() {
          continue;
        }

        // Add line to history
        rl.add_history_entry(input)?;

        // Handle special commands
        if input == "exit" {
          break;
        } else if input == "help" {
          display_help();
        } else if input == "clear" {
          // Clear the screen
          print!("\x1B[2J\x1B[1;1H");
        } else if input == "history" {
          display_history(10); // Show last 10 calculations
        } else if input == "clearhistory" {
          if let Err(err) = rust_calculator::clear_history() {
            eprintln!("Error clearing history: {}", err);
          } else {
            println!("History cleared");
          }
        } else if input == "vars" {
          list_variables();
        } else if input.starts_with("let ") {
          // Handle variable assignment: let varname = value
          define_variable(&input[4..]);
        } else {
          // Evaluate expression
          match rust_calculator::evaluate_expression(input) {
            Ok(result) => {
              println!("= {}", result);
              // Add to history
              if let Err(err) = rust_calculator::add_to_history(input, result) {
                eprintln!("Error saving to history: {}", err);
              }
            }
            Err(err) => eprintln!("Error: {}", err),
          }
        }
      }
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break;
      }
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break;
      }
      Err(err) => {
        eprintln!("Error: {}", err);
        process::exit(1);
      }
    }
  }

  // Save history
  let history_path = rust_calculator::get_history_path();
  if let Err(err) = rl.save_history(&history_path) {
    eprintln!("Error saving history: {}", err);
  }

  Ok(())
}

/// Display the last n entries from the calculation history
fn display_history(count: usize) {
  match rust_calculator::get_history(count) {
    Ok(entries) => {
      if entries.is_empty() {
        println!("No history available");
      } else {
        println!("Calculation History:");
        for entry in entries {
          println!("  {}", entry);
        }
      }
    }
    Err(err) => eprintln!("Error retrieving history: {}", err),
  }
}

/// Define a variable with the format "let varname = expression"
fn define_variable(input: &str) {
  // Parse the variable definition
  let parts: Vec<&str> = input.splitn(2, '=').collect();
  if parts.len() != 2 {
    eprintln!("Error: Invalid variable assignment. Format: let varname = value");
    return;
  }

  let var_name = parts[0].trim();
  let expression = parts[1].trim();

  // Evaluate the expression to get the value
  match rust_calculator::evaluate_expression(expression) {
    Ok(value) => {
      // Set the variable
      if let Err(err) = rust_calculator::set_variable(var_name, value) {
        eprintln!("Error setting variable: {}", err);
      } else {
        println!("Variable {} = {}", var_name, value);
      }
    }
    Err(err) => eprintln!("Error evaluating expression: {}", err),
  }
}

/// List all defined variables
fn list_variables() {
  println!("Defined Variables:");
  match rust_calculator::get_all_variables() {
    Ok(vars) => {
      if vars.is_empty() {
        println!("  No variables defined");
      } else {
        for (name, value) in vars {
          println!("  {} = {}", name, value);
        }
      }
    }
    Err(err) => eprintln!("Error retrieving variables: {}", err),
  }
}

/// Display help information
fn display_help() {
  println!("Rust Calculator Help:");
  println!("  - Type mathematical expressions to evaluate them");
  println!("  - Special commands:");
  println!("    * exit         - Exit the calculator");
  println!("    * help         - Display this help message");
  println!("    * clear        - Clear the screen");
  println!("    * history      - Show calculation history");
  println!("    * clearhistory - Clear calculation history");
  println!("    * vars         - List all defined variables");
  println!("    * let x = expr - Define a variable");
  println!();
  println!("  Operators (in order of precedence):");
  println!("    * ^  - Exponentiation (right associative)");
  println!("    * *, /, % - Multiplication, division, modulo");
  println!("    * +, - - Addition, subtraction");
  println!();
  println!("  Functions:");
  println!("    * sqrt(x)   - Square root");
  println!("    * sin(x)    - Sine (x in degrees)");
  println!("    * cos(x)    - Cosine (x in degrees)");
  println!("    * tan(x)    - Tangent (x in degrees)");
  println!("    * asin(x)   - Inverse sine (result in degrees)");
  println!("    * acos(x)   - Inverse cosine (result in degrees)");
  println!("    * atan(x)   - Inverse tangent (result in degrees)");
  println!("    * log(x)    - Base-10 logarithm");
  println!("    * ln(x)     - Natural logarithm");
  println!("    * exp(x)    - e raised to the power of x");
  println!("    * abs(x)    - Absolute value");
  println!("    * floor(x)  - Round down to nearest integer");
  println!("    * ceil(x)   - Round up to nearest integer");
  println!("    * fact(x)   - Factorial (x must be non-negative integer)");
  println!();
  println!("  Memory Functions:");
  println!("    * x m+      - Add x to memory");
  println!("    * x m-      - Subtract x from memory");
  println!("    * mr        - Recall memory value");
  println!("    * mc        - Clear memory");
  println!();
  println!("  Unit Conversions:");
  println!("    * x km_to_mi - Convert kilometers to miles");
  println!("    * x mi_to_km - Convert miles to kilometers");
  println!("    * x c_to_f   - Convert Celsius to Fahrenheit");
  println!("    * x f_to_c   - Convert Fahrenheit to Celsius");
  println!("    * (and many more - see documentation)");
  println!();
  println!("  Constants:");
  println!("    * pi  - The mathematical constant π (3.14159...)");
  println!("    * e   - The mathematical constant e (2.71828...)");
  println!("    * tau - 2π (6.28318...)");
  println!("    * phi - Golden ratio (1.61803...)");
  println!();
  println!("  Variables:");
  println!("    * Define: let varname = expression");
  println!("    * Use: varname (directly in expressions)");
  println!("    * List: vars");
}
