# ATP CLI - AT Protocol Command Line Interface

<div align="center">

🌐 **AT Protocol Core Implementation** 🌐

A command-line interface for the AT Protocol, focusing on the core `com.atproto.*` namespace with additional Bluesky social features.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.87+-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/tests-93%20passing-brightgreen.svg)](#testing)

</div>

## 🚀 Features

ATP CLI provides comprehensive access to the AT Protocol with primary focus on core protocol operations:

### Core AT Protocol (`com.atproto.*`)

- **🔐 Authentication & Session Management** - Full session lifecycle support
- **👤 Identity Resolution** - Handle and DID resolution and management  
- **📦 Repository Operations** - Complete CRUD operations for AT Protocol records
- **🔄 Synchronization** - Repository sync and blob management
- **🖥️ Server Management** - Server capabilities and session management

### Additional Features

- **🎯 Bluesky Integration** - Native support for Bluesky social features (`app.bsky.*`)

## 📦 Installation

### From Source

```bash
git clone https://github.com/yourusername/atp
cd atp
cargo build --release
```

### Using Cargo

```bash
cargo install atp
```

## 🛠️ Usage

### Authentication

```bash
# Login with your credentials
atp auth login --identifier your.handle --password your-app-password

# Check current session
atp auth session
```

### Core AT Protocol Operations

#### Identity Management

```bash
# Resolve handle to DID
atp atproto identity resolve-handle --handle alice.bsky.social

# Resolve DID to DID document
atp atproto identity resolve-did --did did:plc:example123

# Update your handle
atp atproto identity update-handle --handle new.handle.com
```

#### Repository Operations

```bash
# Create a new record
atp atproto repo create-record \
  --repo did:plc:example \
  --collection app.bsky.feed.post \
  --record '{"text": "Hello AT Protocol!", "createdAt": "2024-01-01T00:00:00Z"}'

# Get a specific record
atp atproto repo get-record \
  --repo did:plc:example \
  --collection app.bsky.feed.post \
  --rkey 3k2a4b5c6d7e8f9g

# List records in a collection
atp atproto repo list-records \
  --repo did:plc:example \
  --collection app.bsky.feed.post \
  --limit 50

# Delete a record
atp atproto repo delete-record \
  --repo did:plc:example \
  --collection app.bsky.feed.post \
  --rkey 3k2a4b5c6d7e8f9g

# Upload a blob
atp atproto repo upload-blob --file image.jpg

# Describe a repository
atp atproto repo describe-repo --repo did:plc:example
```

#### Server Operations

```bash
# Create a session (alternative to auth login)
atp atproto server create-session \
  --identifier your.handle \
  --password your-password

# Get current session info
atp atproto server get-session

# Refresh session tokens
atp atproto server refresh-session

# Delete session (logout)
atp atproto server delete-session

# Get server capabilities
atp atproto server describe-server
```

#### Synchronization Operations

```bash
# Get a blob
atp atproto sync get-blob --did did:plc:example --cid bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi

# Get repository head
atp atproto sync get-head --did did:plc:example

# Get latest commit
atp atproto sync get-latest-commit --did did:plc:example

# Get repository status
atp atproto sync get-repo-status --did did:plc:example

# List repositories
atp atproto sync list-repos --limit 100
```

### Bluesky Social Features

```bash
# View a user's profile
atp bsky actor profile --actor alice.bsky.social

# Get multiple profiles
atp bsky actor profiles --actors alice.bsky.social,bob.bsky.social

# Search for users
atp bsky actor search --query "rust developer" --limit 25

# Get user suggestions
atp bsky actor suggestions --limit 50

# Get user preferences
atp bsky actor preferences
```

## 📊 AT Protocol Core Support (`com.atproto.*`)

### Current Implementation Status

| Namespace | Commands | Coverage | Status |
|-----------|----------|----------|--------|
| **`com.atproto.identity`** | 3/9 | 🟡 **33%** | Core identity operations |
| **`com.atproto.repo`** | 6/12 | 🟡 **50%** | Repository management |
| **`com.atproto.server`** | 5/25 | 🔴 **20%** | Server operations |
| **`com.atproto.sync`** | 5/17 | 🔴 **29%** | Synchronization |
| **`com.atproto.admin`** | 0/15 | 🔴 **0%** | Administrative functions |
| **`com.atproto.label`** | 0/3 | 🔴 **0%** | Content labeling |
| **`com.atproto.moderation`** | 0/3 | 🔴 **0%** | Moderation tools |
| **`com.atproto.temp`** | 0/4 | 🔴 **0%** | Temporary operations |

### Detailed Feature Matrix

<details>
<summary><strong>✅ Implemented Features (Click to expand)</strong></summary>

#### Identity Management (`com.atproto.identity`)

- ✅ `resolveHandle` - Resolve handle to DID
- ✅ `resolveDid` - Resolve DID to DID document
- ✅ `updateHandle` - Update account handle

#### Repository Operations (`com.atproto.repo`)

- ✅ `createRecord` - Create new record
- ✅ `getRecord` - Get specific record
- ✅ `listRecords` - List records in collection
- ✅ `deleteRecord` - Delete record
- ✅ `uploadBlob` - Upload blob
- ✅ `describeRepo` - Get repository metadata

#### Server Management (`com.atproto.server`)

- ✅ `createSession` - Login/create session
- ✅ `getSession` - Get current session info
- ✅ `refreshSession` - Refresh tokens
- ✅ `deleteSession` - Logout
- ✅ `describeServer` - Get server capabilities

#### Synchronization (`com.atproto.sync`)

- ✅ `getBlob` - Get blob data
- ✅ `getHead` - Get repository head
- ✅ `getLatestCommit` - Get latest commit
- ✅ `getRepoStatus` - Get repository status
- ✅ `listRepos` - List repositories

</details>

<details>
<summary><strong>🚧 Planned Core Features (Click to expand)</strong></summary>

#### High Priority

- ❌ `com.atproto.repo.applyWrites` - Batch repository operations
- ❌ `com.atproto.repo.putRecord` - Update existing records
- ❌ `com.atproto.server.createAccount` - Account creation
- ❌ `com.atproto.identity.getRecommendedDidCredentials` - DID credential management
- ❌ `com.atproto.moderation.createReport` - Content reporting

#### Medium Priority

- ❌ `com.atproto.label.queryLabels` - Query content labels
- ❌ `com.atproto.sync.subscribeRepos` - Subscribe to repository events
- ❌ `com.atproto.server.createAppPassword` - App password management
- ❌ `com.atproto.identity.signPlcOperation` - PLC operations
- ❌ `com.atproto.repo.importRepo` - Repository import/export

#### Low Priority

- ❌ `com.atproto.admin.*` - Administrative operations (15 commands)
- ❌ `com.atproto.temp.*` - Temporary/experimental features (4 commands)

</details>

### Overall Progress

| Category | Implemented | Total | Coverage |
|----------|-------------|-------|----------|
| **Core AT Protocol** | 19 | 66 | 🟡 **29%** |
| **Bluesky Features** | 5 | 95+ | 🔴 **5%** |
| **Total** | 24 | 161+ | 🔴 **15%** |

## 🧪 Testing

ATP CLI has comprehensive test coverage with **93 passing integration tests** covering:

- ✅ All command-line argument validation
- ✅ Authentication and authorization flows  
- ✅ Error handling and edge cases
- ✅ Parameter validation and boundary testing
- ✅ JSON validation and parsing
- ✅ Help system consistency
- ✅ Unicode and special character handling

```bash
# Run all tests
cargo test

# Run only integration tests
cargo test --test integration_tests

# Run with verbose output
cargo test -- --nocapture
```

## 🔧 Configuration

ATP CLI stores configuration and session data in your system's local config directory:

- **Linux**: `~/.config/atp/config.toml`
- **macOS**: `~/Library/Application Support/atp/config.toml`  
- **Windows**: `%APPDATA%\atp\config.toml`

### Configuration Format

```toml
[session]
did = "did:plc:example123"
handle = "alice.bsky.social"
email = "alice@example.com"
accessJwt = "..."
refreshJwt = "..."
```

## 🏗️ Architecture

ATP CLI is built with:

- **🦀 Rust 2024 Edition** - Modern, safe systems programming
- **⚡ Tokio** - Async runtime for high-performance networking
- **🔧 Clap** - Powerful command-line argument parsing
- **🌐 Reqwest** - HTTP client with JSON support
- **📝 Serde** - Serialization/deserialization
- **✅ TDD Approach** - Test-driven development with comprehensive coverage

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Priority Areas for Contribution

1. **Core AT Protocol Features** - Complete the `com.atproto.*` namespace implementation
2. **Batch Operations** - Implement `applyWrites` and bulk operations
3. **Identity Management** - Complete DID and handle operations
4. **Event Streaming** - Add support for real-time subscriptions
5. **Documentation** - Improve examples and API documentation

### Development Setup

```bash
git clone https://github.com/yourusername/atp
cd atp
cargo build
cargo test
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on the [AT Protocol](https://atproto.com/) specification
- Compatible with [Bluesky](https://bsky.app) and other AT Protocol servers
- Inspired by the vision of a decentralized social web
- Thanks to all contributors and testers

## 🔗 Resources

### AT Protocol Documentation

- [AT Protocol Specification](https://atproto.com/specs/at-protocol)
- [Lexicon Schema Language](https://atproto.com/specs/lexicon)
- [Identity & DIDs](https://atproto.com/specs/did)
- [Repository Structure](https://atproto.com/specs/repository)

### API References

- [Core Protocol API](https://docs.bsky.app/docs/api#at-protocol-xrpc-api)
- [Authentication Guide](https://atproto.com/specs/xrpc)
- [Data Model](https://atproto.com/specs/data-model)

---

<div align="center">

**[AT Protocol](https://atproto.com) • [Issues](https://github.com/yourusername/atp/issues) • [Discussions](https://github.com/yourusername/atp/discussions)**

Made with ❤️ for the decentralized social web

</div>
