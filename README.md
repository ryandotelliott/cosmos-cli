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

## Acknowledgments

Thanks to all the contributors and the Rust community for the support and libraries that made this project possible.
