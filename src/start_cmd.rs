use std::{env};
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use base58::FromBase58;
use dotenv::dotenv;

pub fn main() {
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
    println!("==================== 🥥 Start coconut Bot! ====================");
    dotenv().ok();
    // check config.json is existed
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