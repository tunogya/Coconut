use std::env;
use clap::{Arg, Command};
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use base58::FromBase58;
use std::fs;
use std::fmt::Display;


fn main() {
    // load .env file
    dotenv().ok();

    let matches = Command::new("coconut-bot")
        .version("1.0")
        .author("Abandon Inc. <tom@abandon.ai>")
        .about("Solana Pump.fun Clipper Bot")
        .subcommand(Command::new("start").about("Start the bot"))
        .subcommand(Command::new("status").about("Check bot status"))
        .subcommand(Command::new("config").about("Configure the bot"))
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            start_command();
        }
        Some(("status", _)) => {
            println!("==================== 🥥 coconut-bot Bot Status ====================");
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
}