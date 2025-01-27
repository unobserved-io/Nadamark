use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use scraper::{ElementRef, Html, Selector};
use serde_json::{self, Value};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use crate::{
    database::{self, DbConnection, Pool},
    models::{Bookmark, Folder},
};

pub async fn import_bookmarks_html(
    State(pool): State<Arc<Pool>>,
    bookmarks_html: String,
) -> Response {
    let mut connection = pool.get().expect("Failed to get connection from pool");

    match parse_bookmarks_html(&mut connection, &bookmarks_html) {
        Ok((folders_to_import, bookmarks_to_import)) => {
            if let Err(e) = database::insert_folders(&mut connection, folders_to_import) {
                eprintln!("Failed to import folders: {}", e);
            };
            if let Err(e) = database::insert_bookmarks(&mut connection, bookmarks_to_import) {
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

pub async fn import_bookmarks_linkwarden(
    State(pool): State<Arc<Pool>>,
    linkwarden_json: String,
) -> Response {
    let mut connection = pool.get().expect("Failed to get connection from pool");

    match serde_json::from_str(&linkwarden_json) {
        Ok(json_values) => match parse_linkwarden_json(&mut connection, &json_values) {
            Ok((folders_to_import, bookmarks_to_import)) => {
                if let Err(e) = database::insert_folders(&mut connection, folders_to_import) {
                    eprintln!("Failed to import folders: {}", e);
                };
                if let Err(e) = database::insert_bookmarks(&mut connection, bookmarks_to_import) {
                    eprintln!("Failed to import bookmarks: {}", e);
                };
                StatusCode::OK.into_response()
            }
            Err(e) => {
                eprintln!("Error parsing Linkwarden JSON: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        },
        Err(e) => {
            eprintln!("Error parsing Linkwarden JSON: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

fn parse_bookmarks_html(
    mut connection: &mut DbConnection,
    html: &str,
) -> Result<(Vec<Folder>, Vec<Bookmark>), String> {
    let dom = Html::parse_document(&html);

    let mut folders_to_import: Vec<Folder> = Vec::new();
    let mut bookmarks_to_import: Vec<Bookmark> = Vec::new();

    let mut bookmark_id_counter =
        database::get_highest_bookmark_id(&mut connection).unwrap_or(0) + 1;
    let mut folder_id_counter = database::get_highest_folder_id(&mut connection).unwrap_or(0) + 1;

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
            favorite: false,
        });

        folder_id_counter += 1;
    }

    for bookmark_element in dom.select(&bookmark_selector) {
        if let Some(url) = bookmark_element.value().attr("href") {
            let name = bookmark_element.text().collect::<String>();
            let favicon = bookmark_element.value().attr("icon").map(String::from);
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
                favicon,
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

fn parse_linkwarden_json(
    mut connection: &mut DbConnection,
    json_data: &Value,
) -> Result<(Vec<Folder>, Vec<Bookmark>), String> {
    let mut folders_to_import: Vec<Folder> = Vec::new();
    let mut bookmarks_to_import: Vec<Bookmark> = Vec::new();

    let mut bookmark_id_counter =
        database::get_highest_bookmark_id(&mut connection).unwrap_or(0) + 1;
    let mut folder_id_counter = database::get_highest_folder_id(&mut connection).unwrap_or(0) + 1;

    let mut folder_id_counterparts: HashMap<i32, i32> = HashMap::new();

    if let Some(collections) = json_data["collections"].as_array() {
        for collection in collections {
            let folder_name = collection["name"].as_str().unwrap_or("");
            let folder_id = match collection["id"].as_i64() {
                Some(id) => id as i32,
                None => 0,
            };
            let folder_parent_id = match collection["parentId"].as_i64() {
                Some(id) => Some(id as i32),
                None => None,
            };
            let folder_created = parse_created_date(collection["createdAt"].as_str());

            folders_to_import.push(Folder {
                id: folder_id_counter,
                name: folder_name.to_string(),
                created: folder_created,
                parent_id: folder_parent_id,
                favorite: false,
            });

            folder_id_counterparts.insert(folder_id, folder_id_counter);

            if let Some(links) = collection["links"].as_array() {
                for link in links {
                    if let Some(bookmark_url) = link["url"].as_str() {
                        let mut bookmark_name = link["name"].as_str().unwrap_or("Missing Name");
                        if bookmark_name.trim().is_empty() {
                            bookmark_name = "Missing Name";
                        }
                        let bookmark_created = parse_created_date(collection["createdAt"].as_str());

                        bookmarks_to_import.push(Bookmark {
                            id: bookmark_id_counter,
                            name: bookmark_name.to_string(),
                            url: bookmark_url.to_string(),
                            favicon: None,
                            favicon_url: None,
                            created: bookmark_created,
                            folder_id: Some(folder_id_counter),
                            favorite: false,
                        });

                        bookmark_id_counter += 1;
                    }
                }
            }

            folder_id_counter += 1;
        }
    }

    // Re-assign parent_id to proper Nadamark folder id
    for folder in &mut folders_to_import {
        if folder.parent_id.is_some() {
            let new_parent_id = folder_id_counterparts
                .get(&folder.parent_id.unwrap())
                .copied();
            folder.parent_id = new_parent_id;
        }
    }

    Ok((folders_to_import, bookmarks_to_import))
}

fn parse_created_date(date_str: Option<&str>) -> OffsetDateTime {
    match date_str {
        Some(ds) => match OffsetDateTime::parse(ds, &Rfc3339) {
            Ok(dt) => dt,
            Err(e) => {
                eprintln!("Error converting DateTime: {}", e);
                time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc())
            }
        },
        None => time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc()),
    }
}
