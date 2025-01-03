<script lang="ts">
	import MVList from '$lib/mv/mvlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<MVList
	title="Számlák"
	type={get_type_number('számla')}
	{data}
	editdes="számlájának"
	extraText="Összeg"
	am={data.layout.am}
	tools={['edit']}
/>
