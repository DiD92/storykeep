use crate::cli::constants::{COMPARE_SUBCMD, COMPARE_SUBCMD_ALIAS};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_compare_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(COMPARE_SUBCMD)
        .visible_alias(COMPARE_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Compares different revisions of the keep")
        .display_order(4)
}
