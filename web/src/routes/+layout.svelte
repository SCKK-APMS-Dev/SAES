<script>
	import '../app.css';
	import { navigating } from '$app/stores';

	import { loading } from '$lib/loading';
	$: $loading = !!$navigating;
</script>

<head>
	<title>SCKK</title>
	<meta name="description" content="A SeeCity Közlekedési Központ weboldala" />
</head>

<slot />

{#if $loading}
	<div
		class="absolute bottom-1/2 left-1/2 transform rounded-3xl -translate-x-1/2 -translate-y-1/2 w-20 h-20 bg-slate-500 text-center justify-center items-center flex"
	>
		<span class="loader"></span>
	</div>
{/if}
