use crate::cli::constants::{
    CONFIG_FILE_LOCATION, CONFIG_FILE_LOCATION_APP, CONFIG_FILE_LOCATION_ARG,
    CONFIG_FILE_LOCATION_AUTO, CONFIG_FILE_LOCATION_KEEP, CONFIG_GET_ALIAS, CONFIG_GET_SIMPLE_FLAG,
    CONFIG_GET_SIMPLE_FLAG_LONG, CONFIG_GET_SIMPLE_FLAG_SHORT, CONFIG_GET_SUBCMD,
    CONFIG_LIST_ALIAS, CONFIG_LIST_SUBCMD, CONFIG_SET_ALIAS, CONFIG_SET_SUBCMD, CONFIG_SUBCMD,
    CONFIG_SUBCMD_ALIAS, CONFIG_SUBCMD_KEY_ARG, CONFIG_SUBCMD_VALUE_ARG,
};
use clap::{App, AppSettings, Arg, SubCommand};

pub fn get_config_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_SUBCMD)
        .visible_alias(CONFIG_SUBCMD_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Interact with the app's or the keep's settings")
        .arg(
            Arg::with_name(CONFIG_FILE_LOCATION)
                .help(
                    "Which config file to target, by default \
                    targets the keep's config if withim a keep\n\
                    otherwise targets the app config.",
                )
                .next_line_help(true)
                .long(CONFIG_FILE_LOCATION_ARG)
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
        .display_order(2)
}

fn get_list_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_LIST_SUBCMD)
        .visible_alias(CONFIG_LIST_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("List settings for config file")
        .display_order(1)
}

fn get_get_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_GET_SUBCMD)
        .visible_alias(CONFIG_GET_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Get value of setting for config file")
        .arg(
            Arg::with_name(CONFIG_SUBCMD_KEY_ARG)
                .help("Configuration key to check the value for")
                .required(true),
        )
        .arg(
            Arg::with_name(CONFIG_GET_SIMPLE_FLAG)
                .help("Print the key value only")
                .long(CONFIG_GET_SIMPLE_FLAG_LONG)
                .short(CONFIG_GET_SIMPLE_FLAG_SHORT),
        )
        .display_order(2)
}

fn get_set_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(CONFIG_SET_SUBCMD)
        .visible_alias(CONFIG_SET_ALIAS)
        .setting(AppSettings::DisableVersion)
        .about("Set value of setting for config file")
        .arg(
            Arg::with_name(CONFIG_SUBCMD_KEY_ARG)
                .index(1)
                .help("Configuration key to set the value for")
                .required(true),
        )
        .arg(
            Arg::with_name(CONFIG_SUBCMD_VALUE_ARG)
                .index(2)
                .help("Value to set to the configuration key")
                .required(true),
        )
        .display_order(3)
}
