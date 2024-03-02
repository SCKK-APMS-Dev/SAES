<script lang="ts">
	export let data;
	console.log(data);
</script>

{#if data.prev}
	{#each data.cucc as puszedli}
		{#if new Date(puszedli.date) > data?.prev && new Date(puszedli.date) < data?.next}<img
				src={`https://api.sckk.hu/img/data/${puszedli.id}`}
				alt="kép"
			/>{/if}
	{/each}
{/if}
