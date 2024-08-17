<script lang="ts">
	import { navigating, page } from '$app/stores';
	import Error from '$lib/error.svelte';
	import { getRealText, getAlterText } from '$lib/public';
	interface calls {
		[key: string]: number;
	}
	interface Copy {
		[key: string]: boolean;
	}
	let copied: Copy = {};
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
	function copyClip(str: string, id: string) {
		navigator.clipboard.writeText(str);
		copied[id] = true;
		setTimeout(() => {
			copied[id] = false;
		}, 3000);
	}
</script>

<Error {data}>
	<div class="flex">
		<div class="m-auto text-center text-white">
			{#if data.date}
				<div class="mt-2">
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
						<h1 class="mt-3 text-3xl font-bold">{getRealText(key)}</h1>
						{#each Object.entries(value) as [key2, value2]}
							{#if data.week === 'previous'}
								<div class="flex items-center justify-center">
									<h2 class="text-2xl">
										{key2}: {key.endsWith('számla') ? value2 + '$' : value2 + ' db'}
									</h2>
									<button
										class="ml-1 flex items-center justify-center rounded-full bg-gray-600 p-1 transition-colors duration-200 hover:bg-gray-800"
										on:click={() =>
											copyClip(
												`${$page.url.origin}/list/${key2.replace(' ', '_')}/${getAlterText(key)}`,
												`${key}_${key2}`
											)}
										>{#if copied[`${key}_${key2}`]}
											<span class="icon-[ic--twotone-check] h-6 w-6 text-green-400"></span>
										{:else}
											<span class="icon-[mdi--clipboard-outline] text-taxi h-6 w-6"></span>
										{/if}
									</button>
									<a
										class="ml-1 flex items-center justify-center rounded-full bg-gray-600 p-1 transition-colors duration-200 hover:bg-gray-800"
										href={`${$page.url.origin}/list/${key2.replace(' ', '_')}/${getAlterText(key)}`}
										target="”_blank”"
									>
										<span class="icon-[ion--open-outline] h-6 w-6 text-blue-500"></span></a
									>
								</div>
							{:else}
								<h2>{key2}: {key.endsWith('számla') ? value2 + '$' : value2 + ' db'}</h2>
							{/if}
						{/each}
					{/each}
				</div>
			{/if}
		</div>
	</div>
</Error>
