<script lang="ts">
	import { loading } from '$lib/loading';
	import { onMount } from 'svelte';
	export let title = '';
	export let type = '';
	export let editdes = '';
	export let extraText = '';
	let modal: HTMLDialogElement;
	let bindbtn: HTMLButtonElement;
	let potleks: {
		data: {
			date: Date;
			id: number;
			owner: string;
			reason: string | null;
			status: string;
			extra: string | null;
		}[];
		api: string;
	};
	let jona = 'feltöltve';
	let current = true;
	let bindEdit: any = {};
	let editid = 0;
	async function getPotleks(status: string, current: boolean) {
		$loading = true;
		const fatcs = await fetch('/api/admin', {
			headers: {
				status,
				type: type,
				current: current.toString()
			}
		});
		$loading = false;
		if (fatcs.ok) {
			return await fatcs.json();
		}
	}
	onMount(async () => {
		potleks = await getPotleks('feltöltve', true);
		console.log(potleks);
	});

	const potlekas: { [key: string]: boolean } = {};

	function edit(id: number) {
		modal.showModal();
		bindEdit = potleks.data[id];
		editid = id;
	}
	async function rerun() {
		potleks = await getPotleks(jona, current);
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
				potleks.data[editid] = cucc;
			} else {
				potleks = await getPotleks('feltöltve', current);
			}
		}
		bindbtn.classList.remove('cursor-not-allowed');
		bindbtn.classList.remove('bg-orange-700');
		bindbtn.disabled = false;
	}
</script>

<dialog bind:this={modal} class="bg-gray-500 w-[60%] h-[60%] rounded-3xl text-white text-center">
	<button
		class="text-red-600 text-3xl font-bold absolute top-2 right-4 duration-150 hover:text-red-400"
		on:click={() => modal.close()}>X</button
	>
	<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
		<form on:submit|preventDefault={() => editDone()}>
			<div class="grid grid-cols-2 gap-3">
				<h1 class=" col-span-2 text-3xl font-bold mx-2">{bindEdit.owner} {editdes} szerkesztése</h1>
				<label for="type" class="text-xl">Státusz átállítása</label>
				<select name="type" class="bg-green-800 text-xl" bind:value={bindEdit.status}>
					<option value="feltöltve">feltöltve</option>
					<option value="elfogadva">elfogadva</option>
					<option value="elutasítva">elutasítva</option>
				</select>

				<label for="reason" class="text-xl">Megjegyzés</label>
				<input
					type="text"
					name="reason"
					id="reason"
					class="text-black text-xl"
					bind:value={bindEdit.reason}
				/>
				<label for="extra" class="text-xl">{extraText}</label>
				<input
					type="text"
					name="extra"
					id="extra"
					class="text-black text-xl"
					bind:value={bindEdit.extra}
				/>
				<button
					type="submit"
					bind:this={bindbtn}
					id="dialogbtn"
					class="bg-orange-500 col-span-2 hover:bg-orange-700 transition-all duration-200 text-2xl px-2 py-1 rounded-xl"
					>Mentés</button
				>
			</div>
		</form>
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if potleks}
			{#if potleks.data}
				<h1 class="text-2xl font-bold">{title}</h1>
				<div class="flex justify-center gap-2">
					<h2>Filter</h2>
					<select id="potlek-type" class="bg-green-800" bind:value={jona} on:change={() => rerun()}>
						<option value="feltöltve">Feltöltve</option>
						<option value="elfogadva">Elfogadva</option>
						<option value="elutasítva">Elutasítva</option>
					</select>
					<h2>Aktuális hét</h2>
					<input type="checkbox" name="csekd" bind:checked={current} on:change={() => rerun()} />
				</div>
				<table class="table-auto p-10 rounded-2xl">
					<thead class="bg-red-700 rounded-2xl">
						<tr class="child:p-2">
							<th>Dátum</th>
							<th>IG Név</th>
							<th>Kép (Kattints rá)</th>
							<th>Státusz</th>
							<th>Megjegyzés</th>
							{#if extraText}
								<th>{extraText}</th>
							{/if}
							<th>Műveletek</th>
						</tr>
					</thead>
					<tbody>
						{#each potleks.data as potle}
							<tr class="bg-slate-800">
								<td
									>{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
										1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
										1}:{new Date(potle.date).getUTCMinutes()}</td
								>
								<td>{potle.owner}</td>
								<td>
									{#if type !== 'leintés'}
										{#if potlekas[potle.id]}
											<a href={`${potleks.api}/img/data/${potle.id}`} target="”_blank”"
												><img
													src={`${potleks.api}/img/data/${potle.id}`}
													alt=""
													class="max-w-52"
												/></a
											>
										{:else}
											<button
												class="bg-green-600 font-bold px-2 rounded-lg hover:bg-green-700 transition-all duration-200"
												on:click={() => {
													potlekas[potle.id] = true;
												}}>Mutasd</button
											>
										{/if}
									{:else if potlekas[potle.id]}
										<div class="flex flex-col xl:flex-row">
											<a href={`${potleks.api}/img/data/${potle.id}/0`} target="”_blank”"
												><img
													src={`${potleks.api}/img/data/${potle.id}/0`}
													alt=""
													class="max-w-52"
												/></a
											>
											<a href={`${potleks.api}/img/data/${potle.id}/1`} target="”_blank”"
												><img
													src={`${potleks.api}/img/data/${potle.id}/1`}
													alt=""
													class="max-w-52"
												/></a
											>
										</div>
									{:else}
										<button
											class="bg-green-600 font-bold px-2 rounded-lg hover:bg-green-700 transition-all duration-200"
											on:click={() => {
												potlekas[potle.id] = true;
											}}>Mutasd</button
										>
									{/if}
								</td>
								<td>{potle.status}</td>
								<td>{potle.reason ? potle.reason : 'nincs'}</td>
								{#if extraText}
									<td>{potle.extra}</td>
								{/if}
								<td
									><button
										class="icon-[lucide--edit] bg-green-600 w-6 h-6 font-bold px-2 py-1 rounded-xl hover:bg-green-500 transition-all duration-150"
										on:click={() => edit(potleks.data.indexOf(potle))}
									></button></td
								>
							</tr>
						{/each}
					</tbody>
				</table>
				{#if potleks.data.length === 0}
					<h2>Nincs ilyen elem az adatbázisban!</h2>
				{/if}
			{/if}
		{:else}
			<h2>Sikertelen lekérdezés</h2>
		{/if}
	</div>
</div>
