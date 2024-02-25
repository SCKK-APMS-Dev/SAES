<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-1">
	<a href="/user/potlekok/upload" class="bg-green-500 text-2xl p-2 drop-shadow-xl font-bold"
		>Feltöltés</a
	>
	<h1 class="text-5xl font-bold mb-3 drop-shadow-xl">Pótlékaid:</h1>
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
						{new Date(potle.date).getFullYear()}.{new Date(potle.date).getMonth()}.{new Date(
							potle.date
						).getDate()}. {new Date(potle.date).getHours()}:{new Date(potle.date).getMinutes()}
					</h1>
					<h1 class="font-bold drop-shadow-xl">Státusz: {potle.status}</h1>
					<img
						src={`https://sckk-api.ampix.hu/img/data/${potle.id}`}
						alt=""
						class="m-auto py-2 max-w-xl max-h-xl drop-shadow-xl"
					/>
				</div>
			{/each}
		{:else}
			<h2>Még nem töltöttél fel pótlékokat!</h2>
		{/if}
	</div>
</div>
