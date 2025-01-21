<script lang="ts">
	import { contextMenuStore } from '$lib/stores/contextMenuStore';
	import { refreshTree } from '$lib/stores/rootItemsStore';
	import type { Bookmark } from '$lib/types';
	import { dev } from '$app/environment';

	let {
		showNewItemModal = $bindable(),
		newItemModalType = $bindable(),
		showEditModal = $bindable(),
		editModalType = $bindable()
	} = $props<{
		showNewItemModal: boolean;
		newItemModalType: string;
		showEditModal: boolean;
		editModalType: string;
	}>();

	function handleShowNewItemModal(type: string) {
		newItemModalType = type;
		showNewItemModal = true;
	}

	function handleShowEditModal(type: string) {
		editModalType = type;
		showEditModal = true;
	}

	// Favorite
	async function toggleFavorite() {
		$contextMenuStore.isOpen = false;
		try {
			const response = await fetch(
				dev ? 'http://localhost:8663/api/favorite-bookmark' : '/api/favorite-bookmark',
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify($contextMenuStore.data?.id)
				}
			);

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			} else {
				refreshTree();
			}
		} catch (error) {
			console.error('Error toggling favorite:', error);
		}
	}

	// Delete Item
	async function deleteItem(type: string) {
		$contextMenuStore.isOpen = false;
		let apiUrl: string = '';
		try {
			if (type == 'folder') {
				apiUrl = dev ? 'http://localhost:8663/api/delete-folder' : '/api/delete-folder';
			} else if (type == 'bookmark') {
				apiUrl = dev ? 'http://localhost:8663/api/delete-bookmark' : '/api/delete-bookmark';
			}
			const response = await fetch(apiUrl, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify($contextMenuStore.data?.id)
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			} else {
				refreshTree();
			}
		} catch (error) {
			console.error('Error deleting item:', error);
		}
	}

	// Open menu 'up' if it goes past the bottom of the page
	let menuElement = $state<HTMLElement>();
	let adjustedY = $state($contextMenuStore.position.y);

	$effect(() => {
		if (menuElement && $contextMenuStore.isOpen) {
			const menuHeight = menuElement.offsetHeight;
			const viewportHeight = window.innerHeight;
			const scrollY = window.scrollY;
			const wouldOverflow = $contextMenuStore.position.y + menuHeight > viewportHeight + scrollY;

			adjustedY = wouldOverflow
				? $contextMenuStore.position.y - menuHeight
				: $contextMenuStore.position.y;
		}
	});
</script>

{#if $contextMenuStore.isOpen}
	<nav
		bind:this={menuElement}
		style="position: absolute; top: {adjustedY}px; left: {$contextMenuStore.position.x}px"
		class="context-menu"
	>
		<div>
			<ul>
				{#if $contextMenuStore.type === 'folder'}
					<li>
						<button onclick={() => handleShowNewItemModal('folder')}>New folder</button>
					</li>
					<li>
						<button onclick={() => handleShowNewItemModal('bookmark')}>New bookmark</button>
					</li>
					<li>
						<button onclick={() => handleShowEditModal('folder')}>Edit</button>
					</li>
					<li>
						<button onclick={() => deleteItem('folder')}>Delete</button>
					</li>
				{:else if $contextMenuStore.type === 'bookmark'}
					<li>
						<button onclick={toggleFavorite}
							>{($contextMenuStore.data as Bookmark)?.favorite ? 'Unfavorite' : 'Favorite'}</button
						>
					</li>
					<li>
						<button onclick={() => handleShowEditModal('bookmark')}>Edit</button>
					</li>
					<li>
						<button onclick={() => deleteItem('bookmark')}>Delete</button>
					</li>
				{/if}
			</ul>
		</div>
	</nav>
{/if}

<style>
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
		padding: 0em;
	}

	.context-menu li {
		width: 100%;
		display: flex;
		transition: background-color 0.2s;
	}

	.context-menu button {
		background: none;
		border: none;
		cursor: pointer;
		padding: 0.5em;
		width: 100%;
		text-align: left;
	}

	.context-menu li:hover {
		background-color: #f5f5f5;
	}
</style>
