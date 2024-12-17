<script lang="ts">
	import Adminlist from '$lib/mv/mvlist.svelte';
	let { data } = $props();
</script>

<Adminlist
	title="Leintések"
	type="leintés"
	editdes="leintésének"
	{data}
	tools={['accept', 'decline', 'edit']}
/>
