<script lang="ts">
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
							if (new Date(jana.date).getHours() >= 22 || new Date(jana.date).getHours() < 4) {
								if (aha['pótlék_éjszakai']) {
									if (aha['pótlék_éjszakai'][jana.owner]) {
										aha['pótlék_éjszakai'][jana.owner]++;
									} else {
										aha['pótlék_éjszakai'][jana.owner] = 1;
									}
								} else {
									aha['pótlék_éjszakai'] = {};
									aha['pótlék_éjszakai'][jana.owner] = 1;
								}
							}
							if (new Date(jana.date).getHours() >= 8 && new Date(jana.date).getHours() < 15) {
								if (aha['pótlék_délelőtti']) {
									if (aha['pótlék_délelőtti'][jana.owner]) {
										aha['pótlék_délelőtti'][jana.owner]++;
									} else {
										aha['pótlék_délelőtti'][jana.owner] = 1;
									}
								} else {
									aha['pótlék_délelőtti'] = {};
									aha['pótlék_délelőtti'][jana.owner] = 1;
								}
							}
							if (new Date(jana.date).getUTCHours() > 22) {
								if (aha['pótlék_éjszakai']) {
									if (aha['pótlék_éjszakai'][jana.owner]) {
										aha['pótlék_éjszakai'][jana.owner]++;
									} else {
										aha['pótlék_éjszakai'][jana.owner] = 1;
									}
								} else {
									aha['pótlék_éjszakai'] = {};
									aha['pótlék_éjszakai'][jana.owner] = 1;
								}
							}
						} else {
							if (aha[jana.type]) {
								if (aha[jana.type][jana.owner]) {
									aha[jana.type][jana.owner]++;
								} else {
									aha[jana.type][jana.owner] = 1;
								}
							} else {
								aha[jana.type] = {};
								aha[jana.type][jana.owner] = 1;
							}
						}
					} else {
						if (aha[jana.type]) {
							if (aha[jana.type][jana.owner]) {
								aha[jana.type][jana.owner] += Number(jana.reason);
							} else {
								aha[jana.type][jana.owner] = Number(jana.reason);
							}
						} else {
							aha[jana.type] = {};
							aha[jana.type][jana.owner] = Number(jana.reason);
						}
					}
				}
			}
		}
	});
</script>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if false}
			<h1>Tagok</h1>
			<table class="table-auto p-10">
				<thead class="bg-red-700">
					<tr class="child:p-2">
						<th>Discord ID</th>
						<th>IG Név</th>
						<th>Rang</th>
					</tr>
				</thead>
				<tbody>
					{#each data?.admin as tag}
						<tr>
							<td>{tag.discordid}</td>
							<td>{tag.name}</td>
							<td>{tag.rang}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{/if}
		{#if data.date}
			<div>
				<h1 class="text-3xl font-bold">
					Előző hét ({`${data.date?.prev.getUTCMonth() + 1}.${data.date?.prev.getUTCDate()}. - ${data.date?.next.getUTCMonth() + 1}.${data.date?.next.getUTCDate()}`})
				</h1>
				{#each Object.entries(aha) as [key, value]}
					<h1 class="text-xl font-bold">{key}</h1>
					{#each Object.entries(value) as [key2, value2]}
						<h2>{key2}: {key === 'számla' ? value2 + '$' : value2 + ' db'}</h2>
					{/each}
				{/each}
			</div>
		{/if}
	</div>
</div>
