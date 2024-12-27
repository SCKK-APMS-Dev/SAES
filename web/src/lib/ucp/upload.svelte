<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import Error from '$lib/error.svelte';
	import { loading } from '$lib/loading.svelte';
	import { get_type_number } from './types';
	let formerror = $state('');
	interface Props {
		data: any;
		display?: string;
		tipus: number;
		warning?: string;
		agent?: string;
	}

	let { data, display = '', tipus, warning = '', agent = '' }: Props = $props();
	let seli: string[][] = [];
	let fileas: string[] = $state([]);
	let tope = $state('col');
	let uploading = false;
	beforeNavigate((ev) => {
		if (uploading) {
			ev.cancel();
		}
	});
	async function upload() {
		loading.value = true;
		uploading = true;
		let files = document.getElementById('file') as HTMLInputElement;
		if (files.files) {
			const formData = new FormData();
			let dates: string[] = [];
			for (let i = 0; i < files.files.length; i++) {
				const offsetMs =
					new Date(Number(files.files[i].lastModified.toString())).getTimezoneOffset() * 60 * 1000;
				formData.append('files', files.files[i]);
				dates.push((Number(files.files[i].lastModified.toString()) + offsetMs).toString());
			}
			formerror = '';
			const mama = await fetch('/web-api/upload', {
				method: 'POST',
				headers: {
					tip: tipus.toString(),
					dates: JSON.stringify(dates)
				},
				body: formData
			});
			loading.value = false;
			uploading = false;
			const ret = await mama.json();
			if (ret.error === 'toobig') {
				formerror =
					'A feltöltött fájlok túl lépték a 64MB-os határértéket. Lehetséges, hogy pár fel lett tölve, ezért nézd meg azokat a rendes, nem feltöltési oldalrészen!';
			} else if (ret.error === 'leintestipik') {
				formerror = 'Leintéshez kérlek 2-vel osztható mennyiségű képet tölts fel!';
			} else {
				fileas = ret;
				console.log(fileas);
			}
		}
	}
</script>

<Error {data}>
	<div class="text-center text-white">
		<div class="ml-16 mr-16 mt-16 rounded-lg bg-gradient-to-tr from-green-500 to-emerald-400 p-2">
			<h2 class="font-bold text-red-800 drop-shadow-xl">{formerror ? formerror : ''}</h2>
			<h1 class="mb-2 text-3xl font-bold uppercase">{display} feltöltése</h1>
			{#if agent.includes('Firefox')}
				<h1 class="mb-2 text-xl font-bold">
					Firefoxon (és az azon alapuló böngészőkön) jelenlegi állás szerint nem lehet elemeket
					feltölteni. Ez idő alatt kérlek használj egy Chromium alapú böngészőt! (Pl. Chrome, Opera,
					Edge, Brave, Arc, stb.)
				</h1>
			{:else}
				<form onsubmit={() => upload()} enctype="multipart/form-data">
					<input
						class="file:text-black"
						type="file"
						name="file"
						id="file"
						accept="image/*"
						required
						multiple
					/>
					<button
						type="submit"
						class="from-taxi hover:bg-pos-100 bg-size-200 bg-pos-0 rounded-full bg-gradient-to-r via-amber-600 to-green-500 px-3 py-1 text-xl uppercase drop-shadow-lg transition-all duration-500"
						>Feltöltés</button
					>
					<h2>
						{warning}
					</h2>
				</form>
				<h2 class="rounded-xl bg-red-600 px-2 text-xl font-bold">{''}</h2>
			{/if}
		</div>
		<div class="flex-row items-center justify-center align-middle">
			{#if !agent.includes('Firefox')}
				<h2 class="font-bold">Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
			{/if}
			{#each fileas as nyam}
				{#if tipus === get_type_number('leintés')}
					<div class="flex flex-col">
						<img
							loading="lazy"
							src={`${data.api}/img?id=${nyam[0]}`}
							alt=""
							class="max-h-5xl m-auto max-w-5xl py-3"
						/>
						<img
							loading="lazy"
							src={`${data.api}/img?id=${nyam[1]}`}
							alt=""
							class="max-h-5xl m-auto max-w-5xl py-3"
						/>
					</div>
				{:else}
					<img
						loading="lazy"
						src={`${data.api}/img?id=${nyam[0]}`}
						alt=""
						class="max-h-5xl m-auto max-w-5xl py-3"
					/>
				{/if}
			{/each}
		</div>
	</div>
</Error>
