<script lang="ts">
	import Upload from '$lib/upload.svelte';
	export let data;
</script>

<Upload
	adat={data}
	title="Pótlék"
	underhood="potlek"
	desc="Egyszerre feltölthetsz többet, csak figyelj oda, hogy a megfelelő pótlékhoz töltöd fel!"
/>
