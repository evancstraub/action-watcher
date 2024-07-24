
# Action Watcher
## *Under Development*

Action Watcher is a universal file watcher tool designed to monitor predetermined filesets of a project. When changes are detected, it runs a series of specified actions and displays the results in a dynamic HTML interface with tabbed navigation.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
    - [Configuration](#configuration)
    - [CLI Commands](#cli-commands)
- [Development](#development)
- [Contributing](#contributing)
- [License](#license)

## Features
- **Universal File Watching**: Monitor files in any project and trigger actions on changes.
- **Action Execution**: Run a series of commands (e.g., tests, linters) when changes are detected.
- **HTML Reports**: Display output reports in a single HTML page with tabbed navigation.
- **CLI and Daemon Mode**: Run in interactive mode or as a background daemon.
- **Customizable**: Define commands and settings via a configuration file.

## Installation

To install Action-Watcher, you need to have Rust and Cargo installed. Follow the instructions [here](https://www.rust-lang.org/learn/get-started) to install Rust.

Clone the repository and build the project:

```bash
git clone https://github.com/evancstraub/action-watcher.git
cd action-watcher
cargo build --release
```

## Usage

### Configuration

Create a configuration file (`config.yaml` or `config.ini`) in the project directory. Here is an example configuration file in YAML format:

```yaml
watch:
  - path: ./src
    patterns:
      - "*.py"
      - "*.js"
  - path: ./tests
    patterns:
      - "*.py"

commands:
  - name: "Run Python tests"
    cmd: "pytest"
    args: ["--html=reports/pytest/report.html"]
  - name: "Run coverage"
    cmd: "coverage"
    args: ["run", "-m", "pytest"]
  - name: "Generate coverage report"
    cmd: "coverage"
    args: ["html", "-d", "reports/coverage"]
  - name: "Run mypy"
    cmd: "mypy"
    args: [".", "--html-report", "reports/mypy"]

report_dir: "./reports"
```

### CLI Commands

Run Action-Watcher with the following command:

```bash
./target/release/action-watcher --config config.yaml
```

**Proposed CLI options:**

- `--config <FILE>`: Specify the configuration file.
- `--daemon`: Run Action-Watcher in daemon mode.
- `--debug`: Enable debug mode.
- `--verbosity <LEVEL>`: Set verbosity level (0-3).
- `--help`: Show help message.

### Example Usage

```bash
./target/release/action-watcher --config config.yaml --daemon --verbosity 2
```

This command starts Action Watcher in daemon mode with the specified configuration and verbosity level.

## Development

To set up the development environment, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/evancstraub/action-watcher.git
    cd action-watcher
    ```
2. Build the project:
    ```bash
    cargo build
    ```
3. Run the tests:
    ```bash
    cargo test
    ```

## Contributing

We welcome contributions! Please read our [CONTRIBUTING](docs/CONTRIBUTING.md) guide for more information on how to contribute.

## License

This project is licensed under the Mozilla Public License 2.0. See the [LICENSE](docs/LICENSE) file for details.
