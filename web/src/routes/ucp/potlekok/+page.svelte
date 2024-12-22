<script lang="ts">
	import List from '$lib/ucp/list.svelte';
	import { get_type_number } from '$lib/ucp/types.js';
	let { data } = $props();
</script>

<List {data} display="Pótlékaid" tipus={get_type_number('pótlék')} />
