# Minigrep

Minigrep is a very basic command-line utility that performs text searches in files, similar to the traditional grep tool. It's designed to be straightforward and easy to use for basic text searching needs.

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

## Usage

1. Search for "rust" in a file:

   ```bash
   minigrep rust file.txt
   ```
   ![image](./assets/1.png)
2. Case-insensitive search for "RUST" in a file:

   ```bash
   minigrep -i RUST file.txt
   ```
   ![image](./assets/2.png)
3. Show line numbers:

   ```bash
   minigrep -l memory file.txt
   ```
   ![image](./assets/3.png)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
