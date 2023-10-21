use att_cli::commands::{self, att_command, AttCommand, AttContext};
use bpaf::{Args, ParseFailure};
use std::process::ExitCode;

fn run(command: AttCommand) {
    let context = AttContext {};

    match command {
        AttCommand::Status => commands::status::execute(&context),
        AttCommand::Run => commands::run::execute(&context),
    }
}

fn main() -> ExitCode {
    let command = att_command().run_inner(Args::current_args());

    match command {
        Ok(command) => {
            run(command);
            ExitCode::SUCCESS
        }
        Err(failure) => {
            return match &failure {
                ParseFailure::Stdout(out, _) => {
                    println!("{}", out.to_string());
                    ExitCode::SUCCESS
                }
                ParseFailure::Completion(out) => {
                    println!("{}", out);
                    ExitCode::SUCCESS
                }
                ParseFailure::Stderr(err) => {
                    println!("error: {:?}", err.to_string());
                    ExitCode::FAILURE
                }
            };
        }
    }
}
