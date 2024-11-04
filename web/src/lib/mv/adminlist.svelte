<script lang="ts">
	import { loading } from '$lib/loading';
	import { onMount } from 'svelte';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import {
		Tooltip,
		Button,
		Select,
		Table,
		TableHead,
		TableBody,
		TableBodyRow,
		TableHeadCell,
		TableBodyCell
	} from 'flowbite-svelte';
	import { page } from '$app/stores';
	import { beforeNavigate, goto } from '$app/navigation';
	import type { PageData } from '../../routes/ucp/mv/leintesek/$types';
	export let title = '';
	export let data: PageData;
	export let type = '';
	export let editdes = '';
	export let extraText = '';
	export let des = '';
	let haveadmin = false;
	export let tools: string[] = [];
	export let am: boolean = false;
	let modal: HTMLDialogElement;
	let bindbtn: HTMLButtonElement;
	let editing = false;
	let originallength = 0;
	let potleks: {
		data: {
			items: {
				date: Date;
				id: number;
				owner: string;
				reason: string | null;
				status: string;
				extra: string | null;
				admin: string | null;
				am: boolean;
			}[];
		};

		api: string;
		error: boolean;
	} = { data: { items: [] }, api: '', error: false };
	let jona = data.status;
	let multipage = false;
	let bindEdit: any = {};
	let editid = 0;
	async function render() {
		$loading = true;
		const fatcs = await fetch('/web-api/admin', {
			headers: {
				status: jona as string,
				am: String(am),
				type: type
			}
		});
		if (fatcs.ok) {
			let handled = [];
			potleks.data.items = [];
			let ret = await fatcs.json();

			if (ret.data.items.length > 20 && ret.data.items.length > 0) {
				multipage = true;
				for (let i = pagee * 20; i < (pagee as number) * 20 + 20; i++) {
					if (ret.data.items[i]) {
						handled.push(ret.data.items[i]);
					}
				}
				potleks = {
					api: ret.api,
					error: ret.error,
					data: {
						items: handled
					}
				};
			} else {
				potleks = ret;
			}
			for (const elem of potleks.data.items) {
				if (elem.admin !== null) {
					haveadmin = true;
					break;
				}
				if (potleks.data.items.indexOf(elem) === potleks.data.items.length - 1) {
					haveadmin = false;
				}
			}
			if (potleks.data.items.length === 0) {
				haveadmin = false;
			}
			originallength = ret.data.items.length;
			$loading = false;
		}
	}
	onMount(() => {
		render();
	});

	beforeNavigate((ev) => {
		if (editing) {
			ev.cancel();
		}
	});

	function edit(id: number) {
		modal.showModal();
		bindEdit = potleks.data.items[id];
		bindEdit.custombg = false;
		editid = id;
		editing = true;
	}
	async function quickTools(type: string, id: number) {
		const fatcs = await fetch('/web-api/admin', {
			headers: {
				'Content-Type': 'application/json'
			},
			method: 'POST',
			body: JSON.stringify({
				am: potleks.data.items[id].am,
				id: potleks.data.items[id].id,
				status:
					type === 'accept'
						? 'elfogadva'
						: type === 'decline'
							? 'elutasítva'
							: type === 'de' || 'du'
								? 'elfogadva'
								: potleks.data.items[id].status,
				reason: potleks.data.items[id].reason,
				extra:
					type === 'de' ? 'délelőtti' : type === 'du' ? 'éjszakai' : potleks.data.items[id].extra
			})
		});
		if (fatcs.ok) {
			const cucc = await fatcs.json();
			modal.close();
			if (jona === cucc.status) {
				potleks.data.items[id] = cucc;
			} else {
				await render();
			}
		}
	}
	let pagee = data.page as number;
	function switchPage(mode: 'next' | 'prev') {
		let url = new URL($page.url);
		if (mode === 'next') {
			url.searchParams.set('page', String(Number(pagee) + 1));
			goto(`?${url.searchParams.toString()}`);
			pagee = Number(pagee) + 1;
			render();
		}
		if (mode === 'prev') {
			url.searchParams.set('page', String(Number(pagee) - 1));
			goto(`?${url.searchParams.toString()}`);
			pagee = Number(pagee) - 1;
			render();
		}
	}
	function changestatus(ev: Event) {
		if (ev.target) {
			let url = new URL($page.url);
			// @ts-expect-error
			url.searchParams.set('status', ev.target[ev.target.selectedIndex].value);
			url.searchParams.delete('page', pagee.toString());
			goto(`?${url.searchParams.toString()}`);
			// @ts-expect-error
			jona = ev.target[ev.target.selectedIndex].value;
			pagee = 0;
			render();
		}
	}
	async function editDone() {
		bindbtn.classList.add('cursor-not-allowed');
		bindbtn.classList.add('bg-emerald-700');
		bindbtn.disabled = true;
		const fatcs = await fetch('/web-api/admin', {
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
			editing = false;
			if (jona === cucc.status) {
				potleks.data.items[editid] = cucc;
			} else {
				await render();
			}
		}
		bindbtn.classList.remove('cursor-not-allowed');
		bindbtn.classList.remove('bg-emerald-700');
		bindbtn.disabled = false;
	}
	function closeModal() {
		modal.close();
		editing = false;
	}
</script>

<dialog
	bind:this={modal}
	class="h-screen w-screen rounded-3xl bg-black bg-opacity-20 text-center text-white open:flex lg:h-[800px] lg:w-[600px]"
>
	{#if bindEdit.custombg}
		<img
			src={`${potleks.api}/img?id=${bindEdit.id}`}
			class="absolute left-1/2 top-1/2 h-full -translate-x-1/2 -translate-y-1/2 opacity-90"
			alt=""
		/>
	{/if}
	<button
		aria-label="Kép megnézése"
		class="absolute right-16 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-blue-600 duration-150 hover:bg-opacity-90"
		on:click={() => {
			if (bindEdit.custombg) {
				bindEdit.custombg = false;
			} else {
				bindEdit.custombg = true;
			}
		}}><span class="icon-[mdi--image] m-auto"></span></button
	>
	<button
		aria-label="Bezárás"
		class="absolute right-4 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-red-600 duration-150 hover:bg-opacity-90"
		on:click={() => closeModal()}><span class="icon-[carbon--close-filled] m-auto"></span></button
	>
	{#if !bindEdit.custombg}
		<div class="z-20 m-auto h-max w-max rounded-3xl bg-black bg-opacity-25 p-5 lg:w-[500px]">
			<form on:submit|preventDefault={() => editDone()}>
				<div class="grid grid-cols-2 items-center gap-3">
					<h1 class=" col-span-2 mx-2 text-3xl font-bold">
						{bindEdit.owner}
						{editdes} szerkesztése
					</h1>
					<label for="type" class="text-xl">Státusz</label>
					<Select
						placeholder="Kérlek válassz"
						name="type"
						class="bg-emerald-600 text-xl text-white opacity-80 focus:opacity-100"
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
						class="text-xl text-black opacity-80 focus:opacity-100"
						bind:value={bindEdit.reason}
					/>
					{#if extraText}
						<label for="extra" class="text-xl">{extraText}</label>
						{#if type === 'pótlék'}
							<Select
								placeholder="Kérlek válassz"
								name="potlek-type"
								class="bg-emerald-600 text-xl text-white opacity-80 focus:opacity-100"
								bind:value={bindEdit.extra}
							>
								<option value="délelőtti">délelőtti</option>
								<option value="éjszakai">éjszakai</option>
							</Select>
						{:else}
							<input
								type="text"
								name="extra"
								id="extra"
								class="text-xl text-black opacity-80 focus:opacity-100"
								bind:value={bindEdit.extra}
							/>
						{/if}
					{/if}
					<button
						type="submit"
						bind:this={bindbtn}
						id="dialogbtn"
						class="col-span-2 rounded-xl bg-emerald-500 px-2 py-1 text-2xl opacity-80 transition-all duration-200 hover:bg-emerald-700 hover:opacity-100"
						>Mentés</button
					>
				</div>
			</form>
		</div>
	{/if}
</dialog>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		{#if potleks && !potleks.error}
			<div class="flex items-end justify-center gap-1 text-end">
				<h1 class="text-2xl font-bold">{title}</h1>
				{#if originallength > 0}
					<h2 class="text-taxi text-xl font-bold">{originallength} db</h2>
				{/if}
			</div>
			<h1 class="mb-2 text-xl text-gray-400">{des}</h1>
			<div class="flex items-center justify-center gap-2 text-center">
				<h2 class="text-xl font-bold text-black dark:text-white">Filter</h2>
				<Select
					placeholder="Kérlek válassz"
					id="potlek-type"
					class="bg-emerald-600 font-bold text-white"
					bind:value={jona}
					on:input={(e) => changestatus(e)}
				>
					<option value="feltöltve" class="font-bold">Feltöltve</option>
					<option value="elfogadva" class="font-bold">Elfogadva</option>
					<option value="elutasítva" class="font-bold">Elutasítva</option>
				</Select>
			</div>
			<Table class="mt-5 table-auto p-10 text-center text-white">
				<TableHead class="rounded-xl">
					<TableHeadCell>Dátum</TableHeadCell>
					<TableHeadCell>IG Név</TableHeadCell>
					<TableHeadCell>Kép (Kattints rá)</TableHeadCell>
					<TableHeadCell>Státusz</TableHeadCell>
					<TableHeadCell>Megjegyzés</TableHeadCell>
					{#if extraText}
						<TableHeadCell>{extraText}</TableHeadCell>
					{/if}
					{#if haveadmin}
						<TableHeadCell>Kezelte</TableHeadCell>
					{/if}
					{#if tools.length > 0}
						<TableHeadCell>Műveletek</TableHeadCell>
					{/if}
				</TableHead>
				<TableBody>
					{#each potleks.data.items as potle}
						<TableBodyRow>
							<TableBodyCell
								>{formatRelative(
									new Date(new Date(potle.date).valueOf() - data.offset!),
									new Date(),
									{ locale }
								)}</TableBodyCell
							>
							<TableBodyCell
								>{potle.owner}
								{#if potle.am}
									(Autómentős)
								{/if}</TableBodyCell
							>
							<TableBodyCell>
								{#if type == 'leintés'}
									<div class="flex flex-col xl:flex-row">
										<a href={`${potleks.api}/limg?id=${potle.id}&ver=0`} target="”_blank”"
											><img
												loading="lazy"
												src={`${potleks.api}/limg?id=${potle.id}&ver=0`}
												alt=""
												class="lg:w-52"
											/></a
										>
										<a href={`${potleks.api}/limg?id=${potle.id}&ver=1`} target="”_blank”"
											><img
												loading="lazy"
												src={`${potleks.api}/limg?id=${potle.id}&ver=1`}
												alt=""
												class="lg:w-52"
											/></a
										>
									</div>
								{:else}
									<a href={`${potleks.api}/img?id=${potle.id}`} target="”_blank”"
										><img
											loading="lazy"
											src={`${potleks.api}/img?id=${potle.id}`}
											alt=""
											class="lg:w-52"
										/></a
									>
								{/if}
							</TableBodyCell>
							<TableBodyCell>{potle.status}</TableBodyCell>
							<TableBodyCell>{potle.reason ? potle.reason : 'nincs'}</TableBodyCell>
							{#if extraText}
								<TableBodyCell>{potle.extra ? potle.extra : 'nincs'}</TableBodyCell>
							{/if}
							{#if haveadmin}
								<TableBodyCell>{potle.admin ? potle.admin : '-'}</TableBodyCell>
							{/if}
							{#if tools.length > 0}
								<TableBodyCell>
									{#if tools.includes('délelőtti') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--sun] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-yellow-300"
											on:click={() => quickTools('de', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása délelőttiként</Tooltip>
									{/if}
									{#if tools.includes('éjszakai') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--moon] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-blue-800"
											on:click={() => quickTools('du', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása éjszakaiként</Tooltip>
									{/if}
									{#if tools.includes('accept') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--check] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-green-500"
											on:click={() => quickTools('accept', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elfogadása</Tooltip>
									{/if}
									{#if tools.includes('decline') && jona === 'feltöltve'}
										<Button
											class="icon-[lucide--x] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-red-600"
											on:click={() => quickTools('decline', potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} elutasítása</Tooltip>
									{/if}
									{#if tools.includes('edit')}
										<Button
											class="icon-[lucide--edit] h-6 w-6 rounded-xl bg-white font-bold transition-all duration-150 hover:bg-slate-500"
											on:click={() => edit(potleks.data.items.indexOf(potle))}
										></Button>
										<Tooltip class="bg-slate-500">{type} szerkesztése</Tooltip>
									{/if}
								</TableBodyCell>
							{/if}
						</TableBodyRow>
					{/each}
				</TableBody>
			</Table>
			{#if potleks.data.items.length === 0}
				<h2>Nincs ilyen elem az adatbázisban!</h2>
			{/if}
		{:else}
			<h2>Sikertelen lekérdezés</h2>
		{/if}
	</div>
</div>
{#if multipage}
	<div class="mb-5 mt-5 flex items-center justify-center gap-4">
		{#if pagee > 0}
			<button
				aria-label="Előző oldal"
				on:click={() => switchPage('prev')}
				class="hover:bg-pos-100 bg-size-200 bg-pos-0 rounded-full bg-gradient-to-r from-emerald-500 via-teal-600 to-red-500 text-white duration-300"
				style="width: calc(5vw*2.5); height: 5vh;"
				><span class="icon-[solar--map-arrow-left-bold] h-full w-full"></span></button
			>
		{/if}
		{#if Math.ceil(originallength / 10) - 1 > pagee}
			<button
				aria-label="Következő oldal"
				on:click={() => switchPage('next')}
				class="hover:bg-pos-100 bg-size-200 bg-pos-0 rounded-full bg-gradient-to-r from-emerald-500 via-teal-600 to-red-500 text-white duration-300"
				style="width: calc(5vw*2.5); height: 5vh;"
				><span class="icon-[solar--map-arrow-right-bold] h-full w-full"></span></button
			>
		{/if}
	</div>
{/if}
