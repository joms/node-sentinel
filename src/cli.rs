use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Node Sentinel")]
#[command(version = "1.0.0")]
#[command(about = "Does awesome things", long_about = None)]
pub struct Args {
    pub directory: Option<String>,

    #[arg(short, long)]
    pub auto_switch: bool,

    #[arg(short, long)]
    pub check_only: bool,
}

pub fn parse_cli_args() -> Args {
    Args::parse()
}
