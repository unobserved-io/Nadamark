import { writable } from 'svelte/store';
import type { Bookmark, FolderNode } from '$lib/types';

type ContextMenuState = {
	isOpen: boolean;
	type: 'folder' | 'bookmark' | null;
	data: FolderNode | Bookmark | null;
	position: { x: number; y: number };
};

const initialState: ContextMenuState = {
	isOpen: false,
	type: null,
	data: null,
	position: { x: 0, y: 0 }
};

export const contextMenuStore = writable<ContextMenuState>(initialState);

export function openContextMenu(
	type: 'folder' | 'bookmark',
	data: FolderNode | Bookmark,
	position: { x: number; y: number }
) {
	contextMenuStore.set({
		isOpen: true,
		type,
		data,
		position
	});
}
