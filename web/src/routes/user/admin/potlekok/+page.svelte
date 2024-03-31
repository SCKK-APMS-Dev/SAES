<script lang="ts">
	import Adminlist from '$lib/adminlist.svelte';
	export let data;
</script>

<Adminlist
	title="Pótlékok"
	type="pótlék"
	editdes="pótlékának"
	extraText="Típus"
	am={data.layout.am}
	tools={['délelőtti', 'éjszakai', 'edit']}
/>
