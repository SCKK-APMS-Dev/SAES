<script lang="ts">
	import Upload from '$lib/upload.svelte';

	export let data;
</script>

<Upload adat={data} title="Szereltetési számla" underhood="szamla" />
