use std::{fs::File, io::Read};

use rfd::FileDialog;
use scraper::{ElementRef, Html, Selector};
use time::OffsetDateTime;

use crate::{
    database::{get_highest_bookmark_id, get_highest_folder_id},
    models::{Bookmark, Folder},
};

pub fn parse_bookmarks_html(html: &str) -> Result<(), String> {
    // let selected_file = FileDialog::new()
    //     .set_title("Open Bookmark HTML file")
    //     .add_filter("HTML", &["html"])
    //     .set_can_create_directories(false)
    //     .pick_file()
    //     .ok_or("Failed to select file")
    //     .map_err(|e| format!("{}", e))?;

    // let mut file = File::open(selected_file).map_err(|e| format!("Failed to read file: {}", e))?;
    // let mut content = String::new();
    // file.read_to_string(&mut content)
    //     .map_err(|e| format!("Failed to read file: {}", e))?;

    let dom = Html::parse_document(&html);

    let mut folders_to_import: Vec<Folder> = Vec::new();
    let mut bookmarks_to_import: Vec<Bookmark> = Vec::new();

    let mut bookmark_id_counter = get_highest_bookmark_id().unwrap_or(0) + 1;
    let mut folder_id_counter = get_highest_folder_id().unwrap_or(0) + 1;

    let folder_selector = Selector::parse("DL > DT > H3")
        .map_err(|e| format!("Failed to create folder_selector: {}", e))?;
    let bookmark_selector = Selector::parse("DT > A")
        .map_err(|e| format!("Failed to create bookmark_selector selector: {}", e))?;

    for folder_element in dom.select(&folder_selector) {
        let folder_name = folder_element.text().collect::<String>();

        let parent_folder_id = match find_parent_folder(&folder_element, &folders_to_import) {
            Some(parent) => Some(parent.id),
            None => None,
        };

        folders_to_import.push(Folder {
            id: folder_id_counter,
            name: folder_name,
            parent_id: parent_folder_id,
        });

        folder_id_counter += 1;
    }

    for bookmark_element in dom.select(&bookmark_selector) {
        if let Some(url) = bookmark_element.value().attr("href") {
            let name = bookmark_element.text().collect::<String>();
            let favicon_url = bookmark_element.value().attr("icon_uri").map(String::from);

            // Find parent folder
            let folder_id = match find_parent_folder(&bookmark_element, &folders_to_import) {
                Some(parent) => Some(parent.id),
                None => None,
            };

            let created = bookmark_element
                .value()
                .attr("add_date")
                .and_then(|timestamp| timestamp.parse::<i64>().ok())
                .map(|unix_timestamp| {
                    time::OffsetDateTime::from_unix_timestamp(unix_timestamp).ok()
                })
                .flatten()
                .unwrap_or(OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()));

            bookmarks_to_import.push(Bookmark {
                id: bookmark_id_counter,
                name,
                url: url.to_string(),
                favicon_url,
                created,
                folder_id,
            });

            bookmark_id_counter += 1;
        }
    }

    Ok(())
}

fn find_parent_folder<'a>(element: &ElementRef, folders: &'a [Folder]) -> Option<&'a Folder> {
    let mut current = element.parent()?;

    while let Some(parent) = current.parent() {
        if let Some(element) = parent.value().as_element() {
            if element.name() == "dl" {
                if let Some(header) = parent
                    .parent()?
                    .first_child()?
                    .first_child()?
                    .value()
                    .as_text()
                {
                    return folders.iter().find(|f| f.name == header.trim());
                }
            }
        }
        current = parent;
    }

    None
}
