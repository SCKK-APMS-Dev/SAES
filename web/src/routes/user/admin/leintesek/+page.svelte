<script lang="ts">
	import Adminlist from '$lib/admin/adminlist.svelte';
	export let data;
</script>

<Adminlist
	title="Leintések"
	type="leintés"
	editdes="leintésének"
	{data}
	tools={['accept', 'decline', 'edit']}
/>
