tonic::include_proto!("todo_tags");

#[derive(Debug)]
pub enum Action {
    Add(primatives::FolderName),
    Edit(primatives::FolderName),
    Display(primatives::FolderName),
    Write(primatives::Item),
    Remove(primatives::Item),
    Show(primatives::Item),
    Find(primatives::Query),
}   


pub mod primatives {
    pub trait Conversion {}

    pub type Item = (String, String, Vec<String>);
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
               file_path: v.2,
            }
        }
    }

    impl From<&Item> for super::Item {
        fn from(v: &Item) -> Self {
            Self {
               tag: v.0.clone(),
               description: v.1.clone(),
               file_path: v.2.clone(),
            }
        }
    }


    impl From<ItemsWithCount> for super::Items {
        fn from(v: ItemsWithCount) -> Self {
            Self {
                count: v.0,
                items: v.1.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<&ItemsWithCount> for super::Items {
        fn from(v: &ItemsWithCount) -> Self {
            Self {
                count: v.0.clone(),
                items: v.1.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<ItemsWithOutCount> for super::Items {
        fn from(v: ItemsWithOutCount) -> Self {
            Self {
                count: v.len() as u64,
                items: v.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

    impl From<&ItemsWithOutCount> for super::Items {
        fn from(v: &ItemsWithOutCount) -> Self {
            Self {
                count: v.len() as u64,
                items: v.iter().map(|item| item.into()).collect::<Vec<super::Item>>(),
            }
        }
    }

}





