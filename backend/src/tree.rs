use crate::{
    database,
    models::{Bookmark, FolderNode, RootItems},
};
use axum::{extract::Path, response::IntoResponse, Json};
use std::collections::HashMap;

pub async fn refresh_tree() -> impl IntoResponse {
    build_tree(None).into_response()
}

pub async fn refresh_branch(Path(folder_id): Path<String>) -> impl IntoResponse {
    let folder_id = if folder_id == "root" {
        None
    } else {
        match folder_id.parse::<i32>() {
            Ok(id) => Some(id),
            Err(e) => {
                eprintln!("Error parsing folder_id: {}", e);
                None
            }
        }
    };

    build_tree(folder_id).into_response()
}

// Handle both full and partial tree builds
fn build_tree(parent_folder_id: Option<i32>) -> Json<RootItems> {
    let folders = match if parent_folder_id.is_none() {
        database::get_all_folders()
    } else {
        database::get_all_child_folders(&parent_folder_id)
    } {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching folders: {}", e);
            vec![]
        }
    };

    let bookmarks = match if parent_folder_id.is_none() {
        database::get_all_bookmarks()
    } else {
        database::get_all_child_bookmarks(&parent_folder_id)
    } {
        Ok(items) => items,
        Err(e) => {
            eprintln!("Error fetching bookmarks: {}", e);
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
            None => {
                if parent_folder_id == bookmark.folder_id {
                    root_bookmarks.push(bookmark.clone())
                }
            }
        }
    }

    if parent_folder_id.is_none() {
        for folder in folders.iter() {
            if let Some(parent_id) = folder.parent_id {
                if let Some(child_node) = folder_map.remove(&folder.id) {
                    if let Some(parent_node) = folder_map.get_mut(&parent_id) {
                        parent_node.children.push(child_node);
                    } else {
                        for folder_node in folder_map.values_mut() {
                            traverse_folder_nodes(folder_node, child_node.clone());
                        }
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

    Json(root_items)
}

fn traverse_folder_nodes(folder_node: &mut FolderNode, child_node: FolderNode) {
    if let Some(parent_id) = child_node.folder.parent_id {
        for child in &mut folder_node.children {
            if child.folder.id == parent_id {
                child.children.push(child_node);
                return;
            } else {
                traverse_folder_nodes(child, child_node.clone()); // TODO: efficiency (clone)
            }
        }
    };
}
