<script lang="ts">
	import FolderTreeItem from '$lib/components/FolderTreeItem.svelte';
	import type { FolderNode } from '$lib/types';

	interface PageData {
		folderTree: FolderNode[];
	}

	export let data: PageData;
</script>

<h1>Bookmarks</h1>
<div class="tree-view">
	<ul>
		{#each data.folderTree as folder}
			<li>
				<FolderTreeItem {folder} />
			</li>
		{/each}
	</ul>
</div>

<style>
	.tree-view ul {
		list-style: none;
		padding-left: 1em;
	}
</style>
