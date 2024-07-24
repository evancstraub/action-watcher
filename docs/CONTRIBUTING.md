# Contributing to Action-Watcher

Thank you for considering contributing to Action-Watcher! We appreciate your time and effort in helping improve the project. Please follow the guidelines below to ensure a smooth contribution process.

## Table of Contents
1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
    - [Reporting Bugs](#reporting-bugs)
    - [Suggesting Features](#suggesting-features)
    - [Submitting Changes](#submitting-changes)
3. [Development Setup](#development-setup)
4. [Style Guide](#style-guide)
5. [Commit Messages](#commit-messages)
6. [Pull Request Process](#pull-request-process)
7. [Additional Resources](#additional-resources)

## Code of Conduct
By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). Please read it to understand the expected behavior.

## How Can I Contribute?

### Reporting Bugs
If you find a bug, please [open an issue](https://github.com/action-watcher/issues) and include the following information:
- A clear and descriptive title.
- A detailed description of the problem.
- Steps to reproduce the issue.
- Any relevant screenshots or log files.
- Your environment details (OS, Rust version, etc.).

### Suggesting Features
We welcome new feature ideas! To suggest a feature:
1. Check if the feature is already requested by searching the [issues](https://github.com/[your-repo]/issues).
2. If not, [open a new issue](https://github.com/evancstraub/issues) and include:
    - A clear and descriptive title.
    - A detailed description of the proposed feature.
    - Any relevant examples or use cases.

### Submitting Changes
Before making changes, please open an issue to discuss your plans. This helps avoid duplicate efforts and ensures your contribution aligns with the project's direction.

## Development Setup
Follow these steps to set up the project locally:
1. Fork the repository and clone your fork:
    ```bash
    git clone https://github.com/evancstraub/action-watcher.git
    ```
2. Navigate to the project directory:
    ```bash
    cd action-watcher
    ```
3. Install Rust and Cargo if you haven't already. You can follow the instructions [here](https://www.rust-lang.org/learn/get-started).
4. Build the project:
    ```bash
    cargo build
    ```
5. Run the tests:
    ```bash
    cargo test
    ```

## Style Guide
Please follow these coding style guidelines to maintain consistency:
- **Rust**: Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/style/).
- Ensure your code is well-documented and includes relevant comments.

## Commit Messages
Write clear and descriptive commit messages. Follow these conventions:
- Use the present tense ("Add feature" not "Added feature").
- Capitalize the first letter.
- Keep messages concise but informative.

Example:

## Pull Request Process
```bash
git pull origin main
```

## Additional Resources