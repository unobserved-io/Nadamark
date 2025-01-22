<script lang="ts">
	import type { Bookmark } from '$lib/types';
	import { rootItemsStore } from '$lib/stores/rootItemsStore';
	import { getAllFavorites } from '$lib/utils/allBookmarks';
	import { handleContextMenu } from '$lib/stores/contextMenuStore';
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';

	let allFavorites: Bookmark[] = $state([]);
	let visibleFavorites: Bookmark[] = $state([]);
	let overflowFavorites: Bookmark[] = $state([]);
	let favoritesContainer = $state<HTMLDivElement>();
	let isOverflowMenuOpen = $state(false);

	$effect(() => {
		if ($rootItemsStore.data && !$rootItemsStore.loading) {
			allFavorites = getAllFavorites($rootItemsStore.data);
			requestAnimationFrame(() => {
				updateVisibleFavorites();
			});
		}
	});

	function updateVisibleFavorites() {
		if (!favoritesContainer) return;
		const ul = favoritesContainer.querySelector('ul');
		if (!ul) return;

		visibleFavorites = allFavorites;

		requestAnimationFrame(() => {
			const containerWidth = ul.clientWidth;
			const items = Array.from(ul.querySelectorAll('li:not(.overflow-menu)'));

			const OVERFLOW_BUTTON_WIDTH = 40;
			const ITEM_GAP = 8;
			const availableWidth = containerWidth - OVERFLOW_BUTTON_WIDTH - ITEM_GAP;

			let currentWidth = 0;
			let breakIndex = 0;

			// Include gap in width calculations
			for (let i = 0; i < items.length; i++) {
				const item = items[i] as HTMLLIElement;
				const itemWidth = item.offsetWidth + ITEM_GAP;

				if (currentWidth + itemWidth > availableWidth) {
					break;
				}

				currentWidth += itemWidth;
				breakIndex = i + 1;
			}

			visibleFavorites = allFavorites.slice(0, breakIndex);
			overflowFavorites = allFavorites.slice(breakIndex);
		});
	}

	onMount(() => {
		const handleResize = () => {
			updateVisibleFavorites();
		};

		window.addEventListener('resize', handleResize);

		const handleClickOutside = (event: MouseEvent) => {
			if (isOverflowMenuOpen && event.target instanceof Node) {
				const overflowMenu = favoritesContainer?.querySelector('.overflow-menu');
				if (overflowMenu && !overflowMenu.contains(event.target)) {
					isOverflowMenuOpen = false;
				}
			}
		};

		document.addEventListener('click', handleClickOutside);

		return () => {
			document.removeEventListener('click', handleClickOutside);
			window.removeEventListener('resize', handleResize);
		};
	});
</script>

{#if allFavorites.length > 0}
	<div class="favorites" bind:this={favoritesContainer}>
		<ul>
			{#each visibleFavorites as favorite}
				<li>
					<a
						href={favorite.url}
						target="_blank"
						oncontextmenu={(event) => handleContextMenu(event, 'bookmark', favorite)}
						>{favorite.name}</a
					>
				</li>
			{/each}
			<li class="overflow-menu" class:hidden={overflowFavorites.length === 0}>
				<button
					type="button"
					onclick={() => (isOverflowMenuOpen = !isOverflowMenuOpen)}
					class="overflow-button"
					title="Show more bookmarks">...</button
				>
				{#if isOverflowMenuOpen}
					<div class="overflow-dropdown" transition:slide={{ duration: 200 }} role="menu">
						<ul>
							{#each overflowFavorites as favorite}
								<li>
									<a
										href={favorite.url}
										target="_blank"
										oncontextmenu={(event) => handleContextMenu(event, 'bookmark', favorite)}
										>{favorite.name}</a
									>
								</li>
							{/each}
						</ul>
					</div>
				{/if}
			</li>
		</ul>
	</div>
{/if}

<style>
	.favorites {
		padding: 1rem;
		position: relative;
		width: 100%;
	}

	.favorites ul {
		border: 1px dashed #000;
		padding: 0.3rem 0.5rem;
		list-style: none;
		display: flex;
		flex-wrap: nowrap;
		gap: 8px;
		align-items: center;
		width: 100%;
	}

	.favorites li {
		flex: 0 0 auto;
		white-space: nowrap;
	}

	.favorites li a {
		text-decoration: underline;
		color: #0000ee;
		display: block;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.overflow-menu {
		position: relative;
		margin-left: auto;
		z-index: 1000;
	}

	.overflow-menu.hidden {
		display: none;
	}

	.overflow-button {
		background: none;
		border: none;
		cursor: pointer;
		font-weight: bold;
		padding: 0.25rem 0.5rem;
		font-size: 1.2em;
		border-radius: 4px;
	}

	.overflow-button:hover {
		background-color: #f0f0f0;
	}

	.overflow-dropdown {
		position: absolute;
		top: 100%;
		right: 0;
		background: white;
		border: 1px solid #ccc;
		border-radius: 4px;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		z-index: 1000;
		max-width: 80vw !important;
		margin-top: 4px;
	}

	.overflow-dropdown ul {
		border: none;
		padding: 0.5rem;
		display: block;
		width: auto;
		flex-direction: column;
		overflow: visible;
	}

	.overflow-dropdown li {
		margin: 0.25rem 0;
	}
</style>
