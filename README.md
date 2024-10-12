# Minigrep

Minigrep is a simple command-line utility that performs text searches in files or standard input, similar to the traditional grep tool. It's designed to be straightforward and easy to use for basic text searching needs.

## Features

- Search for standard input
- Case-sensitive and case-insensitive search options

## Installation

To install Minigrep, you can download binaries or follow these steps:

1. Clone the repository:
   ```bash
   git clone https://github.com/akoken/minigrep.git
   cd minigrep
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available in `target/release/minigrep`

## Examples

1. Search for "error" in a log file:
   ```bash
   minigrep error app.log
   ```

2. Case-insensitive search for "warning" in a log file:
   ```bash
   export IGNORE_CASE=1
   minigrep warning app.log
   ```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
