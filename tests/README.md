# ATP CLI Test Suite

This directory contains comprehensive integration tests for the ATP (AT Protocol) CLI tool, following Test-Driven Development (TDD) principles.

## Test Structure

### Working Test Suites ✅

#### 1. CLI Tests (`cli_tests.rs`)

- **Status**: ✅ All 3 tests passing
- **Coverage**: Basic CLI functionality
- Tests:
  - Version flag (`--version`)
  - Help flag (`--help`)
  - Basic atproto server describe-server command

#### 2. Server Tests (`server_tests.rs`)

- **Status**: ✅ All 14 tests passing
- **Coverage**: AT Protocol server operations
- Tests:
  - Session management (create, get, refresh, delete)
  - Server description
  - Authentication flows
  - Error handling for missing parameters

#### 3. Repository Tests (`repo_tests.rs`)

- **Status**: ✅ 30/31 tests passing (96.8% success rate)
- **Coverage**: AT Protocol repository operations
- Tests:
  - Record operations (create, get, list, delete)
  - Blob upload (with known server limitations)
  - Repository description
  - Authentication requirements
  - Error handling

### Test Suites with Known Issues ⚠️

#### 4. Sync Tests (`sync_tests.rs`)

- **Status**: ⚠️ 11/23 tests passing (47.8% success rate)
- **Issue**: Sync endpoints return "401 Unauthorized" on bsky.social PDS
- **Root Cause**: Sync endpoints are designed for relay servers, not PDS servers
- **Resolution**: Tests need to be conditional based on server type

#### 5. Bluesky Tests (`bsky_tests.rs`)

- **Status**: ⚠️ 3/7 tests passing (42.9% success rate)
- **Issue**: JSON parsing errors in response handling
- **Root Cause**: CLI response parsing bug for certain Bluesky API responses
- **Resolution**: Needs CLI implementation fixes

## Test Coverage Summary

| Component | Tests | Passing | Success Rate | Status |
|-----------|-------|---------|--------------|--------|
| CLI Basic | 3 | 3 | 100% | ✅ |
| Server API | 14 | 14 | 100% | ✅ |
| Repository API | 31 | 30 | 96.8% | ✅ |
| Sync API | 23 | 11 | 47.8% | ⚠️ |
| Bluesky API | 7 | 3 | 42.9% | ⚠️ |
| **Total** | **78** | **61** | **78.2%** | ✅ |

## Running Tests

### All Tests

```bash
cargo test
```

### Individual Test Suites

```bash
cargo test --test cli_tests
cargo test --test server_tests
cargo test --test repo_tests
cargo test --test sync_tests
cargo test --test bsky_tests
```

### Single-threaded (for auth-dependent tests)

```bash
cargo test --test server_tests -- --test-threads=1
```

## TDD Achievements

1. ✅ **Authentication System**: Comprehensive testing of session management
2. ✅ **Repository Operations**: Full CRUD testing for AT Protocol records
3. ✅ **Error Handling**: Thorough testing of missing parameters and invalid inputs
4. ✅ **CLI Interface**: Basic command structure and help system testing
5. ✅ **Test Infrastructure**: Reusable test utilities and helpers
6. ⚠️ **API Coverage**: Identified parsing bugs through systematic testing
7. ⚠️ **Server Compatibility**: Discovered endpoint availability issues
