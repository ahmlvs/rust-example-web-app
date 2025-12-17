# 🔗 rust-shortener-cqrs

A URL shortener service built with Rust, implementing CQRS (Command Query Responsibility Segregation) pattern and Clean Architecture principles.

## ✨ Features

- 🏗️ **CQRS Architecture** - Separate read and write operations
- 🧹 **Clean Architecture** - Clear separation of concerns
- ⚡ **Fast & Async** - Built on Tokio and Axum
- 🧪 **Well-tested** - Comprehensive test coverage
- 🎯 **Type-safe** - Leveraging Rust's type system
- 📦 **In-memory storage** - DashMap for concurrent access

## 🏛️ Architecture

[![CQRS Architecture](docs/images/architecture.png)](docs/images/architecture.png)

[Architecture in Excalidraw](https://excalidraw.com/#json=pMIsbO0KML-Z4vO9A3y9B,69SSznF4i1F_5ggz0OoMrg)

### Project Structure

```
src/
├── app/              # Business logic
│   ├── command/      # Write operations (CQRS Command)
│   └── query/        # Read operations (CQRS Query)
├── adapters/         # Infrastructure adapters
│   └── inmemory/     # In-memory repository implementation
├── ports/            # External interfaces
│   └── httpapi/      # HTTP API (Axum)
├── di/               # Dependency Injection container
├── error.rs          # Error types
└── id_provider.rs    # ID generation (NanoID)
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ (2021 edition)

### Run

```bash
cargo run
```

Server starts on `http://localhost:3001`

## 📚 API Documentation

### Create Short URL

**Request:**

```bash
curl -X POST http://localhost:3001/ \
  -H 'Content-Type: application/json' \
  -d '{"url": "https://google.com"}'
```

**Response:**

```json
{
  "id": "abc123"
}
```

### Get Full URL

**Request:**

```bash
curl http://localhost:3001/abc123
```

**Response:**

```json
{
  "url": "https://google.com/"
}
```

### Error Responses

**Invalid URL:**

```bash
# Request
curl -X POST http://localhost:3001/ \
  -H 'Content-Type: application/json' \
  -d '{"url": "not-a-valid-url"}'

# Response (400 Bad Request)
{
  "message": "Invalid URL"
}
```

**URL Not Found:**

```bash
# Request
curl http://localhost:3001/nonexistent

# Response (404 Not Found)
{
  "message": "Not found"
}
```

## 🧪 Testing

Run all tests:

```bash
cargo test
```

Run tests with output:

```bash
cargo test -- --nocapture
```

Run specific test:

```bash
cargo test test_invalid_url
```

## 🏗️ Development

### Build

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

### Run release build

```bash
./target/release/rust-shortener-cqrs
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with Clippy
cargo clippy --all-targets --all-features -- -D warnings
```

## ⚙️ Configuration

Currently, the server port is hardcoded to `3001` in `main.rs`.

## 🛠️ Technology Stack

- **[Axum](https://github.com/tokio-rs/axum)** - Web framework
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[DashMap](https://github.com/xacrimon/dashmap)** - Concurrent HashMap
- **[NanoID](https://docs.rs/nanoid/latest/nanoid/)** - Unique ID generation
- **[Serde](https://serde.rs/)** - Serialization/Deserialization
- **[Tower](https://github.com/tower-rs/tower)** - Middleware
- **[Tracing](https://github.com/tokio-rs/tracing)** - Logging

## 📝 TODO / Roadmap

- [ ] Add persistent storage (PostgreSQL/Redis)
- [ ] Environment-based configuration
- [ ] Rate limiting middleware
- [ ] URL scheme validation (http/https only)
- [ ] SSRF protection
- [ ] HTTP redirects instead of JSON response
- [ ] Metrics and monitoring
- [ ] Health check endpoint
- [ ] Docker support
- [ ] CI/CD pipeline

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 🔍 Learn More

- [CQRS Pattern](https://martinfowler.com/bliki/CQRS.html)
- [Clean Architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
