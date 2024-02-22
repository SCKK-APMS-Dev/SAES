<script lang="ts">
	export let data;
</script>

{#if data?.error}
	<h1 class="text-3xl text-red-600 font-bold">{data?.error?.msg}</h1>
{/if}

<div class="h-screen flex">
	<div class="m-auto">
		<h1 class="text-white text-3xl">Üdv {data?.dc?.name}!</h1>
	</div>
</div>
