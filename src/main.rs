#[macro_use]
extern crate clap;
use clap::{App, Arg, SubCommand};

mod persistence;
mod serialize;
mod todo;

fn main() {
    let app = App::new("Todo")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Manages your list of things to do")
        .subcommand(
            SubCommand::with_name("show")
                .about("display your todos")
                .arg(
                    Arg::with_name("all")
                        .short("a")
                        .long("all")
                        .required(false)
                        .help("display all todos"),
                ),
        )
        .subcommand(SubCommand::with_name("set").about("check or uncheck todos"))
        .subcommand(SubCommand::with_name("remove").about("permanently remove todos"))
        .subcommand(SubCommand::with_name("new").about("create new todo"))
        .subcommand(
            SubCommand::with_name("list")
                .about("display all available lists")
                .arg(
                    Arg::with_name("ratio")
                        .short("r")
                        .long("ratio")
                        .help("display ratio of completed and uncompleted todos in list"),
                ),
        )
        .get_matches();

    // let todos = persistence::load::<Vec<todo::Todo>>("todos").unwrap_or(Vec::new());
}
