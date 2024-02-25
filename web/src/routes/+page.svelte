<script lang="ts">
	export let data;
</script>

<div class="h-screen flex">
	<div class="m-auto text-center">
		{#if data?.noauth}
			<h1 class="text-white text-3xl font-bold">Az oldal használatához kérlek lépj be</h1>
			<a
				href="user"
				class="text-white bg-green-500 text-2xl font-bold px-3 rounded-xl mt-2 hover:bg-green-800 transition-all duration-200"
				>Belépés Discordal</a
			>
		{/if}
	</div>
</div>
