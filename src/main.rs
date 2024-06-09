use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[clap(name = "monea", about = "A simple CLI for Monea.")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the Monea command
    Run(RunDeployArgs),

    /// Deploy the Monea command
    Deploy(RunDeployArgs),

    /// Manage templates
    #[clap(subcommand)]
    Templates(TemplateCommands),
}

#[derive(Subcommand, Debug)]
enum TemplateCommands {
    /// Create a new template
    New(NewTemplateArgs),

    /// List all templates
    List,

    /// Delete a template
    Delete(DeleteTemplateArgs),
}

#[derive(Parser, Debug)]
struct RunDeployArgs {
    #[clap(flatten)]
    common: CommonArgs,

    /// Template(s)
    #[clap(short, long)]
    template: Option<Vec<String>>,

    /// Positional template names
    #[clap()]
    positional_templates: Vec<String>,
}

#[derive(Parser, Debug)]
struct NewTemplateArgs {
    /// Template name
    #[clap(short, long)]
    name: String,

    #[clap(flatten)]
    common: CommonArgs,
}

#[derive(Parser, Debug)]
struct DeleteTemplateArgs {
    /// Template name
    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Parser, Debug, Clone)]
struct CommonArgs {
    /// Execution layer
    #[clap(short, long, value_enum)]
    execution: Option<ExecutionOptions>,

    /// Settlement layer
    #[clap(short, long, value_enum)]
    settlement: Option<SettlementOptions>,

    /// Sequencer
    #[clap(short = 'q', long, value_enum)]
    sequencer: Option<SequencerOptions>,

    /// Data availability
    #[clap(short, long, value_enum)]
    data_availability: Option<DataAvailabilityOptions>,
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
enum ExecutionOptions {
    OpStack,
    PolygonCDK,
    ArbOrbit,
    Rollkit,
    Polaris,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        ExecutionOptions::OpStack
    }
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
enum SettlementOptions {
    Local,
    Sepolia,
    EthereumMainnet,
    Base,
}

impl Default for SettlementOptions {
    fn default() -> Self {
        SettlementOptions::Local
    }
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
enum SequencerOptions {
    Default,
    Espresso,
}

impl Default for SequencerOptions {
    fn default() -> Self {
        SequencerOptions::Default
    }
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
enum DataAvailabilityOptions {
    Default, // the settlement layer used
    Celestia,
    Eigen,
    Avail,
}

impl Default for DataAvailabilityOptions {
    fn default() -> Self {
        DataAvailabilityOptions::Default
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    template_name: Option<String>,
    execution: ExecutionOptions,
    settlement: SettlementOptions,
    sequencer: SequencerOptions,
    data_availability: DataAvailabilityOptions,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run(args) => {
            if args.template.is_none() && args.positional_templates.is_empty() {
                let selected_template = select_template().unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
                run_command_with_template(&selected_template);
            } else {
                run_command(args);
            }
        }
        Commands::Deploy(args) => {
            if args.template.is_none() && args.positional_templates.is_empty() {
                let selected_template = select_template().unwrap_or_else(|err| {
                    eprintln!("{}", err);
                    std::process::exit(1);
                });
                deploy_command_with_template(&selected_template);
            } else {
                deploy_command(args);
            }
        }
        Commands::Templates(cmd) => match cmd {
            TemplateCommands::New(args) => {
                create_template(args);
            }
            TemplateCommands::List => {
                list_templates();
            }
            TemplateCommands::Delete(args) => {
                delete_template(args);
            }
        },
    }
}

fn get_global_templates_dir() -> PathBuf {
    let mut dir = home_dir().expect("Could not find home directory");
    dir.push(".monea-cli/templates");
    dir
}

fn select_template() -> Result<String, String> {
    let folder_path = get_global_templates_dir();

    if !folder_path.exists() {
        return Err("No templates directory found.".to_string());
    }

    let mut templates = vec![];
    for entry in fs::read_dir(folder_path).expect("Failed to read templates directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().unwrap_or_default() == "yaml" {
            templates.push(path.file_stem().unwrap().to_str().unwrap().to_string());
        }
    }

    if templates.is_empty() {
        return Err("No templates found.".to_string());
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a template")
        .items(&templates)
        .default(0)
        .interact()
        .expect("Failed to select template");

    Ok(templates[selection].clone())
}

fn run_command(args: &RunDeployArgs) {
    let template_names = args.template.clone().unwrap_or_default();
    let template_names = [&template_names[..], &args.positional_templates[..]].concat();

    for template_name in template_names {
        let config = load_template(&template_name).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        run_command_with_template(&template_name);
    }
}

fn run_command_with_template(template_name: &str) {
    let config = load_template(template_name).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    print_config(
        &format!(
            "Running a local Docker setup with template '{}':",
            template_name
        ),
        &config,
    );
}

fn deploy_command(args: &RunDeployArgs) {
    let template_names = args.template.clone().unwrap_or_default();
    let template_names = [&template_names[..], &args.positional_templates[..]].concat();

    for template_name in template_names {
        let config = load_template(&template_name).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        deploy_command_with_template(&template_name);
    }
}

fn deploy_command_with_template(template_name: &str) {
    let config = load_template(template_name).unwrap_or_else(|err| {
        eprintln!("{}", err);
        std::process::exit(1);
    });
    print_config(
        &format!(
            "Deploying to Monea Cloud with template '{}':",
            template_name
        ),
        &config,
    );
}

fn create_template(args: &NewTemplateArgs) {
    let config = Config {
        template_name: Some(args.name.clone()),
        execution: args.common.execution.clone().unwrap_or_default(),
        settlement: args.common.settlement.clone().unwrap_or_default(),
        sequencer: args.common.sequencer.clone().unwrap_or_default(),
        data_availability: args.common.data_availability.clone().unwrap_or_default(),
    };

    let yaml = serde_yaml::to_string(&config).expect("Failed to serialize config to YAML");
    let folder_path = get_global_templates_dir();

    // Create the templates directory if it doesn't exist
    if !folder_path.exists() {
        fs::create_dir_all(&folder_path).expect("Failed to create templates directory");
    }

    let file_path = folder_path.join(format!("{}.yaml", args.name));
    fs::write(file_path, yaml).expect("Failed to write YAML file");

    print_config(&format!("Created a new template '{}'", args.name), &config);
}

fn list_templates() {
    let folder_path = get_global_templates_dir();

    if !folder_path.exists() {
        println!("No templates directory found.");
        return;
    }

    let mut table = Table::new();
    table.add_row(row![
        "Name",
        "Execution",
        "Settlement",
        "Sequencer",
        "Data Availability"
    ]);

    for entry in fs::read_dir(folder_path).expect("Failed to read templates directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().unwrap_or_default() == "yaml" {
            let yaml = fs::read_to_string(&path).expect("Failed to read template file");
            let config: Config = serde_yaml::from_str(&yaml).expect("Failed to parse template");

            table.add_row(row![
                path.file_stem().unwrap().to_str().unwrap(),
                format!("{:?}", config.execution),
                format!("{:?}", config.settlement),
                format!("{:?}", config.sequencer),
                format!("{:?}", config.data_availability),
            ]);
        }
    }

    table.printstd();
}

fn delete_template(args: &DeleteTemplateArgs) {
    let template_name = match &args.name {
        Some(name) => name.clone(),
        None => {
            // Interactive selection of the template
            let folder_path = get_global_templates_dir();

            if !folder_path.exists() {
                println!("No templates directory found.");
                return;
            }

            let mut templates = vec![];
            for entry in fs::read_dir(folder_path).expect("Failed to read templates directory") {
                let entry = entry.expect("Failed to read directory entry");
                let path = entry.path();
                if path.extension().unwrap_or_default() == "yaml" {
                    templates.push(path.file_stem().unwrap().to_str().unwrap().to_string());
                }
            }

            if templates.is_empty() {
                println!("No templates found.");
                return;
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Select a template to delete")
                .items(&templates)
                .default(0)
                .interact()
                .expect("Failed to select template");

            templates[selection].clone()
        }
    };

    let file_path = get_global_templates_dir().join(format!("{}.yaml", template_name));
    if file_path.exists() {
        fs::remove_file(&file_path).expect("Failed to delete template file");
        println!("Deleted template '{}'", template_name);
    } else {
        println!("No template named '{}' found", template_name);
    }
}

fn load_template(template_name: &str) -> Result<Config, String> {
    let file_path = get_global_templates_dir().join(format!("{}.yaml", template_name));
    if !file_path.exists() {
        return Err(format!("No template named '{}' exists", template_name));
    }

    let yaml =
        fs::read_to_string(file_path).map_err(|err| format!("Failed to read template: {}", err))?;
    let mut config: Config =
        serde_yaml::from_str(&yaml).map_err(|err| format!("Failed to parse template: {}", err))?;
    config.template_name = Some(template_name.to_string());
    Ok(config)
}

fn print_config(title: &str, config: &Config) {
    println!("{}", title);
    let mut table = Table::new();
    table.add_row(row![
        "Template",
        "Execution",
        "Settlement",
        "Sequencer",
        "Data Availability"
    ]);
    table.add_row(row![
        config.template_name.clone().unwrap_or_default(),
        format!("{:?}", config.execution),
        format!("{:?}", config.settlement),
        format!("{:?}", config.sequencer),
        format!("{:?}", config.data_availability),
    ]);
    table.printstd();
}
