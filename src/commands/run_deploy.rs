use crate::args::{CommonArgs, RunDeployArgs};
use crate::utils::config::Config;
use crate::utils::{load_template, print_config, select_template};

pub fn run_command(args: &RunDeployArgs) {
    if args.template.is_none()
        && args.positional_templates.is_empty()
        && no_common_args_provided(&args.common)
    {
        let selected_template = select_template().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        run_command_with_template(Some(&selected_template), &args.common);
    } else {
        execute_command(args, run_command_with_template);
    }
}

pub fn deploy_command(args: &RunDeployArgs) {
    if args.template.is_none()
        && args.positional_templates.is_empty()
        && no_common_args_provided(&args.common)
    {
        let selected_template = select_template().unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        deploy_command_with_template(Some(&selected_template), &args.common);
    } else {
        execute_command(args, deploy_command_with_template);
    }
}

fn no_common_args_provided(common: &CommonArgs) -> bool {
    common.execution.is_none()
        && common.settlement.is_none()
        && common.sequencer.is_none()
        && common.data_availability.is_none()
}

fn execute_command(args: &RunDeployArgs, command_fn: fn(Option<&str>, &CommonArgs)) {
    let template_names = args.template.clone().unwrap_or_else(Vec::new);
    let template_names = [&template_names[..], &args.positional_templates[..]].concat();

    if template_names.is_empty() {
        command_fn(None, &args.common);
    } else {
        for template_name in template_names {
            command_fn(Some(&template_name), &args.common);
        }
    }
}

fn run_command_with_template(template_name: Option<&str>, common: &CommonArgs) {
    let config = match template_name {
        Some(name) => load_template(name).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        }),
        None => Default::default(), // Create a default config if no template is provided
    };

    let config = apply_common_args(config, common);

    print_config(
        &format!(
            "Running a local Docker setup with template '{}':",
            template_name.unwrap_or(""),
        ),
        &config,
    );
}

fn deploy_command_with_template(template_name: Option<&str>, common: &CommonArgs) {
    let config = match template_name {
        Some(name) => load_template(name).unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        }),
        None => Default::default(), // Create a default config if no template is provided
    };

    let config = apply_common_args(config, common);

    print_config(
        &format!(
            "Deploying to Monea Cloud with template '{}':",
            template_name.unwrap_or(""),
        ),
        &config,
    );
}

fn apply_common_args(mut config: Config, common: &CommonArgs) -> Config {
    config.execution = common.execution.clone().unwrap_or_default();
    config.settlement = common.settlement.clone().unwrap_or_default();
    config.sequencer = common.sequencer.clone().unwrap_or_default();
    config.data_availability = common.data_availability.clone().unwrap_or_default();
    config
}
