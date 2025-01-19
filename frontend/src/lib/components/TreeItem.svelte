<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import type { Bookmark, FolderNode } from '$lib/types';
	import Self from './TreeItem.svelte';
	import { contextMenuStore, openContextMenu } from '$lib/stores/contextMenuStore';
	import { refreshTree } from '$lib/stores/rootItemsStore';
	import ContextMenu from './ContextMenu.svelte';

	let { item, type } = $props<{
		item: FolderNode | Bookmark;
		type: 'folder' | 'bookmark';
	}>();

	function isFolder(item: FolderNode | Bookmark): item is FolderNode {
		return 'children' in item;
	}

	function isBookmark(item: FolderNode | Bookmark): item is Bookmark {
		return 'url' in item;
	}

	// Open/closed folders
	let isOpen = $state(false);
	function handleToggle() {
		isOpen = !isOpen;
	}

	// Drag handlers
	function handleDragStart(e: DragEvent, type: 'folder' | 'bookmark', item: FolderNode | Bookmark) {
		if (!e.dataTransfer) return;
		e.dataTransfer.setData(
			'application/json',
			JSON.stringify({
				type,
				id: item.id
			})
		);
	}

	function handleDragOver(e: DragEvent) {
		e.preventDefault();
		if (e.currentTarget instanceof HTMLElement) {
			e.currentTarget.classList.add('drag-over');
		}
	}

	function handleDragLeave(e: DragEvent) {
		if (e.currentTarget instanceof HTMLElement) {
			e.currentTarget.classList.remove('drag-over');
		}
	}

	async function handleDrop(e: DragEvent) {
		e.preventDefault();
		if (!e.dataTransfer) return;

		if (e.currentTarget instanceof HTMLElement) {
			e.currentTarget.classList.remove('drag-over');
		}

		try {
			const data = JSON.parse(e.dataTransfer.getData('application/json'));

			const response = await fetch('http://localhost:3096/api/move', {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					item_type: data.type,
					item_id: data.id,
					target_folder_id: item.id
				})
			});

			if (!response.ok) {
				throw new Error('Failed to move item');
			}

			refreshTree();
		} catch (err) {
			console.error('Drop failed:', err);
		}
	}

	// Context menu
	function handleContextMenu(
		event: MouseEvent,
		type: 'folder' | 'bookmark',
		data: FolderNode | Bookmark
	) {
		event.preventDefault();
		openContextMenu(type, data, { x: event.pageX, y: event.pageY });
	}

	onMount(() => {
		// Close context menu if user clicks outside of menu
		const handleClick = (event: MouseEvent) => {
			if ($contextMenuStore.isOpen && event.target instanceof Node) {
				const contextMenu = document.querySelector('.context-menu');
				if (contextMenu && !contextMenu.contains(event.target)) {
					$contextMenuStore.isOpen = false;
				}
			}
		};

		document.addEventListener('click', handleClick);
		return () => {
			document.removeEventListener('click', handleClick);
		};
	});
</script>

{#if type === 'folder' && isFolder(item)}
	<div class="tree-item">
		<details bind:open={isOpen} ontoggle={handleToggle}>
			<summary
				draggable="true"
				ondragstart={(e) => handleDragStart(e, 'folder', item)}
				ondragover={handleDragOver}
				ondragleave={handleDragLeave}
				ondrop={handleDrop}
				oncontextmenu={(event) => handleContextMenu(event, 'folder', item)}
				class="folder-summary"
				><Icon
					icon={isOpen
						? 'material-symbols-light:folder-open-sharp'
						: 'material-symbols-light:folder-sharp'}
					class="folder-icon"
				/>
				{item.name}</summary
			>
			<ul class="folder-children">
				{#each item.children as child}
					<li class="folder-item">
						<Self item={child} type="folder" />
					</li>
				{/each}
				{#each item.bookmarks as bookmark}
					<li
						class="bookmark-item"
						draggable="true"
						ondragstart={(e) => handleDragStart(e, 'bookmark', bookmark)}
					>
						<a
							href={bookmark.url}
							target="_blank"
							oncontextmenu={(event) => handleContextMenu(event, 'bookmark', bookmark)}
						>
							{bookmark.name}
						</a>
					</li>
				{/each}
			</ul>
		</details>
	</div>
{:else if type === 'bookmark' && isBookmark(item)}
	<li
		class="bookmark-root-item"
		draggable="true"
		ondragstart={(e) => handleDragStart(e, 'bookmark', item)}
	>
		<a
			href={item.url}
			target="_blank"
			oncontextmenu={(event) => handleContextMenu(event, 'bookmark', item)}
		>
			{item.name}
		</a>
	</li>
{/if}

<ContextMenu />

<style>
	.tree-item {
		position: relative;
	}

	ul {
		list-style: none;
		padding-left: 1.5em;
		margin: 0;
		position: relative;
	}

	.folder-children {
		position: relative;
	}

	.folder-children::before {
		content: '';
		position: absolute;
		left: -0.5em;
		top: -1.2em;
		bottom: 0.7em;
		width: 1px;
		background: #ccc;
	}

	li.folder-item,
	li.bookmark-item,
	li.bookmark-root-item {
		margin: 0.5em 0;
		position: relative;
	}

	li.folder-item::before,
	li.bookmark-item::before {
		content: '';
		position: absolute;
		left: -2em;
		top: 0.8em;
		width: 1.5em;
		height: 1px;
		background: #ccc;
	}

	li.folder-item:last-child::after,
	li.bookmark-item:last-child::after {
		content: '';
		position: absolute;
		left: 0;
		top: 0.7em;
		bottom: -0.7em;
		width: 1px;
		background: white;
	}

	summary {
		list-style: none;
		cursor: pointer;
		display: inline-block;
		width: fit-content;
	}

	summary::-webkit-details-marker {
		display: none;
	}

	.folder-summary {
		display: inline-flex;
		align-items: center;
		cursor: pointer;
	}

	a {
		text-decoration: underline;
		color: #0000ee;
		display: inline-flex;
	}

	.folder-summary {
		display: flex;
		align-items: center;
		cursor: pointer;
	}

	:global(.folder-icon) {
		margin-right: 0.5em;
		font-size: 1.2em;
		vertical-align: middle;
	}

	:global(.drag-over) {
		background-color: rgba(0, 0, 0, 0.1);
		border-radius: 4px;
	}
</style>
