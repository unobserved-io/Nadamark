<script lang="ts">
	import '../app.css';
	import Favicon from '$lib/components/Favicon.svelte';
	let { children } = $props();
</script>

<Favicon />

{@render children()}
