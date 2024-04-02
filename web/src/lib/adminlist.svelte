<script lang="ts">
	import { loading } from '$lib/loading';
	import { onMount } from 'svelte';
	import { Tooltip, Button, Select, Checkbox } from 'flowbite-svelte';
	export let title = '';
	export let type = '';
	export let editdes = '';
	export let extraText = '';
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
				am,
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
				am,
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

				am,
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

<dialog bind:this={modal} class="bg-gray-500 w-[60%] h-[60%] rounded-3xl text-white text-center">
	<button
		class="text-red-600 text-3xl font-bold absolute top-2 right-4 duration-150 hover:text-red-400"
		on:click={() => modal.close()}>X</button
	>
	<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
		<form on:submit|preventDefault={() => editDone()}>
			<div class="grid grid-cols-2 gap-3 items-center">
				<h1 class=" col-span-2 text-3xl font-bold mx-2">{bindEdit.owner} {editdes} szerkesztése</h1>
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
					class="text-black text-xl"
					bind:value={bindEdit.reason}
				/>
				{#if extraText}
					<label for="extra" class="text-xl">{extraText}</label>
					<input
						type="text"
						name="extra"
						id="extra"
						class="text-black text-xl"
						bind:value={bindEdit.extra}
					/>
				{/if}
				<button
					type="submit"
					bind:this={bindbtn}
					id="dialogbtn"
					class="bg-emerald-500 col-span-2 hover:bg-emerald-700 transition-all duration-200 text-2xl px-2 py-1 rounded-xl"
					>Mentés</button
				>
			</div>
		</form>
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-white">
		{#if potleks && !potleks.error}
			<h1 class="text-2xl font-bold">{title}</h1>
			<div class="flex justify-center text-center items-center gap-2">
				<h2 class="text-xl font-bold">Filter</h2>
				<Select
					placeholder="Kérlek válassz"
					id="potlek-type"
					class="bg-emerald-600 text-white font-bold"
					bind:value={jona}
					on:change={() => rerun()}
				>
					<option value="feltöltve" class="font-bold">Feltöltve</option>
					<option value="elfogadva" class="font-bold">Elfogadva</option>
					<option value="elutasítva" class="font-bold">Elutasítva</option>
				</Select>
				<h2 class="text-xl font-bold">Aktuális hét</h2>
				<Checkbox name="csekd" bind:checked={current} on:change={() => rerun()} />
			</div>
			<table class="table-auto p-10 mt-5 rounded-2xl">
				<thead class="bg-green-700 rounded-2xl">
					<tr class="child:p-2">
						<th>Dátum</th>
						<th>IG Név</th>
						<th>Kép (Kattints rá)</th>
						<th>Státusz</th>
						<th>Megjegyzés</th>
						{#if extraText}
							<th>{extraText}</th>
						{/if}
						{#if tools.length > 0}
							<th>Műveletek</th>
						{/if}
					</tr>
				</thead>
				<tbody>
					{#each potleks.data as potle}
						<tr class="bg-slate-800">
							<td
								>{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
									1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
									2}:{new Date(potle.date).getUTCMinutes()}</td
							>
							<td>{potle.owner}</td>
							<td>
								{#if type == 'leintés'}
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
										><img src={`${potleks.api}/img/data/${potle.id}`} alt="" class="max-w-52" /></a
									>
								{/if}
							</td>
							<td>{potle.status}</td>
							<td>{potle.reason ? potle.reason : 'nincs'}</td>
							{#if extraText}
								<td>{potle.extra ? potle.extra : 'nincs'}</td>
							{/if}
							{#if tools.length > 0}
								<td>
									{#if tools.includes('délelőtti') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--sun] bg-white w-6 h-6 font-bold rounded-xl hover:bg-yellow-300 transition-all duration-150"
											on:click={() => quickTools('de', potleks.data.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása délelőttiként</Tooltip>
									{/if}
									{#if tools.includes('éjszakai') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--moon] bg-white w-6 h-6 font-bold rounded-xl hover:bg-blue-800 transition-all duration-150"
											on:click={() => quickTools('du', potleks.data.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása éjszakaiként</Tooltip>
									{/if}
									{#if tools.includes('accept') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--check] bg-white w-6 h-6 font-bold rounded-xl hover:bg-green-500 transition-all duration-150"
											on:click={() => quickTools('accept', potleks.data.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása</Tooltip>
									{/if}
									{#if tools.includes('decline') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--x] bg-white w-6 h-6 font-bold rounded-xl hover:bg-red-600 transition-all duration-150"
											on:click={() => quickTools('decline', potleks.data.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elutasítása</Tooltip>
									{/if}
									{#if tools.includes('edit')}
										<Button
											class="icon-[lucide--edit] bg-white w-6 h-6 font-bold rounded-xl hover:bg-slate-500 transition-all duration-150"
											on:click={() => edit(potleks.data.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} szerkesztése</Tooltip>
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
