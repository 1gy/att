use crate::{contest::load_contest, workspace::load_workspace};

use super::AttContext;

pub fn execute(_context: &AttContext) {
    let workspace = match load_workspace() {
        Some(workspace) => workspace,
        None => {
            println!("workspace not found");
            return;
        }
    };
    println!("{:?}", workspace);

    let contest = match load_contest() {
        Some(contest) => contest,
        None => {
            println!("contest not found");
            return;
        }
    };
    println!("{:?}", contest);
}
