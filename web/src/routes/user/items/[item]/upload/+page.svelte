<script lang="ts">
	import Error from '$lib/error.svelte';
	import { loading } from '$lib/loading';
	export let data;
	let formerror = '';
	let seli: string[][] = [];
	let fileas: string[] = [];
	let tope = 'col';
	async function upload() {
		$loading = true;
		let files = document.getElementById('file') as HTMLInputElement;
		if (files.files && data.real) {
			const formData = new FormData();
			let dates: string[] = [];
			for (let i = 0; i < files.files.length; i++) {
				formData.append('files', files.files[i]);
				dates.push(files.files[i].lastModified.toString());
			}
			formerror = '';
			const mama = await fetch('/api/upload', {
				method: 'POST',
				headers: {
					tip: data.real[0],
					dates: JSON.stringify(dates)
				},
				body: formData
			});
			$loading = false;
			const ret = await mama.json();
			if (ret.error === 'toobig') {
				formerror =
					'A feltöltött fájlok túl lépték a 10MB-os határértéket. Lehetséges, hogy az egyik fel lett tölve, ezért nézd meg azokat a rendes, nem feltöltési oldalrészen!';
			} else {
				fileas = ret;
			}
			// for (let i = 0; i < files.files.length / 2; i++) {
			// 	seli.push([]);
			// 	seli[i].push(URL.createObjectURL(files.files[i]));
			// 	seli[i].push(URL.createObjectURL(files.files[i + 1]));
			// 	seli = seli;
			// }
			// $loading = false;
		}
	}
	const switchTope = () => {
		if (tope === 'col') {
			tope = 'row';
		} else {
			tope = 'col';
		}
	};
</script>

<Error {data}>
	{#if data.real}
		<div class="text-center text-white">
			<div class="ml-16 mr-16 mt-16 rounded-lg bg-green-500 p-2">
				<h2 class="font-bold text-red-800 drop-shadow-xl">{formerror ? formerror : ''}</h2>
				<h1 class="text-3xl font-bold">{data.real[2]} feltöltése</h1>
				<form on:submit|preventDefault={() => upload()} enctype="multipart/form-data">
					<input type="text" class="hidden" name="name" value={data.layout?.name} />
					<input type="file" name="file" id="file" accept="image/*" required multiple />
					<button
						type="submit"
						class="rounded-xl bg-gradient-to-r from-white to-red-600 px-2 text-xl font-bold text-black"
						>Feltöltés</button
					>
					<h2>
						{data.real[0] === 'pótlék'
							? data.layout.am
								? '8:00-16:00: Délelőtti pótlék, 20:00-4:00: Éjszakai pótlék'
								: '8:00-15:00: Délelőtti pótlék, 22:00-4:00: Éjszakai pótlék'
							: ''}
					</h2>
				</form>
				<h2 class="rounded-xl bg-red-600 px-2 text-xl font-bold">{''}</h2>
			</div>
			<div class="flex-row items-center justify-center align-middle">
				<h2 class="font-bold">Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
				<button
					class="mb-2 hidden rounded-lg bg-red-600 px-2 transition-all duration-200 hover:bg-red-800"
					on:click={switchTope}
					>{#if tope === 'row'}
						Sorban
					{:else}
						Oszlopban
					{/if}</button
				>
				{#each seli as asd}
					<div
						class="flex items-center justify-center bg-slate-500 py-4 align-middle"
						class:flex-col={tope === 'col'}
						class:flex-row={tope === 'row'}
					>
						<div>
							<button class="bg-slate-900 p-2">
								<img src={asd[0]} alt="asd" class="max-h-5xl m-auto max-w-5xl" />
							</button>
							<h2>Kép a 10-12-ről</h2>
						</div>
						<h1 class="text-5xl font-bold">+</h1>
						<div>
							<button class="bg-slate-900 p-2">
								<img src={asd[1]} alt="asd" class="max-h-5xl m-auto max-w-5xl" />
							</button>
							<h2>Kép a "xy kifizette az utazást"-ról</h2>
						</div>
					</div>
				{/each}
				{#each fileas as nyam}
					{#if data.real[0] === 'leintés'}
						<div class="flex flex-col">
							<img
								src={`${data.api}/limg?id=${nyam}&ver=0`}
								alt=""
								class="max-h-5xl m-auto max-w-5xl py-3"
							/>
							<img
								src={`${data.api}/limg?id=${nyam}&ver=1`}
								alt=""
								class="max-h-5xl m-auto max-w-5xl py-3"
							/>
						</div>
					{:else}
						<img
							src={`${data.api}/img?id=${nyam}`}
							alt=""
							class="max-h-5xl m-auto max-w-5xl py-3"
						/>
					{/if}
				{/each}
			</div>
		</div>
	{/if}
</Error>
