use crate::args::{DataAvailabilityOptions, ExecutionOptions, SequencerOptions, SettlementOptions};
use prettytable::{row, Table};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub template_name: Option<String>,
    pub execution: ExecutionOptions,
    pub settlement: SettlementOptions,
    pub sequencer: SequencerOptions,
    pub data_availability: DataAvailabilityOptions,
}

pub fn load_template(template_name: &str) -> Result<Config, String> {
    let file_path =
        crate::utils::template::get_global_templates_dir().join(format!("{}.yaml", template_name));
    if !file_path.exists() {
        return Err(format!("No template named '{}' exists", template_name));
    }

    let yaml = std::fs::read_to_string(file_path)
        .map_err(|err| format!("Failed to read template: {}", err))?;
    let mut config: Config =
        serde_yaml::from_str(&yaml).map_err(|err| format!("Failed to parse template: {}", err))?;
    config.template_name = Some(template_name.to_string());
    Ok(config)
}

pub fn print_config(title: &str, config: &Config) {
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
