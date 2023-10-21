use std::process;

use crate::workspace::load_workspace;

use super::AttContext;

pub fn execute(_context: &AttContext) {
    let workspace = match load_workspace() {
        Some(workspace) => workspace,
        None => {
            println!("workspace not found");
            return;
        }
    };
    let run_config = &workspace.config.run;
    let mut child = process::Command::new(&run_config.program)
        .args(&run_config.args)
        .spawn()
        .expect("failed to execute process");
    child.wait().unwrap();
}
