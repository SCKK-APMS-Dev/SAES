<script lang="ts">
	import Upload from '$lib/upload.svelte';
	import Error from '$lib/error.svelte';
	export let data;
</script>

<Error {data}>
	<Upload
		adat={data}
		title="Pótlék"
		underhood="pótlék"
		desc="8:00-15:00: Délelőtti pótlék, 22:00-4:00: Éjszakai pótlék"
	/>
</Error>
