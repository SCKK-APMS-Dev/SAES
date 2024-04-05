<script lang="ts">
	export let data;
	let counter;
</script>

{#each data.cucc as puszedli}
	<h2 class="text-xl text-white text-center mt-5">{data.cucc.indexOf(puszedli) + 1}:</h2>
	{#if data.type === 'leintés'}
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}/1`} alt="kép" />
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}/2`} alt="kép" />
	{:else}
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}`} alt="kép" />
	{/if}
{/each}
