<script lang="ts">
	import type { Folder } from '$lib/types';
	import { getAllFolders } from '$lib/utils/allFolders';
	import { contextMenuStore } from '$lib/stores/contextMenuStore';
	import { rootItemsStore, treeOperations } from '$lib/stores/rootItemsStore';
	import { tick } from 'svelte';

	let {
		showModal = false,
		type,
		close
	} = $props<{
		showModal: boolean;
		type: string;
		close: () => void;
	}>();

	let allFolders: Folder[] = $state([]);
	let selectedFolder: number | null = $state(null);
	let itemName = $state('');
	let itemUrl = $state('');
	let errorMsg: string | null = $state(null);
	let nameInput = $state<HTMLInputElement>();

	$effect(() => {
		if (showModal) {
			if ($rootItemsStore.data && !$rootItemsStore.loading) {
				allFolders = getAllFolders($rootItemsStore.data);
			}
			selectedFolder = $contextMenuStore.data?.id ?? null;
			tick().then(() => {
				setTimeout(() => {
					nameInput?.focus();
				}, 100);
			});
		}
	});

	async function handleSave(event: Event) {
		event.preventDefault();
		if ((type == 'folder' || validateUrl(itemUrl)) && itemName.length > 0) {
			if (type == 'folder') {
				treeOperations.newFolder(itemName, selectedFolder);
				closeModal();
			} else if (type == 'bookmark') {
				treeOperations.newBookmark(itemName, itemUrl, selectedFolder);
				closeModal();
			} else {
				console.error('No data in contextMenuStore.data.');
			}
		} else {
			if (itemName.length <= 0) {
				errorMsg = 'Please give it a name.';
			} else if (!validateUrl(itemUrl)) {
				errorMsg = 'Please enter a valid URL (with https:// etc.)';
			}
		}
	}

	function validateUrl(url: string) {
		try {
			new URL(url);
			return true;
		} catch {
			return false;
		}
	}

	function handleKeyDown(event: KeyboardEvent) {
		switch (event.key) {
			case 'Escape':
				closeModal();
				break;
		}
	}

	function closeModal() {
		itemName = '';
		itemUrl = '';
		allFolders = [];
		selectedFolder = null;
		errorMsg = null;
		close();
	}
</script>

<svelte:window onkeydown={handleKeyDown} />

{#if showModal}
	<div class="modal">
		<div class="modal-content">
			<button class="close" onclick={closeModal}>&times;</button>
			{#if type == 'folder'}
				<h1>New Folder</h1>
				{#if errorMsg}
					<div class="error-message">
						<span>{errorMsg}</span>
					</div>
				{/if}
				<form onsubmit={handleSave}>
					<div class="form-group">
						<label for="name">Name</label>
						<input
							type="text"
							name="name"
							autocomplete="off"
							data-1p-ignore
							data-lpignore="true"
							data-protonpass-ignore="true"
							bind:value={itemName}
							bind:this={nameInput}
						/>
					</div>

					<div class="form-group">
						<label for="location">Location</label>
						<select name="folders" id="location" bind:value={selectedFolder}>
							<option value={null}></option>
							{#each allFolders as folder}
								<option value={folder.id}>{folder.name}</option>
							{/each}
						</select>
					</div>

					<center>
						<button type="submit" class="save">Save</button>
					</center>
				</form>
			{:else if type == 'bookmark'}
				<h1>New Bookmark</h1>
				{#if errorMsg}
					<div class="error-message">
						<span>{errorMsg}</span>
					</div>
				{/if}
				<form onsubmit={handleSave}>
					<div class="form-group">
						<label for="name">Name</label>
						<input
							type="text"
							name="name"
							autocomplete="off"
							data-1p-ignore
							data-lpignore="true"
							data-protonpass-ignore="true"
							bind:value={itemName}
							bind:this={nameInput}
						/>
					</div>

					<div class="form-group">
						<label for="url">URL</label>
						<input
							type="text"
							name="url"
							autocomplete="off"
							data-1p-ignore
							data-lpignore="true"
							data-protonpass-ignore="true"
							bind:value={itemUrl}
						/>
					</div>

					<div class="form-group">
						<label for="location">Location</label>
						<select name="folders" id="location" bind:value={selectedFolder}>
							<option value={null}></option>
							{#each allFolders as folder}
								<option value={folder.id}>{folder.name}</option>
							{/each}
						</select>
					</div>

					<center>
						<button type="submit" class="save">Save</button>
					</center>
				</form>
			{/if}
		</div>
	</div>
{/if}

<style>
	.modal {
		position: fixed;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background: rgba(0, 0, 0, 0.5);
		backdrop-filter: blur(4px);
		z-index: 1000;
	}

	.modal-content {
		background: white;
		border: 1px solid black;
		position: relative;
		margin: 15% auto;
		padding: 2rem;
		border-radius: 12px;
		max-width: 500px;
		width: 90%;
	}

	.close {
		position: absolute;
		right: 1rem;
		top: 0.5rem;
		font-size: 1.5rem;
		cursor: pointer;
	}

	h1 {
		text-align: center;
		margin-bottom: 1.7rem;
		font-size: 1.8rem;
	}

	.form-group {
		margin-bottom: 1rem;
		padding: 0;
	}

	label {
		display: block;
		margin-bottom: 0.5rem;
		color: var(--text-primary);
	}

	input {
		width: 100%;
		padding: 0.75rem;
		border: 1px solid black;
		border-radius: 6px;
		font-size: 1rem;
		box-sizing: border-box;
		transition: border-color 0.2s ease;
	}

	input:focus {
		outline: none;
		border-color: black;
	}

	.save {
		background: black;
		color: white;
		border: none;
		padding: 0.75rem 1rem;
		border-radius: 6px;
		width: 7rem;
		font-size: 1rem;
		cursor: pointer;
		transition: background-color 0.3s;
	}

	.save:hover {
		background: rgba(0, 0, 0, 0.7);
	}

	.error-message {
		background-color: #f8d7da;
		border: 1px solid #f5c6cb;
		color: #721c24;
		padding: 0.75rem;
		border-radius: 4px;
		margin-bottom: 1rem;
	}
</style>
