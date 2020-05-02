mod cli;

extern crate storykeep as sk_api;

fn main() {
    let matches = cli::get_cli_app().get_matches();

    match matches.subcommand() {
        (cli::constants::INIT_SUBCMD, Some(sub_matches)) => {
            process_init_subcommand(sub_matches.is_present(cli::constants::INIT_CHECK_ONLY));
        }
        (_, Some(&_)) => {}
        (_, None) => {
            panic!("No valid subcommand parsed!");
        }
    }
}

fn process_init_subcommand(check_only: bool) {
    if check_only {
        if sk_api::can_initialize_keep_at_wd() {
            println!("Keep can be created at current working directory!");
        } else {
            println!("Keep already present or invalid directory!");
        }
    } else {
        match sk_api::initialize_keep() {
            Ok(message) => println!("{}", message),
            Err(message) => eprintln!("{}", message),
        }
    }
}
