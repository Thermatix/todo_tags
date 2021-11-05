tonic::include_proto!("todo_tags");

use super::clap::ArgMatches;

pub mod data_types {
    pub use super::{Item, Items, Query, Project,  ResType, Response};
}

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


pub mod primatives {
    pub trait Conversion {}

    pub type Item = (String, String, String);
    pub type ItemsWithCount = (u64, Vec<Item>);
    pub type ItemsWithOutCount = Vec<Item>;
    pub type FolderName = (String, String);
    pub type FolderNameItems = (String, String, ItemsWithCount);
    pub type Query = String;

    impl From<&Query> for super::Query {
        fn from(v: &Query) -> Self {
            Self {
                tag_query: v.clone(),
            }
        }
    }

    impl Conversion for super::Query {}
    impl Conversion for super::Project {}
    impl Conversion for super::Item {}
    impl Conversion for super::Items {}
    impl Conversion for super::Response {}

    impl From<Query> for super::Query {
        fn from(v: Query) -> Self {
            Self {
                tag_query: v,
            }
        }
    }

    impl From<&FolderNameItems> for  super::Project {
        fn from(v: &FolderNameItems) -> Self {

            Self {
                name: v.0.clone(),
                folder: v.1.clone(),
                items: Some(v.2.clone().into()),
            }
        }
    }

    impl From<FolderNameItems> for  super::Project {
        fn from(v: FolderNameItems) -> Self {

            Self {
                name: v.0,
                folder: v.1,
                items: Some(v.2.into()),
            }
        }
    }

    impl From<FolderName> for  super::Project {
        fn from(v: FolderName) -> Self {

            Self {
                name: v.0,
                folder: v.1,
                items: Some(super::Items::default()),
            }
        }
    }

    impl From<&FolderName> for  super::Project {
        fn from(v: &FolderName) -> Self {

            Self {
                name: v.0.clone(),
                folder: v.1.clone(),
                items: Some(super::Items::default()),
            }
        }
    }

    impl From<Item> for super::Item {
        fn from(v: Item) -> Self {
            Self {
                tag: v.0,
                description: v.1,
                project_path: v.2, 
                file_paths: Vec::new(),
            }
        }
    }

    impl From<&Item> for super::Item {
        fn from(v: &Item) -> Self {
            Self {
                tag: v.0.clone(),
                description: v.1.clone(),
                project_path: v.2.clone(), 
                file_paths: Vec::new(),
            }
        }
    }


    impl From<ItemsWithCount> for super::Items {
        fn from(v: ItemsWithCount) -> Self {
            Self {
                count: v.0,
                values: v.1.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<&ItemsWithCount> for super::Items {
        fn from(v: &ItemsWithCount) -> Self {
            Self {
                count: v.0.clone(),
                values: v.1.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<ItemsWithOutCount> for super::Items {
        fn from(v: ItemsWithOutCount) -> Self {
            Self {
                count: v.len() as u64,
                values: v.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<&ItemsWithOutCount> for super::Items {
        fn from(v: &ItemsWithOutCount) -> Self {
            Self {
                count: v.len() as u64,
                values: v.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

}





