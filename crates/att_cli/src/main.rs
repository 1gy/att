use att_cli::{
    commands::{self, att_command, AttCommand, AttContext},
    error::CommandError,
};
use bpaf::{Args, ParseFailure};
use std::process::ExitCode;

fn run(command: AttCommand) -> Result<(), CommandError> {
    let context = AttContext {};

    match command {
        AttCommand::Status => commands::status::execute(&context).await?,
        AttCommand::Run => commands::run::execute(&context).await?,
        AttCommand::Init => commands::init::execute(&context).await?,
        AttCommand::New { url } => commands::new::execute(&context, &url).await?,
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let command = att_command().run_inner(Args::current_args());

    match command {
        Ok(command) => {
            return match run(command).await {
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    println!("error: {:?}", err.to_string());
                    ExitCode::FAILURE
                }
            }
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
