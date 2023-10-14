pub mod run;
pub mod status;

use bpaf::Bpaf;

use crate::VERSION;

#[derive(Debug, Clone, Bpaf)]
pub struct CliOptions {
    #[bpaf(long("verbose"), fallback(false))]
    pub verbose: bool,
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options, version(VERSION))]
pub enum AttCommand {
    #[bpaf(command("status"))]
    Status,

    #[bpaf(command("run"), short('r'))]
    Run,
}
