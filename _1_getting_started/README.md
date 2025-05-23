# Getting Started

## Rust Installation

### Windows

1. Download the [rustup-init.exe](https://win.rustup.rs/) file.

2. Run the file and follow the onscreen instructions.

3. Open a new command prompt and run `rustup default stable-x86_64-pc-windows-msvc`.

4. Run `rustup update`.

### Linux

1. Open a terminal.

2. Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

3. Run `source $HOME/.cargo/env`.

4. Run `rustup default stable`.

5. Run `rustup update`.

### MacOS

1. Open a terminal.

2. Run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.

3. Run `source $HOME/.cargo/env`.

4. Run `rustup default stable`.

5. Run `rustup update`.

## Rust Editor

### Visual Studio Code

1. Install [Visual Studio Code](https://code.visualstudio.com/).

2. Install the [Rust](https://marketplace.visualstudio.com/items?itemName=1YiB.rust-bundle) extension.

3. Open a new terminal and run `code .` This will open the current directory in Visual Studio Code.

4. Open the command palette and run `Rust: Select Workspace Configuration File`.

5. Select `Cargo.toml` as the configuration file.

6. Open the command palette and run `Rust: Analyzer Status`.

7. If the status is `Analysis Required`, run `Rust: Run Analysis`.

8. If the status is `Analysis Not Required`, run `Rust: Run`.