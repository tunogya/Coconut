mod cmd;

use clap::Command;

fn main() {
    let matches = Command::new("coconut")
        .version("1.0")
        .author("tunogya <tom@abandon.ai>")
        .about("Solana Pump.fun Sniper Bot")
        // manage the tasks
        .subcommand(Command::new("init").about("Initialize bot configuration"))
        .subcommand(Command::new("start").about("Start trading bot"))
        .subcommand(Command::new("stop").about("Stop trading bot"))
        // manage the orders
        .subcommand(Command::new("ps").about("Show active orders"))
        .subcommand(Command::new("sell").about("Manually sell all positions"))
        .subcommand(Command::new("logs").about("Display bot logs"))
        // manage the configs
        .subcommand(Command::new("config").about("Show bot configuration"))
        // manage the wallets
        .subcommand(Command::new("balance").about("Show wallet balance"))
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

