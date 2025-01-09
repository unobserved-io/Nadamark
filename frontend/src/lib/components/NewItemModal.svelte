<script lang="ts">
	import type { Folder, RootItems } from '$lib/types';
	import { getAllFolders } from '$lib/utils/allFolders';
	import { contextMenuStore } from '$lib/stores/contextMenuStore';

	let {
		showModal = false,
		type,
		folderTree,
		refreshTree
	} = $props<{
		showModal: boolean;
		type: string;
		folderTree: RootItems;
		refreshTree: () => void;
	}>();

	let allFolders: Folder[] = $state([]);
	let selectedFolder: number | undefined = $state();
	let itemName = $state('');

	$effect(() => {
		if (showModal) {
			allFolders = getAllFolders(folderTree);
			selectedFolder = $contextMenuStore.data?.id;
		}
	});

	async function handleSave() {
		try {
			if (type == 'folder') {
				const response = await fetch('http://localhost:3096/api/create-folder', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({
						name: itemName,
						parent_id: selectedFolder
					})
				});

				if (!response.ok) {
					throw new Error('Failed to create folder');
				}
			} else if (type == 'bookmark') {
				const response = await fetch('http://localhost:3096/api/create-bookmark', {
					method: 'POST',
					headers: {
						'Content-Type': 'application/json'
					},
					body: JSON.stringify({
						name: itemName,
						parent_id: selectedFolder
					})
				});

				if (!response.ok) {
					throw new Error('Failed to create bookmark');
				}
			}

			refreshTree();
		} catch (err) {
			console.error('Creation failed:', err);
		}

		closeModal();
	}

	function closeModal() {
		showModal = false;
	}
</script>

{#if showModal}
	<div class="modal">
		<div class="modal-content">
			{#if type == 'folder'}
				<button class="close" onclick={closeModal}>&times;</button>
				<h1>New Folder</h1>
				<form>
					<div class="form-group">
						<label for="name">Name</label>
						<input type="text" name="name" required bind:value={itemName} />
					</div>

					<div class="form-group">
						<label for="location">Location</label>
						<select name="folders" id="location" bind:value={selectedFolder}>
							<option value={undefined}></option>
							{#each allFolders as folder}
								<option value={folder.id}>{folder.name}</option>
							{/each}
						</select>
					</div>

					<center>
						<button type="submit" class="save" onclick={handleSave}>Save</button>
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
</style>
