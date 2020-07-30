mod compare_subcmd;
mod config_subcmd;
mod init_subcmd;
mod state_subcmd;
mod track_subcmd;

pub fn get_commands<'a, 'b>() -> Vec<clap::App<'a, 'b>> {
    let mut command_vector = Vec::with_capacity(4);

    command_vector.push(init_subcmd::get_init_subcommand());
    command_vector.push(state_subcmd::get_state_subcommand());
    command_vector.push(track_subcmd::get_track_subcommand());
    command_vector.push(compare_subcmd::get_compare_subcommand());
    command_vector.push(config_subcmd::get_config_subcommand());

    command_vector
}
