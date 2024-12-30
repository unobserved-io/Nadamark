diesel::table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        parent_id -> Nullable<Integer>,
    }
}

diesel::table! {
    bookmarks (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        favicon_url -> Text,
        created -> TimestamptzSqlite,
        folder_id -> Nullable<Integer>,
    }
}
