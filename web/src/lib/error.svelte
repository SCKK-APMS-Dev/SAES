<script lang="ts">
	export let data: { error?: any };
</script>

{#if !data.error}
	<slot></slot>
{:else}
	<div class="child:p-2 md:child:p-16 grid grid-cols-1 grid-rows-1 text-center text-white">
		<h2 class="text-center text-3xl font-bold text-white">Sikertelen API lekérdezés</h2>
		<h2 class="text-center text-xl text-gray-300">{data.error}</h2>
	</div>
{/if}
