<script lang="ts">
	import { loading } from './loading';

	export let title = '';
	export let underhood = '';
	export let adat: { layout: any };
	export let desc = '';
	export let warning = '';
	let formerror = '';

	let fileas: string[] = [];
	async function upload() {
		$loading = true;
		let files = document.getElementById('file') as HTMLInputElement;
		if (files.files) {
			const formData = new FormData();
			for (let i = 0; i < files.files.length; i++) {
				formData.append('files', files.files[i]);
			}
			await fetch('/api/upload', {
				method: 'POST',
				mode: 'no-cors',
				headers: {
					type: underhood
				},
				body: formData
			});
		}
	}
</script>

<div class="text-center text-white">
	<div class="bg-green-500 p-2">
		<h2 class="font-bold text-red-800 drop-shadow-xl">{formerror ? formerror : ''}</h2>
		<h1 class="text-3xl font-bold">{title} feltöltése</h1>
		<form on:submit|preventDefault={() => upload()} enctype="multipart/form-data">
			<input type="text" class="hidden" name="name" value={adat.layout?.name} />
			<input type="file" name="file" id="file" accept="image/*" required multiple />
			<button type="submit" class="bg-red-600 font-bold text-xl px-2 rounded-xl">Feltöltés</button>
			<h2>{desc}</h2>
		</form>
		<h2 class="bg-red-600 font-bold text-xl px-2 rounded-xl">
			{warning}
		</h2>
	</div>
	<div class="flex-row align-middle items-center justify-center">
		<h2 class="font-bold">Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
		{#each fileas as nyam}
			{#if underhood === 'leintés'}
				<div class="flex flex-col">
					<img
						src={`https://api.sckk.hu/img/data/${nyam}/1`}
						alt=""
						class="max-w-5xl max-h-5xl m-auto py-3"
					/>
					<img
						src={`https://api.sckk.hu/img/data/${nyam}/2`}
						alt=""
						class="max-w-5xl max-h-5xl m-auto py-3"
					/>
				</div>
			{:else}
				<img
					src={`https://api.sckk.hu/img/data/${nyam}`}
					alt=""
					class="max-w-5xl max-h-5xl m-auto py-3"
				/>
			{/if}
		{/each}
	</div>
</div>
