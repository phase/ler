extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("ler")
        .about("Distributed Version Control System")
        .subcommand(SubCommand::with_name("add")
            .about("add a file to the staging area")
            .arg(Arg::with_name("all")
                .short("A").long("all")
                .help("add all unstaged files"))
    ).get_matches();
}
