pub mod clean;
pub mod release;
pub mod report;

use clean::Clean;
use release::Release;
use report::Report;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "Git Cli helpers", rename_all = "kebab-case")]
pub enum Git {
    Clean(Clean),
    Release(Release),
    Report(Report),
}

impl Git {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match self {
            Git::Clean(clean) => clean.run(),
            Git::Release(release) => release.run(),
            Git::Report(report) => report.run(),
        }
    }
}
