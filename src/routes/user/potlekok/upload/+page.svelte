<script lang="ts">
	import { onMount } from 'svelte';

	export let data;
	let fileas: string[] = [];
	onMount(() => {
		let file = document.getElementById('file') as HTMLInputElement;
		file.onchange = async function () {
			if (file && file.files) {
				for (const filj of file.files) {
					const reader = new FileReader();

					reader.onloadend = async () => {
						const fatcs = await fetch('/api/upload', {
							method: 'POST',
							mode: 'no-cors',
							headers: {
								'Content-Type': 'text/plain'
							},
							body: reader.result
						});
						fileas.push(await fatcs.text());
						fileas = fileas;
					};
					reader.readAsDataURL(filj);
				}
			}
		};
	});
</script>

<div class="text-center text-white">
	<div class="bg-green-500 p-2">
		<h1 class="text-3xl font-bold">Pótlék feltöltés</h1>
		<form action="javascript:;" enctype="multipart/form-data">
			<select name="type" id="type" class="bg-green-800 text-xl p-2">
				<option value="délelőtti">Délelőtti</option>
				<option value="éjszakai">Éjszakai</option>
			</select>
			<input type="text" class="hidden" name="name" value={data.layout?.doksi.name} />
			<input type="file" name="file" id="file" accept="image/*" required multiple />
		</form>
		<h2>Egyszerre feltölthetsz többet, csak figyelj oda, hogy a megfelelő pótlékhoz töltöd fel!</h2>
	</div>
	<div class="bg-yellow-400 flex-row align-middle items-center justify-center">
		<h2>Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
		{#each fileas as nyam}
			<img src={nyam} alt="" class="max-w-5xl max-h-5xl m-auto py-3" />
		{/each}
	</div>
</div>
