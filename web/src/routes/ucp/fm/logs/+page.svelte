<script lang="ts">
	import {
		Select,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		Tooltip
	} from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import { get_status_string, get_type_string } from '$lib/ucp/types';
	import type { SMGetItemsFull } from '$lib/types';
	import { imageUrl } from '$lib/api';
	import { loading } from '$lib/loading.svelte';

	let modal: HTMLDialogElement | undefined = $state();
	let modalItem: SMGetItemsFull | undefined = $state();
	let modaltype: string | undefined = $state();
	let modalDetails: string | undefined = $state();
	let { data }: { data: PageData } = $props();
	let filter = $state('all');
	let details = $state('');

	function handle_msg(msg: string) {
		let raw = ['status', 'price', 'supp_type', 'reason'];
		let hun = ['állapot', 'összeg', 'típus', 'megjegyzés'];
		let msgs = msg.split(';');
		let done_text = '';
		for (const text of msgs) {
			if (done_text.length > 0) {
				done_text += ', ';
			}
			let letters = text.split(' ');
			for (const letter of letters) {
				if (raw.includes(letter)) {
					done_text += `${hun[raw.indexOf(letter)]}: ${letter === 'status' ? get_status_string(Number(text.split('FROM')[1].split('TO')[0].replaceAll('{saes_semicolon}', ';'))) : text.split('FROM')[1].split('TO')[0].replaceAll('{saes_semicolon}', ';')} -> ${letter === 'status' ? get_status_string(Number(text.split('TO')[1].replaceAll('{saes_semicolon}', ';'))) : text.split('TO')[1].replaceAll('{saes_semicolon}', ';')}`;
				}
			}
		}
		return done_text;
	}

	function get_details(details: string, type: string) {
		if (type === 'UPDATE ITEM') {
			modaltype = type;
			let msg = handle_msg(details);
			modal!.showModal();
			modalDetails = msg;
		}
		if (type === 'UPLOAD ITEM') {
			loading.value = true;
			let jsoff: { type: number; id: number } = JSON.parse(details);
			fetch('/web-api/fm/get_by_id', {
				headers: {
					item_id: jsoff.id.toString(),
					item_type: jsoff.type.toString()
				}
			}).then(async (res) => {
				let body: SMGetItemsFull = await res.json();
				loading.value = false;
				modalItem = body;
				modaltype = type;
				modal!.showModal();
			});
		}
	}
	function closeModal() {
		modal!.close();
		modalItem = undefined;
		modaltype = undefined;
	}
</script>

<dialog
	bind:this={modal}
	class="h-screen w-screen rounded-3xl bg-black bg-opacity-75 text-center text-white lg:h-[800px] lg:w-[600px]"
>
	<button
		aria-label="Bezárás"
		class="absolute right-4 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-red-600 duration-150 hover:bg-opacity-90"
		onclick={() => closeModal()}><span class="icon-[carbon--close-filled] m-auto"></span></button
	>
	<div class="m-auto mt-4 items-center justify-center">
		{#if modaltype === 'UPDATE ITEM'}
			<h1 class="mb-3 text-3xl font-bold">Változások:</h1>
			<h2>{modalDetails}</h2>
		{/if}
		{#if modaltype === 'UPLOAD ITEM' && modalItem}
			<h1 class="mb-3 text-3xl font-bold">Feltöltött elem információi:</h1>
			{#if modalItem?.type == 2}
				<div class="flex gap-1">
					<a href={`${imageUrl}/get?id=${modalItem?.img_1}`}
						><img
							class="m-auto"
							src={`${imageUrl}/get?id=${modalItem?.img_1}`}
							width="300vw"
							alt=""
						/></a
					>
					<a href={`${imageUrl}/get?id=${modalItem?.img_2}`}
						><img
							class="m-auto"
							src={`${imageUrl}/get?id=${modalItem?.img_1}`}
							width="300vw"
							alt=""
						/></a
					>
				</div>
			{:else}
				<a href={`${imageUrl}/get?id=${modalItem?.img_1}`}
					><img
						class="m-auto"
						src={`${imageUrl}/get?id=${modalItem?.img_1}`}
						width="300vw"
						alt=""
					/></a
				>
			{/if}
			<h2>Feltöltő: {modalItem.owner}</h2>
			<h2>
				Kép dátuma: {formatRelative(
					new Date(new Date(modalItem.date).valueOf() - data.offset!),
					new Date(),
					{
						locale
					}
				)}
			</h2>
			<h2>Státusz: {get_status_string(modalItem.status)}</h2>
			{#if modalItem.reason}
				<h2>Megjegyzés: {modalItem.reason}</h2>
			{/if}
			{#if modalItem.price}
				<h2>Összeg: {modalItem.price}</h2>
			{/if}
			{#if modalItem.type}
				<h2>Típus: {get_type_string(modalItem.type)}</h2>
			{/if}
			{#if modalItem.handled_by}
				<h2>Kezelte: {modalItem.handled_by}</h2>
			{/if}
		{/if}
	</div>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		<h1 class="font-itim mb-3 mt-2 text-3xl font-bold">Események</h1>
		<div class="flex items-center justify-center gap-5">
			<h1>Filter:</h1>
			<Select bind:value={filter} placeholder="Válassz esemény típust">
				<option value="all">Minden</option>
				<option value="UPLOAD ITEM">Elem feltöltés</option>
				<option value="UPDATE ITEM">Elem szerkesztés</option>
			</Select>
		</div>
		<Table class="mt-5 table-auto p-10 text-center text-white">
			<TableHead class="rounded-xl">
				<TableHeadCell>Kép</TableHeadCell>
				<TableHeadCell>Dátum</TableHeadCell>
				<TableHeadCell>Esemény létrehozója</TableHeadCell>
				<TableHeadCell>Esemény</TableHeadCell>
				<TableHeadCell>Elem (típus/id)</TableHeadCell>
				<TableHeadCell>Részletek</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each data.logs as log}
					{#if filter === 'all' || filter === log.action}
						<TableBodyRow>
							<TableBodyCell
								><span
									class={`${
										log.action === 'UPLOAD ITEM'
											? 'icon-[material-symbols--upload-file] text-green-500'
											: ''
									} ${
										log.action === 'UPDATE ITEM'
											? 'icon-[material-symbols--edit-document] text-blue-600'
											: ''
									} h-10 w-10`}
								></span></TableBodyCell
							>
							<TableBodyCell
								>{formatRelative(
									new Date(new Date(log.date).valueOf() - data.offset!),
									new Date(),
									{
										locale
									}
								)}</TableBodyCell
							>
							<TableBodyCell>{log.owner}</TableBodyCell>
							<TableBodyCell
								>{#if log.action === 'UPLOAD ITEM'}
									Elem feltöltés
								{/if}
								{#if log.action === 'UPDATE ITEM'}
									Elem szerkesztés
								{/if}</TableBodyCell
							>
							<TableBodyCell
								>{log.item_type ? get_type_string(log.item_type) : ''} / {log.item_id}</TableBodyCell
							>
							<TableBodyCell
								>{#if log.action === 'UPDATE ITEM'}<button
										onclick={() => get_details(log.message!, log.action)}
										aria-label="More"
										class="icon-[material-symbols--ad] h-10 w-10 transition-colors duration-150 hover:text-emerald-400"
									></button><Tooltip>Részletek megnézése</Tooltip>
								{/if}
								{#if log.action === 'UPLOAD ITEM'}<button
										onclick={() =>
											get_details(
												JSON.stringify({
													type: log.item_type,
													id: log.item_id
												}),
												log.action
											)}
										aria-label="More"
										class="icon-[material-symbols--ad] h-10 w-10 transition-colors duration-150 hover:text-emerald-400"
									></button><Tooltip>Részletek megnézése</Tooltip>
								{/if}
							</TableBodyCell>
						</TableBodyRow>
					{/if}
				{/each}
			</TableBody>
		</Table>
	</div>
</div>
