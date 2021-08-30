
mod command;
mod error;

use clap::{AppSettings, Clap};
use colored::*;

use error::AnalyzerResult;

#[derive(Clap)]
#[clap(version = "1.0", author = "Yu-Shan Lin (sam123456777@gmail.com)")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand
}

#[derive(Clap)]
enum SubCommand {
    Create(CreateCommand)
}

#[derive(Clap)]
struct CreateCommand {
    fund: String
}

fn main() {
    // Goal for v1.0
    // Inputs: initial assets, transaction records
    // Outputs: the timeline of price per share
    // Others:
    // - Auto save the data
    // - Won't save duplicate transactions

    // Command: create [FUND]
    // Command: add-history [FUND] [HISTORY FILE]
    // Command: generate-timeline [FUND]

    // Parse the command
    let opts: Command = Command::parse();
    if let Err(e) = run(opts) {
        println!("{}: {}", "Error".red(), e);
    }
}

fn run(opts: Command) -> AnalyzerResult<()> {
    match opts.subcmd {
        SubCommand::Create(cmd) => {
            command::create::create(&cmd.fund)?;
        }
    }

    Ok(())
}
