<script lang="ts">
	export let data;
</script>

<div class="h-screen flex">
	<div class="m-auto">
		{#if data?.error}
			<h1 class="text-3xl text-red-600 font-bold">{data?.error?.msg}</h1>
		{/if}
		{#if data?.dc}
			<h1 class="text-white text-3xl font-bold">Üdv {data?.dc?.name}!</h1>
		{/if}
	</div>
</div>
