import { writable } from 'svelte/store';
import type { RootItems } from '$lib/types';
import { dev } from '$app/environment';

type RootItemsState = {
	data: RootItems | null;
	loading: boolean;
};

const initialState: RootItemsState = {
	data: null,
	loading: true
};

export const rootItemsStore = writable<RootItemsState>(initialState);

export async function refreshTree() {
	try {
		const response = await fetch(dev ? 'http://localhost:8663/api/tree' : '/api/tree');
		const data = await response.json();
		rootItemsStore.set({ data, loading: false });
	} catch (error) {
		console.error('Error fetching tree:', error);
		rootItemsStore.set({ data: null, loading: false });
	}
}
