# 🎯 Polymarket Sniper

A high-performance Rust-based tool for monitoring and "sniping" binary options markets on Polymarket. Specifically optimized for the **5-minute Bitcoin (BTC) Up/Down prediction markets**.

## 🚀 Overview

`polymarket-sniper` automates the process of identifying, tracking, and signaling high-probability trades in the final seconds of Polymarket's short-term BTC price markets. It utilizes the Polymarket Gamma API for market discovery and the CLOB WebSocket for ultra-low latency price feeds.

## ✨ Key Features

- **Automated Market Discovery**: Dynamically calculates and targets `btc-updown-5m` slugs based on real-time Unix timestamps.
- **Real-time CLOB Integration**: Connects directly to Polymarket's Central Limit Order Book via WebSockets for live price updates.
- **Momentum Tracking**: Implements a rolling window analysis (10-tick depth) to calculate price momentum and trend strength.
- **Sniper Signal Engine**: Triggers logic-based signals when specific conditions (time-to-expiry, price threshold, and momentum) align.
- **Persistent Logging**: Automatically rotates log files (`market_logN.txt`) with Unix timestamps for backtesting and performance review.
- **Async Runtime**: Built on `Tokio` for efficient, non-blocking I/O.

## 🛠 Tech Stack

- **Language**: Rust (Edition 2024)
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **API Client**: [Reqwest](https://docs.rs/reqwest/)
- **WebSocket**: [Tokio Tungstenite](https://docs.rs/tokio-tungstenite/)
- **Serialization**: [Serde JSON](https://serde.rs/)

## ⚙️ How It Works

### 1. Market Identification
The sniper continuously calculates the current 5-minute interval slug (e.g., `btc-updown-5m-1716819300`). It polls the Gamma API until the market becomes active.

### 2. Live Monitoring
Once an active market is found (with a bid between 35¢ and 65¢), it subscribes to the specific asset ID's price feed via WebSocket.

### 3. Analytics
- **Momentum Calculation**: `((Newest Price - Oldest Price) / Oldest Price) * 100`
- **Tick History**: Maintains the last 10 price points to smooth out noise.

### 4. Sniper Logic
Signals are triggered in the **last 30 seconds** of a market if:
- **UP Signal**: Bid > 70¢ AND Momentum > +5%
- **DOWN Signal**: Bid < 30¢ AND Momentum < -5%

## 📥 Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-username/polymarket-sniper.git
   cd polymarket-sniper
   ```

2. **Install Rust**:
   Ensure you have the latest stable Rust version installed. [Install here](https://rustup.rs/).

3. **Build the project**:
   ```bash
   cargo build --release
   ```

## 🖥 Usage

Run the sniper using Cargo:

```bash
cargo run
```

The tool will start logging to the console and a local `market_logX.txt` file.

## 📝 Example Output

```text
📝 Logging into: market_log1.txt
Trying slug: btc-updown-5m-1716819300
Market: Will Bitcoin be above $68,400 at 2:15 PM? | Bid: 48¢
✅ Active market mili! WebSocket subscribe karte hain...
⏰ Market ends at unix: 1716819600
📡 Subscribed! Live feed:
⏱️ 45s | 💰 Price: 52¢ | Size: 1000 | Bid: 50¢ | Momentum: +2.10%
🎯 SNIPER! ⏱️ 22s | BUY UP @ 72¢ | Momentum: +6.45%
```

## ⚠️ Disclaimer

**This software is for educational and research purposes only.** Trading on prediction markets involves significant risk. The authors are not responsible for any financial losses incurred through the use of this tool. Never trade more than you can afford to lose.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
