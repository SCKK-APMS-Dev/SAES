<script lang="ts">
	import Upload from '$lib/ucp/upload.svelte';

	export let data;
</script>

<Upload
	{data}
	display="Pótlék"
	tipus="pótlék"
	warning={data.layout.am
		? '8:00-16:00: Délelőtti pótlék, 20:00-4:00: Éjszakai pótlék'
		: '8:00-14:00: Délelőtti pótlék, 23:00-4:00: Éjszakai pótlék'}
/>
