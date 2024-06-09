use crate::args::{DeleteTemplateArgs, NewTemplateArgs};
use crate::utils::config::Config;
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::home_dir;
use serde_yaml;
use std::fs;
use std::path::PathBuf;

pub fn get_global_templates_dir() -> PathBuf {
    let mut dir = home_dir().expect("Could not find home directory");
    dir.push(".monea-cli/templates");
    dir
}

pub fn select_template() -> Result<String, String> {
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

pub fn create_template(args: &NewTemplateArgs) {
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

    crate::utils::config::print_config(&format!("Created a new template '{}'", args.name), &config);
}

pub fn list_templates() {
    let folder_path = get_global_templates_dir();

    if !folder_path.exists() {
        println!("No templates directory found.");
        return;
    }

    let mut table = prettytable::Table::new();
    table.add_row(prettytable::row![
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

            table.add_row(prettytable::row![
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

pub fn delete_template(args: &DeleteTemplateArgs) {
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
