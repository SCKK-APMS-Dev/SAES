<script lang="ts">
	import SvelteMarkdown from 'svelte-markdown';
	import { navigating, page } from '$app/stores';
	export let data;
	import Error from '$lib/error.svelte';
	import { Reeler_keys, Reeler_vals } from '$lib/public';
	import { onMount } from 'svelte';
	import { loading } from '$lib/loading.js';
	import { io } from 'socket.io-client';
	import { socket } from '$lib/socket.js';
	import ViewTransition from '$lib/navigation.svelte';
	import Header from '$lib/ucp/header.svelte';
	let maintenance = false;
	let initial_socket = false;
	let announcement = false;
	let nosocket: boolean | string = 'Socket csatlakozás';
	let tip = !data.error && !data.noaccess ? (data.layout.am ? 'TOW' : 'TAXI') : 'SCKK';
	onMount(() => {
		if (!data.noaccess) {
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
				$loading = false;
			});
			$socket.on('disconnect', () => {
				nosocket = 'Socket csatlakozás sikertelen';
			});
			if (data.error) {
				$loading = false;
			} else {
				$loading = true;
			}
		}
	});
</script>

<svelte:head>
	{#if !maintenance || data.maintenance}
		{#if !$navigating}
			{#if $page.url.pathname.includes('mv')}
				<title>Műszakvezetői felület - {tip}</title>
				<meta content="https://sckk.hu/ucp/mv" property="og:url" />
				<meta content="Műszakvezetői felület" property="og:description" />
			{:else if Reeler_keys.some((el) => $page.url.pathname.includes(el))}
				{#if $page.url.pathname.endsWith('/upload')}
					<title
						>{Reeler_vals[Reeler_keys.indexOf($page.url.pathname.split('/')[2])][2]} feltöltés - {tip}</title
					>
					<meta
						content="https://sckk.hu/ucp/{$page.url.pathname.split('/')[2]}"
						property="og:url"
					/>
					<meta
						content="{Reeler_vals[
							Reeler_keys.indexOf($page.url.pathname.split('/')[2])
						][2]} feltöltés"
						property="og:description"
					/>
				{:else}
					<title
						>{Reeler_vals[Reeler_keys.indexOf($page.url.pathname.split('/')[2])][1]} megtekintése - {tip}</title
					>
					<meta
						content="{Reeler_vals[
							Reeler_keys.indexOf($page.url.pathname.split('/')[2])
						][1]} megtekintése"
						property="og:description"
					/>
					<meta
						content="https://sckk.hu/ucp/{$page.url.pathname.split('/')[2]}"
						property="og:url"
					/>
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
	{#if data.noaccess}
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
						<SvelteMarkdown source={announcement} />
					</div>
				</header>
			{/if}
		{/if}
		{#if initial_socket}
			<Header {tip} am={data.layout.am} isAdmin={data.layout.admin} />
			<ViewTransition />
			<main>
				<slot />
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
						<h1 class="text-2xl text-gray-300">Indoklás: {maintenance}</h1>
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
