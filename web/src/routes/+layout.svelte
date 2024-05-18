<script>
	import '../app.css';
	import { navigating } from '$app/stores';

	import { loading } from '$lib/loading';
	// biome-ignore lint/suspicious/noConfusingLabels: <explanation>
	$: $loading = !!$navigating;
</script>

<head>
	<title>SCKK</title>
	<meta name="description" content="A SeeCity Közlekedési Központ weboldala" />
</head>

<slot />

{#if $loading}
	<div
		class="absolute bottom-1/2 left-1/2 flex h-20 w-20 -translate-x-1/2 -translate-y-1/2 transform items-center justify-center rounded-3xl bg-slate-500 text-center"
	>
		<span class="loader"></span>
	</div>
{/if}
