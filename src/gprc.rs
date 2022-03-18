tonic::include_proto!("todo_tags");

pub mod data_types {
    use std::fmt::{Display, Formatter, Result};
    pub use super::{Item, Items, Query, Project,  ResKind, Response};

    // const template_items;

    impl Display for Response {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{:?}: ${}", stringify!(self.kind), self.message)
        }
    }

    impl Display for Project {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, r#"
            # Project: {}

            ## Tags

            {}

            ## Items

            {}

            "#,
            self.name,
            self.items.as_ref().unwrap().values.as_slice().into_iter().map(|item| { format!("+ {}", item.tag) }).collect::<Vec<String>>().join("\n"),
            self.items.as_ref().unwrap()
            )
        }
    }

    impl Display for Item {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "**{}**:{} \n {}",
                   self.tag,
                   self.description,
                   self.file_paths.as_slice().into_iter().map(|fp| format!(" - {} \n", fp) ).collect::<Vec<String>>().join(""))
        }
    }

    impl Display for Items {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            // let ident: u8 = 0;
            // let last:
            write!(f, "{}", self.values.as_slice().into_iter().map(|item| {
                    format!("{}", item)
               }).collect::<Vec<String>>().join("\n")
            )
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





