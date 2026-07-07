# 🎯 Polymarket Sniper

A high-performance Rust-based tool for monitoring and "sniping" binary options markets on Polymarket. Specifically optimized for the **5-minute Bitcoin (BTC) Up/Down prediction markets**.

## 🚀 Overview

`polymarket-sniper` automates the process of identifying, tracking, and signaling high-probability trades in the final seconds of Polymarket's short-term BTC price markets. It utilizes the Polymarket Gamma API for market discovery and the CLOB WebSocket for ultra-low latency price feeds.

## ✨ Key Features

- **Real-time TUI Dashboard**: A polished terminal interface featuring a price sparkline, expiry gauge, and momentum meter.
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

The tool will launch the TUI dashboard. Press **'q'** to exit. All data is also logged to a local `market_logX.txt` file.

## 🖥 TUI Layout

- **Market Header**: Shows the current active question and target slug.
- **Expiry Countdown**: A color-coded gauge showing time remaining until market resolution.
- **Price History**: A live sparkline showing the last 50 price ticks.
- **Stats Panel**: Real-time display of Current Price, Best Bid, Size, and Momentum %.
- **Sniper Signals**: A dedicated log window for high-probability trade alerts.

## ⚠️ Disclaimer

**This software is for educational and research purposes only.** Trading on prediction markets involves significant risk. The authors are not responsible for any financial losses incurred through the use of this tool. Never trade more than you can afford to lose.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## improvment 
  💡 New Features to Add (Roadmap)

  To take this from a "Signaling Tool" to a "Professional Trading Bot," here are the most impactful features you can add:

  1. Automated Execution (The "Real" Sniper)
  Currently, the bot only tells you when to buy.
   * Feature: Integrate Polymarket's Order Book API to place trades automatically.
   * Tech: You'll need to implement EIP-712 signing (using ethers-rs) and manage API Keys/Passphrases.
   * Benefit: Eliminates human latency. The bot buys the instant the 30-second window opens and conditions are met.

  2. TUI Dashboard (Terminal UI)
  The current scrolling text is hard to read during fast moves.
   * Feature: Use the ratatui (https://github.com/ratatui-org/ratatui) crate to create a visual dashboard.
   * Tech: Show a real-time price graph, a countdown timer bar, and a "Live Momentum" meter in the terminal.

  3. Backtesting Module
  You have all those market_log.txt files—use them!
   * Feature: Create a script that "replays" your logs to see how many of your sniper signals would have actually been profitable.
   * Benefit: You can tune your momentum > 5.0 threshold to a more optimal number (like 3.2 or 7.5) based on historical success.

  4. Multi-Asset Monitoring
   * Feature: Monitor ETH 5m markets or "Daily BTC High/Low" markets simultaneously.
   * Tech: Use tokio::spawn to run multiple market trackers in parallel within the same process.

  5. Telegram/Discord Alerts
   * Feature: Send a notification to your phone when a high-probability "🎯 SNIPER!" signal is detected.
   * Tech: Simple HTTP POST request to a Telegram Bot API.

  6. Smart Exit Logic (Take Profit / Stop Loss)
   * Feature: Most snipers only think about the entry. Add logic to "Sell" if the price hits 95¢ or if momentum suddenly reverses.

  7. Configuration File (config.toml)
   * Improvement: Move your constants (like 0.70 bid threshold or the 30s timer) out of main.rs and into a config file so you can change settings without recompiling the code.

  Which of these would you like to explore or implement first? I can help you draft the code for any of them.
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
src/
├── main.rs          (entry point)
├── provider.rs      (TradeProvider - execution engine)
├── market_worker.rs (MarketWorker - websocket handler)  
├── ui.rs            (UI rendering)
├── types.rs         (all structs/enums)
└── strategies.rs    (trading strategies)

WebSocket Feed
      |
      v
Market Monitor
      |
      v
Channel
      |
      v
Signal Engine
      |
      v
Channel
      |
      v
Order Executor
      |
      v
Exchange
here it is 
.....
