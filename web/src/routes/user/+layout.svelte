<script lang="ts">
	export let data;
	import Error from '$lib/error.svelte';
	import { onMount } from 'svelte';
	onMount(() => {
		let audioFile = new Audio('/taxi.wav');
		if (data.layout?.admin) {
			document.getElementById('mvbtn')?.addEventListener('mouseenter', () => {
				audioFile.currentTime = 0;
				audioFile.play();
			});
			document.getElementById('mvbtn')?.addEventListener('mouseleave', () => {
				audioFile.pause();
			});
		}
	});
</script>

<Error {data}>
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
						SCKK {#if data.layout.am}
							Autómentés
						{/if}
					</h1>
				</div>

				<div class="flex items-center justify-end border-l lg:border-l-0">
					<input type="checkbox" name="hamburger" id="hamburger" class="peer opacity-0" hidden />
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
								class="space-y-8 px-6 pt-32 text-gray-700 md:px-12 lg:flex lg:space-x-12 lg:space-y-0 lg:pt-0"
							>
								<li>
									<a
										href="/user"
										class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
									>
										<span class="relative text-black dark:text-white">Kezdőlap</span>
									</a>
								</li>
								<li>
									<a
										href="/user/help"
										class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
									>
										<span class="relative text-black dark:text-white">Segédlet</span>
									</a>
								</li>
								<li>
									<a
										href="/user/items/potlek"
										class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
									>
										<span class="relative text-black dark:text-white">Pótlékok</span>
									</a>
								</li>
								<li>
									<a
										href="/user/items/leintes"
										class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
									>
										<span class="relative text-black dark:text-white"
											>Leintések {#if data.layout.am}/ Bejelentések{/if}</span
										>
									</a>
								</li>
								<li>
									<a
										href="/user/items/szamla"
										class="before:bg-taxi group relative before:absolute before:inset-x-0 before:-bottom-1.5 before:h-2 before:origin-right before:scale-x-0 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
									>
										<span class="relative text-black dark:text-white">Szereltetési számlák</span>
									</a>
								</li>
							</ul>

							<div
								class="border-t px-6 py-8 md:px-12 md:py-16 lg:border-l lg:border-t-0 lg:py-0 lg:pl-6 lg:pr-0"
							>
								{#if data.layout?.admin}
									<a
										href="/user/admin"
										id="mvbtn"
										class="from-taxi hover:bg-pos-100 bg-size-200 bg-pos-0 block rounded-full bg-gradient-to-r via-amber-600 to-red-500 px-6 py-3 text-center font-bold text-white drop-shadow-lg transition-all duration-500"
									>
										<span class="icon-[mingcute--music-line]" style="width: 1rem; height: 1rem;"
										></span>
										Műszakvezetés
										<span class="icon-[mingcute--music-line]" style="width: 1rem; height: 1rem;"
										></span>
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
			Nem vagy biztos valamiben? Nézd meg a <a href="/user/help" class="text-taxi font-bold"
				>segédletet</a
			>!
		</h2>
	</footer>

	<slot />
</Error>
<!--
	régi menü
<nav class="bg-gray-700 grid lg:flex grid-cols-2 text-white justify-between items-center py-2">
	<div class="flex items-center gap-2 ml-[10vw] align-middle content-center">
		<img
			src="/favicon.png"
			class="pointer-events-none drop-shadow-xl"
			width="40"
			height="40"
			alt="SCKK Logó"
		/>
		<h1 class="font-bold text-3xl drop-shadow-xl">
			SCKK {#if data.layout.am}
				Autómentés
			{/if}
		</h1>
	</div>
	<button
		class="cursor-pointer self-center justify-self-end text-3xl font-semibold hover:text-blue-500 duration-200 transition-all lg:hidden mr-[10vw]"
		on:click={tognav}
	>
		<MaterialSymbolsMenu />
	</button>
	<div
		bind:this={nav}
		class="z-10 text-xl hidden col-span-2 flex-col lg:flex child:px-2 child:rounded-lg justify-center items-center text-center lg:mr-[10vw] md:flex-row lg:z-auto child:drop-shadow-xl"
	>
		<a href="/user" class="hover:bg-slate-600 duration-200 transition-all">Kezdőlap</a>
		<a href="/user/help" class="hover:bg-slate-600 duration-200 transition-all">Segédlet</a>
		<a href="/user/potlekok" class="hover:bg-slate-600 duration-200 transition-all">Pótlékok</a>
		<a href="/user/leintesek" class="hover:bg-slate-600 duration-200 transition-all"
			>Leintések {#if data.layout.am}/ Bejelentések{/if}</a
		>
		<a href="/user/szamlak" class="hover:bg-slate-600 duration-200 transition-all"
			>Szereltetési számlák</a
		>
		{#if data.layout?.admin}
			<a href="/user/admin" class="hover:bg-slate-600 duration-200 transition-all">Műszakvezetés</a>
		{/if}
	</div>
</nav>

let nav: HTMLDivElement;
	const tognav = () => {
		if (nav.classList.contains('hidden')) {
			nav.classList.remove('hidden');
			nav.classList.add('flex');
		} else {
			nav.classList.add('hidden');
			nav.classList.remove('flex');
		}
	}; 
	
	
-->
