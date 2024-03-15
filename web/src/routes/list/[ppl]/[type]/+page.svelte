<script lang="ts">
	export let data;
	let counter;
</script>

{#each data.cucc as puszedli}
	<h2 class="text-xl text-white text-center mt-5">{data.cucc.indexOf(puszedli) + 1}:</h2>
	<img src={`https://api.sckk.hu/img/data/${puszedli.id}`} alt="kép" />
{/each}
