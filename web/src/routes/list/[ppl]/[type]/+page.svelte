<script lang="ts">
	export let data;
</script>

<div class="flex flex-col items-center justify-center">
	{#each data.cucc as puszedli}
		{#if data.type === 'szamla'}
			<h2 class="mt-5 text-center text-xl text-white">
				{data.cucc.indexOf(puszedli) + 1} ({puszedli.extra}$):
			</h2>
		{:else}
			<h2 class="mt-5 text-center text-xl text-white">{data.cucc.indexOf(puszedli) + 1}:</h2>
		{/if}
		{#if data.type === 'leintes'}
			<img src={`${data.api}/limg?id=${puszedli.id}&ver=0`} alt="kép" />
			<img src={`${data.api}/limg?id=${puszedli.id}&ver=1`} alt="kép" />
		{:else}
			<img src={`${data.api}/img?id=${puszedli.id}`} alt="kép" />
		{/if}
	{/each}
</div>
