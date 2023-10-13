use att_cli::{att_command, commands, load_contest, load_workspace, AttCommand};
use bpaf::{Args, ParseFailure};
use std::process::ExitCode;

fn run(command: AttCommand) {
    let workspace = match load_workspace() {
        Some(workspace) => workspace,
        None => {
            println!("workspace not found");
            return;
        }
    };

    let contest = match load_contest() {
        Some(contest) => contest,
        None => {
            println!("contest not found");
            return;
        }
    };

    match command {
        AttCommand::Status => commands::status::execute(workspace, contest),
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
