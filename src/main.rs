use std::{collections::HashMap, thread::sleep, time::Duration};

use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::Deserialize;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::{Message, client};
use std::time::{SystemTime, UNIX_EPOCH};


use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct Market {
    question: Option<String>,

    #[serde(rename = "ConditionId")]
    condition_id: Option<String>,

    #[serde(rename = "endDateIso")]
    end_date_iso: Option<String>,

    #[serde(rename = "bestBid")]
    best_bid: Option<f64>,

    #[serde(rename = "bestAsk")]
    best_ask: Option<f64>,

    spread: Option<f64>,

    #[serde(rename = "lastTradePrice")]
    last_trade_price: Option<f64>,

    #[serde(rename = "volume24hr")]
    volume_24hr: Option<f64>,

    #[serde(rename = "acceptingOrders")]
    accepting_orders: Option<bool>,

    #[serde(rename = "clobTokenIds")]
    clob_token_ids: Option<String>,
}

#[derive(Deserialize, Debug)]
struct PriceChange {
    assest_id: String,
    price: String,
    size: String,
    side: String,
    best_bid: String,
    best_ask: String,
}

#[derive(Deserialize, Debug)]
struct WsMessage {
    event_type: Option<String>,
    price_changes: Option<Vec<PriceChange>>,
    timestamp: Option<String>,
}

fn get_next_log_file() -> String {
    let mut i = 1;

    loop {
        let filename = format!("market_log{}.txt", i);

        if !Path::new(&filename).exists() {
            return filename;
        }

        i += 1;
    }
}

#[tokio::main]
async fn main() {
    let log_file = get_next_log_file();

    println!("📝 Logging into: {}", log_file);
    let websocket_url = "wss://ws-subscriptions-clob.polymarket.com/ws/market";
    let client = Client::new();
    let mut price_history: HashMap<String, Vec<f64>> = HashMap::new();

    fn log_to_file(filename: &str, message: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();

    writeln!(file, "[{}] {}", now, message).unwrap();
    }
    fn get_current_btc_slug() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // 5 min = 300 seconds ke multiple pe round karo
        let rounded = (now / 300) * 300;
        format!("btc-updown-5m-{}", rounded)
    }

    fn get_market_end_time(slug: &str) -> u64 {
        let parts: Vec<&str> = slug.split('-').collect();
        if let Some(ts_str) = parts.last() {
            if let Ok(ts)  = ts_str.parse::<u64>(){
                return ts  + 300;
            }
        }
        0
    }

    fn seconds_remaining(end_time: u64) -> i64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        end_time as i64 - now as i64
    }

    loop {
            let slug = get_current_btc_slug();
            let msg = format!("Trying slug: {}", slug);
            println!("{}", msg);
            log_to_file(&log_file,&msg);

            let url = format!("https://gamma-api.polymarket.com/markets?slug={}", slug);
        let res = client
            .get(url)
            .send()
            .await
            .unwrap()
            .json::<Vec<Market>>()
            .await
            .unwrap();

        println!("Total number of markets fetched: {:?}", res.len());

        if res.is_empty() {
            println!("Market nahi mili, 5 sec wait...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            continue;
        }

        let market = &res[0];
        let bid = market.best_bid.unwrap_or(0.0);

        let msg = format!(
            "Market: {} | Bid: {}¢",
            market.question.as_deref().unwrap_or("?"),
            (bid * 100.0) as u32
        );
        println!("{}", msg);
        log_to_file(&log_file,&msg);

        if bid > 0.35 && bid < 0.65 {
            println!("✅ Active market mili! WebSocket subscribe karte hain...");

            let end_time = get_market_end_time(&slug);
                println!("⏰ Market ends at unix: {}", end_time);

            let ids_str = market.clob_token_ids.as_ref().unwrap();
            let ids: Vec<String> = serde_json::from_str(ids_str).unwrap();
            let yes_token = &ids[0];

            let (ws_stream, _) =
                connect_async("wss://ws-subscriptions-clob.polymarket.com/ws/market")
                    .await
                    .expect("WS connect failed");
            let (mut write, mut read) = ws_stream.split();

            let sub_msg = serde_json::json!({
                "type": "market",
                "assets_ids": [yes_token]
            });

            write
                .send(Message::Text(sub_msg.to_string().into()))
                .await
                .unwrap();
            println!("📡 Subscribed! Live feed:");

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(ws_msg) = serde_json::from_str::<serde_json::Value>(&text) {
                            if ws_msg["event_type"] == "price_change" {
                                if let Some(changes) = ws_msg["price_changes"].as_array() {
                                    if let Some(change) = changes.first() {

                                        let asset_id = change["asset_id"].as_str().unwrap_or("");
                                        if asset_id != yes_token.as_str() {
                                            continue;
                                        };

                                        let bid: f64 = change["best_bid"]
                                            .as_str()
                                            .unwrap_or("0")
                                            .parse()
                                            .unwrap_or(0.0);
                                        let price: f64 = change["price"]
                                            .as_str()
                                            .unwrap_or("0")
                                            .parse()
                                            .unwrap_or(0.0);
                                        let size: f64 = change["size"]
                                            .as_str()
                                            .unwrap_or("0")
                                            .parse()
                                            .unwrap_or(0.0);

                                        let history = price_history
                                            .entry("bid".to_string())
                                            .or_insert_with(Vec::new);

                                        if history.last() != Some(&bid) {
                                            history.push(bid);
                                            if history.len() > 10 {history.remove(0);}
                                        }

                                        if history.len() > 10 {
                                            history.remove(0);
                                        }

                                        let momentum = if history.len() >= 2 {
                                            let oldest = history[0];
                                            let newest = history[history.len() - 1];
                                            if oldest > 0.0 {
                                                ((newest - oldest) / oldest) * 100.0
                                            } else {
                                                0.0
                                            }
                                        } else {
                                            0.0
                                        };

                                        let secs_left = seconds_remaining(end_time);
                                        let tick_msg = format!(
                                            "⏱️ {}s | 💰 Price: {}¢ | Size: {:.0} | Bid: {}¢ | Momentum: {:+.2}%",
                                            secs_left,
                                            (price * 100.0) as u32,
                                            size,
                                            (bid * 100.0) as u32,
                                            momentum
                                        );
                                        println!("{}", tick_msg);
                                        log_to_file(&log_file,&tick_msg);

                                        // sniper signal 
                                         if secs_left > 0 && secs_left < 30
                                            && bid > 0.70
                                            && momentum > 5.0
                                        {
                                            let snipermsg = format!(
                                                
                                                "🎯 SNIPER! ⏱️{}s | BUY UP @ {}¢ | Momentum: {:+.2}%",
                                                secs_left,
                                                (bid * 100.0) as u32,
                                                momentum
                                            );
                                            println!("{}", snipermsg);
                                            log_to_file(&log_file, &snipermsg);
                                        } else if secs_left > 0 && secs_left < 30
                                            && bid < 0.30
                                            && momentum < -5.0
                                        {
                                            let snipermsg = format!(
                                                "🎯 SNIPER! ⏱️{}s | BUY DOWN @ {}¢ | Momentum: {:+.2}%",
                                                secs_left,
                                                ((1.0 - bid) * 100.0) as u32,
                                                momentum
                                            );
                                            println!("{}", snipermsg);
                                            log_to_file(&log_file, &snipermsg);
                                        }
                                        if secs_left <= 0 {
                                            let msg = "Market expire ".to_string();
                                            println!("{} ", msg);
                                            log_to_file(&log_file,&msg);
                                            break;
                                        }

                                        if momentum > 3.0 {
                                            println!("🚀 MOMENTUM UP → {:+.2}%", momentum);
                                        } else if momentum < -3.0 {
                                            println!("📉 MOMENTUM DOWN → {:+.2}%", momentum);
                                        }



                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Ping(data)) => {
                        write.send(Message::Pong(data)).await.unwrap();
                    }
                    Ok(_) => {}
                    Err(e) => {
                        println!("WS Error: {:?}", e);
                        break;
                    }
                }
            }

            println!("Market closed — next market dhundh raha hun...");
            price_history.clear();
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
    
}
