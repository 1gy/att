use crate::{contest::Contest, workspace::Workspace};

pub fn execute(workspace: Workspace, contest: Contest) {
    println!("{:?}", workspace);
    println!("{:?}", contest);
}
