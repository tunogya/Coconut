use crate::constants;
use futures_util::{stream::StreamExt, SinkExt};
use serde_json::Value;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use solana_transaction_status::{EncodedTransaction, UiMessage, UiTransactionEncoding};
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
    println!("==================== 🥥 Start Coconut Bot! ====================");

    let (tx, rx) = mpsc::channel(100);

    let buy_task = task::spawn(buy_loop(config.clone(), tx.clone()));
    let sell_task = task::spawn(sell_loop(config.clone(), rx));

    tokio::try_join!(buy_task, sell_task).unwrap();

    Ok(())
}

async fn buy_loop(config: Value, tx: mpsc::Sender<Order>) {
    let private_key = match config["private_key"].as_str() {
        Some(private_key) => private_key,
        None => {
            eprintln!("🥥 Missing private_key in config");
            return;
        }
    };
    let keypair = Keypair::from_base58_string(private_key);
    let rpc_url = match config["rpc_url"].as_str() {
        Some(url) => url,
        None => {
            eprintln!("🥥 Missing rpc_url in config");
            return;
        }
    };
    let client = RpcClient::new(rpc_url.to_string());

    let balance = match client.get_balance(&keypair.pubkey()) {
        Ok(balance) => balance,
        Err(e) => {
            eprintln!("🥥 Failed to get balance: {}", e);
            return;
        }
    };

    println!("🥥 Bot Balance: {} SOL", balance as f64 / 1_000_000_000.0);

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

    let subscription_message = constants::app::LOGS_SUBSCRIBE_MESSAGE;

    if let Err(e) = ws_stream
        .send(Message::Text(Utf8Bytes::from(
            subscription_message,
        )))
        .await
    {
        eprintln!("🥥 Failed to send message: {}", e);
        return;
    }
    println!("🥥 Subscribe programSubscribe!");

    let mut is_buying = false;
    let mut is_bought = false;

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                handle_stream_logs(text, &mut is_buying , &mut is_bought);
            }
            Ok(msg) => println!("🥥 Received non-text message: {:?}", msg),
            Err(e) => {
                eprintln!("🥥 Error receiving message: {}", e);
                break;
            }
        }
    }
}

fn handle_stream_logs(text: Utf8Bytes, is_buying: &mut bool, is_bought: &mut bool) {
    match serde_json::from_str::<Value>(&text) {
        Ok(json) => {
            if let (Some(signature), Some(logs)) = (
                json["params"]["result"]["value"]["signature"].as_str(),
                json["params"]["result"]["value"]["logs"].as_array(),
            ) {
                if logs.iter().any(|log| log.as_str().map_or(false, |s| s.contains("MintTo")))
                    && !*is_buying
                    && !*is_bought
                {
                        *is_buying = true;
                        println!("🥥 Found new token on pump.fun, starting purchase!");
                        println!("🥥 Signature: https://solscan.io/tx/{}", signature);
                }
            }
        }
        Err(e) => {
            eprintln!("🥥 Failed to parse JSON: {}", e);
        }
    }

        // if let (Some(signature), Some(logs)) = (
        //     json["params"]["result"]["value"]["signature"].as_str(),
        //     json["params"]["result"]["value"]["logs"].as_array(),
        // ) {
        //     println!("🥥 Signature: {}", signature);
            // if logs.iter().any(|log| log.as_str().map_or(false, |s| s.contains("MintTo")))
            //     && !is_buying
            //     && !is_bought
            // {
            //     is_buying = true;
            //     println!("🥥 Found new token on pump.fun, starting purchase!");
            //     println!("Signature: {}", signature);
            //
            //     if let Ok(signature_parsed) = signature.parse() {
            //         if let Ok(tx_result) =
            //             client.get_transaction(&signature_parsed, UiTransactionEncoding::JsonParsed)
            //         {
            //             if let EncodedTransaction::Json(tx_json) = tx_result.transaction.transaction {
            //                 match tx_json.message {
            //                     UiMessage::Parsed(parsed_msg) => {
            //                         let account_keys = parsed_msg.account_keys;
            //                         // print account_keys
            //                         println!("🥥 Account keys: {:?}", account_keys);
            //
            //                         // let wallet = account_keys[0].to_string();
            //                         // let mint = account_keys[1].to_string();
            //                         // let token_pool_ata = account_keys[4].to_string();
            //                         // println!("wallet: {}", wallet);
            //                         // println!("mint: {}", mint);
            //                         // println!("token_pool_ata: {}", token_pool_ata);
            //                     }
            //                     UiMessage::Raw(_) => {
            //                         println!("Raw message encountered, no `account_keys` available.");
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // } else {
            //     println!("No pump.fun log")
            // }
        // } else {
        //     eprintln!("🥥 Failed to parse logs");
        // }
    // } else {
    //     eprintln!("🥥 Failed to parse message");
    // }
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
