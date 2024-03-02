<script lang="ts">
	export let data;
</script>

{#if data.prev}
	{#each data.cucc as puszedli}
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}`} alt="kép" />
	{/each}
{/if}
