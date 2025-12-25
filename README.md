# Open Draw Steel Rust (ODSR)

An open-source engine for the game Draw Steel by MCDM.

## Projects
- engine: the core code engine
- tui: a Terminal User Interface used as to test the engine by simulating the game.

## Installation

### Prerequisites

Install Rust and Cargo using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Or on Windows, download and run `rustup-init.exe` from the [rustup website](https://rustup.rs/).

After installation, verify Cargo is installed:

```bash
cargo --version
```

## Tests
```bash
cargo test
```