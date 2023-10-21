use crate::{contest::load_contest, workspace::load_workspace};

use super::AttContext;

#[derive(thiserror::Error, Debug)]
#[error("status command error")]
pub struct StatusCommandError;

pub fn execute(_context: &AttContext) -> Result<(), StatusCommandError> {
    match load_workspace() {
        Some(workspace) => {
            println!("{:?}", workspace);
        }
        None => {
            println!("workspace not found");
        }
    };

    match load_contest() {
        Some(contest) => {
            println!("{:?}", contest);
        }
        None => {
            println!("contest not found");
        }
    };

    Ok(())
}
