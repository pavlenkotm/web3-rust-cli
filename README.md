# 🦀 Web3 Rust CLI

A robust Rust CLI tool that fetches the current **Solana (SOL)** price from multiple cryptocurrency APIs with automatic fallback support.

**Built by PavlenkoTM**

## ✨ Features

- **Multiple API Support**: Automatically tries CoinGecko, CoinCap, and Binance APIs
- **Automatic Fallback**: If one API fails, automatically tries the next one
- **Error Handling**: Comprehensive error handling with helpful messages
- **API Key Support**: Optional CoinGecko API key support via environment variables
- **Tests Included**: Unit tests for core functionality
- **Beautiful Output**: Clean, formatted console output

## 🚀 Quick Start

### Run the application
```bash
cargo run
```

### Run optimized release version
```bash
cargo run --release
```

### Run tests
```bash
cargo test
```

### Build release binary
```bash
cargo build --release
./target/release/web3-rust-cli
```

## 🔑 Using CoinGecko API Key (Optional)

If you have a CoinGecko API key, you can set it as an environment variable:

```bash
export COINGECKO_API_KEY="your-api-key-here"
cargo run
```

Or create a `.env` file:
```bash
COINGECKO_API_KEY=your-api-key-here
```

## 📦 Dependencies

- `reqwest` - HTTP client for API requests
- `serde` - Serialization/deserialization framework
- `serde_json` - JSON support for Serde

## 🧪 Testing

The project includes unit tests. Run them with:
```bash
cargo test
```

## 🛠️ How It Works

1. The application tries to fetch the Solana price from CoinGecko API first
2. If CoinGecko fails (network issues, rate limits, etc.), it tries CoinCap API
3. If CoinCap also fails, it tries Binance API as a final fallback
4. If all APIs fail, it displays an error message with troubleshooting tips

## 📝 Example Output

```
🦀 Web3 Rust CLI - Solana Price Checker
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔗 Trying CoinGecko API...
✅ CoinGecko: Success
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
💰 SOL price: $142.35
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## 🔧 Development

### Project Structure
```
web3-rust-cli/
├── Cargo.toml          # Project dependencies and metadata
├── README.md           # This file
└── src/
    └── main.rs         # Main application code
```

### Clean build artifacts
```bash
cargo clean
```

## 📄 License

This project is open source and available for educational purposes.

## 👤 Author

**PavlenkoTM**
- Email: pavlenko@example.com

## 🤝 Contributing

Contributions, issues, and feature requests are welcome!

## ⭐ Show your support

Give a ⭐️ if this project helped you!
