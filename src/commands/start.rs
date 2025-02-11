use crate::constants;
use futures_util::{stream::StreamExt, SinkExt};
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::{task, time};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tungstenite::Utf8Bytes;
use url::Url;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_logo = constants::app::ASCII_LOGO;
    println!("{}", ascii_logo);
    println!("==================== 🥥 Check Coconut Bot Config ====================");
    // Check coconut.json file first
    let config_path = "coconut.json";
    if !Path::new(config_path).exists() {
        return Err("🥥 You need init this bot first: coconut init".into());
    }

    // Read and parse coconut.json
    let config_content = fs::read_to_string(config_path)?;
    let config: Value = serde_json::from_str(&config_content)?;

    // Validate config values
    let private_key = config["private_key"]
        .as_str()
        .ok_or("Private key not found in config")?;
    let public_key = config["public_key"]
        .as_str()
        .ok_or("Public key not found in config")?;
    let rpc_url = config["rpc_url"]
        .as_str()
        .ok_or("RPC URL not found in config")?;

    // Check if private key and public key match
    let keypair = Keypair::from_base58_string(private_key);
    let derived_public_key = keypair.pubkey().to_string();
    if derived_public_key != public_key {
        return Err("🥥 Private key and public key do not match".into());
    }

    println!("🥥 Public Key: https://solscan.io/account/{}", public_key);
    println!("🥥 RPC URL: {}", rpc_url);

    let client = RpcClient::new(rpc_url.to_string());

    let balance = client.get_balance(&keypair.pubkey())?;
    println!("🥥 Bot Balance: {} SOL", balance as f64 / 1_000_000_000.0);

    println!("==================== 🥥 Start Coconut Bot! ====================");

    let (tx, rx) = mpsc::channel(100);

    let buy_task = task::spawn(buy_loop(config.clone(), tx.clone()));
    let sell_task = task::spawn(sell_loop(config.clone(), rx));

    tokio::try_join!(buy_task, sell_task).unwrap();

    Ok(())
}

async fn buy_loop(config: Value, tx: mpsc::Sender<Order>) {
    let ws_rpc_url = match config["ws_rpc_url"].as_str() {
        Some(url) => url,
        None => {
            eprintln!("🥥 Missing ws_rpc_url in config");
            return;
        }
    };

    let url = match Url::parse(ws_rpc_url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("🥥 Invalid WebSocket URL: {}", e);
            return;
        }
    };

    let (mut ws_stream, _) = match connect_async(url.as_str()).await {
        Ok((ws, _)) => (ws, ()), // Only get WebSocketStream, ignore Response
        Err(e) => {
            eprintln!("🥥 Failed to connect: {}", e);
            return;
        }
    };

    let subscription_message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {
                "mentions": ["6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"]
            },
            {
                "commitment": "finalized"
            }
        ]
    });

    if let Err(e) = ws_stream
        .send(Message::Text(Utf8Bytes::from(
            subscription_message.to_string(),
        )))
        .await
    {
        eprintln!("🥥 Failed to send message: {}", e);
        return;
    }
    println!("🥥 Send programSubscribe success!");

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                // {"jsonrpc":"2.0","method":"programNotification","params":{"result":{"context":{"slot":319931865},"value":{"pubkey":"B4tQwAZt4dG8f8QMvZmSfw7GKwzGaiWJcoif52bWqJdX","account":{"lamports":17035577720,"data":["F7f4N2DYrGAlvgrHc24CAEgpd/MKAAAAJSb4euJvAQBIfVP3AwAAAACAxqR+jQMAAA==","base64"],"owner":"6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P","executable":false,"rentEpoch":18446744073709551615,"space":49}}},"subscription":57587}}
                println!("Received: {}", text)
            }
            Ok(_) => println!("🥥 Received non-text message"),
            Err(e) => {
                eprintln!("🥥 Error receiving message: {}", e);
                break;
            }
        }
    }
}

async fn sell_loop(config: Value, mut rx: mpsc::Receiver<Order>) {
    let mut orders: HashMap<u32, Order> = HashMap::new();

    while let Some(order) = rx.recv().await {
        orders.insert(order.id, order);
    }

    loop {
        println!("check for sell");
        time::sleep(Duration::from_secs(5)).await;
    }
}

#[derive(Debug, Clone)]
struct Order {
    id: u32,
    buy_price: f64,
    target_price: f64,
    stop_loss: f64,
}
