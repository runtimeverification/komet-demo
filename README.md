# Komet Demo

This repository contains an example project for testing a simple `adder` contract using the Komet tool. Komet is a property testing and verification tool for Soroban smart contracts, allowing developers to write property-based tests and run them via fuzzing or symbolic execution.

## Project Structure

The project consists of two contracts:
- `adder`: A simple contract that takes two numbers and returns their sum.
- `test_adder`: A test contract for the `adder` contract, using Komet for testing and verification.

```text
.
├── contracts
│   ├── adder
│   │   ├── src
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   └── test_adder
│       ├── src
│       │   ├── lib.rs
│       │   └── komet.rs
│       ├── Cargo.toml
│       └── kasmer.json
├── Cargo.toml
└── README.md
```

## Installation

### Step 1: Install `kup`

Komet can be installed using the `kup` package manager. To install `kup`, run the following command:

```bash
bash <(curl https://kframework.org/install)
```

### Step 2: Install Komet

Once `kup` is installed, you can install Komet by running:

```bash
kup install komet
```

### Step 3: Verify Installation

After installation, verify it by checking the help menu:

```bash
komet --help
```

## Running Tests

### Step 1: Build the Contracts

Navigate to the root directory of the project and build the contracts using the following command:

```bash
soroban contract build
```

### Step 2: Navigate to the Test Contract Directory

Change directories into the `test_adder` contract folder:

```bash
cd contracts/test_adder/
```

### Step 3: Running Tests

#### Fuzzing

To run the tests using fuzzing, execute the following command:

```bash
komet test
```

This will run the test with randomized inputs.

#### Proving (Symbolic Execution)

To run the tests using symbolic execution, use this command:

```bash
komet prove run
```

To see more options for proving (such as saving proof data or viewing the proof tree), use:

```bash
komet prove --help
```
