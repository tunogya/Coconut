# Coconut 🥥 - Solana Pump.fun Bot

Coconut is a command-line bot built for the **Solana** blockchain, specifically designed to interact with the **Pump.fun** platform. It automates the process of clipping, managing wallet interactions, and provides status updates and logging functionalities.

## Installation

### Prerequisites

- Rust (latest stable version) – [Install Rust](https://www.rust-lang.org/tools/install)
- Solana CLI – [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- SQLite – [Install SQLite](https://www.sqlite.org/download.html)

### Clone the repository

```bash
git clone https://github.com/tunogya/coconut.git
cd coconut
```

### Build the project

Ensure you have Rust installed, then build the project using Cargo:

```bash
cargo build --release
```

### Configuration

Before running the bot, make sure to configure the following environment variables in the .env file:

```
SOLANA_PRIVATE_KEY=your_private_key_here
```

### Running the Bot

After building the project, you can run the bot using the following commands:

Start the bot:

```bash
coconut start
```

Stop the bot:

```bash
coconut stop
```

## Development

### Running Tests

To run the tests for the Coconut project, use:

```bash
cargo test
```

### Contributing

We welcome contributions to Coconut! Here are some ways you can help:
1.	Report Bugs: If you encounter any issues, feel free to open an issue in the GitHub repository.
2.	Submit Pull Requests: If you have an improvement or a bug fix, create a pull request with a clear description of your changes.

### Code Style

We follow the Rust code style guidelines. Make sure your code passes cargo fmt and cargo clippy before submitting a pull request:

```bash
cargo fmt
cargo clippy
```

### License

Coconut is licensed under the MIT License. See the LICENSE file for more details.

### Acknowledgements
•	Solana for the amazing blockchain platform.
•	Pump.fun for providing the clipping platform.
•	Rust for being an awesome systems programming language.
•	crossterm for terminal handling in Rust.

### Contact

For any inquiries or support, please contact tom@abandon.ai.