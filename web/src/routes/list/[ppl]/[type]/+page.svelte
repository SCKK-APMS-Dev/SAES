<script lang="ts">
	export let data;
	let counter;
</script>

{#each data.cucc as puszedli}
	<h2 class="mt-5 text-center text-xl text-white">{data.cucc.indexOf(puszedli) + 1}:</h2>
	{#if data.type === 'leintés'}
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}/0`} alt="kép" />
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}/1`} alt="kép" />
	{:else}
		<img src={`https://api.sckk.hu/img/data/${puszedli.id}`} alt="kép" />
	{/if}
{/each}
