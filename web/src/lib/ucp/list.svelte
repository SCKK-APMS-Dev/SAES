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
		<div class="mb-3 flex flex-auto flex-wrap items-center justify-center gap-3 align-middle">
			{#if handled_potleks}
				{#each handled_potleks as potle}
					<div
						class="rounded-lg bg-gradient-to-bl p-2 drop-shadow-xl"
						class:from-teal-500={potle.status === 'elfogadva'}
						class:to-green-600={potle.status === 'elfogadva'}
						class:from-amber-500={potle.status === 'elutasítva'}
						class:to-red-600={potle.status === 'elutasítva'}
						class:from-cyan-800={potle.status === 'feltöltve'}
						class:to-gray-700={potle.status === 'feltöltve'}
					>
						<h1 class="-mb-2 text-2xl font-bold drop-shadow-xl">{potle.status.toUpperCase()}</h1>
						<h1 class="text-gray-200 drop-shadow-xl">
							{new Date(potle.date).getUTCFullYear()}.{new Date(potle.date).getUTCMonth() +
								1}.{new Date(potle.date).getUTCDate()}. {new Date(potle.date).getUTCHours() +
								2}:{new Date(potle.date).getUTCMinutes()}
						</h1>
						{#if potle.reason}
							<h1 class="drop-shadow-xl">Megjegyzés: {potle.reason}</h1>
						{/if}

						{#if tipus === 'leintés'}
							<div class="flex flex-col xl:flex-row">
								<a
									href={`${data.api}/limg?id=${potle.id}&ver=0`}
									target="_blank"
									on:mouseenter={() => (potle.focus1 = true)}
									on:mouseleave={() => (potle.focus1 = false)}
								>
									<img
										src={`${data.api}/limg?id=${potle.id}&ver=0`}
										alt=""
										class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
										class:blur={potle.focus1}
									/>
									{#if potle.focus1}
										<span
											class="icon-[mdi--gesture-touch-hold] absolute left-1/4 top-1/2 h-24 w-24 -translate-x-1/4 -translate-y-1/2 transform shadow-2xl"
										></span>
									{/if}
								</a>
								<a
									href={`${data.api}/limg?id=${potle.id}&ver=1`}
									target="_blank"
									on:mouseenter={() => (potle.focus2 = true)}
									on:mouseleave={() => (potle.focus2 = false)}
								>
									<img
										src={`${data.api}/limg?id=${potle.id}&ver=1`}
										alt=""
										class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
										class:blur={potle.focus2}
									/>
									{#if potle.focus2}
										<span
											class="icon-[mdi--gesture-touch-hold] absolute left-3/4 top-1/2 h-24 w-24 -translate-x-3/4 -translate-y-1/2 transform shadow-2xl"
										></span>
									{/if}
								</a>
							</div>
						{:else}
							<a
								href={`${data.api}/img?id=${potle.id}`}
								target="_blank"
								on:mouseenter={() => (potle.focus = true)}
								on:mouseleave={() => (potle.focus = false)}
							>
								<img
									src={`${data.api}/img?id=${potle.id}`}
									alt=""
									class="max-h-xl m-auto max-w-xl py-2 drop-shadow-xl"
									class:blur={potle.focus}
								/>
								{#if potle.focus}
									<span
										class="icon-[mdi--gesture-touch-hold] absolute left-1/2 top-1/2 h-24 w-24 -translate-x-1/2 -translate-y-1/2 transform shadow-2xl"
									></span>
								{/if}
							</a>
						{/if}

						{#if potle.admin}
							<h1 class="drop-shadow-xl">Kezelte: {potle.admin}</h1>
						{/if}
					</div>
				{/each}
			{:else}
				<h2>Az elmúlt 2 hétben nem töltöttél fel semmit!</h2>
			{/if}
		</div>
	</div>
	{#if multipage}
		<div class="mb-3 flex items-center justify-center gap-4">
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
