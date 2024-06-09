use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(
    name = "monea",
    about = "An unopinionated CLI tool for playing rollup legos."
)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the Monea command
    Run(crate::args::RunDeployArgs),

    /// Deploy the Monea command
    Deploy(crate::args::RunDeployArgs),

    /// Manage templates
    #[clap(subcommand)]
    Templates(TemplateCommands),

    /// Check for updates
    VersionCheck,
}

#[derive(Subcommand, Debug)]
pub enum TemplateCommands {
    /// Create a new template
    New(crate::args::NewTemplateArgs),

    /// List all templates
    List,

    /// Delete a template
    Delete(crate::args::DeleteTemplateArgs),
}
