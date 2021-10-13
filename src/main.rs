#[macro_use]
mod macros;
mod gprc;
mod server;
mod client;

mod envs;

use clap::{self, Arg, App};
use color_eyre::{self, Report};
use tracing::info;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    setup()?;
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .setting(clap::AppSettings::ArgRequiredElseHelp)
                        .setting(clap::AppSettings::ColoredHelp)
                        .setting(clap::AppSettings::DeriveDisplayOrder)
                        .setting(clap::AppSettings::GlobalVersion)
                        .setting(clap::AppSettings::VersionlessSubcommands)
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .usage("tdt [command]")
                        .subcommand(
                            App::new("add")
                            .about("Add a project folder")
                            .alias("a")
                            .usage("tdt add(a) [project/folder] (name)")
                            .arg(Arg::with_name("FOLDER")
                                    .help("Path to the project folder")
                                    .required(true)
                                    .index(1))
                            .arg(Arg::with_name("NAME")
                                    .help("Name of the project")
                                    .required(false)
                                    .index(2))
                        ).subcommand(
                            App::new("edit")
                            .about("Edit project meta-data")
                            .alias("e")
                            .usage("tdt edit(e) [project name]")
                        ).subcommand(
                            App::new("display")
                            .about("Display the project todo list")
                            .alias("d")
                            .usage("tdt display(d) [project name]")
                        ).subcommand(
                            App::new("write")
                            .about("Write a given item")
                            .alias("w")
                            .usage("tdt write(w) [item] [about]")
                        ).subcommand(
                            App::new("remove")
                            .about("Remove the given item")
                            .alias("r")
                            .usage("tdt remove(r) [item]")
                        ).subcommand(
                            App::new("show")
                            .about("Show the given item")
                            .alias("s")
                            .usage("tdt show [item]")
                        ).get_matches();

    let mut server = server::Server::new();
    server.start().await;
    Ok(())
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

async fn add_action(c: &Context) {
    let mut client = Client::new().await; 
    let  folder_name =
    match (c.args.get(0), c.args.get(1)) {
        (Some(folder), Some(name)) => (folder.clone(), name.clone()),
        (Some(folder), None) => (folder.clone(), folder.clone()),
        _ => panic!("No folder or folder & project name provided")
    };
    match client.request(Action::Add(folder_name.into())).await {
        Ok(r) => {
            let response: data_types::Response = r.into();
        }
    };
}
fn edit_action(c: &Context) {}
fn display_action(c: &Context) {}
fn write_action(c: &Context) {}
fn remove_action(c: &Context) {}
fn show_action(c: &Context) {}
