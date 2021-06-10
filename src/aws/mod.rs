pub mod pr;

use pr::Pr;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Aws Cli Shortcuts", rename_all = "kebab-case")]
pub enum Aws {
    Pr(Pr),
}

impl Aws {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Aws::Pr(pr) => pr.run(),
        }
    }
}
