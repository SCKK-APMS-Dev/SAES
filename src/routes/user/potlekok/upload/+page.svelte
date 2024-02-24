<script lang="ts">
	import { onMount } from 'svelte';

	export let data;
	let teszt: string;
	onMount(() => {
		let file = document.getElementById('file') as HTMLInputElement;
		file.onchange = async function () {
			if (file && file.files) {
				const fsa = file.files[0];
				const reader = new FileReader();

				reader.onloadend = async () => {
					teszt = reader.result as string;
					const csa = await fetch('/api/upload', {
						method: 'POST',
						mode: 'no-cors',
						headers: {
							'Content-Type': 'text/plain'
						},
						body: reader.result
					});
					console.log(await csa.text());
				};
				reader.readAsDataURL(fsa);
			}
		};
	});
</script>

<div class="text-center text-white">
	<h1 class="text-3xl font-bold">Pótlék feltöltés</h1>
	<form action="javascript:;" enctype="multipart/form-data">
		<select name="type" id="type" class="bg-slate-700 text-xl p-2">
			<option value="délelőtti">Délelőtti</option>
			<option value="éjszakai">Éjszakai</option>
		</select>
		<input type="text" class="hidden" name="name" value={data.layout?.doksi.name} />
		<input type="file" name="file" id="file" accept="image/*" required />
		<h2>Darabszám: {data.db}</h2>
		<h2>(Pótlékonként 1 db kép a nyugtáról)</h2>
		<button type="submit">Feltöltés</button>
	</form>
	<img src={teszt} alt="" />
</div>
