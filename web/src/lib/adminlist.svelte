<script lang="ts">
	import { loading } from '$lib/loading';
	import { onMount } from 'svelte';
	export let title = '';
	export let type = '';
	export let editdes = '';
	let modal: HTMLDialogElement;
	let bindbtn: HTMLButtonElement;
	let potleks: any[] = [];
	let jona = 'feltöltve';
	let bindEdit: any = {};
	let editid = 0;
	async function getPotleks(status: string) {
		$loading = true;
		const fatcs = await fetch('/api/admin', {
			headers: {
				status,
				type: type
			}
		});
		$loading = false;
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
		editid = id;
	}
	async function rerun() {
		potleks = await getPotleks(jona);
	}
	async function editDone() {
		bindbtn.classList.add('cursor-not-allowed');
		bindbtn.classList.add('bg-orange-700');
		bindbtn.disabled = true;
		const fatcs = await fetch('/api/admin', {
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'no-cors',
			method: 'POST',
			body: JSON.stringify({
				id: bindEdit.id,
				status: bindEdit.status,
				reason: bindEdit.reason
			})
		});
		if (fatcs.ok) {
			const cucc = await fatcs.json();
			modal.close();
			if (jona === cucc.status) {
				potleks[editid] = cucc;
			} else {
				potleks = await getPotleks('feltöltve');
			}
		}
		bindbtn.classList.remove('cursor-not-allowed');
		bindbtn.classList.remove('bg-orange-700');
		bindbtn.disabled = false;
	}
</script>

<dialog bind:this={modal} class="w-[50%] h-[50%] bg-gray-500 rounded-3xl text-white text-center">
	<h1 class="text-3xl font-bold mx-2">{bindEdit.owner} {editdes}</h1>
	<button
		class="text-red-600 text-3xl font-bold absolute top-2 right-4 duration-150 hover:text-red-400"
		on:click={() => modal.close()}>X</button
	>
	<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
		<form on:submit|preventDefault={() => editDone()}>
			<label for="type" class="text-xl">Státusz átállítása</label>
			<select name="type" class="bg-green-800 text-xl" bind:value={bindEdit.status}>
				<option value="feltöltve">Feltöltve</option>
				<option value="elfogadva">Elfogadva</option>
				<option value="elutasítva">Elutasítva</option>
			</select>
			<label for="reason" class="text-xl">Megjegyzés</label>
			<input
				type="text"
				name="reason"
				id="reason"
				class="text-black text-xl"
				bind:value={bindEdit.reason}
			/>
			<button
				type="submit"
				bind:this={bindbtn}
				id="dialogbtn"
				class="bg-orange-500 hover:bg-orange-700 transition-all duration-200 text-2xl px-2 py-1 rounded-xl"
				>Mentés</button
			>
		</form>
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if potleks}
			<h1 class="text-2xl font-bold">{title}</h1>
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
							<td>
								{#if type !== 'leintés'}
									<a href={`https://sckk-api.ampix.hu/img/data/${potle.id}`} target="”_blank”"
										><img
											src={`https://sckk-api.ampix.hu/img/data/${potle.id}`}
											alt=""
											class="max-w-52"
										/></a
									>
								{:else}
									<div class="flex flex-col xl:flex-row">
										<a href={`https://sckk-api.ampix.hu/img/data/${potle.id}/1`} target="”_blank”"
											><img
												src={`https://sckk-api.ampix.hu/img/data/${potle.id}/1`}
												alt=""
												class="max-w-52"
											/></a
										>
										<a href={`https://sckk-api.ampix.hu/img/data/${potle.id}/2`} target="”_blank”"
											><img
												src={`https://sckk-api.ampix.hu/img/data/${potle.id}/2`}
												alt=""
												class="max-w-52"
											/></a
										>
									</div>
								{/if}
							</td>
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
