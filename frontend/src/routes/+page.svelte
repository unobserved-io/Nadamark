<script lang="ts">
	import TreeItem from '$lib/components/TreeItem.svelte';
	import SearchOverlay from '$lib/components/SearchOverlay.svelte';
	import NewItemModal from '$lib/components/NewItemModal.svelte';
	import Icon from '@iconify/svelte';
	import type { RootItems } from '$lib/types';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';

	let data: { folderTree: RootItems } = { folderTree: { root_folders: [], root_bookmarks: [] } };

	async function refreshTree() {
		const treeResult = await fetch('http://localhost:3096/api/folder-tree');
		data.folderTree = await treeResult.json();
	}

	function handleKeyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'f':
				showSearch = true;
				break;
			case 'Escape':
				showSearch = false;
				break;
		}
	}

	// Drop-down hamburger menu
	let hamburgerMenuIsOpen = false;
	let dropDownRef: HTMLDivElement;

	function handleClickOutside(event: MouseEvent) {
		if (dropDownRef && !dropDownRef.contains(event.target as Node)) {
			hamburgerMenuIsOpen = false;
		}
	}

	function toggleDropDown() {
		hamburgerMenuIsOpen = !hamburgerMenuIsOpen;
	}

	onMount(() => {
		refreshTree();
		document.addEventListener('click', handleClickOutside);
		return () => {
			document.removeEventListener('click', handleClickOutside);
		};
	});

	// Import Bookmarks
	let fileInput: HTMLInputElement;
	let status: string = '';
	let isLoading: boolean = false;

	async function handleFileSelect(event: Event): Promise<void> {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];

		if (!file || !file.name.endsWith('.html')) {
			status = 'Please select a valid HTML bookmarks file';
			return;
		}

		isLoading = true;
		status = 'Reading file...';

		try {
			const fileContent = await file.text();

			const response = await fetch('http://localhost:3096/api/import-bookmarks', {
				method: 'POST',
				headers: {
					'Content-Type': 'text/html'
				},
				body: fileContent
			});

			if (!response.ok) {
				throw new Error(`HTTP error! status: ${response.status}`);
			} else {
				refreshTree();
				// TODO: Have this be a pop-up at the bottom of the page to show success or failure
				status = `Successfully imported bookmarks`;
			}
		} catch (error) {
			console.error('Error importing bookmarks:', error);
			status = `Error importing bookmarks: ${error instanceof Error ? error.message : 'Unknown error'}`;
		} finally {
			isLoading = false;
			// Reset file input
			fileInput.value = '';
		}
	}

	function handleInputChange(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
		handleFileSelect(event);
	}

	// Export bookmarks
	async function exportBookmarksAsHtml() {
		const response = await fetch('http://localhost:3096/api/export');
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
	let showSearch = false;

	// New Item Modal
	let showNewItemModal = false;

	function openNewItemModal() {
		showNewItemModal = true;
	}
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="top-nav">
	<div class="nav-item"><Icon icon={'material-symbols-light:bookmark-add-sharp'} /></div>
	<div class="nav-item"><Icon icon={'material-symbols-light:create-new-folder-sharp'} /></div>
	<div class="nav-dropdown" bind:this={dropDownRef}>
		<button
			type="button"
			class="nav-item"
			onclick={toggleDropDown}
			aria-expanded={hamburgerMenuIsOpen}
			aria-label="Menu"
		>
			<Icon icon={'pepicons-pencil:hamburger'} />
		</button>

		{#if hamburgerMenuIsOpen}
			<div class="dropdown-menu" transition:slide={{ duration: 200 }} role="menu">
				<label for="bookmarkHTMLUpload" class="dropdown-item">
					<Icon icon="material-symbols:upload" />
					Import
				</label>
				<input
					id="bookmarkHTMLUpload"
					bind:this={fileInput}
					type="file"
					accept=".html"
					onchange={handleInputChange}
					disabled={isLoading}
					style="display:none"
				/>
				<button class="dropdown-item" onclick={exportBookmarksAsHtml}>
					<Icon icon="material-symbols:download" />
					Export
				</button>
			</div>
		{/if}
	</div>
</div>
<div class="tree-view">
	<ul>
		{#each data.folderTree.root_folders as folder}
			<li>
				<TreeItem
					item={folder}
					type="folder"
					{refreshTree}
					showNewItemModal={() => (showNewItemModal = true)}
				/>
			</li>
		{/each}
		{#each data.folderTree.root_bookmarks as bookmark}
			<TreeItem
				item={bookmark}
				type="bookmark"
				{refreshTree}
				showNewItemModal={() => (showNewItemModal = true)}
			/>
		{/each}
	</ul>
</div>

<SearchOverlay
	isOpen={showSearch}
	folderTree={data.folderTree}
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

<NewItemModal showModal={showNewItemModal} folderTree={data.folderTree} type="folder" />

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
</style>
