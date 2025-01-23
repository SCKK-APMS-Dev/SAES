<script lang="ts">
	import SMList from '$lib/sm/smlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<SMList
	title="Számlák"
	type={get_type_number('számla')}
	{data}
	editdes="számlájának"
	extraText="Összeg"
	tools={['edit']}
/>
