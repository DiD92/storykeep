use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_compare_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(crate::cli::constants::COMPARE_SUBCMD)
        .visible_alias(crate::cli::constants::COMPARE_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Compares different revisions of the keep")
}
