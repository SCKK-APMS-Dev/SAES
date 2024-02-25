<script lang="ts">
	import { loading } from '$lib/loading.js';

	export let data;
	let fileas: string[] = [];
	let formerror: string | undefined = undefined;
	function upload() {
		$loading = true;
		formerror = undefined;
		let file = document.getElementById('file') as HTMLInputElement;
		if (file && file.files) {
			if (file.files.length % 2 == 0) {
				for (let i = 0; i < file.files.length / 2; i++) {
					const reader = new FileReader();
					const filj = file.files[i * 2];
					reader.onloadend = async () => {
						const reader2 = new FileReader();
						const filj2 = file.files![i * 2 + 1];
						reader2.onloadend = async () => {
							const fatcs = await fetch('/api/upload', {
								method: 'POST',
								mode: 'no-cors',
								headers: {
									'Content-Type': 'application/json'
								},
								body: JSON.stringify({
									img: [reader.result, reader2.result],
									createdAt: filj2.lastModified,
									type: 'leintes'
								})
							});
							fileas.push(await fatcs.text());
							fileas = fileas;
							if (file.files?.item(file.files.length - 1) === filj2) {
								$loading = false;
							}
						};
						reader2.readAsDataURL(filj2);
					};
					reader.readAsDataURL(filj);
				}
			} else {
				$loading = false;
				formerror = 'Kérlek kettesével töltsd fel a képeket!';
			}
		}
	}
</script>

<div class="text-center text-white">
	<div class="bg-green-500 p-2">
		<h2 class="font-bold text-red-800 drop-shadow-xl">{formerror ? formerror : ''}</h2>
		<h1 class="text-3xl font-bold">Leintés feltöltés</h1>
		<form on:submit|preventDefault={() => upload()} enctype="multipart/form-data">
			<input type="text" class="hidden" name="name" value={data.layout?.doksi.name} />
			<input type="file" name="file" id="file" accept="image/*" required multiple />
			<button type="submit" class="bg-red-600 font-bold text-xl px-2 rounded-xl">Feltöltés</button>
		</form>
		<h2>
			FONTOS! Bármennyit feltölthetsz csak a sorrend legyen meg! (Pótlékonként 2 kép egymás után
			kiválasztva)
		</h2>
	</div>
	<div class="flex-row align-middle items-center justify-center">
		<h2 class="font-bold">Ha sikeresen feltöltötted őket akkor itt fognak megjelenni:</h2>
		{#each fileas as nyam}
			<div class="flex flex-col">
				<img
					src={`https://sckk-api.ampix.hu/img/leintesek/${nyam}/1`}
					alt=""
					class="max-w-5xl max-h-5xl m-auto py-3"
				/>
				<img
					src={`https://sckk-api.ampix.hu/img/leintesek/${nyam}/2`}
					alt=""
					class="max-w-5xl max-h-5xl m-auto py-3"
				/>
			</div>
		{/each}
	</div>
</div>
