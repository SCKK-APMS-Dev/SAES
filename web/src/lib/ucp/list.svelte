<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	export let data: PageData;
	import Error from '$lib/error.svelte';
	import { onMount } from 'svelte';
	import type { PageData } from '../../routes/ucp/potlekok/$types';
	let multipage = false;
	export let tipus = '';
	export let display = '';
	let handled_potleks: any = [];
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
	function render() {
		handled_potleks = [];
		if (data.potlekok.length > 10 && data.potlekok.length > 0) {
			multipage = true;
			for (let i = (pagee as number) * 10; i < (pagee as number) * 10 + 10; i++) {
				if (data.potlekok[i]) {
					handled_potleks.push(data.potlekok[i]);
				}
			}
		} else {
			handled_potleks = data.potlekok;
		}
	}
	onMount(() => {
		render();
	});
</script>

<Error {data}>
	<div class="grid grid-cols-1 text-center text-black dark:text-white">
		<a
			href={`${$page.url.pathname}/upload`}
			class="from-taxi hover:bg-pos-100 bg-size-200 bg-pos-0 mb-5 ml-5 mr-5 mt-5 block rounded-full bg-gradient-to-r via-teal-400 to-green-600 px-6 py-3 text-center text-xl font-bold text-white drop-shadow-lg transition-all duration-500"
			>Feltöltés</a
		>
		<h1 class="text-5xl font-bold drop-shadow-xl">{display}:</h1>
		<h2 class="mb-3 text-black dark:text-gray-400">(összesen {data.potlekok.length} darab)</h2>
		<div class="flex flex-auto flex-wrap items-center justify-center gap-3 align-middle">
			{#if handled_potleks}
				{#each handled_potleks as potle}
					<div
						class="rounded-xl p-2 drop-shadow-xl"
						class:bg-green-600={potle.status === 'elfogadva'}
						class:bg-red-600={potle.status === 'elutasítva'}
						class:bg-yellow-600={potle.status === 'feltöltve'}
					>
						<h1 class="font-bold drop-shadow-xl">
							{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
								1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
								2}:{new Date(potle.date).getUTCMinutes()}
						</h1>
						<h1 class="font-bold drop-shadow-xl">Státusz: {potle.status}</h1>
						{#if potle.reason}
							<h1 class="font-bold drop-shadow-xl">Megjegyzés: {potle.reason}</h1>
						{/if}
						{#if tipus === 'leintés'}
							<div class="flex flex-col xl:flex-row">
								<img
									src={`${data.api}/limg?id=${potle.id}&ver=0`}
									alt=""
									class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
								/>
								<img
									src={`${data.api}/limg?id=${potle.id}&ver=1`}
									alt=""
									class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
								/>
							</div>
						{:else}
							<img
								src={`${data.api}/img?id=${potle.id}`}
								alt=""
								class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
							/>
						{/if}
					</div>
				{/each}
			{:else}
				<h2>Az elmúlt 2 hétben nem töltöttél fel semmit!</h2>
			{/if}
		</div>
	</div>
	{#if multipage}
		<div class="mb-5 mt-5 flex items-center justify-center gap-4">
			{#if pagee > 0}
				<button
					on:click={() => switchPage('prev')}
					class="hover:bg-pos-100 bg-size-200 bg-pos-0 rounded-full bg-gradient-to-r from-emerald-500 via-teal-600 to-red-500 text-white duration-300"
					style="width: calc(5vw*2.5); height: 5vh;"
					><span class="icon-[solar--map-arrow-left-bold] h-full w-full"></span></button
				>
			{/if}
			{#if Math.ceil(data.potlekok.length / 10) - 1 > pagee}
				<button
					on:click={() => switchPage('next')}
					class="hover:bg-pos-100 bg-size-200 bg-pos-0 rounded-full bg-gradient-to-r from-emerald-500 via-teal-600 to-red-500 text-white duration-300"
					style="width: calc(5vw*2.5); height: 5vh;"
					><span class="icon-[solar--map-arrow-right-bold] h-full w-full"></span></button
				>
			{/if}
		</div>
	{/if}
</Error>
