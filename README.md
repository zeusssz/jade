![jade](https://github.com/user-attachments/assets/5bf7e97d-fd61-4683-b460-8833b0a889f8)

Jade is a Rust code refactoring tool designed to enhance code quality and streamline development workflows. It provides functionalities for refactoring Rust code, analyzing files, and simulating code metrics to help developers maintain and improve their codebases efficiently.

## Features

- **Refactoring**: Automate code improvements including renaming identifiers, extracting functions, and removing dead code.
- **Analysis**: Examine Rust code to gather insights, detect issues, and ensure code quality.
- **Metrics**: Simulate code execution to provide runtime performance metrics and detect potential runtime errors.

## Installation

### Prerequisites

Ensure that you have Rust and Cargo installed. You can download and install them from [rust-lang.org](https://www.rust-lang.org/).

### Steps

1. **Clone the Repository**

   ```sh
   git clone https://github.com/zeusssz/jade.git
   cd jade
   ```

2. **Build the Project**

   ```sh
   cargo build --release
   ```

   The compiled binary will be available in `target/release/jade`.

3. **(Optional) Install Globally**

   To install Jade globally, you can use:

   ```sh
   cargo install --path .
   ```

   This will allow you to run `jade` from anywhere on your system.

## Usage

Jade provides three main commands to interact with your Rust code. Each command follows the syntax:

```sh
jade [command] [options] [file]
```

### Commands

#### `--refactor`

Refactor the specified Rust file. This command performs various refactoring tasks such as:

- Renaming identifiers
- Extracting functions
- Removing dead code

**Usage:**

```sh
jade --refactor path/to/file.rs
```

>[!NOTE]
>The refactored code will be saved to a new file with the suffix `_refactored.rs`. For example, `file.rs` will be refactored to `file_refactored.rs`.

#### `--analyze`

Analyze the specified Rust file. This command provides insights and checks related to the code, such as:

- Code quality metrics
- Potential issues
- Code structure analysis

**Usage:**

```sh
jade --analyze path/to/file.rs
```

>[!TIP]
>Analysis results are printed to the console. Review the output to understand the state of your code and any potential improvements.

#### `--metrics`

Compute and simulate metrics for the given Rust file. This command simulates the code's execution to provide:

- Runtime performance metrics
- Error detection
- Execution time

**Usage:**

```sh
jade --metrics path/to/file.rs
```

>[!WARNING]
>Simulating code execution may take some time depending on the complexity of the file. Ensure that your code is capable of running without infinite loops or excessive resource usage.

## Example

### Refactoring

To refactor a file named `example.rs`, run:

```sh
jade --refactor example.rs
```

This command creates a new file `example_refactored.rs` with the refactored code.

### Analyzing

To analyze `example.rs`, use:

```sh
jade --analyze example.rs
```

Review the analysis output for insights and potential improvements.

### Metrics

To compute metrics for `example.rs`, execute:

```sh
jade --metrics example.rs
```

Check the simulation results for performance metrics and runtime errors.

## Contributing

Contributions to Jade are welcome! To contribute:

1. Fork the repository on GitHub.
2. Create a new branch for your feature or bug fix.
3. Commit your changes and push them to your fork.
4. Open a pull request to merge your changes into the main repository.

>[!IMPORTANT]
>Please ensure that your contributions follow the project's coding standards and include appropriate tests.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For questions, support, or further information, please contact roboxer_ on discord.
