pub fn main() {
    // Check if coconut.json exists in current directory
    let config_path = "coconut.json";

    if !std::path::Path::new(config_path).exists() {
        // Create new config file with empty private key
        let config = serde_json::json!({
            "private_key": ""
        });

        std::fs::write(
            config_path,
            serde_json::to_string_pretty(&config).unwrap()
        ).expect("🥥 Failed to write config file");

        println!("🥥 Please fixed the coconut.json file.")
    } else {
        println!("🥥 You already have a config file in this folder!");
    }
}