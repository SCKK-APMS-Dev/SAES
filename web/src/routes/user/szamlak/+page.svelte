<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-1">
	<a href="/user/szamlak/upload" class="bg-green-500 text-2xl p-2 drop-shadow-xl font-bold"
		>Feltöltés</a
	>
	<h1 class="text-5xl font-bold mb-3 drop-shadow-xl">Szereltetési számláid:</h1>
	<div class="flex flex-auto flex-wrap align-middle items-center justify-center gap-3">
		{#if data.potlekok}
			{#each data.potlekok as potle}
				<div
					class="rounded-xl p-2 drop-shadow-xl"
					class:bg-green-600={potle.status === 'elfogadva'}
					class:bg-red-600={potle.status === 'elutasítva'}
					class:bg-yellow-600={potle.status === 'feltöltve'}
				>
					<h1 class="font-bold drop-shadow-xl">
						{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
							1}.{new Date(potle.date).getUTCDate()}. {new Date(
							potle.date
						).getUTCHours()}:{new Date(potle.date).getUTCMinutes()}
					</h1>
					<h1 class="font-bold drop-shadow-xl">Státusz: {potle.status}</h1>
					{#if potle.reason}
						<h1 class="font-bold drop-shadow-xl">Megjegyzés: {potle.reason}</h1>
					{/if}
					<img
						src={`https://sckk-api.ampix.hu/img/data/${potle.id}`}
						alt=""
						class="m-auto py-2 max-w-xl max-h-xl drop-shadow-xl"
					/>
				</div>
			{/each}
		{:else}
			<h2>Még nem töltöttél fel számlát!</h2>
		{/if}
	</div>
</div>
