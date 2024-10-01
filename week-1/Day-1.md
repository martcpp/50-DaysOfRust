# Installing Rust and Setting Up Your Development Environment

Follow these steps to install Rust and set up your development environment. For more information on installing Rust and setting up your development environment see https://www.rust-lang.org/tools/install and https://www.youtube.com/watch?v=vo9sMyiqaY0&list=PLDi2liHqCnVp0oM9rNp1Hy_H5aL6QUybN for video tutorials on how to install Rust and setting up your development environment

## Step 1: Install Rust
The easiest way to install Rust is by using **rustup**, the Rust toolchain installer. It installs Rust and the required tools.

1. Open a terminal and run the following command to install Rust:

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Follow the on-screen instructions to complete the installation. Once done, the installer will set up your environment and install the `cargo` command, which is Rust's package manager.

## Step 2: Update Your Path (if necessary)
After installation, ensure that Rust's binaries are available in your shell. The installer will attempt to update your PATH automatically, but if it didn’t succeed, you may need to add it manually.

- Add this line to your shell profile (e.g., `.bashrc`, `.zshrc`, or `.bash_profile`):

   ```bash
   export PATH="$HOME/.cargo/bin:$PATH"
   ```

3. Source your shell configuration or restart the terminal for the changes to take effect.

   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

## Step 3: Verify Installation
To verify Rust is correctly installed, run the following commands:

```bash
rustc --version
cargo --version
```

These commands should print out the installed versions of Rust and Cargo.

## Step 4: Set Up a Code Editor

1. **Visual Studio Code**:
   - Install the **Rust Analyzer** extension from the VS Code marketplace for autocompletion, error checking, and more.
   - Set up the **rustfmt** extension for automatic code formatting.

2. **JetBrains CLion/IntelliJ IDEA**:
   - Install the **Rust** plugin for code suggestions and management.

## Step 5: Test Your Setup

Create and run a simple Rust project to ensure your setup is working.

1. Create a new Rust project:

   ```bash
   cargo new hello_rust
   cd hello_rust
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

This should output:

```bash
Hello, world!
```

You're now ready to start coding in Rust! 🚀
```