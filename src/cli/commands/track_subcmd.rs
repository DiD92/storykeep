use crate::cli::constants::{TRACK_SUBCMD, TRACK_SUBCMD_ALIAS};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_track_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(TRACK_SUBCMD)
        .visible_alias(TRACK_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Adds a file to the keep's tracker")
}
