# PROJECT: minigrep

## Introduction

The minigrep project is a command-line application written in the Rust programming language. It provides basic arithmetic operations such as addition, subtraction, multiplication, and division.

## Features

- Supports four basic arithmetic operations: add, subtract, multiply, and divide.
- Takes command-line arguments for the operation and two operands.
- Handles errors gracefully, including invalid inputs and division by zero.

## Usage

To use the calculator, first build the program.

```
cargo build --release
```

Run the executable followed by the operation and two operands.

```
./target/release/minigrep.exe <operation> <operand1> <operand2>
```

- `<operation>`: The arithmetic operation to perform (add, subtract, multiply, divide).
- `<operand1>`: The first operand, which can be an floating point number.
- `<operand2>`: The second operand, which can be an floating point number.

## Example

```
$ ./target/release/minigrep.exe add 5 3
Result: 8
```

## Error Handling

The calculator application handles the following errors:

- Insufficient number of arguments provided.
- Invalid operation specified.
- Invalid operands (not valid numbers).
- Division by zero.

## Implementation Details

- The application is written in Rust.
- It uses the `std::env` module to parse command-line arguments.
- Arithmetic operations are performed based on the specified operation.
- Input validation is done using Rust's `parse` method to ensure operands are valid numbers.
- Error messages are displayed for various error scenarios.

## Future Enhancements

- Support for more advanced operations (e.g., exponentiation, square root).
- Improved error handling and error messages.

## Conclusion

The minigrep project provides a simple and efficient way to perform basic arithmetic operations via the command line. It is also a project for learning the Rust programming language.
