<script>
	import '../app.css';
	import { navigating } from '$app/stores';

	import { loading } from '$lib/loading';

	import { injectSpeedInsights } from '@vercel/speed-insights/sveltekit';

	injectSpeedInsights();

	$: $loading = !!$navigating;
</script>

<head>
	<title>SCKK A műszak</title>
	<meta name="description" content="SCKK A műszak" />
</head>

<slot />

{#if $loading}
	<div
		class="absolute bottom-1/2 left-1/2 transform rounded-3xl -translate-x-1/2 -translate-y-1/2 w-20 h-20 bg-emerald-500 text-center justify-center items-center flex"
	>
		<span class="loader"></span>
	</div>
{/if}
