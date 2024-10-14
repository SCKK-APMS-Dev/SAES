<script lang="ts">
	import Upload from '$lib/ucp/upload.svelte';

	export let data;
</script>

<Upload
	{data}
	display="Pótlék"
	tipus="pótlék"
	agent={data.agent}
	warning={data.layout.am
		? '8:00-16:00: Délelőtti pótlék, 20:00-4:00: Éjszakai pótlék'
		: '8:00-15:00: Délelőtti pótlék, 22:00-4:00: Éjszakai pótlék'}
/>
