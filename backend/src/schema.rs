diesel::table! {
    folders (id) {
        id -> Integer,
        name -> Text,
        created -> TimestamptzSqlite,
        parent_id -> Nullable<Integer>,
        favorite -> Bool,
    }
}

diesel::table! {
    bookmarks (id) {
        id -> Integer,
        name -> Text,
        url -> Text,
        favicon -> Nullable<Text>,
        favicon_url -> Nullable<Text>,
        created -> TimestamptzSqlite,
        folder_id -> Nullable<Integer>,
        favorite -> Bool,
    }
}

diesel::joinable!(bookmarks -> folders (folder_id));
diesel::allow_tables_to_appear_in_same_query!(folders, bookmarks);
