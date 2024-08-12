<script lang="ts">
	import List from '$lib/user/list.svelte';
	export let data;
</script>

<List {data} display="Számláid" tipus="számla" />
