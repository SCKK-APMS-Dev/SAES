<script lang="ts">
	import Adminlist from '$lib/adminlist.svelte';
	export let data;
</script>

<Adminlist
	title="Számlák"
	type="számla"
	editdes="számlájának"
	extraText="Összeg"
	am={data.layout.am}
	tools={['accept', 'decline', 'edit']}
/>
