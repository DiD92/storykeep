use crate::cli::constants::{STATE_SUBCMD, STATE_SUBCMD_ALIAS};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_state_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(STATE_SUBCMD)
        .visible_alias(STATE_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Show various reports about the keep's state")
}
