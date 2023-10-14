use std::process;

use crate::workspace::Workspace;

pub fn execute(workspace: Workspace) {
    let run_config = &workspace.config.run;
    let mut child = process::Command::new(&run_config.program)
        .args(&run_config.args)
        .spawn()
        .expect("failed to execute process");
    child.wait().unwrap();
}
