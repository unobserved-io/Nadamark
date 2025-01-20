<script lang="ts">
	import type { Bookmark, FolderNode } from '$lib/types';
	import { rootItemsStore } from '$lib/stores/rootItemsStore';
	import { getAllFavorites } from '$lib/utils/allBookmarks';
	import { contextMenuStore, openContextMenu } from '$lib/stores/contextMenuStore';
	import { onMount } from 'svelte';

	let allFavorites: Bookmark[] = $state([]);

	$effect(() => {
		if ($rootItemsStore.data && !$rootItemsStore.loading) {
			allFavorites = getAllFavorites($rootItemsStore.data);
		}
	});

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

{#if allFavorites.length > 0}
	<div class="favorites">
		<ul>
			{#each allFavorites as favorite}
				<li>
					<a
						href={favorite.url}
						target="_blank"
						oncontextmenu={(event) => handleContextMenu(event, 'bookmark', favorite)}
						>{favorite.name}</a
					>
				</li>
			{/each}
		</ul>
	</div>
{/if}

<style>
	/* TODO: Make this max one line height */
	.favorites {
		padding: 1rem;
		white-space: nowrap;
	}

	.favorites ul {
		border: 1px dashed #000;
		padding: 0.3rem 0.5rem;
		list-style: none;
		display: flex;
	}

	.favorites ul > li {
		margin-right: 1rem;
	}

	.favorites ul > li:last-child {
		margin-right: 0px;
	}

	.favorites ul > li > a {
		text-decoration: underline;
		color: #0000ee;
	}
</style>
