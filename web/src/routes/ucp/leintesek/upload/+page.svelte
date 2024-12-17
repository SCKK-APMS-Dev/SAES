<script lang="ts">
	import Upload from '$lib/ucp/upload.svelte';

	let { data } = $props();
</script>

<Upload
	{data}
	display="Leintés"
	tipus="leintés"
	agent={data.agent}
	warning="Egy leintés 2 képből áll! (Egyszerre több leintést is fel lehet tölteni)"
/>
