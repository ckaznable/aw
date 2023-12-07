use clap::Parser;

/// dynamic color wall in terminal
#[derive(Parser, Debug, Clone, Copy)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// reading global keyboard keypress (this options need root permission)
    #[arg(short, long)]
    pub global: bool,

    /// only execute the focused window
    #[arg(long, alias = "only-focused")]
    pub only_focused: bool
}
