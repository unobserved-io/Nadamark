<script lang="ts">
	import { contextMenuStore } from '$lib/stores/contextMenuStore';
	import { treeOperations } from '$lib/stores/rootItemsStore';
	import type { Bookmark } from '$lib/types';

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
		$contextMenuStore.isOpen = false;
		newItemModalType = type;
		showNewItemModal = true;
	}

	function handleShowEditModal(type: string) {
		$contextMenuStore.isOpen = false;
		editModalType = type;
		showEditModal = true;
	}

	// Favorite
	async function toggleFavorite() {
		$contextMenuStore.isOpen = false;
		if ($contextMenuStore.data) {
			await treeOperations.toggleFavorite($contextMenuStore.data.id, $contextMenuStore.parentId);
		}
	}

	// Delete Item
	async function deleteItem(type: string) {
		$contextMenuStore.isOpen = false;
		if ($contextMenuStore.data) {
			await treeOperations.deleteItem($contextMenuStore.data.id, type as 'folder' | 'bookmark');
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
		border: 0px;
		background-color: #f5f5f5;
	}

	.context-menu li:first-child:hover {
		border-top-right-radius: 5px;
		border-top-left-radius: 5px;
	}

	.context-menu li:last-child:hover {
		border-bottom-right-radius: 5px;
		border-bottom-left-radius: 5px;
	}

	@media (prefers-color-scheme: dark) {
		.context-menu {
			background: #303030;
		}

		.context-menu li:hover {
			background-color: #505050;
		}
	}
</style>
