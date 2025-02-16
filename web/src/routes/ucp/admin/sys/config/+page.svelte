<script lang="ts">
	import { Factions } from '$lib/permissions';

	let { data } = $props();

	let announcement = $state(data.config?.global.announcement);
	let maintenance = $state(data.config?.global.maintenance);
</script>

<div class="m-5 grid grid-cols-2 gap-2 text-center text-white">
	<div class="rounded-lg bg-amber-300">
		<h1 class="mt-2 text-3xl font-bold">Global Config</h1>
		<label for="announcement" class="text-xl font-bold">Hírdetmény: </label>
		<input
			class="rounded-lg bg-black/50 text-center font-bold text-white placeholder:text-white"
			type="text"
			placeholder={!announcement ? 'nincs beállítva' : ''}
			name="announcement"
			bind:value={announcement}
		/>
		<label for="maintenance" class="text-xl font-bold">Karbantartás: </label>
		<input
			type="text"
			class="mb-2 rounded-lg bg-black/50 text-center font-bold text-white placeholder:text-white"
			name="maintenance"
			placeholder={!maintenance ? 'nincs beállítva' : ''}
			bind:value={maintenance}
		/>
	</div>
</div>
