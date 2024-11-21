<script>
	import '../app.css';
	import '../snow.css';
	import { navigating } from '$app/stores';
	import { loading } from '$lib/loading';
	import { fade } from 'svelte/transition';
	import Snow from '$lib/snow.svelte';
	// biome-ignore lint/suspicious/noConfusingLabels: <explanation>
	$: $loading = !!$navigating;
	// $: $loading = true; // testing
</script>

<svelte:head>
	<title>SCKK Weboldal</title>
	<meta name="description" content="A See City Közlekedési Központ weboldala" />
	<meta content="SCKK Weboldal" property="og:title" />
	<meta content="https://sckk.hu" property="og:url" />
	<meta content="A See City Közlekedési Központ (SeeMTA v4) weboldala." property="og:description" />
	<meta content="https://sckk.hu/favicon.png" property="og:image" />
	<meta content="#fece01" data-react-helmet="true" name="theme-color" />
</svelte:head>

<Snow />
<slot />

{#if $loading}
	<div
		class="fixed top-0 z-50 h-full w-full bg-[rgba(0,0,0,0.4)]"
		transition:fade={{ duration: 300 }}
	>
		<div
			class="shadow-taxi border-taxi pointer-events-none fixed
	left-1/2 top-1/2 h-96 w-96 -translate-x-1/2 -translate-y-1/2 transform cursor-progress content-center overflow-hidden rounded-full border-2 shadow-lg"
		>
			<!-- <span class="loader"></span> -->
			<img src="/miklos.gif" alt="loading" class=" z-50 block w-[1000px] object-top" />
		</div>
	</div>
{/if}
