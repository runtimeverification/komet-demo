# Komet Demo

This repository contains an example project for testing a simple `adder` contract using the Komet tool. Komet is a property testing and verification tool for Soroban smart contracts, allowing developers to write property-based tests and run them via fuzzing or symbolic execution.

## Project Structure

The project consists of 5 contracts:
- `fungible`: A simple fungible token contract.
- `test_fungible`: A test contract for the `fungible` contract.
- `test_fungible_invariants`: Another test contract for the `fungible` contract.
- `test_adder`: A test contract for demonstrating the prover.



```text
.
├── contracts
│   ├── fungible
│   ├── test_fungible
│   ├── test_fungible_invariants
│   └── test_adder
├── Cargo.toml
└── README.md
```

## Installation

### Step 1: Prerequisites

#### Install `kup`

Komet can be installed using the `kup` package manager. To install `kup`, run the following command:

```bash
bash <(curl https://kframework.org/install)
```

#### Install `stellar-cli`

Install `stellar-cli` version 26.0.0

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
q
```bash
soroban contract build
```

### Step 2: Navigate to the Test Contract Directory

Change directories into the `test_fungible` contract folder:

```bash
cd contracts/test_fungible/
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
