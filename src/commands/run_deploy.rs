use crate::args::RunDeployArgs;
use crate::utils::{load_template, print_config, select_template};

pub fn run_command(args: &RunDeployArgs) {
    if args.template.is_none() && args.positional_templates.is_empty() {
        let selected_template = select_template().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        run_command_with_template(&selected_template);
    } else {
        execute_command(args, run_command_with_template);
    }
}

pub fn deploy_command(args: &RunDeployArgs) {
    if args.template.is_none() && args.positional_templates.is_empty() {
        let selected_template = select_template().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        deploy_command_with_template(&selected_template);
    } else {
        execute_command(args, deploy_command_with_template);
    }
}

fn execute_command(args: &RunDeployArgs, command_fn: fn(&str)) {
    let template_names = args.template.clone().unwrap_or_default();
    let template_names = [&template_names[..], &args.positional_templates[..]].concat();

    for template_name in template_names {
        command_fn(&template_name);
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
