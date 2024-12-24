use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Folder {
    pub id: usize,
    pub name: String,
    pub parent_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bookmark {
    pub id: usize,
    pub name: String,
    pub url: String,
    pub favicon: String,
    pub created: time::OffsetDateTime,
    pub folder_id: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FolderNode {
    #[serde(flatten)]
    pub folder: Folder,
    pub children: Vec<FolderNode>,
    pub bookmarks: Vec<Bookmark>,
}

impl Folder {
    pub fn to_tree(folders: &[Folder], bookmarks: &[Bookmark]) -> Vec<FolderNode> {
        FolderNode::build_tree(folders, bookmarks)
    }
}

impl FolderNode {
    fn build_tree(folders: &[Folder], bookmarks: &[Bookmark]) -> Vec<FolderNode> {
        // Get all folders and bookmarks into their corresponding subfolders.
        let mut folder_map: HashMap<usize, FolderNode> = folders
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

        // Assign bookmarks to their folders
        for bookmark in bookmarks {
            if let Some(folder) = folder_map.get_mut(&bookmark.folder_id) {
                folder.bookmarks.push(bookmark.clone());
            }
        }

        let mut root_folders: Vec<FolderNode> = Vec::new();
        let folder_ids: Vec<usize> = folder_map.keys().cloned().collect();

        for folder_id in folder_ids {
            if let Some(folder) = folders.iter().find(|f| f.id == folder_id) {
                if let Some(node) = folder_map.remove(&folder_id) {
                    if let Some(parent_id) = folder.parent_id {
                        if let Some(parent) = folder_map.get_mut(&parent_id) {
                            parent.children.push(node);
                        }
                    } else {
                        root_folders.push(node);
                    }
                }
            }
        }

        root_folders
    }
}
