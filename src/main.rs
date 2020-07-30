#[doc(hidden)]
mod cli;

extern crate storykeep as sk_api;

#[doc(hidden)]
fn main() {
    let matches = cli::get_cli_app().get_matches();

    let _keep_config = sk_api::get_keep_config();

    let _app_config = sk_api::get_app_config_or_create();

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
        (cli::constants::CONFIG_SUBCMD, Some(_sub_matches)) => {
            // TODO
            println!("Config subcommand!");
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
