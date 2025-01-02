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
