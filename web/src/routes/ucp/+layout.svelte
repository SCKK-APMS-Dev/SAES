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
	let maintenance = false;
	let initial_socket = false;
	let announcement = false;
	let nosocket: boolean | string = 'Socket csatlakozás';
	let audio: HTMLAudioElement | null;
	let played = false;
	let tip = data.layout.am ? 'TOW' : 'TAXI';
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
			if (!data.layout.am) {
				audio = new Audio('/taxi.wav');
				audio.volume = 0.1;
			}
			if (data.error) {
				$loading = false;
			} else {
				$loading = true;
			}
		}
	});

	const playmusic = () => {
		if (!played) {
			audio?.play();
		}
	};
</script>

<svelte:head>
	{#if !maintenance || data.maintenance}
		{#if !$navigating}
			{#if $page.url.pathname.includes('mv')}
				<title>Műszakvezetői felület - SCKK</title>
			{:else if Reeler_keys.some((el) => $page.url.pathname.includes(el))}
				{#if $page.url.pathname.endsWith('/upload')}
					<title
						>{Reeler_vals[Reeler_keys.indexOf($page.url.pathname.split('/')[2])][2]} feltöltés - {tip}</title
					>
				{:else}
					<title
						>{Reeler_vals[Reeler_keys.indexOf($page.url.pathname.split('/')[2])][1]} megtekintése - {tip}</title
					>
				{/if}
			{:else}
				<title>Felhasználói felület - {tip}</title>
			{/if}
		{/if}
	{:else}
		<title>Karbantartás - {tip}</title>
	{/if}
</svelte:head>
<Error {data}>
	{#if data.noaccess}
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
	{:else if !maintenance || data.maintenance}
		{#if nosocket}
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
		{:else}
			{#if maintenance}
				<div
					class="flex items-center justify-center bg-rose-900 text-center text-2xl font-bold uppercase text-white"
				>
					<h1 class="drop-shadow-lg">
						Karbantartás mód aktív {#if typeof maintenance === 'string'}
							- {maintenance}
						{/if}
					</h1>
				</div>
			{/if}
			{#if announcement}
				<div class="flex items-center justify-center bg-blue-500 text-center text-2xl text-white">
					<SvelteMarkdown source={announcement} />
				</div>
			{/if}
		{/if}
		{#if initial_socket}
			<div class="relative z-20 border-b bg-white dark:bg-gray-700 dark:text-white">
				<div class="px-6 lg:container md:px-12 lg:mx-auto lg:px-6 lg:py-4">
					<div class="flex items-center justify-between">
						<div class="relative z-20 flex items-center gap-3">
							<img
								src="/favicon.png"
								class="pointer-events-none ml-5 drop-shadow-xl"
								width="40"
								height="40"
								alt="SCKK Logó"
							/>
							<h1 class="text-3xl font-bold drop-shadow-xl">
								{tip}
							</h1>
						</div>

						<div class="flex items-center justify-end border-l lg:border-l-0">
							<input
								type="checkbox"
								name="hamburger"
								id="hamburger"
								class="peer opacity-0"
								hidden
							/>
							<label
								for="hamburger"
								class="peer-checked:hamburger relative z-20 -mr-6 block cursor-pointer p-6 lg:hidden"
							>
								<div
									aria-hidden="true"
									class="m-auto h-0.5 w-6 rounded bg-white transition duration-300"
								></div>
								<div
									aria-hidden="true"
									class="m-auto mt-2 h-0.5 w-6 rounded bg-white transition duration-300"
								></div>
							</label>

							<div
								class="fixed inset-0 w-[calc(100%-4.5rem)] translate-x-[-100%] border-r bg-white shadow-xl transition duration-300 peer-checked:translate-x-0 lg:static lg:w-auto lg:translate-x-0 lg:border-r-0 lg:shadow-none dark:bg-gray-700"
							>
								<div class="flex h-full flex-col justify-between lg:flex-row lg:items-center">
									<ul
										class="items-center space-y-8 px-6 pt-32 text-center text-gray-700 md:px-12 lg:flex lg:space-x-12 lg:space-y-0 lg:pt-0"
									>
										<li>
											<a
												href="/ucp"
												class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
											>
												<span class="relative text-black dark:text-white">Kezdőlap</span>
											</a>
										</li>
										<li>
											<a
												href="/ucp/szabalyzat"
												class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
											>
												<span class="relative text-black dark:text-white">Szabályzat</span>
											</a>
										</li>
										<li>
											<a
												href="/ucp/potlekok"
												class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
											>
												<span class="relative text-black dark:text-white">Pótlékok</span>
											</a>
										</li>
										<li>
											<a
												href="/ucp/leintesek"
												class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
											>
												<span class="relative text-black dark:text-white"
													>Leintések {#if data.layout.am}/ Bejelentések{/if}</span
												>
											</a>
										</li>
										<li>
											<a
												href="/ucp/szamlak"
												class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
											>
												<span class="relative text-black dark:text-white">Szereltetési számlák</span
												>
											</a>
										</li>
									</ul>

									<div
										class="border-t px-6 py-8 md:px-12 md:py-16 lg:border-l lg:border-t-0 lg:py-0 lg:pl-6 lg:pr-0"
									>
										{#if data.layout?.admin}
											<a
												href="/ucp/mv"
												on:pointerover={playmusic}
												class="from-taxi hover:bg-pos-100 bg-size-200 bg-pos-0 block rounded-full bg-gradient-to-r via-amber-600 to-red-500 px-6 py-3 text-center font-bold text-white drop-shadow-lg transition-all duration-500"
											>
												Műszakvezetés
											</a>
										{/if}
									</div>
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>

			<footer>
				<h2 class="bg-rose-600 py-1 text-center text-xl text-white">
					Nem vagy biztos valamiben? Nézd meg a <a
						href="/ucp/szabalyzat"
						class="text-taxi font-bold">szabályzatot</a
					>!
				</h2>
			</footer>

			<slot />
		{/if}
	{:else}
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
	{/if}
</Error>
