use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::folders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::bookmarks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub favicon_url: Option<String>,
    pub created: time::OffsetDateTime,
    pub folder_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RootItems {
    pub root_folders: Vec<FolderNode>,
    pub root_bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderNode {
    #[serde(flatten)]
    pub folder: Folder,
    pub children: Vec<FolderNode>,
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    Folder,
    Bookmark,
}

#[derive(Debug, Deserialize)]
pub struct MoveItemRequest {
    pub item_type: ItemType,
    pub item_id: i32,
    pub target_folder_id: i32,
}
