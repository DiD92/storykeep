use clap::ArgMatches;

#[doc(hidden)]
mod cli;

extern crate storykeep as sk_api;

#[doc(hidden)]
fn main() {
    let matches = cli::get_cli_app().get_matches();

    match matches.subcommand() {
        (cli::constants::INIT_SUBCMD, Some(sub_matches)) => {
            process_init_subcommand(sub_matches.is_present(cli::constants::INIT_CHECK_ONLY));
        }
        (cli::constants::STATE_SUBCMD, Some(_sub_matches)) => {
            // TODO
            println!("State subcommand!");
        }
        (cli::constants::TRACK_SUBCMD, Some(_sub_matches)) => {
            // TODO
            println!("Track subcommand!");
        }
        (cli::constants::COMPARE_SUBCMD, Some(_sub_matches)) => {
            // TODO
            println!("Compare subcommand!");
        }
        (cli::constants::CONFIG_SUBCMD, Some(sub_matches)) => {
            let file_location = sub_matches.value_of(cli::constants::CONFIG_FILE_LOCATION);
            let config_action = sub_matches.subcommand();
            process_config_subcommand(file_location, config_action);
        }
        (_, Some(&_)) => {}
        (_, None) => {
            panic!("No valid subcommand parsed!");
        }
    }
}

fn process_init_subcommand(check_only: bool) {
    if check_only {
        if sk_api::can_initialize_keep() {
            println!("Keep can be created at current working directory!");
        } else {
            println!("Keep already present or invalid directory!");
        }
    } else {
        match sk_api::initialize_keep() {
            Ok(message) => println!("{}", message),
            Err(message) => println!("{}", message),
        }
    }
}

fn process_config_subcommand(file_location: Option<&str>, subcommand: (&str, Option<&ArgMatches>)) {
    use sk_api::ConfigKind;

    match file_location {
        Some(cli::constants::CONFIG_FILE_LOCATION_AUTO) => {
            if let Some(keep_config) = sk_api::get_keep_config() {
                match subcommand {
                    (cli::constants::CONFIG_LIST_SUBCMD, _) => print!("{}", keep_config),
                    (cli::constants::CONFIG_GET_SUBCMD, Some(sub_matches)) => {
                        if let Some(config_key) =
                            sub_matches.value_of(cli::constants::CONFIG_SUBCMD_KEY_ARG)
                        {
                            match keep_config.get_value(config_key) {
                                Some(value) => {
                                    if sub_matches
                                        .is_present(cli::constants::CONFIG_GET_SIMPLE_FLAG)
                                    {
                                        println!("{}", value);
                                    } else {
                                        println!("{} => {}", config_key, value);
                                    }
                                }
                                None => eprintln!("Key {} not found in configuration!", config_key),
                            }
                        }
                    }
                    (cli::constants::CONFIG_SET_SUBCMD, sub_matches) => {}
                    _ => {}
                }
            } else if let Some(app_config) = sk_api::get_app_config() {
                match subcommand {
                    (cli::constants::CONFIG_LIST_SUBCMD, _) => print!("{}", app_config),
                    (cli::constants::CONFIG_GET_SUBCMD, Some(sub_matches)) => {
                        if let Some(config_key) =
                            sub_matches.value_of(cli::constants::CONFIG_SUBCMD_KEY_ARG)
                        {
                            match app_config.get_value(config_key) {
                                Some(value) => {
                                    if sub_matches
                                        .is_present(cli::constants::CONFIG_GET_SIMPLE_FLAG)
                                    {
                                        println!("{}", value);
                                    } else {
                                        println!("{} => {}", config_key, value);
                                    }
                                }
                                None => eprintln!("Key {} not found in configuration!", config_key),
                            }
                        }
                    }
                    (cli::constants::CONFIG_SET_SUBCMD, sub_matches) => {}
                    _ => {}
                }
            } else {
                eprintln!("No valid configuration file found!")
            }
        }
        Some(cli::constants::CONFIG_FILE_LOCATION_APP) => {
            if let Some(config) = sk_api::get_app_config() {
                match subcommand {
                    (cli::constants::CONFIG_LIST_SUBCMD, _) => print!("{}", config),
                    (cli::constants::CONFIG_GET_SUBCMD, Some(sub_matches)) => {
                        if let Some(config_key) =
                            sub_matches.value_of(cli::constants::CONFIG_SUBCMD_KEY_ARG)
                        {
                            match config.get_value(config_key) {
                                Some(value) => {
                                    if sub_matches
                                        .is_present(cli::constants::CONFIG_GET_SIMPLE_FLAG)
                                    {
                                        println!("{}", value);
                                    } else {
                                        println!("{} => {}", config_key, value);
                                    }
                                }
                                None => eprintln!("Key {} not found in configuration!", config_key),
                            }
                        }
                    }
                    (cli::constants::CONFIG_SET_SUBCMD, sub_matches) => {}
                    _ => {}
                }
            } else {
                eprintln!("No valid app configuration file found!");
            }
        }
        Some(cli::constants::CONFIG_FILE_LOCATION_KEEP) => {
            if let Some(config) = sk_api::get_keep_config() {
                match subcommand {
                    (cli::constants::CONFIG_LIST_SUBCMD, _) => print!("{}", config),
                    (cli::constants::CONFIG_GET_SUBCMD, Some(sub_matches)) => {
                        if let Some(config_key) =
                            sub_matches.value_of(cli::constants::CONFIG_SUBCMD_KEY_ARG)
                        {
                            match config.get_value(config_key) {
                                Some(value) => {
                                    if sub_matches
                                        .is_present(cli::constants::CONFIG_GET_SIMPLE_FLAG)
                                    {
                                        println!("{}", value);
                                    } else {
                                        println!("{} => {}", config_key, value);
                                    }
                                }
                                None => eprintln!("Key {} not found in configuration!", config_key),
                            }
                        }
                    }
                    (cli::constants::CONFIG_SET_SUBCMD, sub_matches) => {}
                    _ => {}
                }
            } else {
                eprintln!("No valid keep configuration file found!");
            }
        }
        _ => {}
    }
}
