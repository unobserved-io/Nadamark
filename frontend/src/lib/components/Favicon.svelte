<script lang="ts">
	import { onMount } from 'svelte';

	const lightModePath = '/favicon-light.png';
	const darkModePath = '/favicon-dark.png';

	let darkMode = false;

	function updateFavicon() {
		let favicon = document.querySelector<HTMLLinkElement>('link[rel="icon"]');
		if (!favicon) {
			favicon = document.createElement('link');
			favicon.rel = 'icon';
			document.head.appendChild(favicon);
		}

		favicon.href = darkMode ? darkModePath : lightModePath;
	}

	onMount(() => {
		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		darkMode = mediaQuery.matches;
		updateFavicon();

		mediaQuery.addEventListener('change', (e) => {
			darkMode = e.matches;
			updateFavicon();
		});
	});
</script>
