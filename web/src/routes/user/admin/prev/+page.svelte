<script lang="ts">
	import { getRealText } from '$lib/public';
	import { onMount } from 'svelte';
	interface calls {
		[key: string]: number;
	}
	interface tipus {
		[key: string]: calls;
	}
	export let data;
	let aha: tipus = {};
	onMount(() => {
		if (data.date) {
			for (const jana of data.stats) {
				if (new Date(jana.date) > data.date?.prev && new Date(jana.date) < data.date?.next) {
					if (jana.type !== 'számla') {
						if (jana.type === 'pótlék') {
							if (jana.extra === 'éjszakai') {
								if (aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai']) {
									if (aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai'][jana.owner]) {
										aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai'][jana.owner]++;
									} else {
										aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai'][jana.owner] = 1;
									}
								} else {
									aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai'] = {};
									aha[jana.am ? 'am_pótlék_éjszakai' : 'pótlék_éjszakai'][jana.owner] = 1;
								}
							}
							if (jana.extra === 'délelőtti') {
								if (aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti']) {
									if (aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti'][jana.owner]) {
										aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti'][jana.owner]++;
									} else {
										aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti'][jana.owner] = 1;
									}
								} else {
									aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti'] = {};
									aha[jana.am ? 'am_pótlék_délelőtti' : 'pótlék_délelőtti'][jana.owner] = 1;
								}
							}
						} else {
							if (aha[jana.am ? `am_${jana.type}` : jana.type]) {
								if (aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner]) {
									aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner]++;
								} else {
									aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner] = 1;
								}
							} else {
								aha[jana.am ? `am_${jana.type}` : jana.type] = {};
								aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner] = 1;
							}
						}
					} else {
						if (aha[jana.am ? `am_${jana.type}` : jana.type]) {
							if (aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner]) {
								aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner] += Number(jana.extra);
							} else {
								aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner] = Number(jana.extra);
							}
						} else {
							aha[jana.am ? `am_${jana.type}` : jana.type] = {};
							aha[jana.am ? `am_${jana.type}` : jana.type][jana.owner] = Number(jana.extra);
						}
					}
				}
			}
		}
	});
</script>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if data.date}
			<div>
				<h1 class="text-3xl font-bold">
					Előző hét ({`${data.date?.prev.getUTCMonth() + 1}.${data.date?.prev.getUTCDate()}. - ${data.date?.next.getUTCMonth() + 1}.${data.date?.next.getUTCDate()}`})
				</h1>
				{#each Object.entries(aha) as [key, value]}
					<h1 class="text-xl font-bold">{getRealText(key)}</h1>
					{#each Object.entries(value) as [key2, value2]}
						<div class="flex gap-2">
							<h2>{key2}: {key.endsWith('számla') ? value2 + '$' : value2 + ' db'}</h2>
							<a
								href={`https://sckk.hu/list/${key2.replace(' ', '_')}/${key.startsWith('am') ? key.split('_')[1] : key}`}
								class="rounded-xl bg-blue-900 px-2"
								target="_blank">Link</a
							>
						</div>
					{/each}
				{/each}
			</div>
		{/if}
	</div>
</div>
