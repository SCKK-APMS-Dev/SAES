<script lang="ts">
	import Upload from '$lib/ucp/upload.svelte';

	let { data } = $props();
</script>

<Upload {data} display="Számla" tipus="számla" agent={data.agent} />
