import type { FolderNode } from '$lib/types';

export const load = async () => {
	const treeRes = await fetch('http://localhost:3096/api/folder-tree');
	const folderTree: FolderNode[] = await treeRes.json();
	return { folderTree };
};
