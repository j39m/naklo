mod control_file;
mod song;
mod tag;
mod value;

use clap::Parser;
use std::path::PathBuf;
use thiserror::Error;

use crate::control_file::{ControlError, MapsToSongs};
use crate::song::LintResult;

#[derive(Parser)]
#[command(version)]
struct Cli {
    /// 1 or more YAML files describing tagging recipe
    #[arg(short, long, required = true)]
    control_files: Vec<PathBuf>,

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

#[derive(Error, Debug)]
pub enum ErrorPassthrough {
    #[error(transparent)]
    ControlFile(#[from] ControlError),
    #[error(transparent)]
    Song(#[from] crate::song::SongError),
}

fn main_impl(cli: Cli) -> Result<(), ErrorPassthrough> {
    let controls = crate::control_file::from(&cli.control_files)?;
    let mut songs = crate::song::from(&cli.files)?;
    controls
        .into_iter()
        .map(|c| c.map_to(&mut songs))
        .collect::<Result<(), ControlError>>()?;
    let lint = LintResult::from(&songs);
    if cli.dewit {
        todo!();
    }
    lint.title_lints.into_iter().for_each(|e| println!("{e}"));
    lint.homogeneity_lints
        .into_iter()
        .for_each(|e| println!("{e}"));
    println!("{}", lint.homogeneous_tags);
    songs.into_iter().for_each(|s| println!("{s}"));
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    assert!(!cli.dewit);
    if let Err(e) = main_impl(cli) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
