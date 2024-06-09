mod args;
mod cli;
mod commands;
mod utils;

use clap::Parser;

fn main() {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Run(args) => commands::run_deploy::run_command(args),
        cli::Commands::Deploy(args) => commands::run_deploy::deploy_command(args),
        cli::Commands::Templates(cmd) => commands::templates::handle_template_commands(cmd),
        cli::Commands::VersionCheck => commands::version_check::version_check(),
    }
}
