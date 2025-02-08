use clap::{Arg, Command};
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer};
use base58::FromBase58;
use std::env;

fn main() {
    // load .env file
    dotenv().ok();

    let ascii_logo = "
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░  ░▒▓█▓▒░
 ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░░▒▓██████▓▒░   ░▒▓█▓▒░

 ";

    let matches = Command::new("Coconut")
        .version("1.0")
        .author("Abandon Inc. <tom@abandon.ai>")
        .about("Solana Pump.fun Clipper Bot")
        .subcommand(Command::new("start").about("Start the bot"))
        .subcommand(Command::new("stop").about("Stop the bot"))
        .subcommand(Command::new("status").about("Check bot status"))
        .subcommand(
            Command::new("config")
                .about("Configure the bot")
                .arg(Arg::new("set").value_parser(clap::value_parser!(String))),
        )
        .subcommand(
            Command::new("logs")
                .about("View logs")
                .arg(Arg::new("tail").action(clap::ArgAction::SetTrue)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("start", _)) => {
            println!("{}", ascii_logo);
            println!("==================== 🥥 Starting Coconut Bot! ====================");
            let private_key_base58 = env::var("SOLANA_PRIVATE_KEY")
                .expect("SOLANA_PRIVATE_KEY not found in .env");
            let private_key_bytes = private_key_base58
                .from_base58()
                .expect("Failed to decode Base58 private key");
            let keypair = Keypair::from_bytes(&private_key_bytes).expect("Invalid private key");
            let public_key = keypair.pubkey();
            println!("Public Key: {}", public_key);

            let rpc_url = "https://api.mainnet-beta.solana.com"; // 根据需求选择RPC节点
            let client = RpcClient::new(rpc_url.to_string());

            match client.get_balance(&public_key) {
                Ok(balance) => {
                    println!("Account Balance: {} SOL", balance as f64 / 1_000_000_000.0);
                }
                Err(e) => {
                    eprintln!("Failed to get balance: {}", e);
                }
            }

            // get PRIVATE_KEY from .env SOLANA_PRIVATE_KEY

            // get pubkey from PRIVATE_KEY

            // get sol balance

            // get config setting

            //
        }
        Some(("stop", _)) => {
            println!("==================== 🥥 Stopped Coconut Bot! ====================");
        }
        Some(("status", _)) => {
            println!("==================== 🥥 Coconut Bot Status ====================");
        }
        Some(("config", sub_m)) => {
            if let Some(config) = sub_m.get_one::<String>("set") {
                println!("Setting config: {}", config);
            }
        }
        Some(("logs", sub_m)) => {
            println!("==================== 🥥 Coconut Bot Logs! ====================");
            if sub_m.contains_id("tail") {
                println!("Tail logs enabled");
            }
        }
        _ => {
            println!("No valid command provided.");
        }
    }
}
