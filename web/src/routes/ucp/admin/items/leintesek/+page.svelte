<script lang="ts">
	import SMList from '$lib/admin/itemlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<SMList
	title="Leintések"
	type={get_type_number('leintés')}
	editdes="leintésének"
	{data}
	tools={['accept', 'decline', 'edit']}
/>
