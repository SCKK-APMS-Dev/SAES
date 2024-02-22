<script lang="ts">
	export let data;
</script>

<div class="h-screen flex">
	<div class="m-auto text-center">
		{#if data?.error}
			<h1 class="text-3xl text-red-600 font-bold">{data.error.msg}</h1>
		{/if}
		{#if data?.dc}
			<h1 class="text-white text-3xl font-bold">Üdv {data.dc.name}!</h1>
			<h2 class="text-gray-400">Jogosultságod: {data.dc.role}</h2>
		{/if}
	</div>
</div>
