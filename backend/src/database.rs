use crate::models::{
    Bookmark, Folder, NewBookmark, NewFolder, UpdateBookmarkRequest, UpdateFolderRequest,
};
use diesel::r2d2::{self, ConnectionManager};
use diesel::{dsl::not, prelude::*, result::Error, sql_query, ExpressionMethods, SqliteConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

fn running_in_container() -> bool {
    // Check for .dockerenv file (Docker)
    if std::path::Path::new("/.dockerenv").exists() {
        return true;
    }

    // Check cgroup (works for Docker, Podman, and other containers)
    if let Ok(contents) = std::fs::read_to_string("/proc/1/cgroup") {
        if !contents.lines().all(|line| line.contains(":/")) {
            return true;
        }
    }

    false
}

pub fn get_data_path() -> PathBuf {
    if running_in_container() {
        PathBuf::from("/bookmarks")
    } else {
        if let Some(project_dir) = ProjectDirs::from("io", "unobserved", "nadamark") {
            let path = PathBuf::from(project_dir.data_dir());
            fs::create_dir_all(&path).expect("Unable to create data directory");
            path
        } else {
            PathBuf::new()
        }
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

pub fn establish_connection_pool() -> Pool {
    let database_url = get_default_database_path();
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Failed to create pool");

    // Initialize the database connection with your PRAGMAs
    let connection = &mut pool.get().expect("Failed to get connection from pool");
    sql_query("PRAGMA foreign_keys = ON")
        .execute(connection)
        .unwrap();
    sql_query("PRAGMA journal_mode = WAL")
        .execute(connection)
        .unwrap();
    sql_query("PRAGMA synchronous = NORMAL")
        .execute(connection)
        .unwrap();
    sql_query("PRAGMA busy_timeout = 5000")
        .execute(connection)
        .unwrap();

    pool
}

pub fn initialize_database() -> Pool {
    let database_url = get_default_database_path();

    // Create database file if it doesn't exist
    if !fs::metadata(&database_url).is_ok() {
        println!("Database file not found, creating a new one...");
        if let Err(e) = fs::File::create(&database_url) {
            eprintln!("Failed to create SQLite file: {}", e);
        }
    } else {
        println!("Database found at {}", &database_url)
    }

    let pool = establish_connection_pool();

    let mut connection = pool.get().expect("Failed to get connection from pool");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Failed to initialize database");

    pool
}

pub fn create_new_folder(
    connection: &mut DbConnection,
    name: String,
    parent_id: Option<i32>,
) -> Result<i32, Error> {
    use crate::schema::folders::{self, id};
    diesel::insert_into(folders::table)
        .values(NewFolder {
            name,
            created: time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc()),
            parent_id,
            favorite: false,
        })
        .returning(id)
        .get_result(connection)
}

pub fn create_new_bookmark(
    connection: &mut DbConnection,
    name: String,
    url: String,
    folder_id: Option<i32>,
) -> Result<i32, Error> {
    use crate::schema::bookmarks::{self, id};
    diesel::insert_into(bookmarks::table)
        .values(NewBookmark {
            name,
            url,
            favicon: None,
            favicon_url: None,
            created: time::OffsetDateTime::now_local().unwrap_or(time::OffsetDateTime::now_utc()),
            folder_id,
            favorite: false,
        })
        .returning(id)
        .get_result(connection)
}

pub fn insert_folders(connection: &mut DbConnection, folders: Vec<Folder>) -> Result<usize, Error> {
    use crate::schema::folders;
    diesel::insert_into(folders::table)
        .values(&folders)
        .execute(connection)
}

pub fn insert_bookmarks(
    connection: &mut DbConnection,
    bookmarks: Vec<Bookmark>,
) -> Result<usize, Error> {
    use crate::schema::bookmarks;
    diesel::insert_into(bookmarks::table)
        .values(&bookmarks)
        .execute(connection)
}

pub fn get_all_folders(connection: &mut DbConnection) -> Result<Vec<Folder>, Error> {
    use crate::schema::folders::dsl::*;
    folders.load(connection)
}

pub fn get_all_bookmarks(connection: &mut DbConnection) -> Result<Vec<Bookmark>, Error> {
    use crate::schema::bookmarks::dsl::*;
    bookmarks.load(connection)
}

pub fn update_folder(
    connection: &mut DbConnection,
    folder: UpdateFolderRequest,
) -> Result<usize, Error> {
    use crate::schema::folders::dsl::*;
    diesel::update(folders.find(folder.id))
        .set((parent_id.eq(folder.parent_id), name.eq(folder.name)))
        .execute(connection)
}

pub fn update_bookmark(
    connection: &mut DbConnection,
    bookmark: UpdateBookmarkRequest,
) -> Result<usize, Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::update(bookmarks.find(bookmark.id))
        .set((
            name.eq(bookmark.name),
            url.eq(bookmark.url),
            folder_id.eq(bookmark.folder_id),
        ))
        .execute(connection)
}

pub fn delete_folder(connection: &mut DbConnection, folder_id: i32) -> Result<usize, Error> {
    use crate::schema::folders::dsl::*;
    diesel::delete(folders.filter(id.eq(folder_id))).execute(connection)
}

pub fn delete_bookmark(connection: &mut DbConnection, bookmark_id: i32) -> Result<usize, Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::delete(bookmarks.filter(id.eq(bookmark_id))).execute(connection)
}

pub fn change_folder_parent(
    connection: &mut DbConnection,
    folder_id: i32,
    new_parent_id: Option<i32>,
) -> Result<usize, Error> {
    use crate::schema::folders::dsl::*;
    diesel::update(folders.find(folder_id))
        .set(parent_id.eq(new_parent_id))
        .execute(connection)
}

pub fn change_bookmark_folder(
    connection: &mut DbConnection,
    bookmark_id: i32,
    new_folder_id: Option<i32>,
) -> Result<usize, Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::update(bookmarks.find(bookmark_id))
        .set(folder_id.eq(new_folder_id))
        .execute(connection)
}

pub fn toggle_bookmark_favorite(
    connection: &mut DbConnection,
    bookmark_id: i32,
) -> Result<usize, Error> {
    use crate::schema::bookmarks::dsl::*;
    diesel::update(bookmarks.find(bookmark_id))
        .set(favorite.eq(not(favorite)))
        .execute(connection)
}

pub fn is_subfolder(
    connection: &mut DbConnection,
    folder_id: i32,
    potential_parent_id: i32,
) -> bool {
    use crate::schema::folders::dsl::*;
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

pub fn get_highest_bookmark_id(connection: &mut DbConnection) -> Result<i32, Error> {
    use crate::schema::bookmarks::dsl::*;
    bookmarks.select(id).order(id.desc()).first(connection)
}

pub fn get_highest_folder_id(connection: &mut DbConnection) -> Result<i32, Error> {
    use crate::schema::folders::dsl::*;
    folders.select(id).order(id.desc()).first(connection)
}

pub fn get_all_child_folders(
    connection: &mut DbConnection,
    parent_folder_id: &Option<i32>,
) -> Result<Vec<Folder>, Error> {
    use crate::schema::folders::dsl::*;
    if parent_folder_id.is_some() {
        folders
            .filter(parent_id.eq(parent_folder_id))
            .load(connection)
    } else {
        folders.filter(parent_id.is_null()).load(connection)
    }
}

pub fn get_all_child_bookmarks(
    connection: &mut DbConnection,
    parent_folder_id: &Option<i32>,
) -> Result<Vec<Bookmark>, Error> {
    use crate::schema::bookmarks::dsl::*;
    if parent_folder_id.is_some() {
        bookmarks
            .filter(folder_id.eq(parent_folder_id))
            .load(connection)
    } else {
        bookmarks.filter(folder_id.is_null()).load(connection)
    }
}
