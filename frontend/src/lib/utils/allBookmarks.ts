import type { Bookmark, RootItems, FolderNode } from '$lib/types';

export function getAllBookmarks(rootItems: RootItems): Bookmark[] {
	const bookmarks: Bookmark[] = [];
	bookmarks.push(...rootItems.root_bookmarks);

	function collectFromFolder(folder: FolderNode) {
		bookmarks.push(...folder.bookmarks);

		for (const childFolder of folder.children) {
			collectFromFolder(childFolder);
		}
	}

	for (const rootFolder of rootItems.root_folders) {
		collectFromFolder(rootFolder);
	}

	return bookmarks;
}

export function getAllFavorites(rootItems: RootItems): Bookmark[] {
	const favorites: Bookmark[] = [];

	for (const bookmark of rootItems.root_bookmarks) {
		if (bookmark.favorite) {
			favorites.push(bookmark);
		}
	}

	function collectFromFolder(folder: FolderNode) {
		for (const bookmark of folder.bookmarks) {
			if (bookmark.favorite) {
				favorites.push(bookmark);
			}
		}

		for (const childFolder of folder.children) {
			collectFromFolder(childFolder);
		}
	}

	for (const rootFolder of rootItems.root_folders) {
		collectFromFolder(rootFolder);
	}

	return favorites;
}
