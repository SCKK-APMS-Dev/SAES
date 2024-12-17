<script lang="ts">
	import List from '$lib/ucp/list.svelte';
	let { data } = $props();
</script>

<List {data} display="Pótlékaid" tipus="potlék" />
