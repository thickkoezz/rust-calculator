//! # Rust Calculator GUI
//!
//! A desktop calculator application written in Rust using the Iced GUI library.
//! This calculator provides a graphical interface to the calculator library.

use iced::{
  Application, Color, Command, Element, Length, Settings, Subscription, Theme, alignment, executor,
  keyboard, subscription,
  widget::{Button, Column, Container, Row, Text, button, container},
  window,
};
use rust_calculator::evaluate_expression;
use std::vec;

// Custom theme colors
const DARK_BG: Color = Color::from_rgb(0.15, 0.15, 0.20);
const DISPLAY_BG: Color = Color::from_rgb(0.2, 0.2, 0.25);
const TEXT_COLOR: Color = Color::from_rgb(0.9, 0.9, 0.9);
const BUTTON_BG: Color = Color::from_rgb(0.25, 0.25, 0.30);
const OPERATOR_BG: Color = Color::from_rgb(0.3, 0.5, 0.8);
const FUNCTION_BG: Color = Color::from_rgb(0.4, 0.4, 0.7);
const CLEAR_BG: Color = Color::from_rgb(0.8, 0.3, 0.3);
const EQUALS_BG: Color = Color::from_rgb(0.3, 0.7, 0.4);
const CONSTANT_BG: Color = Color::from_rgb(0.7, 0.5, 0.2);
const BUTTON_TEXT: Color = Color::WHITE;

// Custom styling for different calculator components
struct CalculatorButtonStyle {
  background: Color,
}

impl From<CalculatorButtonStyle> for iced::theme::Button {
  fn from(style: CalculatorButtonStyle) -> Self {
    iced::theme::Button::Custom(Box::new(style))
  }
}

impl button::StyleSheet for CalculatorButtonStyle {
  type Style = iced::Theme;

  fn active(&self, _style: &Self::Style) -> button::Appearance {
    button::Appearance {
      background: Some(self.background.into()),
      border_radius: 8.0,
      text_color: BUTTON_TEXT,
      shadow_offset: iced::Vector::new(1.0, 2.0),
      border_width: 0.0,
      border_color: Color::TRANSPARENT,
      ..button::Appearance::default()
    }
  }

  fn hovered(&self, style: &Self::Style) -> button::Appearance {
    // Return the same appearance as active, effectively disabling hover effects
    self.active(style)
  }

  fn pressed(&self, style: &Self::Style) -> button::Appearance {
    button::Appearance {
      background: Some(
        Color {
          a: 0.9,
          ..self.background
        }
        .into(),
      ),
      shadow_offset: iced::Vector::new(0.0, 0.0),
      ..self.active(style)
    }
  }
}

// Display container style
struct DisplayStyle;

impl From<DisplayStyle> for iced::theme::Container {
  fn from(_: DisplayStyle) -> Self {
    iced::theme::Container::Custom(Box::new(DisplayStyle))
  }
}

impl container::StyleSheet for DisplayStyle {
  type Style = iced::Theme;

  fn appearance(&self, _style: &Self::Style) -> container::Appearance {
    container::Appearance {
      background: Some(DISPLAY_BG.into()),
      border_radius: 6.0,
      border_width: 1.0,
      border_color: Color {
        a: 0.3,
        ..DISPLAY_BG
      },
      ..container::Appearance::default()
    }
  }
}

// Main calculator container style
struct CalculatorContainerStyle;

impl From<CalculatorContainerStyle> for iced::theme::Container {
  fn from(_: CalculatorContainerStyle) -> Self {
    iced::theme::Container::Custom(Box::new(CalculatorContainerStyle))
  }
}

impl container::StyleSheet for CalculatorContainerStyle {
  type Style = iced::Theme;

  fn appearance(&self, _style: &Self::Style) -> container::Appearance {
    container::Appearance {
      background: Some(DARK_BG.into()),
      border_radius: 0.0,
      border_width: 0.0,
      border_color: Color::TRANSPARENT,
      ..container::Appearance::default()
    }
  }
}

// Text style for display
struct DisplayTextStyle;

impl From<DisplayTextStyle> for iced::theme::Text {
  fn from(_: DisplayTextStyle) -> Self {
    iced::theme::Text::Color(TEXT_COLOR)
  }
}

// Define our Calculator application state
pub struct Calculator {
  input: String,
  result: String,
  last_key: Option<Key>,
  buttons: Vec<CalcButton>,
}

// Different types of calculator buttons
#[derive(Debug, Clone)]
enum CalcButton {
  Number(String),
  Operator(String),
  Function(String),
  Clear,
  ClearEntry,
  Equals,
  Backspace,
  LeftParen,
  RightParen,
  Constant(String),
}

// Represent a key press event
#[derive(Debug, Clone)]
pub enum Message {
  InputChanged(String),
  KeyPressed(Key),
  Calculate,
  Clear,
  ClearEntry,
  Backspace,
  KeyboardInput(keyboard::Event),
}

// Key identification for buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
  Number(u8),
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  Power,
  Equals,
  Clear,
  ClearEntry,
  Backspace,
  Decimal,
  LeftParen,
  RightParen,
  Function(Function),
  Constant(Constant),
}

// Available functions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
  Sqrt,
  Sin,
  Cos,
  Tan,
  ASin,
  ACos,
  ATan,
  Log,
  Ln,
  Exp,
  Abs,
  Floor,
  Ceil,
  Factorial,
}

// Available constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Constant {
  Pi,
  E,
}

impl CalcButton {
  fn new(key: Key) -> Self {
    match key {
      Key::Number(num) => CalcButton::Number(num.to_string()),
      Key::Add => CalcButton::Operator("+".to_string()),
      Key::Subtract => CalcButton::Operator("-".to_string()),
      Key::Multiply => CalcButton::Operator("*".to_string()),
      Key::Divide => CalcButton::Operator("/".to_string()),
      Key::Modulo => CalcButton::Operator("%".to_string()),
      Key::Power => CalcButton::Operator("^".to_string()),
      Key::Clear => CalcButton::Clear,
      Key::ClearEntry => CalcButton::ClearEntry,
      Key::Equals => CalcButton::Equals,
      Key::Backspace => CalcButton::Backspace,
      Key::Decimal => CalcButton::Number(".".to_string()),
      Key::LeftParen => CalcButton::LeftParen,
      Key::RightParen => CalcButton::RightParen,
      Key::Function(func) => {
        let func_name = match func {
          Function::Sqrt => "sqrt",
          Function::Sin => "sin",
          Function::Cos => "cos",
          Function::Tan => "tan",
          Function::ASin => "asin",
          Function::ACos => "acos",
          Function::ATan => "atan",
          Function::Log => "log",
          Function::Ln => "ln",
          Function::Exp => "exp",
          Function::Abs => "abs",
          Function::Floor => "floor",
          Function::Ceil => "ceil",
          Function::Factorial => "fact",
        };
        CalcButton::Function(func_name.to_string())
      }
      Key::Constant(c) => {
        let const_name = match c {
          Constant::Pi => "pi",
          Constant::E => "e",
        };
        CalcButton::Constant(const_name.to_string())
      }
    }
  }

  fn label(&self) -> String {
    match self {
      CalcButton::Number(num) => num.clone(),
      CalcButton::Operator(op) => op.clone(),
      CalcButton::Function(func) => func.clone(),
      CalcButton::Clear => "C".to_string(),
      CalcButton::ClearEntry => "CE".to_string(),
      CalcButton::Equals => "=".to_string(),
      CalcButton::Backspace => "DEL".to_string(),
      CalcButton::LeftParen => "(".to_string(),
      CalcButton::RightParen => ")".to_string(),
      CalcButton::Constant(c) => c.clone(),
    }
  }

  fn key(&self) -> Key {
    match self {
      CalcButton::Number(num) => {
        if num == "." {
          Key::Decimal
        } else {
          Key::Number(num.parse().unwrap_or(0))
        }
      }
      CalcButton::Operator(op) => match op.as_str() {
        "+" => Key::Add,
        "-" => Key::Subtract,
        "*" => Key::Multiply,
        "/" => Key::Divide,
        "%" => Key::Modulo,
        "^" => Key::Power,
        _ => panic!("Unknown operator"),
      },
      CalcButton::Function(func) => {
        let function = match func.as_str() {
          "sqrt" => Function::Sqrt,
          "sin" => Function::Sin,
          "cos" => Function::Cos,
          "tan" => Function::Tan,
          "asin" => Function::ASin,
          "acos" => Function::ACos,
          "atan" => Function::ATan,
          "log" => Function::Log,
          "ln" => Function::Ln,
          "exp" => Function::Exp,
          "abs" => Function::Abs,
          "floor" => Function::Floor,
          "ceil" => Function::Ceil,
          "fact" => Function::Factorial,
          _ => panic!("Unknown function"),
        };
        Key::Function(function)
      }
      CalcButton::Clear => Key::Clear,
      CalcButton::ClearEntry => Key::ClearEntry,
      CalcButton::Equals => Key::Equals,
      CalcButton::Backspace => Key::Backspace,
      CalcButton::LeftParen => Key::LeftParen,
      CalcButton::RightParen => Key::RightParen,
      CalcButton::Constant(c) => {
        let constant = match c.as_str() {
          "pi" => Constant::Pi,
          "e" => Constant::E,
          _ => panic!("Unknown constant"),
        };
        Key::Constant(constant)
      }
    }
  }
}

impl Application for Calculator {
  type Executor = executor::Default;
  type Message = Message;
  type Flags = ();
  type Theme = Theme;

  fn new(_flags: ()) -> (Self, Command<Message>) {
    let mut calculator = Calculator {
      input: String::new(),
      result: String::new(),
      last_key: None,
      buttons: vec![],
    };

    // Create calculator buttons in the desired layout
    calculator.buttons = create_buttons();

    (calculator, Command::none())
  }

  fn title(&self) -> String {
    String::from("Rust Calculator")
  }

  fn update(&mut self, message: Message) -> Command<Message> {
    match message {
      Message::InputChanged(input) => {
        self.input = input;
        Command::none()
      }
      Message::Calculate => {
        match evaluate_expression(&self.input) {
          Ok(result) => {
            // Ensure we format the number properly
            if result.fract() == 0.0 && result.abs() < 1e12 {
              self.result = format!("{:.0}", result);
            } else {
              self.result = format!("{}", result);
            }
          }
          Err(err) => {
            self.result = format!("Error: {}", err);
          }
        }
        Command::none()
      }
      Message::Clear => {
        self.input.clear();
        self.result.clear();
        self.last_key = Some(Key::Clear);
        Command::none()
      }
      Message::ClearEntry => {
        self.input.clear();
        self.last_key = Some(Key::ClearEntry);
        Command::none()
      }
      Message::Backspace => {
        self.input.pop();
        self.last_key = Some(Key::Backspace);
        Command::none()
      }
      Message::KeyboardInput(event) => {
        if let keyboard::Event::KeyPressed {
          key_code,
          modifiers,
        } = event
        {
          match key_code {
            keyboard::KeyCode::Key1 | keyboard::KeyCode::Numpad1 => {
              return self.update(Message::KeyPressed(Key::Number(1)));
            }
            keyboard::KeyCode::Key2 | keyboard::KeyCode::Numpad2 => {
              return self.update(Message::KeyPressed(Key::Number(2)));
            }
            keyboard::KeyCode::Key3 | keyboard::KeyCode::Numpad3 => {
              return self.update(Message::KeyPressed(Key::Number(3)));
            }
            keyboard::KeyCode::Key4 | keyboard::KeyCode::Numpad4 => {
              return self.update(Message::KeyPressed(Key::Number(4)));
            }
            keyboard::KeyCode::Key5 | keyboard::KeyCode::Numpad5 => {
              // On some keyboards, % is Shift+5, so we'll map regular 5 to both number 5 and modulo
              return self.update(Message::KeyPressed(Key::Number(5)));
            }
            keyboard::KeyCode::Key6 | keyboard::KeyCode::Numpad6 => {
              return self.update(Message::KeyPressed(Key::Number(6)));
            }
            keyboard::KeyCode::Key7 | keyboard::KeyCode::Numpad7 => {
              return self.update(Message::KeyPressed(Key::Number(7)));
            }
            keyboard::KeyCode::Key8 | keyboard::KeyCode::Numpad8 => {
              return self.update(Message::KeyPressed(Key::Number(8)));
            }
            keyboard::KeyCode::Key9 => {
              // Check for Shift+9 which is often used for left parenthesis
              if modifiers.shift() {
                return self.update(Message::KeyPressed(Key::LeftParen));
              }
              return self.update(Message::KeyPressed(Key::Number(9)));
            }
            keyboard::KeyCode::Numpad9 => {
              return self.update(Message::KeyPressed(Key::Number(9)));
            }
            keyboard::KeyCode::Key0 => {
              // Check for Shift+0 which is often used for right parenthesis
              if modifiers.shift() {
                return self.update(Message::KeyPressed(Key::RightParen));
              }
              return self.update(Message::KeyPressed(Key::Number(0)));
            }
            keyboard::KeyCode::Numpad0 => {
              return self.update(Message::KeyPressed(Key::Number(0)));
            }
            keyboard::KeyCode::Plus | keyboard::KeyCode::NumpadAdd => {
              return self.update(Message::KeyPressed(Key::Add));
            }
            keyboard::KeyCode::Minus | keyboard::KeyCode::NumpadSubtract => {
              return self.update(Message::KeyPressed(Key::Subtract));
            }
            keyboard::KeyCode::Asterisk | keyboard::KeyCode::NumpadMultiply => {
              return self.update(Message::KeyPressed(Key::Multiply));
            }
            keyboard::KeyCode::Slash | keyboard::KeyCode::NumpadDivide => {
              return self.update(Message::KeyPressed(Key::Divide));
            }
            keyboard::KeyCode::Period | keyboard::KeyCode::NumpadDecimal => {
              return self.update(Message::KeyPressed(Key::Decimal));
            }
            keyboard::KeyCode::Backspace => {
              return self.update(Message::KeyPressed(Key::Backspace));
            }
            keyboard::KeyCode::Escape => return self.update(Message::KeyPressed(Key::Clear)),
            keyboard::KeyCode::Delete => return self.update(Message::KeyPressed(Key::ClearEntry)),
            keyboard::KeyCode::Enter | keyboard::KeyCode::NumpadEnter => {
              return self.update(Message::KeyPressed(Key::Equals));
            }
            keyboard::KeyCode::Equals => return self.update(Message::KeyPressed(Key::Equals)),
            keyboard::KeyCode::Caret => return self.update(Message::KeyPressed(Key::Power)),
            keyboard::KeyCode::LBracket => return self.update(Message::KeyPressed(Key::LeftParen)),
            keyboard::KeyCode::RBracket => {
              return self.update(Message::KeyPressed(Key::RightParen));
            }
            _ => {}
          }
        }
        Command::none()
      }
      Message::KeyPressed(key) => {
        self.last_key = Some(key);

        match key {
          Key::Number(num) => {
            self.input.push_str(&num.to_string());
          }
          Key::Add => self.input.push('+'),
          Key::Subtract => self.input.push('-'),
          Key::Multiply => self.input.push('*'),
          Key::Divide => self.input.push('/'),
          Key::Modulo => self.input.push('%'),
          Key::Power => self.input.push('^'),
          Key::Decimal => self.input.push('.'),
          Key::Equals => {
            return Command::perform(async { () }, |_| Message::Calculate);
          }
          Key::LeftParen => self.input.push('('),
          Key::RightParen => self.input.push(')'),
          Key::Function(func) => {
            let func_str = match func {
              Function::Sqrt => "sqrt",
              Function::Sin => "sin",
              Function::Cos => "cos",
              Function::Tan => "tan",
              Function::ASin => "asin",
              Function::ACos => "acos",
              Function::ATan => "atan",
              Function::Log => "log",
              Function::Ln => "ln",
              Function::Exp => "exp",
              Function::Abs => "abs",
              Function::Floor => "floor",
              Function::Ceil => "ceil",
              Function::Factorial => "fact",
            };
            self.input.push_str(func_str);
            self.input.push('(');
          }
          Key::Constant(c) => {
            let const_str = match c {
              Constant::Pi => "pi",
              Constant::E => "e",
            };
            self.input.push_str(const_str);
          }
          Key::Clear => return Command::perform(async { () }, |_| Message::Clear),
          Key::ClearEntry => return Command::perform(async { () }, |_| Message::ClearEntry),
          Key::Backspace => return Command::perform(async { () }, |_| Message::Backspace),
        }

        Command::none()
      }
    }
  }

  fn view(&self) -> Element<Message> {
    // Create the calculator display - using a non-interactive container with styled text
    // instead of TextInput to prevent mouse interactions
    let input_display = Container::new(
      Text::new(if self.input.is_empty() {
        "Enter expression..."
      } else {
        &self.input
      })
      .size(30)
      .width(Length::Fill)
      .horizontal_alignment(alignment::Horizontal::Left)
      .style(DisplayTextStyle),
    )
    .padding(10)
    .width(Length::Fill)
    .style(DisplayStyle);

    // Create the result display
    let result = Text::new(if self.result.is_empty() {
      "Result will appear here"
    } else {
      &self.result
    })
    .size(24)
    .width(Length::Fill)
    .horizontal_alignment(alignment::Horizontal::Right)
    .style(DisplayTextStyle);

    // Create button grid layout
    let mut button_rows: Vec<Row<'_, Message>> = vec![];
    let mut current_row = Row::new()
      .spacing(5)
      .padding(5)
      .height(Length::FillPortion(1));
    let buttons_per_row = 5; // Define buttons_per_row outside the loop

    for (i, button) in self.buttons.iter().enumerate() {
      // Check if we need to start a new row (buttons per row)
      if i > 0 && i % buttons_per_row == 0 {
        button_rows.push(current_row);
        current_row = Row::new()
          .spacing(5)
          .padding(5)
          .height(Length::FillPortion(1));
      }

      // Create a button with appropriate style and add to the current row
      let btn = Button::new(
        Text::new(button.label())
          .horizontal_alignment(alignment::Horizontal::Center)
          .vertical_alignment(alignment::Vertical::Center)
          .width(Length::Fill)
          .height(Length::Fill)
          .size(20),
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .on_press(Message::KeyPressed(button.key()))
      .style(iced::theme::Button::Custom(Box::new(
        CalculatorButtonStyle {
          background: match button {
            CalcButton::Clear => CLEAR_BG,
            CalcButton::Equals => EQUALS_BG,
            CalcButton::Operator(_) => OPERATOR_BG,
            CalcButton::Function(_) => FUNCTION_BG,
            CalcButton::Constant(_) => CONSTANT_BG,
            _ => BUTTON_BG,
          },
        },
      )));

      current_row = current_row.push(btn);
    }

    // Add the last row if it has any buttons
    if self.buttons.len() % buttons_per_row != 0 {
      button_rows.push(current_row);
    }

    // Combine everything into a column
    let mut content = Column::new()
      .padding(10)
      .spacing(10)
      .width(Length::Fill)
      .height(Length::Fill);

    // Add input and result displays, with smaller vertical proportions
    content = content
      .push(input_display.height(Length::FillPortion(2)))
      .push(result.height(Length::FillPortion(1)));

    // Add all button rows to the column with equal proportions
    for row in button_rows {
      content = content.push(row);
    }

    Container::new(content)
      .width(Length::Fill)
      .height(Length::Fill)
      .padding(5)
      .style(CalculatorContainerStyle)
      .into()
  }

  fn subscription(&self) -> Subscription<Message> {
    // Only subscribe to keyboard events and filter out other events
    subscription::events_with(|event, _| {
      if let iced::Event::Keyboard(keyboard_event) = event {
        Some(Message::KeyboardInput(keyboard_event))
      } else {
        None
      }
    })
  }
}

// Create the calculator buttons layout
fn create_buttons() -> Vec<CalcButton> {
  let mut buttons = Vec::new();

  // First row - Clear, functions, etc.
  buttons.push(CalcButton::new(Key::Clear));
  buttons.push(CalcButton::new(Key::ClearEntry));
  buttons.push(CalcButton::new(Key::Backspace));
  buttons.push(CalcButton::new(Key::LeftParen));
  buttons.push(CalcButton::new(Key::RightParen));

  // Second row - Functions and constants
  buttons.push(CalcButton::new(Key::Function(Function::Sqrt)));
  buttons.push(CalcButton::new(Key::Function(Function::Sin)));
  buttons.push(CalcButton::new(Key::Function(Function::Cos)));
  buttons.push(CalcButton::new(Key::Function(Function::Tan)));
  buttons.push(CalcButton::new(Key::Constant(Constant::Pi)));

  // Third row - More functions
  buttons.push(CalcButton::new(Key::Function(Function::Log)));
  buttons.push(CalcButton::new(Key::Function(Function::Ln)));
  buttons.push(CalcButton::new(Key::Function(Function::Exp)));
  buttons.push(CalcButton::new(Key::Power));
  buttons.push(CalcButton::new(Key::Constant(Constant::E)));

  // Fourth row - Numbers 7, 8, 9 and operators
  buttons.push(CalcButton::new(Key::Number(7)));
  buttons.push(CalcButton::new(Key::Number(8)));
  buttons.push(CalcButton::new(Key::Number(9)));
  buttons.push(CalcButton::new(Key::Divide));
  buttons.push(CalcButton::new(Key::Modulo));

  // Fifth row - Numbers 4, 5, 6 and operators
  buttons.push(CalcButton::new(Key::Number(4)));
  buttons.push(CalcButton::new(Key::Number(5)));
  buttons.push(CalcButton::new(Key::Number(6)));
  buttons.push(CalcButton::new(Key::Multiply));
  buttons.push(CalcButton::new(Key::Function(Function::Abs)));

  // Sixth row - Numbers 1, 2, 3 and operators
  buttons.push(CalcButton::new(Key::Number(1)));
  buttons.push(CalcButton::new(Key::Number(2)));
  buttons.push(CalcButton::new(Key::Number(3)));
  buttons.push(CalcButton::new(Key::Subtract));
  buttons.push(CalcButton::new(Key::Function(Function::Factorial)));

  // Seventh row - 0, decimal, equals and operators
  buttons.push(CalcButton::new(Key::Number(0)));
  buttons.push(CalcButton::new(Key::Decimal));
  buttons.push(CalcButton::new(Key::Equals));
  buttons.push(CalcButton::new(Key::Add));

  buttons
}

fn main() -> iced::Result {
  // Set up application with sensible defaults
  let settings = Settings {
    window: window::Settings {
      size: (400, 600),
      min_size: Some((320, 480)),
      max_size: None,
      resizable: true,
      ..window::Settings::default()
    },
    ..Settings::default()
  };

  Calculator::run(settings)
}
