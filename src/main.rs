mod command;
mod store;

use structopt::StructOpt;

fn main() {
    let opts = command::Opts::from_args();
    command::run_command(opts)
}
