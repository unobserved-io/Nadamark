<script lang="ts">
	import TreeItem from '$lib/components/TreeItem.svelte';
	import Icon from '@iconify/svelte';
	import type { RootItems } from '$lib/types';

	interface PageData {
		folderTree: RootItems;
	}

	export let data: PageData;

	async function refreshTree() {
		const treeResult = await fetch('http://localhost:3096/api/folder-tree');
		data.folderTree = await treeResult.json();
	}
</script>

<div class="top-nav">
	<div class="nav-item"><Icon icon={'material-symbols-light:bookmark-add-sharp'} /></div>
	<div class="nav-item"><Icon icon={'material-symbols-light:create-new-folder-sharp'} /></div>
	<div class="nav-item"><Icon icon={'pepicons-pencil:hamburger'} /></div>
</div>
<div class="tree-view">
	<ul>
		{#each data.folderTree.root_folders as folder}
			<li><TreeItem item={folder} type="folder" on:refreshTree={refreshTree} /></li>
		{/each}
		{#each data.folderTree.root_bookmarks as bookmark}
			<TreeItem item={bookmark} type="bookmark" on:refreshTree={refreshTree} />
		{/each}
	</ul>
</div>

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
		margin-left: 20px; /* Add spacing between items */
	}

	.tree-view ul {
		padding-left: 1em;
	}
</style>
