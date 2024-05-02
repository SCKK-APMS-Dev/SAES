<script lang="ts">
	export let data: {
		potlekok: { status: string; date: Date; reason: string | null; id: number }[];
		api: string;
	};
	export let url = '';
	export let magazo = '';
	export let leintes = false;
</script>

<div class="text-center dark:text-white text-black grid grid-cols-1">
	<a
		href={`/user/${url}/upload`}
		class="bg-gradient-to-r from-white to-red-600 text-2xl p-2 drop-shadow-xl font-bold mr-96 ml-96 mt-16 rounded-lg text-black"
		>Feltöltés</a
	>
	<h1 class="text-5xl font-bold drop-shadow-xl">{magazo}:</h1>
	<h2 class="mb-3 dark:text-white text-black">(az elmúlt két hétben)</h2>
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
							1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
							2}:{new Date(potle.date).getUTCMinutes()}
					</h1>
					<h1 class="font-bold drop-shadow-xl">Státusz: {potle.status}</h1>
					{#if potle.reason}
						<h1 class="font-bold drop-shadow-xl">Megjegyzés: {potle.reason}</h1>
					{/if}
					{#if leintes}
						<div class="flex flex-col xl:flex-row">
							<img
								src={`${data.api}/img/data/${potle.id}/0`}
								alt=""
								class="m-auto py-2 max-w-xl max-h-xl drop-shadow-xl"
							/>
							<img
								src={`${data.api}/img/data/${potle.id}/1`}
								alt=""
								class="m-auto py-2 max-w-xl max-h-xl drop-shadow-xl"
							/>
						</div>
					{:else}
						<img
							src={`${data.api}/img/data/${potle.id}`}
							alt=""
							class="m-auto py-2 max-w-xl max-h-xl drop-shadow-xl"
						/>
					{/if}
				</div>
			{/each}
		{:else}
			<h2>Az elmúlt 2 hétben nem töltöttél fel semmit!</h2>
		{/if}
	</div>
</div>
