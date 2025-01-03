<script lang="ts">
	import { marked } from 'marked';
	import { navigating, page } from '$app/state';
	import Error from '$lib/error.svelte';
	import { Reeler_keys, Reeler_vals } from '$lib/ucp/public.js';
	import { onMount } from 'svelte';
	import { loading } from '$lib/loading.svelte';
	import { io } from 'socket.io-client';
	import { socket } from '$lib/socket.js';
	import ViewTransition from '$lib/navigation.svelte';
	import Header from '$lib/ucp/header.svelte';
	let { data, children } = $props();
	let maintenance = $state(false);
	let initial_socket = $state(false);
	let announcement = $state(false);
	let nosocket: boolean | string = $state('Socket csatlakozás');
	let tip =
		!data.error && !data.noaccess && !data.noauth ? (data.layout.am ? 'TOW' : 'TAXI') : 'SCKK';
	onMount(() => {
		if (!data.noaccess && !data.noauth) {
			$socket = io(data.api as string, {
				auth: {
					auth_token: data.auth
				}
			});
			$socket.on('maintenance', (data) => {
				if (data !== '') {
					if (data === 'nincs') {
						maintenance = true;
					} else {
						maintenance = data;
					}
				}
			});
			$socket.on('announcement', (data) => {
				if (data !== '') {
					announcement = data;
				}
			});
			$socket.on('doneload', () => {
				console.log('Socket csatlakozva');
				nosocket = false;
				initial_socket = true;
				loading.value = false;
			});
			$socket.on('disconnect', () => {
				nosocket = 'Socket csatlakozás sikertelen';
			});
			if (data.error) {
				loading.value = false;
			} else {
				loading.value = true;
			}
		}
	});
</script>

<svelte:head>
	{#if !maintenance || data.maintenance || data.noauth}
		{#if !navigating.type}
			{#if page.url.pathname.includes('mv')}
				<title>Műszakvezetői felület - {tip}</title>
				<meta content="https://sckk.hu/ucp/mv" property="og:url" />
				<meta content="Műszakvezetői felület" property="og:description" />
			{:else if Reeler_keys.some((el) => page.url.pathname.includes(el))}
				{#if page.url.pathname.endsWith('/upload')}
					<title
						>{Reeler_vals[Reeler_keys.indexOf(page.url.pathname.split('/')[2])][2]} feltöltés - {tip}</title
					>
					<meta content="https://sckk.hu/ucp/{page.url.pathname.split('/')[2]}" property="og:url" />
					<meta
						content="{Reeler_vals[
							Reeler_keys.indexOf(page.url.pathname.split('/')[2])
						][2]} feltöltés"
						property="og:description"
					/>
				{:else}
					<title
						>{Reeler_vals[Reeler_keys.indexOf(page.url.pathname.split('/')[2])][1]} megtekintése - {tip}</title
					>
					<meta
						content="{Reeler_vals[
							Reeler_keys.indexOf(page.url.pathname.split('/')[2])
						][1]} megtekintése"
						property="og:description"
					/>
					<meta content="https://sckk.hu/ucp/{page.url.pathname.split('/')[2]}" property="og:url" />
				{/if}
			{:else}
				<title>Felhasználói felület - {tip}</title>
				<meta content="https://sckk.hu/ucp" property="og:url" />
				<meta content="Felhasználói felület" property="og:description" />
			{/if}
		{/if}
	{:else}
		<title>Karbantartás - {tip}</title>
	{/if}
	<meta content="SCKK Weboldal" property="og:title" />

	<meta content="https://sckk.hu/favicon.png" property="og:image" />
	<meta content="#fece01" data-react-helmet="true" name="theme-color" />
</svelte:head>
<Error {data}>
	{#if data.noauth}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="animate-pulse text-3xl font-bold text-black dark:text-white">
						Az oldal használatához kérlek lépj be
					</h1>
					<button
						class="from-taxi hover:bg-pos-100 bg-size-200 bg-pos-0 group relative m-auto mt-3 flex h-12 animate-bounce items-center space-x-2 overflow-hidden rounded-full bg-gradient-to-r via-rose-500 to-red-600 px-6 transition-all duration-500"
					>
						<a href={`${data.apiUrl}/auth?path=${page.url.pathname}`}>
							<span
								class="relative text-xl font-bold text-white transition-colors duration-300 group-hover:text-black"
								>Belépés Discordal</span
							>
						</a>
						<div class="flex translate-x-3 items-center -space-x-3">
							<div
								class="h-[1.6px] w-2.5 origin-left scale-x-0 rounded bg-white transition duration-300 group-hover:scale-x-100 group-hover:bg-black"
							></div>
							<svg
								xmlns="http://www.w3.org/2000/svg"
								class="h-5 w-5 -translate-x-2 stroke-white transition duration-300 group-hover:translate-x-0 group-hover:stroke-black"
								fill="none"
								viewBox="0 0 24 24"
								stroke="currentColor"
								stroke-width="2"
							>
								<path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
							</svg>
						</div>
					</button>
				</div>
			</div>
		</main>
	{:else if data.noaccess}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="text-3xl font-bold text-red-600">Nincs jogosultságod!</h1>
					<a
						href="/logout"
						class="hover:bg-pos-100 bg-size-200 bg-pos-0 mb-5 ml-5 mr-5 mt-5 block rounded-full bg-gradient-to-r from-red-500 via-amber-400 to-rose-600 px-2 py-1 text-center text-lg font-bold text-white drop-shadow-lg transition-all duration-500"
						>Kijelentkezés</a
					>
				</div>
			</div>
		</main>
	{:else if !maintenance || data.maintenance}
		{#if nosocket}
			<header>
				<div
					class="flex items-center justify-center bg-red-500 text-center text-2xl font-semibold uppercase text-white"
				>
					<h1>
						{#if nosocket !== true}
							{nosocket}
						{:else}
							Sikertelen socket csatlakozás
						{/if}
					</h1>
				</div>
			</header>
		{:else}
			{#if maintenance}
				<header>
					<div
						class="flex items-center justify-center bg-rose-900 text-center text-2xl font-bold uppercase text-white"
					>
						<h1 class="drop-shadow-lg">
							Karbantartás mód aktív {#if typeof maintenance === 'string'}
								- {maintenance}
							{/if}
						</h1>
					</div>
				</header>
			{/if}
			{#if announcement}
				<header>
					<div class="flex items-center justify-center bg-blue-500 text-center text-2xl text-white">
						{@html marked(announcement.toString())}
					</div>
				</header>
			{/if}
		{/if}
		{#if initial_socket}
			<Header {tip} am={data.layout.am} isAdmin={data.layout.admin} />
			<ViewTransition />
			<main>
				{@render children?.()}
			</main>
		{/if}
	{:else}
		<main>
			<div class="flex h-screen">
				<div class="m-auto text-center">
					<h1 class="text-5xl font-bold text-red-600">Karbantartás</h1>
					<h1 class="text-3xl text-gray-300">
						Jelenleg karbantartás zajlik, kérlek nézz vissza később!
					</h1>
					{#if typeof maintenance === 'string'}
						<h1 class="text-2xl text-gray-300">Indoklás: {@html marked(maintenance)}</h1>
					{/if}
					{#if data.layout?.admin}
						<a
							href="/ucp/keine"
							class="hover:bg-pos-100 bg-size-200 bg-pos-0 mb-5 ml-5 mr-5 mt-5 block rounded-full bg-gradient-to-r from-red-500 via-amber-400 to-rose-600 px-2 py-1 text-center text-lg font-bold text-white drop-shadow-lg transition-all duration-500"
							>Továbblépés (nyomj rá majd töltsd újra az oldalt)</a
						>
					{/if}
				</div>
			</div>
		</main>
	{/if}
</Error>
