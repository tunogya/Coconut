use solana_sdk::signature::{Keypair, Signer};
use serde_json;
use std::fs;
use std::path::Path;
use bs58;
use crate::constants;

pub fn main() {
    let config_path = "coconut.json";
    let ascii_logo = constants::app::ASCII_LOGO;
    if !Path::new(config_path).exists() {
        let keypair = Keypair::new();

        let private_key_base58 = bs58::encode(keypair.to_bytes()).into_string();

        let config = serde_json::json!({
            "private_key": private_key_base58,
            "public_key": keypair.pubkey().to_string(),
            "rpc_url": "https://api.mainnet-beta.solana.com",
        });

        fs::write(
            config_path,
            serde_json::to_string_pretty(&config).unwrap()
        ).expect("🥥 Failed to write config file");
        println!("{}", ascii_logo);
        println!("==================== 🥥 Init Coconut Bot ====================");
        println!("🥥 New Solana account created. Check 'coconut.json' for details.");
    } else {
        println!("{}", ascii_logo);
        println!("==================== 🥥 Init Coconut Bot ====================");
        println!("🥥 You already have a config file in this folder!");
    }
}