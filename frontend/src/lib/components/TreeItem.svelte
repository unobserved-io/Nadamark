<script lang="ts">
	import { onMount, createEventDispatcher } from 'svelte';
	import Icon from '@iconify/svelte';
	import type { Bookmark, FolderNode } from '$lib/types';

	export let item: FolderNode | Bookmark;
	export let type: 'folder' | 'bookmark';

	const dispatch = createEventDispatcher();

	function isFolder(item: FolderNode | Bookmark): item is FolderNode {
		return 'children' in item;
	}

	function isBookmark(item: FolderNode | Bookmark): item is Bookmark {
		return 'url' in item;
	}

	// Open/closed folders
	let isOpen = false;
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

			dispatch('refreshTree');
		} catch (err) {
			console.error('Drop failed:', err);
		}
	}

	// Context menu
	let showMenu = false;
	let menuType: 'folder' | 'bookmark' | null = null;
	let menuData: FolderNode | Bookmark | null = null;
	let pos = { x: 0, y: 0 };

	function handleContextMenu(
		event: MouseEvent,
		type: 'folder' | 'bookmark',
		data: FolderNode | Bookmark
	) {
		event.preventDefault();
		pos = { x: event.pageX, y: event.pageY };
		menuType = type;
		menuData = data;
		showMenu = true;
	}

	function closeContextMenu() {
		showMenu = false;
		menuType = null;
		menuData = null;
	}

	onMount(() => {
		document.addEventListener('click', closeContextMenu);
		return () => {
			document.removeEventListener('click', closeContextMenu);
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
						<svelte:self item={child} type="folder" on:refreshTree />
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
		class="bookmark-item"
		draggable="true"
		ondragstart={(e) => handleDragStart(e, 'bookmark', item)}
	>
		<a href={item.url} oncontextmenu={(event) => handleContextMenu(event, 'bookmark', item)}>
			{item.name}
		</a>
	</li>
{/if}

{#if showMenu}
	<nav style="position: fixed; top: {pos.y}px; left: {pos.x}px" class="context-menu">
		<div>
			<ul>
				{#if menuType === 'folder'}
					<li>
						<button onclick={() => console.log('New folder inside', menuData)}>New Folder</button>
					</li>
					<li><button onclick={() => console.log('Rename folder', menuData)}>Rename</button></li>
					<li><button onclick={() => console.log('Delete folder', menuData)}>Delete</button></li>
				{:else if menuType === 'bookmark'}
					<li><button onclick={() => console.log('Open bookmark', menuData)}>Open</button></li>
					<li><button onclick={() => console.log('Rename bookmark', menuData)}>Rename</button></li>
					<li><button onclick={() => console.log('Delete bookmark', menuData)}>Delete</button></li>
				{/if}
			</ul>
		</div>
	</nav>
{/if}

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

	li {
		margin: 0.5em 0;
		position: relative;
	}

	li::before {
		content: '';
		position: absolute;
		left: -2em;
		top: 0.8em;
		width: 1.5em;
		height: 1px;
		background: #ccc;
	}

	li:last-child::after {
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

	.context-menu {
		background: #fff;
		border: 1px solid #ccc;
		border-radius: 5px;
		box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
		z-index: 1000;
	}
	.context-menu ul {
		list-style: none;
		margin: 0;
		padding: 0.5em;
	}
	.context-menu li {
		margin: 0.3em 0;
	}
	.context-menu button {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5em;
		width: 100%;
		text-align: left;
	}

	:global(.drag-over) {
		background-color: rgba(0, 0, 0, 0.1);
		border-radius: 4px;
	}
</style>
