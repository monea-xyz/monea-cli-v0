use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct RunDeployArgs {
    #[clap(flatten)]
    pub common: CommonArgs,

    /// Template(s)
    #[clap(short, long)]
    pub template: Option<Vec<String>>,

    /// Positional template names
    #[clap()]
    pub positional_templates: Vec<String>,
}

#[derive(Parser, Debug)]
pub struct NewTemplateArgs {
    /// Template name
    #[clap(short, long)]
    pub name: String,

    #[clap(flatten)]
    pub common: CommonArgs,
}

#[derive(Parser, Debug)]
pub struct DeleteTemplateArgs {
    /// Template name
    #[clap(short, long)]
    pub name: Option<String>,
}

#[derive(Parser, Debug, Clone)]
pub struct CommonArgs {
    /// Execution layer
    #[clap(short, long, value_enum)]
    pub execution: Option<ExecutionOptions>,

    /// Settlement layer
    #[clap(short, long, value_enum)]
    pub settlement: Option<SettlementOptions>,

    /// Sequencer
    #[clap(short = 'q', long, value_enum)]
    pub sequencer: Option<SequencerOptions>,

    /// Data availability
    #[clap(short, long, value_enum)]
    pub data_availability: Option<DataAvailabilityOptions>,
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum ExecutionOptions {
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
pub enum SettlementOptions {
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
pub enum SequencerOptions {
    Default,
    Espresso,
}

impl Default for SequencerOptions {
    fn default() -> Self {
        SequencerOptions::Default
    }
}

#[derive(Debug, ValueEnum, Clone, Serialize, Deserialize)]
pub enum DataAvailabilityOptions {
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
