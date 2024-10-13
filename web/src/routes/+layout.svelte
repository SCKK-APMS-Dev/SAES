<script>
	import '../app.css';
	import { navigating } from '$app/stores';
	import { loading } from '$lib/loading';
	// biome-ignore lint/suspicious/noConfusingLabels: <explanation>
	$: $loading = !!$navigating;
	// $: $loading = true; // testing
</script>

<head>
	<title>SAES Web</title>
	<meta name="description" content="A See City Közlekedési Központ weboldala" />
</head>

<slot />

{#if $loading}
	<div class="fixed top-0 z-50 h-full w-full bg-[rgba(0,0,0,0.4)]">
		<div
			class="shadow-taxi border-taxi pointer-events-none fixed
	left-1/2 top-1/2 h-96 w-96 -translate-x-1/2 -translate-y-1/2 transform cursor-progress content-center overflow-hidden rounded-full border-2 shadow-lg"
		>
			<!-- <span class="loader"></span> -->
			<img src="/macska.gif" alt="loading" class=" z-50 block w-[1000px] object-top" />
		</div>
	</div>
{/if}
