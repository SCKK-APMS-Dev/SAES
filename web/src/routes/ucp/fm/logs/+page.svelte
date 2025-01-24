<script lang="ts">
	import Loglist from '$lib/fm/loglist.svelte';

	let { data } = $props();
</script>

<Loglist {data} filters={['update_item', 'upload_item']} />
