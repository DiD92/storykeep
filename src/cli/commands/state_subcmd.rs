use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_state_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(crate::cli::constants::STATE_SUBCMD)
        .visible_alias(crate::cli::constants::STATE_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Show various reports about the keep's state")
}
