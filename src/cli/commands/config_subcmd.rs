use crate::cli::constants::{
    CONFIG_FILE_LOCATION, CONFIG_FILE_LOCATION_APP, CONFIG_FILE_LOCATION_AUTO,
    CONFIG_FILE_LOCATION_KEEP, CONFIG_GET_ALIAS, CONFIG_GET_SUBCMD, CONFIG_LIST_ALIAS,
    CONFIG_LIST_SUBCMD, CONFIG_SET_ALIAS, CONFIG_SET_SUBCMD, CONFIG_SUBCMD, CONFIG_SUBCMD_ALIAS,
};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_config_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_SUBCMD)
        .visible_alias(CONFIG_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Configure app or keep settings")
        .arg(
            Arg::with_name(CONFIG_FILE_LOCATION)
                .help(
                    "Which config file to edit, \
                by default edit the keep config if withim a keep \
                otherwise edit the app config.",
                )
                .long("location")
                .takes_value(true)
                .possible_values(&[
                    CONFIG_FILE_LOCATION_APP,
                    CONFIG_FILE_LOCATION_KEEP,
                    CONFIG_FILE_LOCATION_AUTO,
                ])
                .default_value(CONFIG_FILE_LOCATION_AUTO),
        )
        .subcommand(get_list_subcommand())
        .subcommand(get_get_subcommand())
        .subcommand(get_set_subcommand())
}

fn get_list_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_LIST_SUBCMD)
        .visible_alias(CONFIG_LIST_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("List settings for config file")
}

fn get_get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_GET_SUBCMD)
        .visible_alias(CONFIG_GET_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Get value of setting for config file")
}

fn get_set_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_SET_SUBCMD)
        .visible_alias(CONFIG_SET_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Set value of setting for config file")
}
