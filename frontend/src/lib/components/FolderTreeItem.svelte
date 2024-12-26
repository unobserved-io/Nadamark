<script lang="ts">
	import { onMount } from 'svelte';
	import Icon from '@iconify/svelte';
	import { dndzone } from 'svelte-dnd-action';
	import type { FolderNode } from '$lib/types';

	export let folder: FolderNode;

	// Open/closed folders
	let isOpen = false;
	function handleToggle() {
		isOpen = !isOpen;
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
		pos = { x: event.clientX, y: event.clientY };
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

<div class="tree-item">
	<details ontoggle={handleToggle}>
		<summary
			oncontextmenu={(event) => handleContextMenu(event, 'folder', folder)}
			class="folder-summary"
			><Icon
				icon={isOpen
					? 'material-symbols-light:folder-open-sharp'
					: 'material-symbols-light:folder-sharp'}
				class="folder-icon"
			/>
			{folder.name}</summary
		>
		<ul class="folder-children">
			{#each folder.children as childFolder}
				<li class="folder-item">
					<svelte:self folder={childFolder} />
				</li>
			{/each}
			{#each folder.bookmarks as bookmark}
				<li class="bookmark-item">
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

{#if showMenu}
	<nav style="position: absolute; top: {pos.y}px; left: {pos.x}px" class="context-menu">
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
		display: inline;
	}

	summary::-webkit-details-marker {
		display: none;
	}

	.folder-summary {
		display: flex;
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
</style>
