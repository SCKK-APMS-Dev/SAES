<script lang="ts">
	export let data: { error: string };
</script>

{#if !data.error}
	<slot>Üres</slot>
{:else}
	<div class="text-center text-white grid grid-cols-1 grid-rows-1 child:p-2 md:child:p-16">
		<h2 class="text-center text-white font-bold text-3xl">Sikertelen API lekérdezés</h2>
		<h2 class="text-center text-gray-300 text-xl">{data.error}</h2>
	</div>
{/if}
