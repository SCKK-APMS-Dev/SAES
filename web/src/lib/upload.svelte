<script lang="ts">
	import { loading } from './loading';

	export let title = '';
	export let underhood = '';
	export let adat: { layout?: any; api?: any };
	export let desc = '';
	export let warning = '';
	let formerror = '';
	let seli: string[][] = [];
	let fileas: string[] = [];
	let tope = 'col';
	async function upload() {
		$loading = true;
		let files = document.getElementById('file') as HTMLInputElement;
		if (files.files) {
			const formData = new FormData();
			let dates: string[] = [];
			for (let i = 0; i < files.files.length; i++) {
				formData.append('files', files.files[i]);
				dates.push(files.files[i].lastModified.toString());
			}
			const mama = await fetch('/api/upload', {
				method: 'POST',
				headers: {
					type: underhood,
					dates: JSON.stringify(dates)
				},
				body: formData
			});
			$loading = false;
			fileas = JSON.parse(await mama.text());
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
		<button
			class="bg-red-600 hidden px-2 mb-2 rounded-lg transition-all hover:bg-red-800 duration-200"
			on:click={switchTope}
			>{#if tope === 'row'}
				Sorban
			{:else}
				Oszlopban
			{/if}</button
		>
		{#each seli as asd}
			<div
				class="flex bg-slate-500 py-4 align-middle items-center justify-center"
				class:flex-col={tope === 'col'}
				class:flex-row={tope === 'row'}
			>
				<div>
					<button class="p-2 bg-slate-900">
						<img src={asd[0]} alt="asd" class="max-w-5xl max-h-5xl m-auto" />
					</button>
					<h2>Kép a 10-12-ről</h2>
				</div>
				<h1 class="text-5xl font-bold">+</h1>
				<div>
					<button class="p-2 bg-slate-900">
						<img src={asd[1]} alt="asd" class="max-w-5xl max-h-5xl m-auto" />
					</button>
					<h2>Kép a "xy kifizette az utazást"-ról</h2>
				</div>
			</div>
		{/each}
		{#each fileas as nyam}
			{#if underhood === 'leintés'}
				<div class="flex flex-col">
					<img
						src={`${adat.api}/img/data/${nyam}/0`}
						alt=""
						class="max-w-5xl max-h-5xl m-auto py-3"
					/>
					<img
						src={`${adat.api}/img/data/${nyam}/1`}
						alt=""
						class="max-w-5xl max-h-5xl m-auto py-3"
					/>
				</div>
			{:else}
				<img src={`${adat.api}/img/data/${nyam}`} alt="" class="max-w-5xl max-h-5xl m-auto py-3" />
			{/if}
		{/each}
	</div>
</div>
