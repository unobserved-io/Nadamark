import { writable } from 'svelte/store';
import type { RootItems, FolderNode } from '$lib/types';
import { dev } from '$app/environment';

type RootItemsState = {
	data: RootItems | null;
	loading: boolean;
};

const initialState: RootItemsState = {
	data: null,
	loading: true
};

const API_BASE = dev ? 'http://localhost:8663/api' : '/api';

export const rootItemsStore = writable<RootItemsState>(initialState);

function updateBranch(tree: RootItems, parentId: number | null, newItems: RootItems): RootItems {
	if (parentId === null) {
		return newItems;
	}

	const updatedTree = JSON.parse(JSON.stringify(tree));

	function updateRecursive(folderNode: FolderNode) {
		if (folderNode.id === parentId) {
			folderNode.children = newItems.root_folders;
			folderNode.bookmarks = newItems.root_bookmarks;
			return true;
		}
		for (const child of folderNode.children) {
			if (updateRecursive(child)) {
				return true;
			}
		}
		return false;
	}

	updatedTree.root_folders.forEach((folder) => updateRecursive(folder));
	return updatedTree;
}

export const treeOperations = {
	// Full tree refresh
	async refreshFullTree() {
		try {
			const response = await fetch(`${API_BASE}/tree`);
			const data = await response.json();
			rootItemsStore.set({ data, loading: false });
		} catch (error) {
			console.error('Error fetching full tree:', error);
			rootItemsStore.set({ data: null, loading: false });
		}
	},

	// Partial tree update
	async refreshBranch(folderId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/tree/${folderId ?? 'root'}`);
			const partialData = await response.json();

			rootItemsStore.update((state) => {
				if (!state.data) return state;
				const updatedData = updateBranch(state.data, folderId, partialData);
				return { data: updatedData, loading: false };
			});
		} catch (error) {
			console.error('Error fetching branch:', error);
		}
	},

	async deleteItem(itemId: number, itemType: 'folder' | 'bookmark', parentId: number | null) {
		rootItemsStore.update((state) => {
			if (!state.data) return state;
			const updatedData = JSON.parse(JSON.stringify(state.data));

			function removeItem(items: RootItems | FolderNode) {
				if ('root_folders' in items) {
					if (itemType === 'folder') {
						items.root_folders = items.root_folders.filter((f) => f.id !== itemId);
					} else {
						items.root_bookmarks = items.root_bookmarks.filter((b) => b.id !== itemId);
					}
					items.root_folders.forEach((folder) => removeItem(folder));
				} else {
					if (itemType === 'folder') {
						items.children = items.children.filter((f) => f.id !== itemId);
					} else {
						items.bookmarks = items.bookmarks.filter((b) => b.id !== itemId);
					}
					items.children.forEach((folder) => removeItem(folder));
				}
			}

			removeItem(updatedData);
			return { data: updatedData, loading: false };
		});

		try {
			const response = await fetch(`${API_BASE}/delete-${itemType}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(itemId)
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			await this.refreshBranch(parentId);
		} catch (error) {
			console.error(`Error deleting ${itemType}:`, error);
			await this.refreshBranch(parentId);
		}
	},

	async editFolder(itemId: number, itemName, parentId: number | null) {
		try {
				if (itemName.trim().length > 0) {
					const response = await fetch(`${API_BASE}/update-${itemType}`)
						{
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify({
								id: itemId,
								name: itemName,
								parent_id: selectedFolder
							})
						}
					);

					if (!response.ok) {
						throw new Error('Failed to update folder');
					}
					refreshTree();
					closeModal();
				}
			} else if (type == 'bookmark') {
				if (itemName.trim().length > 0 && itemUrl.trim().length > 0) {
					const response = await fetch(
						dev ? 'http://localhost:8663/api/update-bookmark' : '/api/update-bookmark',
						{
							method: 'POST',
							headers: {
								'Content-Type': 'application/json'
							},
							body: JSON.stringify({
								id: $contextMenuStore.data?.id,
								name: itemName,
								url: itemUrl,
								folder_id: selectedFolder
							})
						}
					);

					if (!response.ok) {
						throw new Error('Failed to update bookmark');
					}
					refreshTree();
					closeModal();
				}
			}
		} catch (err) {
			console.error('Edit failed:', err);
			closeModal();
		}

		try {
			const response = await fetch(`${API_BASE}/delete-${itemType}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(itemId)
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			await this.refreshBranch(parentId);
		} catch (error) {
			console.error(`Error deleting ${itemType}:`, error);
			await this.refreshBranch(parentId);
		}
	},

	async toggleFavorite(bookmarkId: number, parentId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/favorite-bookmark`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify(bookmarkId)
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			await this.refreshBranch(parentId);
		} catch (error) {
			console.error('Error toggling favorite:', error);
			await this.refreshBranch(parentId);
		}
	}
};

export const refreshTree = treeOperations.refreshFullTree;
