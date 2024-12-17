<script lang="ts">
	import Adminlist from '$lib/mv/mvlist.svelte';
	let { data } = $props();
</script>

<Adminlist
	title="Számlák"
	type="számla"
	{data}
	editdes="számlájának"
	extraText="Összeg"
	am={data.layout.am}
	tools={['edit']}
/>
