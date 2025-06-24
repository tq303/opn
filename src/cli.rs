use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "opn",
    version,
    about = "Context-aware file/folder command runner"
)]
#[command(help_template = "\
{before-help}{name} {version}

{about}

{usage-heading} {usage}

{all-args}{after-help}")]
pub struct Cli {
    /// The file/folder context
    #[arg(value_name = "CONTEXT")]
    pub context: String,

    /// The command to run for the context
    #[arg(value_name = "COMMAND")]
    pub command: Option<String>,

    /// The config file to use
    #[arg(short, long)]
    pub config: Option<String>,

    /// Prints command to be executed without running it
    #[arg(short, long)]
    pub print: bool,

    /// Add config with context and command
    #[arg(short, long, value_name = "ADD_TO_CONFIG")]
    pub add: Option<String>,
}

pub fn parse() -> Cli {
    Cli::parse()
}
