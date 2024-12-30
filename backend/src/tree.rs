use std::collections::HashMap;

use axum::{response::IntoResponse, Json};

use crate::{
    database,
    models::{Bookmark, FolderNode, RootItems},
};

pub async fn refresh_tree() -> impl IntoResponse {
    let folders = match database::get_all_folders() {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching folders: {}", e);
            vec![]
        }
    };
    let bookmarks = match database::get_all_bookmarks() {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching folders: {}", e);
            vec![]
        }
    };

    let mut folder_map: HashMap<i32, FolderNode> = folders
        .iter()
        .map(|f| {
            (
                f.id,
                FolderNode {
                    folder: f.clone(),
                    children: Vec::new(),
                    bookmarks: Vec::new(),
                },
            )
        })
        .collect();

    let mut root_bookmarks: Vec<Bookmark> = Vec::new();

    for bookmark in bookmarks {
        match bookmark.folder_id.and_then(|id| folder_map.get_mut(&(id))) {
            Some(folder) => folder.bookmarks.push(bookmark.clone()),
            None => root_bookmarks.push(bookmark.clone()),
        }
    }

    let mut root_folders: Vec<FolderNode> = Vec::new();

    // First, organize children under their parents
    for folder in folders.iter() {
        if let Some(parent_id) = folder.parent_id {
            if let Some(child_node) = folder_map.get(&folder.id).cloned() {
                if let Some(parent_node) = folder_map.get_mut(&parent_id) {
                    parent_node.children.push(child_node)
                }
            }
        }
    }

    // Then collect root folders
    // Don't combine these, otherwise we could remove root folders before all
    // their children are added.
    for folder in folders.iter() {
        if folder.parent_id.is_none() {
            if let Some(node) = folder_map.remove(&folder.id) {
                root_folders.push(node);
            }
        }
    }

    Json(RootItems {
        root_folders,
        root_bookmarks,
    })
    .into_response()
}
