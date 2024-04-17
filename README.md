# Solana Wallet Generator

This Rust program generates a Solana wallet with starting characters of your choice i.e
tyler8wqkHVJWnrF9eTJ4pEY6UVLcJx8VFiWHpyNW6V6

It uses the Solana Rust SDK to create a new wallet and checks if the generated public key starts with the specified string.

## Features

- Generates Solana wallets with a starting string
- Supports both lowercase and uppercase starting strings
- Utilizes multithreading to speed up the wallet generation process
- Provides a timer to track elapsed time until a matching wallet is found

## Getting Started

1. Ensure you have Rust and Cargo installed. You can install them from [https://www.rust-lang.org/learn/get-started](https://www.rust-lang.org/learn/get-started).
2. Clone this repository to your local machine.
3. Navigate to the project directory.
4. Run `cargo run` to build and execute the program.

## Usage

Upon running the program, it will start generating Solana wallets with random keys. It will print out the public key of each generated wallet and check if it starts with the specified starting string. Once a matching wallet is found, the program will print the public key and exit.

You can customize the starting string by modifying the `name` variable in the `main.rs` file.

## Example

```bash
$ cargo run
```

## Output
```
Generating Solana wallets...
Elapsed Time: 1 seconds
Elapsed Time: 2 seconds
Found matching wallet!
Wallet Public Key: 3tylJ8wqkHVJWnrF9eTJ4pEY6UVLcJx8VFiWHpyNW6V6
Total Elapsed Time: 2 seconds
```

## Contributing
Contributions are welcome! If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request.

## License
This project is licensed under the MIT License - see the LICENSE file for details.