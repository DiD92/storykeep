#[doc(hidden)]
mod cli;

mod hashing;

#[doc(hidden)]
fn main() {
    let matches = cli::get_cli_app().get_matches();

    match matches.subcommand_name() {
        Some(cli::constants::INIT_SUBCMD) => {
            println!("Initi repository");
            // TODO
        }
        Some(&_) => {}
        None => {
            panic!("No valid subcommand parsed!");
        }
    }
}
