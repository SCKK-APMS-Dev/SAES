<script lang="ts">
	import Upload from '$lib/upload.svelte';
	import Error from '$lib/error.svelte';
	export let data;
</script>

<Error {data}>
	<Upload
		adat={data}
		title="Pótlék"
		underhood="potlek"
		desc="Egyszerre feltölthetsz többet, csak figyelj oda, hogy a megfelelő pótlékhoz töltöd fel!"
	/>
</Error>
