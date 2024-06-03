<script lang="ts">
	export let data;
	import List from '$lib/list.svelte';
	import Error from '$lib/error.svelte';
	console.log(data);
</script>

<Error {data}>
	<List {data} magazo="Leintéseid" url="leintesek" leintes={true} />
</Error>
