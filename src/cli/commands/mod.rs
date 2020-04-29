mod init_subcmd;

pub fn get_commands<'a, 'b>() -> Vec<clap::App<'a, 'b>> {
    let mut command_vector = Vec::with_capacity(1);

    command_vector.push(init_subcmd::get_init_subcommand());

    command_vector
}
