# Secret Sharing Implementation in Rust

A Rust implementation of secret sharing using Lagrange interpolation. This project provides functionality to split a secret into multiple shares and recover it using a minimum number of those shares.

## 📁 Project Structure

```
project_root/
│
├── src/
│   ├── main.rs             # Entry point of the application
│   ├── lib.rs              # Library root to expose the modules
│   ├── fraction.rs         # Handles Fraction operations and arithmetic
│   └── secret.rs           # Manages secret recovery logic using interpolation
└── Cargo.toml              # Cargo configuration
```

## 🚀 Features

- Secret recovery using Lagrange interpolation
- Fraction arithmetic implementation
- GCD calculation for fraction reduction
- Modular design with separate concerns
- Comprehensive test coverage

## 📋 Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## 🛠️ Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd secret-sharing
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## 💻 Usage

Run the project with the following command:
```bash
cargo run
```

### Example Code

```rust
let shares: Vec<(i128, i128)> = vec![
    (1, 28735619723837),
    (2, 733237416150),
    (3, 1847513024929),
    (4, 1555012395734),
    (5, 1145883719345),
    (6, 28735619654702),
    (7, 1348206808900350),
    (8, 10998949706788),
    (9, 1020905571696340),
];

let k = 6; // Minimum number of shares needed
recover_secret(&shares, k);
```

## 🧪 Testing

Run the test suite using:
```bash
cargo test
```

### Available Tests

- Fraction arithmetic operations
  - Addition
  - Multiplication
  - GCD calculation
- Secret recovery functionality
  - Basic secret generation
  - Edge cases

## 📚 Module Overview

### main.rs
Entry point of the application where shares are defined and the recovery process is initiated.

### lib.rs
Exposes the project's modules to make them accessible throughout the codebase.

### fraction.rs
Implements the `Fraction` struct with:
- Basic arithmetic operations (Add, Multiply)
- Fraction reduction using GCD
- Utility functions for fraction manipulation

### secret.rs
Contains the core secret recovery logic:
- Lagrange interpolation implementation
- Share validation
- Secret generation and recovery functions

## 🔍 Implementation Details

The project uses Lagrange interpolation to recover secrets from shares. The process involves:

1. Validating the number of available shares
2. Converting shares into x and y coordinates
3. Applying Lagrange interpolation formula
4. Reducing fractions for accurate calculations
5. Generating the final secret value

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request