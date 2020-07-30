use crate::cli::constants::{CONFIG_FILE_LOCATION, CONFIG_SUBCMD, CONFIG_SUBCMD_ALIAS};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_config_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_SUBCMD)
        .visible_alias(CONFIG_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Configure app or keep settings")
        .arg(
            Arg::with_name(CONFIG_FILE_LOCATION)
                .help("Which config file to edit")
                .long("location")
                .takes_value(true)
                .possible_values(&["app", "keep", "auto"])
                .default_value("auto"),
        )
}
