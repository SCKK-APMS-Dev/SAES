<script lang="ts">
	export let data;
	import List from '$lib/list.svelte';
</script>

<List {data} magazo="Leintéseid" url="leintesek" leintes={true} />
