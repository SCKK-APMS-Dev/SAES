<script lang="ts">
	import Grid from '$lib/admin/grid.svelte';

	let { data } = $props();
</script>

<Grid
	{data}
	title="Műszakvezetés"
	items={[
		{
			title: 'Aktuális hét',
			description: 'Az aktuális heti statisztika',
			href: '/ucp/admin/shift/stat/current',
			border: 'border-violet-600',
			background: 'hover:bg-violet-600',
			faction: ['SCKK', 'TOW']
		},
		{
			title: 'Előző hét',
			description: 'Az előző heti statisztika',
			href: '/ucp/admin/shift/stat/previous',
			border: 'border-rose-600',
			background: 'hover:bg-rose-600',
			faction: ['SCKK', 'TOW']
		}
	]}
/>
