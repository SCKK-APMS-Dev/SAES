<script lang="ts">
	export let data;
	import List from '$lib/list.svelte';
	import Error from '$lib/error.svelte';
</script>

<Error {data}>
	<List {data} magazo={data.real[1]} url={data.type} />
</Error>
