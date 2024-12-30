import type { RootItems } from '$lib/types';

export const load = async () => {
	const treeResult = await fetch('http://localhost:3096/api/folder-tree');
	const folderTree: RootItems = await treeResult.json();
	return { folderTree };
};
