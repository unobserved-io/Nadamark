use axum::response::{Html, IntoResponse};

use crate::database::{self, get_all_bookmarks, get_all_folders};

pub async fn export_bookmarks() -> impl IntoResponse {
    if !get_all_folders().unwrap_or(vec![]).is_empty()
        || !get_all_bookmarks().unwrap_or(vec![]).is_empty()
    {
        let html = traverse_bookmarks(None, &mut String::new(), 0);
        return Html(html).into_response();
    }

    Html("").into_response()
}

fn traverse_bookmarks(parent_folder_id: Option<i32>, html: &mut String, tabs: usize) -> String {
    let child_folders = database::get_all_child_folders(&parent_folder_id).unwrap();
    let child_bookmarks = database::get_all_child_bookmarks(&parent_folder_id).unwrap_or(vec![]);
    let add_tags = !child_folders.is_empty() || !child_bookmarks.is_empty();
    let mut further_tabs = tabs;

    if add_tags {
        if parent_folder_id.is_none() {
            html.push_str(
                r#"
<!DOCTYPE NETSCAPE-Bookmark-file-1>
<!-- This is an automatically generated file.
    It will be read and overwritten.
    DO NOT EDIT! -->
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Bookmarks</TITLE>
<H1>Bookmarks</H1>

"#,
            );
        }
        html.push_str(&format!("{}<DL><p>\n", tab_chars(tabs)));
        further_tabs += 1;
    }

    for folder in child_folders {
        let add_date = folder.created.unix_timestamp();
        html.push_str(&format!(
            "{}<DT><H3 ADD_DATE=\"{}\">{}</H3>\n",
            tab_chars(further_tabs),
            add_date,
            folder.name
        ));
        traverse_bookmarks(Some(folder.id), html, further_tabs);
    }

    for bookmark in child_bookmarks {
        // TODO: Add Icon base64
        let add_date = bookmark.created.unix_timestamp();
        let icon_uri = match bookmark.favicon_url {
            Some(url) => &format!(" ICON_URI=\"{}\"", url),
            None => "",
        };
        html.push_str(&format!(
            "{}<DT><A HREF=\"{}\" ADD_DATE=\"{}\"{}>{}</A>\n",
            tab_chars(further_tabs),
            bookmark.url,
            add_date,
            icon_uri,
            bookmark.name
        ));
    }

    if parent_folder_id.is_none() {
        html.push_str("</DL>\n");
    } else if add_tags {
        html.push_str(&format!("{}</DL><p>\n", tab_chars(tabs)));
    }

    html.clone()
}

fn tab_chars(tabs: usize) -> String {
    "\t".repeat(tabs)
}