use std::env;
use clap::{Arg, Command};
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use base58::FromBase58;
use std::fs;
use std::io::{self, Write};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::fmt::Display;


fn main() {
    // load .env file
    dotenv().ok();

    let matches = Command::new("coconut-bot")
        .version("1.0")
        .author("Abandon Inc. <tom@abandon.ai>")
        .about("Solana Pump.fun Clipper Bot")
        .subcommand(Command::new("start").about("Start the bot"))
        .subcommand(Command::new("stop").about("Stop the bot"))
        .subcommand(Command::new("status").about("Check bot status"))
        .subcommand(Command::new("config").about("Configure the bot"))
        .subcommand(
            Command::new("logs")
                .about("View logs")
                .arg(Arg::new("tail").action(clap::ArgAction::SetTrue)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            start_command();
        }
        Some(("stop", _)) => {
            println!("==================== 🥥 Stopped coconut-bot Bot! ====================");
        }
        Some(("status", _)) => {
            println!("==================== 🥥 coconut-bot Bot Status ====================");
        }
        Some(("config", _)) => {
            config_command();
        }
        Some(("logs", sub_m)) => {
            println!("==================== 🥥 coconut-bot Bot Logs! ====================");
            if sub_m.contains_id("tail") {
                println!("Tail logs enabled");
            }
        }
        _ => {
            println!("No valid command provided.");
        }
    }
}

fn start_command() {
    let ascii_logo = "
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░
 ";
    println!("{}", ascii_logo);
    println!("==================== 🥥 Start coconut-bot Bot! ====================");
    // check config.json is existed
    let config_path = "config.json";
    if !fs::metadata(config_path).is_ok() {
        println!("Not Find Config File. Use `coconut-bot config` command first.");
        return;
    }
    let private_key_base58 = env::var("SOLANA_PRIVATE_KEY")
        .expect("SOLANA_PRIVATE_KEY Not Found");
    let private_key_bytes = private_key_base58
        .from_base58()
        .expect("Failed to decode Base58 private key");
    let keypair = Keypair::from_bytes(&private_key_bytes).expect("Invalid private key");
    let public_key = keypair.pubkey();
    println!("Public Key: {}\n", public_key);
    let rpc_url = "https://api.mainnet-beta.solana.com"; // 根据需求选择RPC节点
    let client = RpcClient::new(rpc_url.to_string());

    match client.get_balance(&public_key) {
        Ok(balance) => {
            println!("Account Balance: {} SOL\n", balance as f64 / 1_000_000_000.0);
        }
        Err(e) => {
            eprintln!("Failed to get balance: {}\n", e);
        }
    }

    let config = Config::load_from_file(config_path);
    println!("Buy Amount: {}\n", config.buy_amount);
    println!("Jito Fee: {}\n", config.jito_fee);
    println!("Price Check Interval: {}\n", config.price_check_interval);
    println!("Take Profit: {}\n", config.take_profit);
    println!("Stop Loss: {}\n", config.stop_loss);
    println!("Sell Slippage: {}\n", config.sell_slippage);
    println!("Skip Selling If Lost More Than: {}\n", config.skip_selling_if_lost_more_than);
    println!("Price Check Duration: {}\n", config.price_check_duration);
    println!("Auto Sell: {}\n", config.auto_sell);
    println!("Max Sell Retries: {}\n", config.max_sell_retries);

    println!("==================== 🥥 Monitor online transactions ====================");
    // TODO: Listen to pump.fun transactions on the solana network

    // TODO: Find transactions and print it

    // TODO: Buy it
}


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    buy_amount: f64,
    jito_fee: f64,
    price_check_interval: u64,
    take_profit: f64,
    stop_loss: f64,
    sell_slippage: f64,
    skip_selling_if_lost_more_than: f64,
    price_check_duration: u64,
    auto_sell: bool,
    max_sell_retries: u32,
}

impl Config {
    fn default() -> Self {
        Self {
            buy_amount: 1.0,
            jito_fee: 0.02,
            price_check_interval: 10,
            take_profit: 10.0,
            stop_loss: 5.0,
            sell_slippage: 0.5,
            skip_selling_if_lost_more_than: 20.0,
            price_check_duration: 60,
            auto_sell: true,
            max_sell_retries: 3,
        }
    }

    fn load_from_file(path: &str) -> Self {
        fs::read_to_string(path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_else(Self::default)
    }

    fn save_to_file(&self, path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, json);
        }
    }
}

fn prompt<T>(msg: &str, default: T) -> T
where
    T: FromStr + Display + Copy,
    <T as FromStr>::Err: std::fmt::Debug,
{
    print!("{} (Default: {}): ", msg, default);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let trimmed = input.trim();
    if trimmed.is_empty() {
        return default;
    }

    match trimmed.parse::<T>() {
        Ok(value) => value,
        Err(err) => {
            println!("🥥 Input Error: {:?}, Use Default Value {}", err, default);
            default
        }
    }
}

fn config_command() {
    let ascii_logo = "
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░

 ";
    println!("{}", ascii_logo);
    println!("==================== 🥥 Setting coconut-bot Bot! ====================");
    let config_path = "coconut.json";
    let mut config = Config::load_from_file(config_path);

    config.buy_amount = prompt("Buy Amount", config.buy_amount);
    config.jito_fee = prompt("Jito Fee", config.jito_fee);
    config.price_check_interval = prompt("Price Check Interval (s)", config.price_check_interval);
    config.take_profit = prompt("Take Profit", config.take_profit);
    config.stop_loss = prompt("Stop Loss", config.stop_loss);
    config.sell_slippage = prompt("Sell Slippage", config.sell_slippage);
    config.skip_selling_if_lost_more_than = prompt("Skip Selling If Loss More Than", config.skip_selling_if_lost_more_than);
    config.price_check_duration = prompt("Price Check Duration (s)", config.price_check_duration);
    config.auto_sell = prompt("Auto Sell (true/false)", config.auto_sell);
    config.max_sell_retries = prompt("Max Sell Retries", config.max_sell_retries);

    config.save_to_file(config_path);
    println!("🥥 Config save to {}", config_path);
}
