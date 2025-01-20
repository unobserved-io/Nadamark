<script lang="ts">
	import { contextMenuStore } from '$lib/stores/contextMenuStore';
	import { refreshTree } from '$lib/stores/rootItemsStore';
	import type { Bookmark } from '$lib/types';
	import EditModal from './EditModal.svelte';
	import NewItemModal from './NewItemModal.svelte';

	let { showNewItemModal, newItemModalType } = $props<{
		showNewItemModal: boolean;
		newItemModalType: string;
	}>();

	// New Item Modal
	// let newItemModalType = $state('');
	// let showNewItemModal = $state(false);

	function handleShowNewItemModal(type: string) {
		newItemModalType = type;
		showNewItemModal = true;
	}

	// Edit Modal
	let editModalType = $state('');
	let showEditModal = $state(false);

	function handleShowEditModal(type: string) {
		editModalType = type;
		showEditModal = true;
	}

	// Favorite
	async function toggleFavorite() {
		$contextMenuStore.isOpen = false;
		try {
			const response = await fetch('http://localhost:3096/api/favorite-bookmark', {
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
			console.error('Error toggling favorite:', error);
		}
	}

	// Delete Item
	async function deleteItem(type: string) {
		$contextMenuStore.isOpen = false;
		let apiUrl: string = '';
		try {
			if (type == 'folder') {
				apiUrl = 'http://localhost:3096/api/delete-folder';
			} else if (type == 'bookmark') {
				apiUrl = 'http://localhost:3096/api/delete-bookmark';
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
</script>

{#if $contextMenuStore.isOpen}
	<nav
		style="position: fixed; top: {$contextMenuStore.position.y}px; left: {$contextMenuStore.position
			.x}px"
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

<NewItemModal
	showModal={showNewItemModal}
	type={newItemModalType}
	close={() => (showNewItemModal = false)}
/>

<EditModal showModal={showEditModal} type={editModalType} close={() => (showEditModal = false)} />

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
