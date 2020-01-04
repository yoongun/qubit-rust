![](https://github.com/cfr2ak/qubit-rust/workflows/Rust/badge.svg)

# :rabbit: qubit-rust

A single qubit simulation in Rust

## How to use

### Build it

First, build the project
```bash
cargo build
```
then run the program with the command below.
```bash
./target/debug/qubit-rust
```

### Or more simply, run it directly
```bash
cargo run
```

The simulation of a qubit which is initialized with the Hadamarad gate will be measured
1000 times and show you a result on the screen.

## Run the test

There are tests for

- Initialize a qubit with |0> state
- Measuring the qubit and get a proper result
- Check whether a measurement induces collapse of state to the qubit

You can test these with the command below.

```bash
cargo test
```
