use std::{fs};
use std::path::Path;
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use crate::constants;
use serde_json::Value;
use tokio::{task, time};
use tokio::sync::mpsc;
use std::time::Duration;
use tokio_tungstenite::connect_async;
use std::collections::HashMap;

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
    let private_key = config["private_key"].as_str()
        .ok_or("Private key not found in config")?;
    let public_key = config["public_key"].as_str()
        .ok_or("Public key not found in config")?;
    let rpc_url = config["rpc_url"].as_str()
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
    let ws_rpc_url = config["ws_rpc_url"].as_str();
    // create a websocket connection
    let (stream, response) = connect_async(ws_rpc_url.unwrap()).await.unwrap();

    println!("🥥 WebSocket connect success...");

    println!("🥥 WebSocket response: {:?}", response);
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