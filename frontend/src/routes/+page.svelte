<script lang="ts">
	import TreeItem from '$lib/components/TreeItem.svelte';
	import SearchOverlay from '$lib/components/SearchOverlay.svelte';
	import Icon from '@iconify/svelte';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { rootItemsStore, treeOperations } from '$lib/stores/rootItemsStore';
	import FavoritesBar from '$lib/components/FavoritesBar.svelte';
	import NewItemModal from '$lib/components/NewItemModal.svelte';
	import { resetContextMenu } from '$lib/stores/contextMenuStore';
	import ContextMenu from '$lib/components/ContextMenu.svelte';
	import EditModal from '$lib/components/EditModal.svelte';
	import { dev } from '$app/environment';
	// @ts-expect-error known issue with loading-spinners (#49)
	import Jellyfish from 'svelte-loading-spinners/Jellyfish.svelte';

	function handleKeyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'f':
				if (!showNewItemModal && !showEditModal) {
					showSearch = true;
				}
				break;
			case 'Escape':
				showSearch = false;
				showNewItemModal = false;
				break;
		}
	}

	// Drop-down hamburger menu
	let hamburgerMenuIsOpen = $state(false);
	let dropDownRef = $state<HTMLDivElement>();

	function handleClickOutside(event: MouseEvent) {
		if (dropDownRef && !dropDownRef.contains(event.target as Node)) {
			hamburgerMenuIsOpen = false;
		}
	}

	function toggleDropDown() {
		hamburgerMenuIsOpen = !hamburgerMenuIsOpen;
	}

	onMount(() => {
		treeOperations.refreshFullTree();
		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});

	// Import Bookmarks
	let isLoading = $state(false);

	async function handleFileSelect(event: Event): Promise<void> {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];

		if (!file || !(file.name.endsWith('.html') || file.name.endsWith('.json'))) {
			return;
		}

		let api_url = '';
		let content_type = '';
		isLoading = true;

		try {
			const fileContent = await file.text();

			if (file.name.endsWith('.html')) {
				api_url = dev ? 'http://localhost:8663/api/import-html' : '/api/import-html';
				content_type = 'text/html';
			} else if (file.name.endsWith('.json')) {
				api_url = dev ? 'http://localhost:8663/api/import-linkwarden' : '/api/import-linkwarden';
				content_type = 'text/plain';
			}

			const response = await fetch(api_url, {
				method: 'POST',
				headers: {
					'Content-Type': content_type
				},
				body: fileContent
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			}

			treeOperations.refreshFullTree();
		} catch (error) {
			console.error('Error importing bookmarks:', error);
		} finally {
			isLoading = false;
			hamburgerMenuIsOpen = false;
		}
	}

	function handleInputChange(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		handleFileSelect(event);
	}

	// Export bookmarks
	async function exportBookmarksAsHtml() {
		const response = await fetch(dev ? 'http://localhost:8663/api/export' : '/api/export');
		const bookmarksHtml = await response.text();
		if (bookmarksHtml.trim().length > 0) {
			downloadHtml(bookmarksHtml);
		} // TODO: Otherwise show a pop-up at bottom "No bookmarks to export"
	}

	function downloadHtml(htmlString: string): void {
		const blob = new Blob([htmlString], { type: 'text/html' });
		const url = URL.createObjectURL(blob);

		const link = document.createElement('a');
		link.href = url;
		link.download = 'nadamark-export.html';
		document.body.appendChild(link);
		link.click();

		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}

	// Search
	let showSearch = $state(false);

	// Drag to root
	let isDragging = $state(false);

	async function handleRootDrop(e: DragEvent) {
		e.preventDefault();
		isDragging = false;

		if (!e.dataTransfer) return;

		const data = JSON.parse(e.dataTransfer.getData('application/json'));

		treeOperations.moveItem(data.id, data.type, null);
	}

	// New Item
	let showNewItemModal = $state(false);
	let newItemModalType = $state('');

	// Edit Item
	let showEditModal = $state(false);
	let editModalType = $state('');
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="top-nav">
	<button
		type="button"
		class="nav-item"
		onclick={() => {
			resetContextMenu();
			newItemModalType = 'bookmark';
			showNewItemModal = true;
		}}
	>
		<Icon icon={'material-symbols-light:bookmark-add-sharp'} />
	</button>
	<button
		type="button"
		class="nav-item"
		onclick={() => {
			resetContextMenu();
			newItemModalType = 'folder';
			showNewItemModal = true;
		}}
		><Icon icon={'material-symbols-light:create-new-folder-sharp'} />
	</button>
	<div class="nav-dropdown" bind:this={dropDownRef}>
		<button
			type="button"
			class="nav-item"
			onclick={toggleDropDown}
			aria-expanded={hamburgerMenuIsOpen}
			aria-label="Menu"
		>
			<Icon icon={'game-icons:hamburger-menu'} />
		</button>

		{#if hamburgerMenuIsOpen}
			<div class="dropdown-menu" transition:slide={{ duration: 200 }} role="menu">
				<div class="dropdown-item with-submenu dropdown-item-top">
					<div class="item-content">
						<Icon icon="material-symbols:upload" />
						Import
						<span class="submenu-arrow">
							<Icon icon="material-symbols:chevron-right" />
						</span>
					</div>
					<div class="submenu">
						<label for="bookmarkHTMLUpload" class="dropdown-item dropdown-item-top">
							<Icon icon="material-symbols:upload" />
							Import HTML
						</label>
						<input
							id="bookmarkHTMLUpload"
							type="file"
							accept=".html"
							onchange={handleInputChange}
							disabled={isLoading}
							style="display:none"
						/>
						<label for="bookmarkLinkwardenUpload" class="dropdown-item dropdown-item-bottom">
							<Icon icon="material-symbols:upload" />
							Import Linkwarden JSON
						</label>
						<input
							id="bookmarkLinkwardenUpload"
							type="file"
							accept=".json"
							onchange={handleInputChange}
							disabled={isLoading}
							style="display:none"
						/>
					</div>
				</div>
				<button class="dropdown-item dropdown-item-bottom" onclick={exportBookmarksAsHtml}>
					<Icon icon="material-symbols:download" />
					Export
				</button>
			</div>
		{/if}
	</div>
</div>
{#if !$rootItemsStore.loading}
	{#if $rootItemsStore.data && ($rootItemsStore.data.root_bookmarks.length > 0 || $rootItemsStore.data.root_folders.length > 0)}
		<FavoritesBar />

		<div
			class="root-drop-zone top"
			class:drag-active={isDragging}
			ondragover={(e) => e.preventDefault()}
			ondragenter={() => (isDragging = true)}
			ondragleave={() => (isDragging = false)}
			ondrop={handleRootDrop}
			role="region"
			aria-label="Drop zone for root level items"
		>
			Drop here to un-nest
		</div>

		<div class="tree-view">
			<ul>
				{#each $rootItemsStore.data?.root_folders ?? [] as folder (folder.id)}
					<li>
						{#key folder.id}
							<TreeItem item={folder} type="folder" />
						{/key}
					</li>
				{/each}
				{#each $rootItemsStore.data?.root_bookmarks ?? [] as bookmark (bookmark.id)}
					{#key bookmark.id}
						<TreeItem item={bookmark} type="bookmark" />
					{/key}
				{/each}
			</ul>
		</div>

		<div
			class="root-drop-zone bottom"
			class:drag-active={isDragging}
			ondragover={(e) => e.preventDefault()}
			ondragenter={() => (isDragging = true)}
			ondragleave={() => (isDragging = false)}
			ondrop={handleRootDrop}
			role="region"
			aria-label="Drop zone for root level items"
		>
			Drop here to un-nest
		</div>

		<ContextMenu
			bind:showNewItemModal
			bind:newItemModalType
			bind:showEditModal
			bind:editModalType
		/>

		<SearchOverlay
			isOpen={showSearch}
			close={() => (showSearch = false)}
			select={(bookmark) => {
				showSearch = false;
				const a = document.createElement('a');
				a.href = bookmark.url;
				a.target = '_blank';
				a.rel = 'noreferrer';
				a.click();
			}}
		/>
	{:else if !$rootItemsStore || (!$rootItemsStore.data?.root_bookmarks?.length && !$rootItemsStore.data?.root_folders?.length)}
		<div class="no-data-icon">
			<center
				><Icon
					icon={'material-symbols-light:bookmark-add-sharp'}
					style="font-size:calc(32px + 10vw);"
				/>
				<p style="font-size:calc(12px + 1vw);">
					Add a bookmark or folder, or import some, to get started.
				</p></center
			>
		</div>
	{/if}

	<NewItemModal
		showModal={showNewItemModal}
		type={newItemModalType}
		close={() => (showNewItemModal = false)}
	/>

	<EditModal showModal={showEditModal} type={editModalType} close={() => (showEditModal = false)} />
{:else}
	<div class="no-data-icon">
		<Jellyfish size="60" color="#000" unit="px" duration="1s" />
	</div>
{/if}

<style>
	.top-nav {
		width: 100%;
		height: 1.5em;
		display: flex;
		justify-content: flex-end;
		align-items: center;
		font-size: 2em;
		padding-right: 0.5em;
	}

	.nav-item {
		margin-left: 20px;
		background: none;
		border: none;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.tree-view ul {
		padding-left: 1em;
	}

	.nav-dropdown {
		position: relative;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		background: white;
		border: 1px solid #eee;
		border-radius: 4px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		min-width: 180px;
		z-index: 1000;
		font-size: 0.5em; /* Scale down the dropdown text relative to the nav */
	}

	.dropdown-item {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		color: #333;
		text-decoration: none;
		transition: background-color 0.2s;
		cursor: pointer;
	}

	.dropdown-item:hover {
		background-color: #f5f5f5;
	}

	.dropdown-item.with-submenu {
		position: relative;
	}

	.item-content {
		width: 100%;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.submenu-arrow {
		margin-left: auto;
	}

	.submenu {
		position: absolute;
		left: -230px;
		top: 0;
		background: white;
		border: 1px solid #eee;
		border-radius: 4px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		min-width: 180px;
		display: none;
	}

	.dropdown-item.with-submenu:hover .submenu {
		display: block;
	}

	.root-drop-zone {
		display: none;
		padding: 1rem;
		margin: 0.5rem;
		border: 2px dashed #ccc;
		border-radius: 4px;
		text-align: center;
		color: #666;
	}

	.root-drop-zone.drag-active {
		background-color: rgba(0, 0, 0, 0.05);
	}

	:global(body.dragging) .root-drop-zone {
		display: block;
	}

	.no-data-icon {
		margin: auto;
		position: absolute;
		top: 0;
		bottom: 0;
		left: 0;
		right: 0;
		height: fit-content;
		width: fit-content;
	}

	.dropdown-item-top:hover {
		border-top-right-radius: 5px;
		border-top-left-radius: 5px;
	}

	.dropdown-item-bottom:hover {
		border-bottom-right-radius: 5px;
		border-bottom-left-radius: 5px;
	}

	@media (prefers-color-scheme: dark) {
		.dropdown-menu,
		.submenu {
			background: #303030;
		}

		.dropdown-item {
			color: var(--color-gray-300);
		}

		.dropdown-item:hover {
			background-color: #505050;
		}
	}
</style>
