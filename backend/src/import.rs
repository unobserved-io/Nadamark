use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use scraper::{ElementRef, Html, Selector};
use time::OffsetDateTime;

use crate::{
    database,
    models::{Bookmark, Folder},
};

pub async fn import_bookmarks(bookmarks_html: String) -> Response {
    match parse_bookmarks_html(&bookmarks_html) {
        Ok((folders_to_import, bookmarks_to_import)) => {
            if let Err(e) = database::insert_folders(folders_to_import) {
                eprintln!("Failed to import folders: {}", e);
            };
            if let Err(e) = database::insert_bookmarks(bookmarks_to_import) {
                eprintln!("Failed to import bookmarks: {}", e);
            };
            StatusCode::OK.into_response()
        }
        Err(e) => {
            eprintln!("Error parsing bookmarks: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn parse_bookmarks_html(html: &str) -> Result<(Vec<Folder>, Vec<Bookmark>), String> {
    let dom = Html::parse_document(&html);

    let mut folders_to_import: Vec<Folder> = Vec::new();
    let mut bookmarks_to_import: Vec<Bookmark> = Vec::new();

    let mut bookmark_id_counter = database::get_highest_bookmark_id().unwrap_or(0) + 1;
    let mut folder_id_counter = database::get_highest_folder_id().unwrap_or(0) + 1;

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

        let created = folder_element
            .value()
            .attr("add_date")
            .and_then(|timestamp| timestamp.parse::<i64>().ok())
            .map(|unix_timestamp| time::OffsetDateTime::from_unix_timestamp(unix_timestamp).ok())
            .flatten()
            .unwrap_or(OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc()));

        folders_to_import.push(Folder {
            id: folder_id_counter,
            name: folder_name,
            created,
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
                favorite: false,
            });

            bookmark_id_counter += 1;
        }
    }

    Ok((folders_to_import, bookmarks_to_import))
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
