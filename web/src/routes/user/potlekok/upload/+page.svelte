<script lang="ts">
	import { loading } from '$lib/loading.js';

	export let data;
	let fileas: string[] = [];
	function upload() {
		$loading = true;
		let file = document.getElementById('file') as HTMLInputElement;
		if (file && file.files) {
			for (const filj of file.files) {
				const reader = new FileReader();

				reader.onloadend = async () => {
					const fatcs = await fetch('/api/upload', {
						method: 'POST',
						mode: 'no-cors',
						headers: {
							'Content-Type': 'application/json'
						},
						body: JSON.stringify({
							img: reader.result,
							createdAt: filj.lastModified,
							type: 'potlek'
						})
					});
					fileas.push(await fatcs.text());
					fileas = fileas;
					if (file.files?.item(file.files.length - 1) === filj) {
						$loading = false;
					}
				};
				reader.readAsDataURL(filj);
			}
		}
	}
</script>

<div class="text-center text-white">
	<div class="bg-green-500 p-2">
		<h1 class="text-3xl font-bold">Pótlék feltöltés</h1>
		<form on:submit|preventDefault={() => upload()} enctype="multipart/form-data">
			<input type="text" class="hidden" name="name" value={data.layout?.doksi.name} />
			<input type="file" name="file" id="file" accept="image/*" required multiple />
			<button type="submit" class="bg-red-600 font-bold text-xl px-2 rounded-xl">Feltöltés</button>
		</form>
		<h2>Egyszerre feltölthetsz többet, csak figyelj oda, hogy a megfelelő pótlékhoz töltöd fel!</h2>
	</div>
	<div class="flex-row align-middle items-center justify-center">
		<h2 class="font-bold">Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
		{#each fileas as nyam}
			<img
				src={`https://sckk-api.ampix.hu/img/data/${nyam}`}
				alt=""
				class="max-w-5xl max-h-5xl m-auto py-3"
			/>
		{/each}
	</div>
</div>
