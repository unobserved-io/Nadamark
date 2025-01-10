import { writable } from 'svelte/store';
import type { RootItems } from '$lib/types';

const initialState: RootItems = {
	root_folders: [],
	root_bookmarks: []
};

export const rootItemsStore = writable<RootItems>(initialState);

export async function refreshTree() {
	const treeResult = await fetch('http://localhost:3096/api/folder-tree');
	rootItemsStore.set(await treeResult.json());
}
