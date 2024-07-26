<script>
	import '../app.css';
	import { navigating } from '$app/stores';

	import { loading } from '$lib/loading';
	// biome-ignore lint/suspicious/noConfusingLabels: <explanation>
	// $: $loading = !!$navigating;
	$: $loading = true;
</script>

<head>
	<title>SCKK Weboldal</title>
	<meta name="description" content="A See City Közlekedési Központ weboldala" />
</head>

<slot />

{#if $loading}
	<div
		class="shadow-taxi border-taxi absolute left-1/2
 top-1/2 h-24 w-24 -translate-x-1/2 -translate-y-1/2 transform cursor-progress content-center overflow-hidden rounded-full border-2 bg-gray-500 shadow-lg"
	>
		<!-- <span class="loader"></span> -->
		<img src="/kaktusz-unscreen.gif" alt="kaktusz" />
	</div>
{/if}
