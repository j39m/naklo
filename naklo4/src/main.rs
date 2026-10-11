mod control_file;
mod value;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// 1 or more YAML files describing tagging recipe
    #[arg(short, long, required = true)]
    control_file: Vec<PathBuf>,

    /// Preserve existing tags
    #[arg(short, long, default_value = "false")]
    preserve_existing: bool,

    /// Commit tags (default action is dry-run)
    #[arg(long, default_value = "false")]
    dewit: bool,

    /// Tag only these files
    #[arg(long)]
    only: Option<String>,

    /// Files to tag
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    assert!(!cli.dewit);
}
