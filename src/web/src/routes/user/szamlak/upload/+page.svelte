<script lang="ts">
	import Upload from '$lib/upload.svelte';
	import Error from '$lib/error.svelte';
	export let data;
</script>

<Error {data}>
	<Upload
		adat={data}
		title="Szereltetési számla"
		underhood="számla"
		desc="Kérlek ha teheted, csak a számlát töltsd fel, a háttér nélkül!"
	/>
</Error>
