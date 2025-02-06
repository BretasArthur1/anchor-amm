# AMN Anchor

AMN Anchor is a decentralized finance (DeFi) application built using the Anchor framework on the Solana blockchain. It provides functionalities for initializing, depositing, withdrawing, and swapping tokens in a liquidity pool.

## Features

- **Initialization**: Set up the liquidity pool with specified parameters.
- **Deposit**: Add liquidity to the pool and receive LP tokens in return.
- **Withdraw**: Remove liquidity from the pool by burning LP tokens.
- **Swap**: Exchange tokens within the pool using a constant product curve.

## Project Structure

- **Rust Programs**: The core logic of the application is implemented in Rust using the Anchor framework.
  - `src/lib.rs`: Main entry point for the program.
  - `src/instructions/`: Contains modules for each instruction (init, deposit, withdraw, swap).
  - `src/state/`: Defines the state structures used in the program.
  - `src/errors.rs`: Custom error definitions for the program.

- **TypeScript Tests**: Tests for the program are written in TypeScript using Mocha and Chai.
  - `tests/amn-anchor.ts`: Contains test cases for the AMN Anchor program.

- **Configuration Files**:
  - `Cargo.toml`: Rust package configuration.
  - `Anchor.toml`: Anchor-specific configuration.
  - `package.json`: Node.js package configuration for testing and development.

## Getting Started

### Prerequisites

- Rust and Cargo
- Node.js and Yarn
- Solana CLI

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/BretasArthur1/anchor-amm
   cd amn-anchor
   ```

2. Install Node.js dependencies:
   ```bash
   yarn install
   ```

3. Build the Rust program:
   ```bash
   anchor build
   ```

### Running Tests

To run the tests, use the following command:

```bash
anchor test
```

This will execute the TypeScript tests located in the `tests/` directory.

## License

This project is licensed under the ISC License.

## Acknowledgments

- Built with the [Anchor framework](https://github.com/project-serum/anchor) on Solana.
- Utilizes the [constant-product-curve](https://github.com/deanmlittle/constant-product-curve) library for swap calculations.
