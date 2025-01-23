<script lang="ts">
	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PageData } from './$types';
	import { formatRelative } from 'date-fns';
	import { locale } from '$lib/time';
	import { get_status_string, get_type_string } from '$lib/ucp/types';

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
</script>

<div class="flex">
	<div class="m-auto text-center text-black dark:text-white">
		<h1 class="font-itim text-3xl font-bold">Események</h1>
		<Table class="mt-5 table-auto p-10 text-center text-white">
			<TableHead class="rounded-xl">
				<TableHeadCell>Dátum</TableHeadCell>
				<TableHeadCell>Esemény létrehozója</TableHeadCell>
				<TableHeadCell>Esemény</TableHeadCell>
				<TableHeadCell>Elem (típus/id)</TableHeadCell>
				<TableHeadCell>Elem változtatása</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each data.logs as log}
					<TableBodyRow>
						<TableBodyCell
							>{formatRelative(new Date(new Date(log.date).valueOf() - data.offset!), new Date(), {
								locale
							})}</TableBodyCell
						>
						<TableBodyCell>{log.owner}</TableBodyCell>
						<TableBodyCell
							>{log.action === 'UPLOAD'
								? 'Feltöltés'
								: log.action === 'UPDATE ITEM'
									? 'Elem módosítás'
									: ''}</TableBodyCell
						>
						<TableBodyCell
							>{log.item_type ? get_type_string(log.item_type) : ''} / {log.item_id}</TableBodyCell
						>
						<TableBodyCell>{log.message ? handle_msg(log.message) : ''}</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	</div>
</div>
