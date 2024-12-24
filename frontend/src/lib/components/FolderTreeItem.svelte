<script lang="ts">
	import type { FolderNode } from '$lib/types';

	export let folder: FolderNode;
</script>

<details>
	<summary>{folder.name}</summary>
	<ul>
		{#each folder.children as childFolder}
			<li>
				<svelte:self folder={childFolder} />
			</li>
		{/each}
		{#each folder.bookmarks as bookmark}
			<li>
				<a href={bookmark.url}>{bookmark.name}</a>
			</li>
		{/each}
	</ul>
</details>

<style>
	ul {
		list-style: none;
		padding-left: 1.5em;
	}

	li {
		margin: 0.5em 0;
	}

	summary {
		cursor: pointer;
		font-weight: bold;
	}

	a {
		text-decoration: none;
		color: #0066cc;
	}
</style>
