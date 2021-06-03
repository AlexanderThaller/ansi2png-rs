use std::path::PathBuf;
use structopt::{
    clap::AppSettings::{
        ColoredHelp,
        GlobalVersion,
        NextLineHelp,
        VersionlessSubcommands,
    },
    StructOpt,
};

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(
    global_settings = &[ColoredHelp, VersionlessSubcommands, NextLineHelp, GlobalVersion]
)]
pub(super) struct Opt {
    /// Path to the input file. File should contain utf8 text that uses ANSI
    /// escape codes.
    #[structopt(short, long, default_value = "input.ansi")]
    pub(super) input_path: PathBuf,

    /// Path to output file. Will always write a png regardless of file
    /// extenstion.
    #[structopt(short, long, default_value = "output.png")]
    pub(super) output_path: PathBuf,
}
