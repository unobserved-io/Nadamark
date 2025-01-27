use std::sync::Arc;

use axum::{
    extract::State,
    response::{Html, IntoResponse},
};

use crate::database::{self, get_all_bookmarks, get_all_folders, DbConnection, Pool};

pub async fn export_bookmarks(State(pool): State<Arc<Pool>>) -> impl IntoResponse {
    let mut connection = pool.get().expect("Failed to get connection from pool");
    if !get_all_folders(&mut connection)
        .unwrap_or(vec![])
        .is_empty()
        || !get_all_bookmarks(&mut connection)
            .unwrap_or(vec![])
            .is_empty()
    {
        let html = traverse_bookmarks(&mut connection, None, &mut String::new(), 0);
        return Html(html).into_response();
    }

    Html("").into_response()
}

fn traverse_bookmarks(
    mut connection: &mut DbConnection,
    parent_folder_id: Option<i32>,
    html: &mut String,
    tabs: usize,
) -> String {
    let child_folders =
        database::get_all_child_folders(&mut connection, &parent_folder_id).unwrap();
    let child_bookmarks =
        database::get_all_child_bookmarks(&mut connection, &parent_folder_id).unwrap_or(vec![]);
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
        traverse_bookmarks(&mut connection, Some(folder.id), html, further_tabs);
    }

    for bookmark in child_bookmarks {
        let add_date = bookmark.created.unix_timestamp();
        let icon = match bookmark.favicon {
            Some(url) => &format!(" ICON=\"{}\"", url),
            None => "",
        };
        let icon_uri = match bookmark.favicon_url {
            Some(url) => &format!(" ICON_URI=\"{}\"", url),
            None => "",
        };

        html.push_str(&format!(
            "{}<DT><A HREF=\"{}\" ADD_DATE=\"{}\"{}{}>{}</A>\n",
            tab_chars(further_tabs),
            bookmark.url,
            add_date,
            icon,
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
