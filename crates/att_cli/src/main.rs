use att_cli::{att_command, load_workspace, AttCommand};
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
    println!("{:?}", workspace);
    println!("{:?}", command);
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
