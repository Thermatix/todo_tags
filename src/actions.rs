use super::clap::ArgMatches;
use super::gprc::*;

#[derive(Debug)]
pub enum Action {
    Add(Project),
    Edit(Project),
    Display(Project),
    Write(Item),
    Remove(Item),
    Show(Item),
    Find(Query),
}

impl<'a> From<(&'a str, Option<&ArgMatches<'a >>)> for Action {
    fn from((cmd, args): (&'a str, Option<&ArgMatches<'a>>)) -> Self {
        if let Some(args) = args {
            match cmd {
                "add" => {
                    Self::Add(
                        (
                            args.value_of("FOLDER").unwrap().to_owned(),
                            args.value_of("NAME").unwrap_or(args.value_of("FOLDER").unwrap()).to_owned()
                        ).into()
                        )
                },               "edit" => {
                    Self::Edit(
                        (
                            args.value_of("FOLDER").unwrap().to_owned(),
                            args.value_of("NAME").unwrap_or(args.value_of("FOLDER").unwrap()).to_owned()
                        ).into()
                        )
                },
                "display" => {
                    Self::Display(
                        (
                            args.value_of("FOLDER").unwrap().to_owned(),
                            args.value_of("NAME").unwrap_or(args.value_of("FOLDER").unwrap()).to_owned()
                        ).into()
                        )
                },
                "write" => {
                    Self::Write(
                        (
                            args.value_of("NAME").unwrap().to_owned(),
                            args.value_of("description").unwrap().to_owned(),
                            args.value_of("PATH").unwrap().to_owned(),

                        ).into()
                        )
                },               "remove" => {
                    Self::Remove(
                        (
                            args.value_of("NAME").unwrap().to_owned(),
                            "".to_owned(),
                            args.value_of("PATH").unwrap().to_owned(),

                        ).into()
                        )
                },               "show" => {
                    Self::Show(
                        (
                            args.value_of("NAME").unwrap().to_owned(),
                            "".to_owned(),
                            args.value_of("PATH").unwrap().to_owned(),
                        ).into()
                        )
                },               "find" => {
                    Self::Find(
                        args.value_of("QUERY").unwrap().to_owned().into()
                        )
                },
                _ => { panic!("Unknown action, given {}", cmd) }
            }
        } else {
            panic!("No args found!")
        }
    }
}
