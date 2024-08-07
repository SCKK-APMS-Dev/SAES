<script lang="ts">
	import { navigating } from '$app/stores';
	import Error from '$lib/error.svelte';
	import { getRealText } from '$lib/public';
	interface calls {
		[key: string]: number;
	}
	interface tipus {
		[key: string]: calls;
	}
	export let data;
	let aha: tipus = {};
	$: if (!$navigating) render();
	function render() {
		aha = {};
		if (data.date) {
			for (const potlek of data.stats.potlekok) {
				if (potlek.extra === 'délelőtti') {
					if (!aha['pótlék_délelőtti']) aha['pótlék_délelőtti'] = {};
					if (aha['pótlék_délelőtti'][potlek.owner]) {
						aha['pótlék_délelőtti'][potlek.owner]++;
					} else {
						aha['pótlék_délelőtti'][potlek.owner] = 1;
					}
				}
				if (potlek.extra === 'éjszakai') {
					if (!aha['pótlék_éjszakai']) aha['pótlék_éjszakai'] = {};
					if (aha['pótlék_éjszakai'][potlek.owner]) {
						aha['pótlék_éjszakai'][potlek.owner]++;
					} else {
						aha['pótlék_éjszakai'][potlek.owner] = 1;
					}
				}
			}

			for (const leintes of data.stats.leintesek) {
				if (!aha['leintés']) aha['leintés'] = {};
				if (aha['leintés'][leintes.owner]) {
					aha['leintés'][leintes.owner]++;
				} else {
					aha['leintés'][leintes.owner] = 1;
				}
			}

			for (const szamla of data.stats.szamlak) {
				if (!aha['számla']) aha['számla'] = {};
				if (aha['számla'][szamla.owner]) {
					aha['számla'][szamla.owner] += Number(szamla.extra);
				} else {
					aha['számla'][szamla.owner] = Number(szamla.extra);
				}
			}
		}
	}
</script>

<Error {data}>
	<div class="flex">
		<div class="m-auto text-center text-white">
			{#if data.date}
				<div class="mt-2">
					{#if data.week === 'previous'}
						<h1 class="font-bold text-red-500">
							Jelenleg extra tesztelés céljából került be a weboldalra, linkek még nem elérhetőek.
						</h1>
					{/if}
					<h1 class="text-3xl font-bold">
						{#if data.week === 'current'}
							Jelenlegi hét
						{:else if data.week === 'previous'}
							Előző hét
						{/if} ({`${new Date(data.date?.prev).getMonth() + 1}.${new Date(data.date?.prev).getDate()}. - ${new Date(data.date?.next).getMonth() + 1}.${new Date(data.date.next).getDate()}.`})
					</h1>
					{#if data.week === 'current'}
						<h2 class="mb-5 text-black dark:text-gray-400">
							A jelenlegi hétnél nincsen link, péntek 22:00-után az előző heti linkek ezeket az
							értékeket fogják mutatni
						</h2>
					{/if}
					{#each Object.entries(aha) as [key, value]}
						<h1 class="text-xl font-bold">{getRealText(key)}</h1>
						{#each Object.entries(value) as [key2, value2]}
							<h2>{key2}: {key.endsWith('számla') ? value2 + '$' : value2 + ' db'}</h2>
						{/each}
					{/each}
				</div>
			{/if}
		</div>
	</div>
</Error>
