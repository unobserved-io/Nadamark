<script lang="ts">
	import { tick } from 'svelte';
	import { fade } from 'svelte/transition';
	import type { Bookmark } from '$lib/types';
	import { getAllBookmarks } from '$lib/utils/allBookmarks';
	import { rootItemsStore } from '$lib/stores/rootItemsStore';

	let {
		isOpen = false,
		placeholder = 'Search...',
		close,
		select
	} = $props<{
		isOpen?: boolean;
		placeholder?: string;
		close: () => void;
		select: (bookmark: Bookmark) => void;
	}>();

	let allBookmarks: Bookmark[] = $state([]);
	let searchTerm = $state('');
	let results: Bookmark[] = $state([]);
	let selectedIndex = $state(-1);
	let searchInput = $state<HTMLInputElement>();

	$effect(() => {
		if (isOpen) {
			if ($rootItemsStore.data && !$rootItemsStore.loading) {
				allBookmarks = getAllBookmarks($rootItemsStore.data);
			}
			tick().then(() => {
				setTimeout(() => {
					searchInput?.focus();
				}, 100);
			});
		}
	});

	function preventDefault(fn: (event: MouseEvent) => void) {
		return function (this: HTMLAnchorElement, event: MouseEvent) {
			event.preventDefault();
			fn.call(this, event);
		};
	}

	async function handleSearch() {
		if (!searchTerm) {
			results = [];
			return;
		}

		results = allBookmarks.filter((item) =>
			item.name.toLowerCase().includes(searchTerm.toLowerCase())
		);
	}

	function handleClose() {
		searchTerm = '';
		results = [];
		close();
	}

	function selectItem() {
		if (selectedIndex >= 0) {
			select(results[selectedIndex]);
			selectedIndex = -1;
			handleClose();
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		switch (event.key) {
			case 'Escape':
				handleClose();
				break;
			case 'ArrowDown':
				selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
				event.preventDefault();
				break;
			case 'ArrowUp':
				selectedIndex = Math.max(selectedIndex - 1, -1);
				event.preventDefault();
				break;
			case 'Enter':
				selectItem();
				break;
		}
	}
</script>

{#if isOpen}
	<div role="dialog" aria-modal="true" class="fixed inset-0 z-40" transition:fade>
		<button
			type="button"
			class="absolute inset-0 h-full w-full cursor-default bg-black/50"
			onclick={handleClose}
			aria-label="Close search"
		></button>

		<div class="fixed inset-x-4 top-8 z-50 mx-auto max-w-2xl" transition:fade>
			<div class="rounded-xl bg-white shadow-xl">
				<input
					type="search"
					bind:this={searchInput}
					bind:value={searchTerm}
					oninput={handleSearch}
					onkeydown={handleKeydown}
					{placeholder}
					class="w-full rounded-xl border-none p-4 focus:ring-0"
				/>

				{#if results.length > 0}
					<ul class="max-h-96 overflow-y-auto border-t">
						{#each results as result, i}
							<li>
								<a
									href={result.url}
									class="block w-full cursor-pointer p-4 text-left"
									class:bg-gray-100={i === selectedIndex}
									class:rounded-b-xl={i === results.length - 1}
									onclick={preventDefault(() => selectItem())}
									onfocus={() => ({})}
									onmouseover={() => (selectedIndex = i)}
								>
									<h3 class="font-medium">{result.name}</h3>
								</a>
							</li>
						{/each}
					</ul>
				{/if}
			</div>
		</div>
	</div>
{/if}
