<script lang="ts">
	export let data: {
		potlekok: { status: string; date: Date; reason: string | null; id: number }[];
		api: string;
	};
	export let url = '';
	export let magazo = '';
	export let leintes = false;
</script>

<div class="grid grid-cols-1 text-center text-black dark:text-white">
	<a
		href={`/user/${url}/upload`}
		class="ml-96 mr-96 mt-16 rounded-lg bg-gradient-to-r from-white to-red-600 p-2 text-2xl font-bold text-black drop-shadow-xl"
		>Feltöltés</a
	>
	<h1 class="text-5xl font-bold drop-shadow-xl">{magazo}:</h1>
	<h2 class="mb-3 text-black dark:text-white">(az elmúlt két hétben)</h2>
	<div class="flex flex-auto flex-wrap items-center justify-center gap-3 align-middle">
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
								class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
							/>
							<img
								src={`${data.api}/img/data/${potle.id}/1`}
								alt=""
								class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
							/>
						</div>
					{:else}
						<img
							src={`${data.api}/img/data/${potle.id}`}
							alt=""
							class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
						/>
					{/if}
				</div>
			{/each}
		{:else}
			<h2>Az elmúlt 2 hétben nem töltöttél fel semmit!</h2>
		{/if}
	</div>
</div>
