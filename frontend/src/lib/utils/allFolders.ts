import type { Folder, RootItems, FolderNode } from '$lib/types';

export function getAllFolders(rootItems: RootItems): Folder[] {
	const folders: Folder[] = [];

	function collectFromFolder(folder: FolderNode) {
		folders.push({
			id: folder.id,
			name: folder.name,
			parent_id: folder.parent_id
		});

		for (const childFolder of folder.children) {
			collectFromFolder(childFolder);
		}
	}

	for (const rootFolder of rootItems.root_folders) {
		collectFromFolder(rootFolder);
	}

	return folders;
}
