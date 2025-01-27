use std::{collections::HashMap, sync::Arc};

use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    database::{self, Pool},
    models::{Bookmark, FolderNode, RootItems},
};

pub async fn refresh_tree(State(pool): State<Arc<Pool>>) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");

    let folders = match database::get_all_folders(&mut connection) {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching folders: {}", e);
            vec![]
        }
    };
    let bookmarks = match database::get_all_bookmarks(&mut connection) {
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

    // Find all folders in the folders vec without children and add them to their parent folder
    for folder in folders.iter() {
        if let Some(parent_id) = folder.parent_id {
            if let Some(child_node) = folder_map.remove(&folder.id) {
                if let Some(parent_node) = folder_map.get_mut(&parent_id) {
                    parent_node.children.push(child_node);
                } else {
                    for folder_node in folder_map.values_mut() {
                        traverse_folder_nodes(folder_node, &child_node);
                    }
                }
            }
        }
    }

    let mut root_items = RootItems {
        root_folders: folder_map.values().cloned().collect(),
        root_bookmarks,
    };
    root_items.sort_by_name();

    Json(root_items).into_response()
}

fn traverse_folder_nodes(folder_node: &mut FolderNode, child_node: &FolderNode) {
    if let Some(parent_id) = child_node.folder.parent_id {
        for child in &mut folder_node.children {
            if child.folder.id == parent_id {
                child.children.push(child_node.clone());
                return;
            } else {
                traverse_folder_nodes(child, child_node);
            }
        }
    };
}
