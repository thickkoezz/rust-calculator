# Rust Calculator

A powerful calculator written in Rust with both command-line and desktop graphical interfaces. This calculator supports various arithmetic operations, mathematical functions, unit conversions, and user-defined variables with a clean and intuitive interface.

![Version](https://img.shields.io/badge/version-0.2.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Build Status](https://img.shields.io/github/workflow/status/thickkoezz/rust-calculator/Rust%20Calculator%20CI)
![Last Updated](https://img.shields.io/badge/last%20updated-May%202025-orange)

## Features

- **Basic Operations**: Addition, subtraction, multiplication, division, modulo, and exponentiation
- **Mathematical Functions**: Square root, trigonometric functions (sin, cos, tan), inverse trigonometric functions (asin, acos, atan), logarithms (log, ln), exponential, absolute value, floor, ceiling, and factorial
- **Mathematical Constants**: Pi (π), e, tau (τ), phi (φ), and infinity
- **Unit Conversions**: 
  - Length (km to mi, mi to km, in to cm, cm to in)
  - Weight (kg to lb, lb to kg)
  - Temperature (C to F, F to C)
  - Volume (gal to l, l to gal)
  - Angular (degrees to radians, radians to degrees)
- **Memory Functions**: Memory add (M+), memory subtract (M-), memory recall (MR), memory clear (MC)
- **User-defined Variables**: Create, store, and use custom variables in expressions
- **Expression History**: Persistent calculation history between sessions
- **Multiple Interfaces**:
  - Command-line interface with history support and tab completion
  - Desktop graphical interface with keyboard support and modern styling
- **Error Handling**: Robust error handling for invalid inputs and mathematical errors

## Installation

### Prerequisites

- Rust and Cargo installed on your system. If not, install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Building from Source

1. Clone the repository:
   ```bash
   git clone https://github.com/thickkoezz/rust-calculator.git
   cd rust-calculator
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the command-line calculator:
   ```bash
   ./target/release/rust-calculator-cli
   ```

4. Run the desktop GUI calculator:
   ```bash
   ./target/release/rust-calculator-gui
   ```

### Pre-built Binaries

Pre-built binaries for Windows, macOS, and Linux are available on the [Releases](https://github.com/thickkoezz/rust-calculator/releases) page.

## Usage

### Command-line Interface

Once the CLI calculator is running, you can enter mathematical expressions in the following formats:

#### Binary Operations

Format: `number operator number`

```
> 5 + 3
= 8
> 10 - 4
= 6
> 6 * 7
= 42
> 20 / 5
= 4
> 10 % 3
= 1
> 2 ^ 3
= 8
```

#### Unary Functions

Format: `function(number)` or `function number`

```
> sqrt 16
= 4
> sin 90
= 1
> cos 0
= 1
> tan 45
= 1
> log 100
= 2
> ln 1
= 0
> exp 1
= 2.718281828459045
> abs -5
= 5
> floor 3.7
= 3
> ceil 3.2
= 4
> fact 5
= 120
```

#### Mathematical Constants

Format: `constant` or `constant operator number`

```
> pi
= 3.141592653589793
> 2 * e
= 5.43656365691809
> tau
= 6.283185307179586
> phi
= 1.618033988749895
```

#### Unit Conversions

Format: `number conversion_function` or `number<space>conversion_function`

```
> 10 km_to_mi
= 6.21371
> 5 mi_to_km
= 8.0467
> 100 c_to_f
= 212
> 32 f_to_c
= 0
> 180 deg_to_rad
= 3.141592653589793
> pi rad_to_deg
= 180
```

#### Memory Functions

```
> 5 m+    # Add 5 to memory
= 5
> 3 m+    # Add 3 to memory
= 8
> mr      # Recall memory value
= 8
> 2 m-    # Subtract 2 from memory
= 6
> mc      # Clear memory
= 0
```

#### User-defined Variables

```
> let x = 10
Variable x = 10
> let y = 5
Variable y = 5
> x + y
= 15
> vars    # List all variables
Defined Variables:
  x = 10
  y = 5
```

#### Special Commands

```
> help           # Display help information
> history        # Show calculation history
> clearhistory   # Clear calculation history
> clear          # Clear the screen
> exit           # Exit the calculator
```

#### Complex Expressions

The calculator supports complex expressions with proper operator precedence:

```
> 2 + 3 * 4
= 14
> (2 + 3) * 4
= 20
> sin(45) + cos(45)
= 1.414213562373095
> log(100) / ln(e^2)
= 1
```

### Desktop Interface

The desktop calculator provides a graphical interface with:
- Button grid for quick access to numbers, operators, and functions
- Text input field for complex expressions
- Result display area
- Full keyboard support
- Modern, styled interface with visual feedback
- Error handling with visual feedback

You can use the desktop calculator in several ways:
1. Click buttons on the interface to build expressions
2. Type directly into the input field
3. Use keyboard shortcuts for common operations

#### Keyboard Shortcuts
- Number keys (0-9): Input numbers
- `+`, `-`, `*`, `/`: Basic operations
- `^`: Exponentiation
- `(`, `)`: Parentheses
- `Enter` or `=`: Calculate result
- `Esc`: Clear all
- `Delete`: Clear entry
- `Backspace`: Delete last character

## Error Handling

The calculator handles various errors gracefully:

- Division by zero
- Square root of negative numbers
- Logarithm of non-positive numbers
- Factorial of negative numbers or non-integers
- Invalid number formats
- Unknown operators or functions
- Incorrect expression formats
- Mismatched parentheses

## Documentation

The project includes comprehensive documentation:

### Code Documentation

Generate the API documentation using Rust's built-in documentation tool:

```bash
cargo doc --open
```

This will generate and open the API documentation in your browser, showing detailed information about the calculator's functions, structures, and interfaces.

### User Manual

For more detailed information on using the calculator, refer to the [User Manual](https://github.com/thickkoezz/rust-calculator/wiki/User-Manual) in the project wiki.

## Testing

Run the test suite with:

```bash
cargo test
```

The project includes:
- Unit tests for core functionality
- Integration tests for the calculator library
- Property-based tests to verify mathematical properties

Run the benchmarks with:

```bash
cargo bench
```

Benchmarks measure performance of various calculator operations, helping to identify potential optimizations.

## Project Structure

```
rust-calculator/
├── src/
│   ├── lib.rs       # Core calculator functionality
│   ├── main.rs      # CLI interface implementation
│   └── gui.rs       # Desktop GUI implementation
├── tests/
│   ├── calculator_tests.rs    # Standard tests
│   └── property_tests.rs      # Property-based testing
├── benches/
│   └── calculator_benchmark.rs # Performance benchmarks
├── Cargo.toml       # Project configuration
└── README.md        # This file
```

## Dependencies

- [rustyline](https://github.com/kkawakam/rustyline) - Line editing for the CLI
- [iced](https://github.com/iced-rs/iced) - GUI framework
- [directories](https://github.com/dirs-dev/directories-rs) - Cross-platform file paths
- [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) - Lazy static initialization
- [proptest](https://github.com/AltSysrq/proptest) - Property-based testing
- [criterion](https://github.com/bheisler/criterion.rs) - Benchmarking

## Continuous Integration

This project uses GitHub Actions for continuous integration. Every push and pull request triggers:
- Running all tests
- Building the project
- Running benchmarks
- Checking code formatting
- Running Clippy for linting

## Contributing

Contributions are welcome! Feel free to submit pull requests or open issues to improve the calculator.

To contribute:
1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit your changes: `git commit -m 'Add amazing feature'`
4. Push to the branch: `git push origin feature/amazing-feature`
5. Open a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

[Mieky Sofyan Yudinata] - [@thickkoezz](https://github.com/thickkoezz)