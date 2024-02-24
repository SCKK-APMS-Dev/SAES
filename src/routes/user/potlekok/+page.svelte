<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-1">
	<a href="/user/potlekok/upload" class="bg-green-500 text-xl">Feltöltés</a>
	<div class="bg-yellow-400">
		<h1 class="text-5xl font-bold mb-3">Pótlékaid:</h1>
		<div class="flex flex-auto flex-wrap align-middle items-center justify-center gap-3">
			{#if data.potlekok}
				{#each data.potlekok as potle}
					<div class="bg-yellow-700 rounded-xl">
						<h1>
							{new Date(potle.date).getFullYear()}.{new Date(potle.date).getMonth()}.{new Date(
								potle.date
							).getDate()}. {new Date(potle.date).getHours()}:{new Date(potle.date).getMinutes()}
						</h1>
						<h1>Státusz: {potle.status}</h1>
						<img src={potle.kep} alt="" class="m-auto py-2 max-w-xl max-h-xl" />
					</div>
				{/each}
			{/if}
		</div>
	</div>
</div>
