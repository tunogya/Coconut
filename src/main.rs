use clap::Command;
use solana_sdk::signature::Signer;
use base58::FromBase58;
use std::fmt::Display;

mod start_cmd;
mod init_cmd;
mod stop_cmd;
mod ps_cmd;
mod sell_cmd;
mod logs_cmd;
mod config_cmd;
mod balance_cmd;

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
            init_cmd::main();
        }
        Some(("start", _)) => {
            start_cmd::main();
        }
        Some(("stop", _)) => {
            stop_cmd::main();
        }
        Some(("ps", _)) => {
            ps_cmd::main();
        }
        Some(("sell", _)) => {
            sell_cmd::main();
        }
        Some(("logs", _)) => {
            logs_cmd::main();
        }
        Some(("config", _)) => {
            config_cmd::main();
        }
        Some(("balance", _)) => {
            balance_cmd::main();
        }
        _ => {
            println!("No valid command provided.");
        }
    }
}

