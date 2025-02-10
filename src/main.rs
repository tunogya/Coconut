mod commands;

use clap::Command;

fn main() {
    let matches = Command::new("coconut")
        .version("1.0")
        .author("tunogya <tom@abandon.ai>")
        .about("Solana Pump.fun Sniper Bot")
        // manage the tasks
        .subcommand(Command::new("init").about("Init config of the bot"))
        .subcommand(Command::new("start").about("Start buying and selling"))
        .subcommand(Command::new("stop").about("Stop buying more"))
        // manage the orders
        .subcommand(Command::new("ps").about("List all orders"))
        .subcommand(Command::new("sell").about("Sell all orders manually"))
        .subcommand(Command::new("logs").about("Get logs"))
        // manage the configs
        .subcommand(Command::new("config").about("Get the config of the bot"))
        // manage the wallets
        .subcommand(Command::new("balance").about("Get the balance of the bot"))
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            commands::init::main();
        }
        Some(("start", _)) => {
            commands::start::main();
        }
        Some(("stop", _)) => {
            commands::stop::main();
        }
        Some(("ps", _)) => {
            commands::ps::main();
        }
        Some(("sell", _)) => {
            commands::sell::main();
        }
        Some(("logs", _)) => {
            commands::logs::main();
        }
        Some(("config", _)) => {
            commands::config::main();
        }
        Some(("balance", _)) => {
            commands::balance::main();
        }
        _ => {
            println!("No valid command provided.");
        }
    }
}

