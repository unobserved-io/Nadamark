use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::folders)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub created: time::OffsetDateTime,
    pub parent_id: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable)]
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

impl RootItems {
    pub fn sort_by_name(&mut self) {
        self.root_folders.sort_by(|a, b| {
            a.folder
                .name
                .to_lowercase()
                .cmp(&b.folder.name.to_lowercase())
        });

        self.root_bookmarks
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for folder in &mut self.root_folders {
            folder.sort_by_name();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FolderNode {
    #[serde(flatten)]
    pub folder: Folder,
    pub children: Vec<FolderNode>,
    pub bookmarks: Vec<Bookmark>,
}

impl FolderNode {
    pub fn sort_by_name(&mut self) {
        self.children.sort_by(|a, b| {
            a.folder
                .name
                .to_lowercase()
                .cmp(&b.folder.name.to_lowercase())
        });

        self.bookmarks
            .sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

        for child in &mut self.children {
            child.sort_by_name();
        }
    }
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

#[derive(Debug, Deserialize)]
pub struct NewFolderRequest {
    pub name: String,
    pub parent_id: Option<i32>,
}
