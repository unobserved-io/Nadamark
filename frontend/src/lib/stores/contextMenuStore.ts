import { writable } from 'svelte/store';
import type { Bookmark, FolderNode } from '$lib/types';

type ContextMenuState = {
	isOpen: boolean;
	type: 'folder' | 'bookmark' | null;
	data: FolderNode | Bookmark | null;
	position: { x: number; y: number };
	parentId: number | null;
};

const initialState: ContextMenuState = {
	isOpen: false,
	type: null,
	data: null,
	position: { x: 0, y: 0 },
	parentId: null
};

export const contextMenuStore = writable<ContextMenuState>(initialState);

export function openContextMenu(
	type: 'folder' | 'bookmark',
	data: FolderNode | Bookmark,
	position: { x: number; y: number }
) {
	const parentId =
		type === 'folder' ? (data as FolderNode).parent_id : (data as Bookmark).folder_id;

	contextMenuStore.set({
		isOpen: true,
		type,
		data,
		position,
		parentId
	});
}

export function resetContextMenu() {
	contextMenuStore.set(initialState);
}

export function handleContextMenu(
	event: MouseEvent,
	type: 'folder' | 'bookmark',
	data: FolderNode | Bookmark
) {
	event.preventDefault();
	openContextMenu(type, data, { x: event.pageX, y: event.pageY });
}
