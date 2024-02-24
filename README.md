# README for Cosmos CLI

_README generated using Cursor because I am lazy_

## Overview

Cosmos CLI is a command-line interface application designed for searching and retrieving information about satellites. It leverages various Rust crates to provide a user-friendly interface for satellite data exploration.

## Features

- **Satellite Search**: Users can search for satellites by name.
- **Pagination**: Supports pagination for search results.
- **Satellite Information Display**: Displays detailed information about satellites, including distance from Earth.

## Installation

### Prerequisites

- Rust (edition 2021 or later)
- Cargo

### Steps

1. Clone the repository:

```bash
git clone https://github.com/your-repository/cosmos_cli.git
```

2. Setup environment variables (from [N2YO API](https://www.n2yo.com/api/))

```bash
echo "N2YO_API_KEY=<YOUR KEY>" > .env
```

3. Navigate to the project directory:

```bash
cd cosmos_cli
```

4. Build the project using Cargo:

```bash
cargo build --release
```

5. Run the application:

```bash
cargo run
```

## Usage

After running the application, follow the on-screen prompts to search for satellites and view detailed information about them.

## Dependencies

Cosmos CLI uses several crates to function properly:

- `reqwest` for HTTP requests.
- `tokio` for asynchronous runtime.
- `serde` and `serde_json` for serialization and deserialization.
- `chrono` for date and time functionality.
- `crossterm`, `dialoguer`, and `indicatif` for CLI interface and progress indication.
- `clearscreen` for clearing the terminal screen.
- `image` for image processing.
- `sgp4` for satellite tracking.
- `dotenvy` for loading environment variables.

See `Cargo.toml` for specific versions and additional details.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to suggest improvements or add new features.

## License

Specify your license here. If not specified, it's recommended to choose an open-source license suitable for your project.

## Acknowledgments

Thanks to all the contributors and the Rust community for the support and libraries that made this project possible.
