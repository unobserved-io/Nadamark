use crate::models::{Bookmark, Folder};
use diesel::{prelude::*, result::Error, Connection, ExpressionMethods, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn get_data_path() -> PathBuf {
    if let Some(project_dir) = ProjectDirs::from("io", "unobserved", "nadamark") {
        let path = PathBuf::from(project_dir.data_dir());
        fs::create_dir_all(&path).expect("Unable to create data directory");
        path
    } else {
        PathBuf::new()
    }
}

pub fn get_default_database_path() -> String {
    let mut path = get_data_path();
    path.extend(&["nadamark.db"]);
    if let Some(db_path) = path.to_str() {
        db_path.to_string()
    } else {
        String::new()
    }
}

fn establish_connection() -> SqliteConnection {
    let database_url = get_default_database_path();
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn initialize_database() {
    let database_url = get_default_database_path();
    if !fs::metadata(&database_url).is_ok() {
        // TODO: Change println to info
        println!("Database file not found, creating a new one...");
        if let Err(e) = fs::File::create(&database_url) {
            eprintln!("Failed to create SQLite file: {}", e);
        }
    } else {
        println!("Database found at {}", &database_url)
    }

    let connection = &mut establish_connection();
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to initialize database");
}

pub fn insert_folders(folders: Vec<Folder>) -> Result<usize, Error> {
    use crate::schema::folders;

    let connection = &mut establish_connection();

    diesel::insert_into(folders::table)
        .values(&folders)
        .execute(connection)
}

pub fn insert_bookmarks(bookmarks: Vec<Bookmark>) -> Result<usize, Error> {
    use crate::schema::bookmarks;

    let connection = &mut establish_connection();

    diesel::insert_into(bookmarks::table)
        .values(&bookmarks)
        .execute(connection)
}

pub fn get_all_folders() -> Result<Vec<Folder>, Error> {
    use crate::schema::folders::dsl::*;

    let connection = &mut establish_connection();

    folders.load(connection)
}

pub fn get_all_bookmarks() -> Result<Vec<Bookmark>, Error> {
    use crate::schema::bookmarks::dsl::*;

    let connection = &mut establish_connection();

    bookmarks.load(connection)
}

pub fn change_folder_parent(folder_id: i32, new_parent_id: i32) -> Result<usize, Error> {
    use crate::schema::folders::dsl::*;

    let connection = &mut establish_connection();

    diesel::update(folders.find(folder_id))
        .set(parent_id.eq(new_parent_id))
        .execute(connection)
}

pub fn change_bookmark_folder(bookmark_id: i32, new_folder_id: i32) -> Result<usize, Error> {
    use crate::schema::bookmarks::dsl::*;

    let connection = &mut establish_connection();

    diesel::update(bookmarks.find(bookmark_id))
        .set(folder_id.eq(new_folder_id))
        .execute(connection)
}

pub fn is_subfolder(folder_id: i32, potential_parent_id: i32) -> bool {
    use crate::schema::folders::dsl::*;

    let connection = &mut establish_connection();

    let mut current_id = Some(potential_parent_id);

    while let Some(found_id) = current_id {
        if found_id == folder_id {
            return true;
        }

        current_id = folders
            .select(parent_id)
            .find(found_id)
            .first::<Option<i32>>(connection)
            .unwrap_or(None);
    }

    false
}

pub fn get_highest_bookmark_id() -> Result<i32, Error> {
    use crate::schema::bookmarks::dsl::*;

    let connection = &mut establish_connection();

    bookmarks.select(id).order(id.desc()).first(connection)
}

pub fn get_highest_folder_id() -> Result<i32, Error> {
    use crate::schema::folders::dsl::*;

    let connection = &mut establish_connection();

    folders.select(id).order(id.desc()).first(connection)
}
