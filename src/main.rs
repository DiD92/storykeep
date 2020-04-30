mod cli;

extern crate storykeep as sk_api;

fn main() {
    let matches = cli::get_cli_app().get_matches();

    match matches.subcommand_name() {
        Some(cli::constants::INIT_SUBCMD) => match sk_api::initialize_keep() {
            Ok(message) => println!("{}", message),
            Err(message) => eprintln!("{}", message),
        },
        Some(&_) => {}
        None => {
            panic!("No valid subcommand parsed!");
        }
    }
}
