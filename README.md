# ATP CLI - Authenticated Transfer Protocol

<div align="center">

🚧 **Work in Progress** 🚧

A powerful command-line interface for interacting with the AT Protocol (Bluesky), built in Rust.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

</div>

## Installation

```bash
cargo install atp
```

## 📖 Usage

### Authentication

```bash
# Login with your credentials
cargo run -- auth login --identifier <handle> --password <application_password>

# Check current session
cargo run -- auth session
```

### Profile Operations

```bash
# View a user's profile
cargo run -- bsky actor profile --actor @username

# Get multiple profiles
cargo run -- bsky actor profiles --actors @user1,@user2

# Search for users
cargo run -- bsky actor search --query "search_term" --limit 25

# Get user suggestions
cargo run -- bsky actor suggestions --limit 50
```

## 🔧 Configuration

The CLI stores configuration and session data in your system's local config directory:
- Linux: `~/.config/atp/`
- macOS: `~/Library/Application Support/atp/`
- Windows: `%APPDATA%\atp\`

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Built on the [AT Protocol](https://atproto.com/) specification
- Currently supports the [Bluesky](https://bsky.app) lexicon (`app.bsky.*`)
- More AT Protocol lexicons planned for future releases
- Inspired by the Bluesky community and AT Protocol ecosystem

## 🔗 Related Projects

- [AT Protocol Specification](https://atproto.com)
- [Bluesky Social](https://bsky.app)
