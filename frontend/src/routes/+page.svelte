<script lang="ts">
	import TreeItem from '$lib/components/TreeItem.svelte';
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
	.tree-view ul {
		padding-left: 1em;
	}
</style>
