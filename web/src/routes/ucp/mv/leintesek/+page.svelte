<script lang="ts">
	import Adminlist from '$lib/mv/mvlist.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<Adminlist
	title="Leintések"
	type={get_type_number('leintés')}
	editdes="leintésének"
	{data}
	tools={['accept', 'decline', 'edit']}
/>
