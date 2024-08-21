<script lang="ts">
	import List from '$lib/ucp/list.svelte';
	export let data;
</script>

<List {data} display="Számláid" tipus="számla" />
