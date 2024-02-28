<script lang="ts">
	import { onMount } from 'svelte';
	let modal: HTMLDialogElement;
	let potleks: any[] = [];
	let jona = 'feltöltve';
	let bindEdit: any = {};
	async function getPotleks(status: string) {
		const fatcs = await fetch('/api/admin', {
			headers: {
				status,
				type: 'potlekok'
			}
		});
		if (fatcs.ok) {
			return await fatcs.json();
		}
	}
	onMount(async () => {
		potleks = await getPotleks('feltöltve');
	});
	function edit(id: number) {
		modal.showModal();
		bindEdit = potleks[id];
	}
	async function rerun() {
		potleks = await getPotleks(jona);
	}
</script>

<dialog bind:this={modal} class="w-[50%] h-[50%] bg-gray-500 rounded-3xl text-white text-center">
	<h1 class="text-3xl font-bold mx-2">{bindEdit.owner} pótlékának szerkesztése</h1>
	<button
		class="text-red-600 text-3xl font-bold absolute top-2 right-4 duration-150 hover:text-red-400"
		on:click={() => modal.close()}>X</button
	>
	<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
		<form>
			<label for="type">Státusz átállítása</label>
			<select name="type" class="bg-green-800" bind:value={bindEdit.status}>
				<option value="feltöltve">Feltöltve</option>
				<option value="elfogadva">Elfogadva</option>
				<option value="elutasítva">Elutasítva</option>
			</select>
			<label for="reason">Megjegyzés</label>
			<input type="text" name="reason" id="reason" bind:value={bindEdit.reason} />
			<h2 class="text-red-500">még nem működik</h2>
			<button type="submit">Mentés</button>
		</form>
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if potleks}
			<h1 class="text-2xl font-bold">Pótlékok</h1>
			<select id="potlek-type" class="bg-green-800" bind:value={jona} on:change={() => rerun()}>
				<option value="feltöltve">Feltöltve</option>
				<option value="elfogadva">Elfogadva</option>
				<option value="elutasítva">Elutasítva</option>
			</select>
			<table class="table-auto p-10 rounded-2xl">
				<thead class="bg-red-700 rounded-2xl">
					<tr class="child:p-2">
						<th>Dátum</th>
						<th>IG Név</th>
						<th>Kép (Kattints rá)</th>
						<th>Státusz</th>
						<th>Megjegyzés</th>
						<th>Szerkesztés</th>
					</tr>
				</thead>
				<tbody>
					{#each potleks as potle}
						<tr class="bg-slate-800">
							<td
								>{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
									1}.{new Date(potle.date).getUTCDate()}. {new Date(
									potle.date
								).getUTCHours()}:{new Date(potle.date).getUTCMinutes()}</td
							>
							<td>{potle.owner}</td>
							<td
								><a href={`https://sckk-api.ampix.hu/img/data/${potle.id}`} target="”_blank”"
									><img
										src={`https://sckk-api.ampix.hu/img/data/${potle.id}`}
										alt=""
										class="max-w-52"
									/></a
								></td
							>
							<td>{potle.status}</td>
							<td>{potle.reason ? potle.reason : 'nincs'}</td>
							<td
								><button
									class="bg-green-800 font-bold px-2 py-1 rounded-xl hover:bg-green-600 transition-all duration-150"
									on:click={() => edit(potleks.indexOf(potle))}>Szerkesztés</button
								></td
							>
						</tr>
					{/each}
				</tbody>
			</table>
		{:else}
			<h2>Sikertelen lekérdezés</h2>
		{/if}
	</div>
</div>
