<script lang="ts">
	import Upload from '$lib/upload.svelte';
	export let data;
	import Error from '$lib/error.svelte';
</script>

<Error {data}>
	<Upload adat={data} title="Leintés" underhood="leintés" />
</Error>
