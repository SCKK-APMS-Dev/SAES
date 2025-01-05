<script lang="ts">
	import { get_type_number } from '$lib/ucp/types.js';
	import Upload from '$lib/ucp/upload.svelte';

	let { data } = $props();
</script>

<Upload
	{data}
	display="Leintés"
	tipus={get_type_number('leintés')}
	agent={data.agent}
	warning="Egy leintés 2 képből áll! (Egyszerre több leintést is fel lehet tölteni)"
/>
