# Rust CLI: Grep and Calculator

Welcome to the Rust CLI project! This simple command-line interface supports two primary functions:

- A grep-like command for searching text
- A basic calculator for arithmetic operations

## Features

1. **Grep Command**:
   - Search for specific text within files
   - Supports case-sensitive and case-insensitive searches

2. **Calculator**:
   - Perform basic arithmetic operations: addition, subtraction, multiplication, and division

## Installation

To install this CLI, ensure you have Rust installed on your system. Then, clone the repository and build the project:

```sh
git clone https://github.com/skushagra9/rust-cli.git
cd rust-cli
cargo build --release

```

## Usage

### Grep Command

To search for text within a file:

```sh
cargo run grep <search_term> <file_path>
cargo run  grep "hello" example.txt
```

### Calculator

```sh
cargo run calc <operation> <operand1> <operand2>
cargo run calc add 4 5
```

## Tests

This project includes tests to ensure the correctness of the grep command and calculator. To run the tests, use:

```sh
cargo test
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a new branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin feature-branch`)
5. Create a new Pull Request

Please ensure your code passes the existing tests and write new tests for your changes.

## License

This project is licensed under the MIT License. See the LICENSE file for details.
