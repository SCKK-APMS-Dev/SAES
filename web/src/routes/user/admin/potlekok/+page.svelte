<script lang="ts">
	import Adminlist from '$lib/admin/adminlist.svelte';
	export let data;
</script>

<Adminlist
	title="Pótlékok"
	des={`Taxi: 8-15 Délelőtti, 22-4 Éjszakai; AM: 8-16 Délelőtti, 20-4 Éjszakai`}
	type="pótlék"
	editdes="pótlékának"
	extraText="Típus"
	{data}
	am={data.layout.am}
	tools={['délelőtti', 'éjszakai', 'edit']}
/>
