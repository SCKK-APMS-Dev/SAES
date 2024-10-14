<script lang="ts">
	import Upload from '$lib/ucp/upload.svelte';

	export let data;
</script>

<Upload {data} display="Számla" tipus="számla" agent={data.agent} />
