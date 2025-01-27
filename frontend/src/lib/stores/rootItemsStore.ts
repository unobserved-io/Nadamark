import { writable } from 'svelte/store';
import type { RootItems, FolderNode, Bookmark } from '$lib/types';
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

	async deleteItem(itemId: number, itemType: 'folder' | 'bookmark') {
		try {
			const response = await fetch(`${API_BASE}/delete-${itemType}`, {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(itemId)
			});

			if (response.ok) {
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					const updatedData = structuredClone(state.data);

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
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error(`Error deleting ${itemType}:`, error);
		}
	},

	async editFolder(itemId: number, itemName: string, parentId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/update-folder`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					id: itemId,
					name: itemName,
					parent_id: parentId
				})
			});

			if (response.ok) {
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					state.data.root_folders = updateFolderInTree(
						state.data.root_folders,
						itemId,
						itemName,
						parentId
					);
					state.data = sortRootItems(state.data);
					return state;
				});

				function updateFolderInTree(
					nodes: FolderNode[],
					targetId: number,
					newName: string,
					newParentId: number | null
				): FolderNode[] {
					return nodes.map((node) => {
						if (node.id === targetId) {
							return {
								...node,
								name: newName,
								parent_id: newParentId
							};
						}

						const updatedChildren = updateFolderInTree(
							node.children,
							targetId,
							newName,
							newParentId
						);

						if (updatedChildren !== node.children) {
							return {
								...node,
								children: updatedChildren
							};
						}

						return node;
					});
				}
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error('Failed to edit folder:', error);
		}
	},

	async editBookmark(itemId: number, itemName: string, itemUrl: string, parentId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/update-bookmark`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					id: itemId,
					name: itemName,
					url: itemUrl,
					folder_id: parentId
				})
			});

			if (response.ok) {
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					return {
						...state,
						data: sortRootItems({
							...state.data,
							root_bookmarks: state.data.root_bookmarks.map((bookmark) =>
								bookmark.id === itemId
									? { ...bookmark, name: itemName, url: itemUrl, folder_id: parentId }
									: bookmark
							),
							root_folders: updateBookmarkInTree(
								state.data.root_folders,
								itemId,
								itemName,
								itemUrl,
								parentId
							)
						})
					};
				});

				function updateBookmarkInTree(
					nodes: FolderNode[],
					targetId: number,
					newName: string,
					newUrl: string,
					newFolderId: number | null
				): FolderNode[] {
					return nodes.map((node) => {
						const hasUpdatedBookmark = node.bookmarks.some((b) => b.id === targetId);
						const updatedBookmarks = node.bookmarks.map((bookmark) =>
							bookmark.id === targetId
								? { ...bookmark, name: newName, url: newUrl, folder_id: newFolderId }
								: bookmark
						);

						const updatedChildren = updateBookmarkInTree(
							node.children,
							targetId,
							newName,
							newUrl,
							newFolderId
						);

						if (hasUpdatedBookmark || updatedChildren !== node.children) {
							return {
								...node,
								bookmarks: updatedBookmarks,
								children: updatedChildren
							};
						}

						return node;
					});
				}
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error('Failed to edit bookmark:', error);
		}
	},

	async newFolder(itemName: string, parentId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/create-folder`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					name: itemName,
					parent_id: parentId
				})
			});

			if (response.ok) {
				const data = await response.json();
				const newId = data.id;
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					const newFolder: FolderNode = {
						id: newId, // TODO get from response
						name: itemName,
						parent_id: parentId,
						children: [],
						bookmarks: []
					};

					if (parentId === null) {
						// Add to root level
						return {
							...state,
							data: sortRootItems({
								...state.data,
								root_folders: [...state.data.root_folders, newFolder]
							})
						};
					} else {
						// Add to parent folder
						return {
							...state,
							data: sortRootItems({
								...state.data,
								root_folders: addFolderToParent(state.data.root_folders, parentId, newFolder)
							})
						};
					}
				});
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error('Failed to create folder:', error);
		}
	},

	async newBookmark(itemName: string, itemUrl: string, parentId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/create-bookmark`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					name: itemName,
					url: itemUrl,
					folder_id: parentId
				})
			});

			if (response.ok) {
				const data = await response.json();
				const newId = data.id;
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					const newBookmark: Bookmark = {
						id: newId,
						name: itemName,
						url: itemUrl,
						favicon: '',
						favicon_url: '',
						created: new Date().toISOString(),
						folder_id: parentId,
						favorite: false
					};

					if (parentId === null) {
						// Add to root level
						return {
							...state,
							data: sortRootItems({
								...state.data,
								root_bookmarks: [...state.data.root_bookmarks, newBookmark]
							})
						};
					} else {
						// Add to parent folder
						return {
							...state,
							data: sortRootItems({
								...state.data,
								root_folders: addBookmarkToParent(state.data.root_folders, parentId, newBookmark)
							})
						};
					}
				});
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error('Failed to create bookmark:', error);
		}
	},

	async moveItem(itemId: number, itemType: string, targetId: number | null) {
		try {
			const response = await fetch(`${API_BASE}/move`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					item_type: itemType,
					item_id: itemId,
					target_folder_id: targetId
				})
			});

			if (response.ok) {
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					let movedItem: Bookmark | FolderNode | null = null;

					// First create new state without the moved item
					const newState = {
						...state,
						data: {
							...state.data,
							// Remove from root_bookmarks if it's a bookmark
							root_bookmarks:
								itemType === 'bookmark'
									? state.data.root_bookmarks.filter((b) => {
											if (b.id === itemId) {
												movedItem = { ...b, folder_id: targetId };
												return false;
											}
											return true;
										})
									: state.data.root_bookmarks,
							// Remove from root_folders if it's a folder
							root_folders:
								itemType === 'folder'
									? state.data.root_folders.filter((f) => {
											if (f.id === itemId) {
												movedItem = { ...f, parent_id: targetId };
												return false;
											}
											return true;
										})
									: state.data.root_folders
						}
					};

					// If item wasn't in root, look in the tree
					if (!movedItem) {
						newState.data.root_folders = removeFromTree(
							newState.data.root_folders,
							itemId,
							itemType,
							(item) => {
								movedItem =
									itemType === 'bookmark'
										? { ...item, folder_id: targetId }
										: { ...item, parent_id: targetId };
							}
						);
					}

					if (!movedItem) return state;

					// Then add the item to its new location
					if (targetId === null) {
						// Moving to root
						if (itemType === 'folder') {
							newState.data.root_folders = [...newState.data.root_folders, movedItem as FolderNode];
						} else {
							newState.data.root_bookmarks = [
								...newState.data.root_bookmarks,
								movedItem as Bookmark
							];
						}
					} else {
						// Moving to a folder
						newState.data.root_folders = addToFolder(
							newState.data.root_folders,
							targetId,
							movedItem,
							itemType
						);
					}
					newState.data = sortRootItems(newState.data);

					return newState;
				});
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (err) {
			console.error('Drop failed:', err);
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

			if (response.ok) {
				rootItemsStore.update((state) => {
					if (!state.data) return state;

					// If bookmark is in root_bookmarks
					if (parentId === null) {
						return {
							...state,
							data: {
								...state.data,
								root_bookmarks: state.data.root_bookmarks.map((bookmark) =>
									bookmark.id === bookmarkId
										? { ...bookmark, favorite: !bookmark.favorite }
										: bookmark
								)
							}
						};
					}

					// If bookmark is nested in a folder
					return {
						...state,
						data: {
							...state.data,
							root_folders: updateBookmarkFavorite(state.data.root_folders, bookmarkId)
						}
					};
				});
			} else {
				throw new Error(`HTTP error! status: ${response.status}`);
			}
		} catch (error) {
			console.error('Error toggling favorite:', error);
		}
	}
};

function addFolderToParent(
	nodes: FolderNode[],
	parentId: number,
	newFolder: FolderNode
): FolderNode[] {
	return nodes.map((node) => {
		if (node.id === parentId) {
			return {
				...node,
				children: [...node.children, newFolder]
			};
		}

		const updatedChildren = addFolderToParent(node.children, parentId, newFolder);

		if (updatedChildren !== node.children) {
			return {
				...node,
				children: updatedChildren
			};
		}

		return node;
	});
}

function addBookmarkToParent(
	nodes: FolderNode[],
	parentId: number,
	newBookmark: Bookmark
): FolderNode[] {
	return nodes.map((node) => {
		if (node.id === parentId) {
			return {
				...node,
				bookmarks: [...node.bookmarks, newBookmark]
			};
		}

		const updatedChildren = addBookmarkToParent(node.children, parentId, newBookmark);

		if (updatedChildren !== node.children) {
			return {
				...node,
				children: updatedChildren
			};
		}

		return node;
	});
}

function removeFromTree(
	nodes: FolderNode[],
	itemId: number,
	itemType: string,
	onFound: (item: Bookmark | FolderNode) => void
): FolderNode[] {
	return nodes.map((node) => {
		if (itemType === 'bookmark') {
			const newBookmarks = node.bookmarks.filter((b) => {
				if (b.id === itemId) {
					onFound(b);
					return false;
				}
				return true;
			});

			if (newBookmarks.length !== node.bookmarks.length) {
				return { ...node, bookmarks: newBookmarks };
			}
		}

		if (itemType === 'folder') {
			const newChildren = node.children.filter((f) => {
				if (f.id === itemId) {
					onFound(f);
					return false;
				}
				return true;
			});

			if (newChildren.length !== node.children.length) {
				return { ...node, children: newChildren };
			}
		}

		const updatedChildren = removeFromTree(node.children, itemId, itemType, onFound);
		if (updatedChildren !== node.children) {
			return { ...node, children: updatedChildren };
		}

		return node;
	});
}

function addToFolder(
	nodes: FolderNode[],
	targetId: number,
	item: Bookmark | FolderNode,
	itemType: string
): FolderNode[] {
	return nodes.map((node) => {
		if (node.id === targetId) {
			return {
				...node,
				bookmarks: itemType === 'bookmark' ? [...node.bookmarks, item as Bookmark] : node.bookmarks,
				children: itemType === 'folder' ? [...node.children, item as FolderNode] : node.children
			};
		}

		const updatedChildren = addToFolder(node.children, targetId, item, itemType);
		if (updatedChildren !== node.children) {
			return {
				...node,
				children: updatedChildren
			};
		}

		return node;
	});
}

function updateBookmarkFavorite(nodes: FolderNode[], bookmarkId: number): FolderNode[] {
	return nodes.map((node) => {
		const updatedBookmarks = node.bookmarks.map((bookmark) =>
			bookmark.id === bookmarkId ? { ...bookmark, favorite: !bookmark.favorite } : bookmark
		);

		if (updatedBookmarks.some((b) => b.id === bookmarkId)) {
			return {
				...node,
				bookmarks: updatedBookmarks
			};
		}

		const updatedChildren = updateBookmarkFavorite(node.children, bookmarkId);

		if (updatedChildren !== node.children) {
			return {
				...node,
				children: updatedChildren
			};
		}

		return node;
	});
}

function sortRootItems(items: RootItems): RootItems {
	const sorted = { ...items };

	sorted.root_folders.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));

	sorted.root_bookmarks?.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));

	const sortFolderNode = (folder: FolderNode) => {
		folder.bookmarks?.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
		folder.children?.sort((a, b) => a.name.toLowerCase().localeCompare(b.name.toLowerCase()));
		folder.children?.forEach(sortFolderNode);
	};

	sorted.root_folders.forEach(sortFolderNode);

	return sorted;
}
