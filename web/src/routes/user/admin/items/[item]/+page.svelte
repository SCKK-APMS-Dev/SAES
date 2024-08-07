<script lang="ts">
	import Error from '$lib/error.svelte';
	import { loading } from '$lib/loading';
	import { onMount } from 'svelte';
	import { Tooltip, Button, Select, Checkbox } from 'flowbite-svelte';
	export let data;
	export let tools: string[] = [];
	export let am: boolean = false;
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
			am: boolean;
		}[];
		api: string;
		error: boolean;
	} = { data: [], api: '', error: false };
	let jona = 'feltöltve';
	let current = true;
	let bindEdit: any = {};
	let editid = 0;
	async function getPotleks(status: string, current: boolean) {
		$loading = true;
		const fatcs = await fetch('/api/admin', {
			headers: {
				status,
				am: String(am),
				type: data.type as string,
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
	});

	function edit(id: number) {
		modal.showModal();
		bindEdit = potleks.data[id];
		editid = id;
	}
	async function quickTools(type: string, id: number) {
		const fatcs = await fetch('/api/admin', {
			headers: {
				'Content-Type': 'application/json'
			},
			method: 'POST',
			body: JSON.stringify({
				am: potleks.data[id].am,
				id: potleks.data[id].id,
				status:
					type === 'accept'
						? 'elfogadva'
						: type === 'decline'
							? 'elutasítva'
							: type === 'de' || 'du'
								? 'elfogadva'
								: potleks.data[id].status,
				reason: potleks.data[id].reason,
				extra: type === 'de' ? 'délelőtti' : type === 'du' ? 'éjszakai' : potleks.data[id].extra
			})
		});
		if (fatcs.ok) {
			const cucc = await fatcs.json();
			modal.close();
			if (jona === cucc.status) {
				potleks.data[id] = cucc;
			} else {
				potleks = await getPotleks('feltöltve', current);
			}
		}
	}
	async function rerun() {
		potleks = await getPotleks(jona, current);
	}
	async function editDone() {
		bindbtn.classList.add('cursor-not-allowed');
		bindbtn.classList.add('bg-emerald-700');
		bindbtn.disabled = true;
		const fatcs = await fetch('/api/admin', {
			headers: {
				'Content-Type': 'application/json'
			},
			mode: 'no-cors',
			method: 'POST',
			body: JSON.stringify({
				id: bindEdit.id,
				am: bindEdit.am,
				status: bindEdit.status,
				reason: bindEdit.reason,
				extra: bindEdit.extra
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
		bindbtn.classList.remove('bg-emerald-700');
		bindbtn.disabled = false;
	}
</script>

<Error {data}>
	{#if data.real}
		<dialog
			bind:this={modal}
			class="h-[60%] w-[60%] rounded-3xl bg-gray-500 text-center text-white"
		>
			<button
				class="absolute right-4 top-2 text-3xl font-bold text-red-600 duration-150 hover:text-red-400"
				on:click={() => modal.close()}>X</button
			>
			<div class="absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2">
				<form on:submit|preventDefault={() => editDone()}>
					<div class="grid grid-cols-2 items-center gap-3">
						<h1 class=" col-span-2 mx-2 text-3xl font-bold">
							{bindEdit.owner}
							{data.real[0]} szerkesztése
						</h1>
						<label for="type" class="text-xl">Státusz átállítása</label>
						<Select
							placeholder="Kérlek válassz"
							name="type"
							class="bg-emerald-600 text-xl text-white"
							bind:value={bindEdit.status}
						>
							<option value="feltöltve">feltöltve</option>
							<option value="elfogadva">elfogadva</option>
							<option value="elutasítva">elutasítva</option>
						</Select>

						<label for="reason" class="text-xl">Megjegyzés</label>
						<input
							type="text"
							name="reason"
							id="reason"
							class="text-xl text-black"
							bind:value={bindEdit.reason}
						/>
						{#if data.type}
							<label for="extra" class="text-xl">{data.type}</label>
							<input
								type="text"
								name="extra"
								id="extra"
								class="text-xl text-black"
								bind:value={bindEdit.extra}
							/>
						{/if}
						<button
							type="submit"
							bind:this={bindbtn}
							id="dialogbtn"
							class="col-span-2 rounded-xl bg-emerald-500 px-2 py-1 text-2xl transition-all duration-200 hover:bg-emerald-700"
							>Mentés</button
						>
					</div>
				</form>
			</div>
		</dialog>

		<div class="flex">
			<div class="m-auto text-center text-black dark:text-white">
				{#if potleks && !potleks.error}
					<h1 class="text-2xl font-bold">{data.real[1]}</h1>
					<h1 class="text-xl font-bold">{data.real[2]}</h1>
					<div class="flex items-center justify-center gap-2 text-center">
						<h2 class="text-xl font-bold text-black dark:text-white">Filter</h2>
						<Select
							placeholder="Kérlek válassz"
							id="potlek-type"
							class="bg-emerald-600 font-bold text-white"
							bind:value={jona}
							on:change={() => rerun()}
						>
							<option value="feltöltve" class="font-bold">Feltöltve</option>
							<option value="elfogadva" class="font-bold">Elfogadva</option>
							<option value="elutasítva" class="font-bold">Elutasítva</option>
						</Select>
						<h2 class="text-xl font-bold text-black dark:text-white">Aktuális hét</h2>
						<Checkbox name="csekd" bind:checked={current} on:change={() => rerun()} />
					</div>
					<table class="mt-5 table-auto p-10 text-white">
						<thead class="rounded-xl bg-green-700">
							<tr class="child:p-2">
								<th>Dátum</th>
								<th>IG Név</th>
								<th>Kép (Kattints rá)</th>
								<th>Státusz</th>
								<th>Megjegyzés</th>
								{#if data.type}
									<th>{data.type}</th>
								{/if}
								{#if tools.length > 0}
									<th>Műveletek</th>
								{/if}
							</tr>
						</thead>
						<tbody>
							{#each potleks.data as potle}
								<tr class:bg-slate-800={!potle.am} class:bg-blue-800={potle.am}>
									<td
										>{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
											1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
											2}:{new Date(potle.date).getUTCMinutes()}</td
									>
									<td
										>{potle.owner}
										{#if potle.am}
											(Autómentős)
										{/if}</td
									>
									<td>
										{#if data.type == 'leintes'}
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
											<a href={`${potleks.api}/img/data/${potle.id}`} target="”_blank”"
												><img
													src={`${potleks.api}/img/data/${potle.id}`}
													alt=""
													class="max-w-52"
												/></a
											>
										{/if}
									</td>
									<td>{potle.status}</td>
									<td>{potle.reason ? potle.reason : 'nincs'}</td>
									{#if data.type}
										<td>{potle.extra ? potle.extra : 'nincs'}</td>
									{/if}
									{#if tools.length > 0}
										<td>
											{#if tools.includes('délelőtti') && jona === 'feltöltve'}
												<Button
													class="icon-[lucide--sun] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-yellow-300"
													on:click={() => quickTools('de', potleks.data.indexOf(potle))}
												></Button>
												<Tooltip class="bg-slate-500"
													>{data.real[0]} elfogadása délelőttiként</Tooltip
												>
											{/if}
											{#if tools.includes('éjszakai') && jona === 'feltöltve'}
												<Button
													class="icon-[lucide--moon] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-blue-800"
													on:click={() => quickTools('du', potleks.data.indexOf(potle))}
												></Button>
												<Tooltip class="bg-slate-500"
													>{data.real[0]} elfogadása éjszakaiként</Tooltip
												>
											{/if}
											{#if tools.includes('accept') && jona === 'feltöltve'}
												<Button
													class="icon-[lucide--check] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-green-500"
													on:click={() => quickTools('accept', potleks.data.indexOf(potle))}
												></Button>
												<Tooltip class="bg-slate-500">{data.real[0]} elfogadása</Tooltip>
											{/if}
											{#if tools.includes('decline') && jona === 'feltöltve'}
												<Button
													class="icon-[lucide--x] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-red-600"
													on:click={() => quickTools('decline', potleks.data.indexOf(potle))}
												></Button>
												<Tooltip class="bg-slate-500">{data.real[0]} elutasítása</Tooltip>
											{/if}
											{#if tools.includes('edit')}
												<Button
													class="icon-[lucide--edit] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
													on:click={() => edit(potleks.data.indexOf(potle))}
												></Button>
												<Tooltip class="bg-slate-500">{data.real[0]} szerkesztése</Tooltip>
											{/if}
										</td>
									{/if}
								</tr>
							{/each}
						</tbody>
					</table>
					{#if potleks.data.length === 0}
						<h2>Nincs ilyen elem az adatbázisban!</h2>
					{/if}
				{:else}
					<h2>Sikertelen lekérdezés</h2>
				{/if}
			</div>
		</div>
	{/if}
</Error>
