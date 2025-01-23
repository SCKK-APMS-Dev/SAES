<script lang="ts">
	import {
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

	let modal: HTMLDialogElement | undefined = $state();
	let { data }: { data: PageData } = $props();

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

	function details(details: string, type: string) {
		if (type === 'UPDATE ITEM') {
			let msg = handle_msg(details);
			modal!.showModal();
		}
	}
	function closeModal() {
		modal!.close();
	}
</script>

<dialog
	bind:this={modal}
	class="h-screen w-screen rounded-3xl bg-black bg-opacity-75 text-center text-white open:flex lg:h-[800px] lg:w-[600px]"
>
	<h1>Cső</h1>
	<button
		aria-label="Bezárás"
		class="absolute right-4 top-2 flex items-center justify-center rounded-xl bg-black bg-opacity-40 p-2 text-3xl font-bold text-red-600 duration-150 hover:bg-opacity-90"
		onclick={() => closeModal()}><span class="icon-[carbon--close-filled] m-auto"></span></button
	>
</dialog>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		<h1 class="font-itim mt-2 text-3xl font-bold">Események</h1>
		<Table class="mt-5 table-auto p-10 text-center text-white">
			<TableHead class="rounded-xl">
				<TableHeadCell></TableHeadCell>
				<TableHeadCell>Dátum</TableHeadCell>
				<TableHeadCell>Esemény létrehozója</TableHeadCell>
				<TableHeadCell>Esemény</TableHeadCell>
				<TableHeadCell>Elem (típus/id)</TableHeadCell>
				<TableHeadCell>Részletek</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each data.logs as log}
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
							>{formatRelative(new Date(new Date(log.date).valueOf() - data.offset!), new Date(), {
								locale
							})}</TableBodyCell
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
							>{#if log.action === 'UPDATE ITEM' || log.action === 'UPLOAD ITEM'}<button
									onclick={() => details(log.message!, log.action)}
									aria-label="More"
									class="icon-[material-symbols--ad] h-10 w-10 transition-colors duration-150 hover:text-emerald-400"
								></button><Tooltip>Részletek megnézése</Tooltip>
							{/if}
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</div>
