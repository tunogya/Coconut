use std::{fs};
use std::path::Path;
use solana_sdk::signature::{Keypair, Signer};
use solana_client::rpc_client::RpcClient;
use crate::constants;
use serde_json::Value;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    let config_rpc_url = config["rpc_url"].as_str()
        .ok_or("RPC URL not found in config")?;

    // Check if private key and public key match
    let keypair = Keypair::from_base58_string(private_key);
    let derived_public_key = keypair.pubkey().to_string();
    if derived_public_key != public_key {
        return Err("🥥 Private key and public key do not match".into());
    }

    println!("🥥 Public Key: https://solscan.io/account/{}", public_key);
    println!("🥥 RPC URL: {}", config_rpc_url);

    let client = RpcClient::new(config_rpc_url.to_string());

    let balance = client.get_balance(&keypair.pubkey())?;
    println!("🥥 Bot Balance: {} SOL", balance as f64 / 1_000_000_000.0);

    println!("==================== 🥥 Start Coconut Bot! ====================");

    Ok(())
}