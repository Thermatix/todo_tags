use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};

pub fn new() -> App {
    App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("tdt [command]")
        .command(
            Command::new("add")
            .description("Add a project folder")
            .alias("a")
            .usage("tdt add(a) [project/folder] (name)")
            .action(add_action)
        ).command(
            Command::new("edit")
            .description("Edit project meta-data")
            .alias("e")
            .usage("tdt edit(e) [project name]")
            .action(edit_action)
        ).command(
            Command::new("display")
            .description("Display the project todo list")
            .alias("d")
            .usage("tdt display(d) [project name]")
            .action(display_action)
        ).command(
            Command::new("write")
            .description("Write a given item")
            .alias("w")
            .usage("tdt write(w) [item] [description]")
            .action(write_action)
        ).command(
            Command::new("remove")
            .description("Remove the given item")
            .alias("r")
            .usage("tdt remove(r) [item]")
            .action(remove_action)
        ).command(
            Command::new("show")
            .description("Show the given item")
            .alias("s")
            .usage("tdt show [item]")
            .action(show_action)
        )
}
fn add_action(c: &Context) {}
fn edit_action(c: &Context) {}
fn display_action(c: &Context) {}
fn write_action(c: &Context) {}
fn remove_action(c: &Context) {}
fn show_action(c: &Context) {}
