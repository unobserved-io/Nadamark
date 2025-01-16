<script lang="ts">
	import type { Bookmark } from '$lib/types';
	import { rootItemsStore } from '$lib/stores/rootItemsStore';
	import { getAllFavorites } from '$lib/utils/allBookmarks';

	let allFavorites: Bookmark[] = $state([]);

	$effect(() => {
		allFavorites = getAllFavorites($rootItemsStore);
	});
</script>

{#if allFavorites.length > 0}
	<div class="favorites">
		<ul>
			{#each allFavorites as favorite}
				<li><a href={favorite.url} target="_blank">{favorite.name}</a></li>
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
