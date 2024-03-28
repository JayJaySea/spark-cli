use std::process::ExitCode;
use console::style;
use clap::command;
use spark::{cli::{error::CliError, subcommands} , controller::Controller, init_db::setup_database};


fn main() -> ExitCode {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(subcommands::add())
        .subcommand(subcommands::list())
        .subcommand(subcommands::get())
        .subcommand(subcommands::update())
        .subcommand(subcommands::set())
        .get_matches();

    let conn = setup_database();
    let contr = Controller::new(conn);
    let result = contr.handle_command(matches);

    exit(result)
}

fn exit(result: Result<&'static str, CliError>) -> ExitCode {
    match result {
        Ok(message) => {
            if !message.is_empty() {
                eprintln!("{}", style(message).bold().green());
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}", style(error).bold().red());
            ExitCode::FAILURE
        }
    }
}
