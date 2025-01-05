<script lang="ts">
	import { get_type_number } from '$lib/ucp/types.js';
	import Upload from '$lib/ucp/upload.svelte';

	let { data } = $props();
</script>

<Upload {data} display="Számla" tipus={get_type_number('számla')} agent={data.agent} />
