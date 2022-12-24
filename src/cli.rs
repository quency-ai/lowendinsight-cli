use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {
    /// Run LowEndInsight against a git repository
    Analyze {
        /// the remote git repo to analyze
        #[structopt()]
        url: String,
        /// Adjust level of stdout, 0 is no output, max 2 (debug)
        #[structopt(short, long, default_value="1")]
        verbosity: u8,
    }
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "lei",
    about = "A command-line interface to LowEndInsight (https://lowendinsight.dev"
)]

pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,
}