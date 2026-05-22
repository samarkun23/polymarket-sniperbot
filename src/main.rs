use std::{thread::sleep, time::Duration};

use reqwest::Client;
use serde::Deserialize;

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
}


#[tokio::main]
async fn main() {
    let client = Client::new();
    loop {
        println!("\n Scanning markets..");
        
        let res = client
            .get("https://gamma-api.polymarket.com/markets?active=true&limit=20")
            .send()
            .await
            .unwrap()
            .json::<Vec<Market>>()
            .await
            .unwrap();
    
        println!("Total number of markets fetched: {}", res.len());
        println!("-------------");
    
        for market in &res {
            let question = market.question.as_deref().unwrap_or("unknown");
            let bid = market.best_bid.unwrap_or(0.0);
            let ask = market.best_ask.unwrap_or(0.0);
            let spread = market.spread.unwrap_or(0.0);
            let last = market.last_trade_price.unwrap_or(0.0);
            let vol = market.volume_24hr.unwrap_or(0.0);
            let accepting = market.accepting_orders.unwrap_or(false);
    
            println!("Market  : {}", question);
            println!(
                "Bid/Ask : {}¢ / {}¢",
                (bid * 100.0) as u32,
                (ask * 100.0) as u32
            );
            println!("Spread  : {}¢", (spread * 100.0) as u32);
            println!("Last    : {}¢", (last * 100.0) as u32);
            println!("Vol 24h : ${:.0}", vol);
            println!("Orders  : {}", accepting);
            if spread <= 0.01 && vol > 1000.0 && bid > 0.3 {
                println!("🎯 BUY SIGNAL: {}", question);
                println!("   Reason: Tight spread + High volume + Leaning YES");
            }else{
                println!("No signal -> spread:{} , vol: {}, bid: {}", spread, vol, bid);
            }
            println!("---");
        }

        sleep(Duration::from_secs(10));
    }
}
