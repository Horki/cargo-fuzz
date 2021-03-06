use crate::{options::BuildOptions, project::FuzzProject, RunCommand};
use anyhow::Result;
use structopt::StructOpt;

use std::path::PathBuf;

#[derive(Clone, Debug, StructOpt)]
pub struct Fmt {
    #[structopt(flatten)]
    pub build: BuildOptions,

    /// Name of fuzz target
    pub target: String,

    /// Path to the input testcase to debug print
    pub input: PathBuf,
}

impl RunCommand for Fmt {
    fn run_command(&mut self) -> Result<()> {
        let project = FuzzProject::find_existing()?;
        project.debug_fmt_input(self)
    }
}
