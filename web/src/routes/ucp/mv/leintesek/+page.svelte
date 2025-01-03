<script lang="ts">
	import MVList from '$lib/mv/mvlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<MVList
	title="Leintések"
	type={get_type_number('leintés')}
	editdes="leintésének"
	{data}
	tools={['accept', 'decline', 'edit']}
/>
