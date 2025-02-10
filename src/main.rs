mod cmd;

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
            cmd::init::main();
        }
        Some(("start", _)) => {
            cmd::start::main();
        }
        Some(("stop", _)) => {
            cmd::stop::main();
        }
        Some(("ps", _)) => {
            cmd::ps::main();
        }
        Some(("sell", _)) => {
            cmd::sell::main();
        }
        Some(("logs", _)) => {
            cmd::logs::main();
        }
        Some(("config", _)) => {
            cmd::config::main();
        }
        Some(("balance", _)) => {
            cmd::balance::main();
        }
        _ => {
            println!("No valid command provided.");
        }
    }
}

