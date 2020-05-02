use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_init_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(crate::cli::constants::INIT_SUBCMD)
        .visible_alias(crate::cli::constants::INIT_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Initializes a new storykeep keep")
        .arg(
            Arg::with_name(crate::cli::constants::INIT_CHECK_ONLY)
                .help("Only check if keep can be created")
                .long("check")
                .short("c"),
        )
}
