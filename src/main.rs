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
    use std::env::current_dir;
    setup()?;
    let cwd = current_dir()?.to_str().unwrap().to_owned();
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .setting(clap::AppSettings::ArgRequiredElseHelp)
                        .setting(clap::AppSettings::ColoredHelp)
                        .setting(clap::AppSettings::DeriveDisplayOrder)
                        .setting(clap::AppSettings::GlobalVersion)
                        .setting(clap::AppSettings::VersionlessSubcommands)
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .subcommand(
                            App::new("add")
                                .about("Add a project folder")
                                .alias("a")
                                .arg(Arg::with_name("FOLDER")
                                     .help("Path to the project folder")
                                     .required(true)
                                     .index(1))
                                .arg(Arg::with_name("NAME")
                                     .help("Name of the project, will default to folder name")
                                     .required(false)
                                     .index(2))
                        ).subcommand(
                            App::new("edit")
                                .about("Edit project meta-data")
                                .alias("e")
                                .arg(Arg::with_name("FOLDER")
                                     .help("Path to the project folder")
                                     .required(true)
                                     .index(1))
                                .arg(Arg::with_name("NAME")
                                     .help("Name of the project, will default to folder name")
                                     .required(false)
                                     .index(2))
                        ).subcommand(
                            App::new("display")
                                .about("Display the project todo list")
                                .alias("d")
                                .arg(Arg::with_name("FOLDER")
                                     .help("Path to the project folder")
                                     .required(true)
                                     .index(1))
                                .arg(Arg::with_name("NAME")
                                     .help("Name of the project, will default to folder name")
                                     .required(false)
                                     .index(2))
                        ).subcommand(
                                App::new("write")
                                .about("Write a given item")
                                .alias("w")
                                .arg(Arg::with_name("NAME")
                                     .help("Tag Name")
                                     .required(true)
                                     .index(1))
                                .arg(Arg::with_name("DESCRIPTION")
                                     .help("Tag Description")
                                     .required(true)
                                     .index(2))
                                .arg(Arg::with_name("PATH")
                                     .hidden(true)
                                     .help("File Path of project the tag belongs to")
                                     .long("File Path of project the tag belongs to, defaults to pwd")
                                     .default_value(&cwd)
                                     .required(false)
                                     .index(3))
                        ).subcommand(
                            App::new("remove")
                            .about("Remove the given item")
                            .alias("r")
                            .arg(Arg::with_name("NAME")
                                 .help("Tag Name")
                                 .required(true)
                                 .index(1))
                            .arg(Arg::with_name("PATH")
                                     .hidden(true)
                                     .help("File Path of project the tag belongs to")
                                     .long("File Path of project the tag belongs to, defaults to pwd")
                                     .default_value(&cwd)
                                     .required(false)
                                     .index(3))
                        ).subcommand(
                            App::new("show")
                            .about("Show the given item")
                            .alias("s")
                            .arg(Arg::with_name("NAME")
                                 .help("Tag Name")
                                 .required(true)
                                 .index(1))
                            .arg(Arg::with_name("PATH")
                                     .hidden(true)
                                     .help("File Path of project the tag belongs to")
                                     .long("File Path of project the tag belongs to, defaults to pwd")
                                     .default_value(&cwd)
                                     .required(false)
                                     .index(3))
                        .subcommand(
                            App::new("find")
                            .about("Find a tag and display any matching tags")
                            .alias("f")
                            .arg(Arg::with_name("QUERY")
                                 .help("Pattern to match against Tag NAMEs")
                                 .required(true)
                                 .index(1)
                                 )
                            )
                        ).get_matches();

    server::Server::new().start().await;

    let mut client = client::Client::new().await; 

    match client.request(matches.subcommand().into()).await {
        Ok(r) => {
            let response  = r;
            println!("RESPONE: {:#?}", response);
        },
        Err(e) => {
            eprintln!("Unable to make request to TDT gRPC server, {}", e);
        }
    };

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

