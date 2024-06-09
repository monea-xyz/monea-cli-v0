use crate::cli::TemplateCommands;
use crate::utils::{create_template, delete_template, list_templates};

pub fn handle_template_commands(cmd: &TemplateCommands) {
    match cmd {
        TemplateCommands::New(args) => create_template(args),
        TemplateCommands::List => list_templates(),
        TemplateCommands::Delete(args) => delete_template(args),
    }
}
