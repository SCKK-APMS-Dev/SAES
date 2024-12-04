<script lang="ts">
	export let data;
	let hour = new Date().getHours();
	let greet = '';
	let end = '!';
	if (hour >= 19) {
		greet = 'Szép estét';
	}
	if (hour >= 13 && hour < 19) {
		greet = 'Szép délutánt';
	}
	if (hour >= 12 && hour < 13) {
		greet = 'Szép delet';
	}
	if (hour >= 8 && hour < 12) {
		greet = 'Jó reggelt';
	}
	if (hour >= 4 && hour < 8) {
		greet = 'Jó reggelt, bár nem kapsz pótlékot';
	}
	if (hour >= 0 && hour < 1) {
		greet = 'Boldog új évet';
	}
	if (hour >= 1 && hour < 4) {
		greet = 'Miért nem alszol';
		end = '?';
	}
	// const sendResponse = async () => {
	// 	await fetch('/api/upload', {
	// 		method: 'POST',
	// 		headers: {
	// 			tip: 'midnightReason'
	// 		},
	// 		body: reason
	// 	});
	// };
</script>

<div class="child:p-2 md:child:p-16 grid grid-cols-1 grid-rows-1 text-center text-white">
	{#if !data.error}
		<div class="ml-16 mr-16 mt-16 rounded-lg bg-amber-600">
			<h1 class="text-2xl drop-shadow-lg md:text-5xl md:font-bold">
				{greet}, {data.layout?.name}{end}
			</h1>
			<!-- {#if end === '?'}
				<form class="flex items-center justify-center gap-2 md:mb-4" on:submit={sendResponse}>
					<input
						bind:value={reason}
						type="text"
						class="w-[30vw] rounded-3xl border-2 border-white bg-amber-600 text-center font-bold placeholder:text-center placeholder:text-xl placeholder:font-bold placeholder:text-white"
						name="response"
						placeholder="Válasz"
					/>
					<button
						type="submit"
						class="text-xl font-bold transition-colors duration-200 hover:text-blue-600"
						>Küld</button
					>
				</form>
			{/if} -->
			{#if !data.layout.am}
				{#if data.calls.app === null}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Hívásaid (app nem megy, csak leintés): {data.calls?.leintes}
					</h2>
				{:else}
					<h2 class="text-xl drop-shadow-lg md:text-2xl">
						Hívásaid (app+leintés): {data.calls?.app}+{data.calls?.leintes}={Number(
							data.calls.app
						) + Number(data.calls?.leintes)}
					</h2>
				{/if}
			{/if}
			<h2 class="text-xl drop-shadow-lg md:text-2xl">
				Elfogadott pótlékaid: délelőtti: {data.calls?.potlek.de}, éjszakai: {data.calls?.potlek.du}
			</h2>
		</div>
	{/if}
</div>
{#if data.error}
	<h2 class="text-center text-3xl font-bold text-white">Sikertelen API lekérdezés</h2>
	<h2 class="text-center text-xl text-gray-300">{data.error}</h2>
{/if}
