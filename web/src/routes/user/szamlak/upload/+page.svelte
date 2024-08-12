<script lang="ts">
	import Upload from '$lib/user/upload.svelte';

	export let data;
</script>

<Upload {data} display="Számla" tipus="számla" />
