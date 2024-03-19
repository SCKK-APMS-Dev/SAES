<script lang="ts">
	export let data;
	import MaterialSymbolsMenu from '~icons/material-symbols/menu';
	import MaterialSymbolsLogout from '~icons/material-symbols/logout';
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
</script>

<h2 class="text-center bg-blue-600 text-white py-1 text-xl">
	Nem vagy biztos valamiben? Nézd meg a <a href="/user/help" class="text-yellow-400 font-bold"
		>segédletet</a
	>!
</h2>
<nav class="bg-gray-700 grid grid-cols-2 lg:flex text-white justify-between items-center py-2">
	<div class="flex items-center gap-2 ml-[10vw] align-middle content-center">
		<img
			src="/favicon.png"
			class="pointer-events-none drop-shadow-xl"
			width="40"
			height="40"
			alt="SCKK Logó"
		/>
		<a href="/info" class="font-bold text-3xl drop-shadow-xl">SCKK</a>
	</div>
	<button
		class="cursor-pointer self-center justify-self-end text-3xl font-semibold hover:text-blue-500 duration-200 transition-all lg:hidden mr-[10vw]"
		on:click={tognav}
	>
		<MaterialSymbolsMenu />
	</button>
	<div
		bind:this={nav}
		class="gap-2 text-xl col-span-2 lg:col-span-1 flex-col lg:!flex items-center hidden text-center lg:mr-[10vw] lg:flex-row lg:gap-5 xl:gap-10 2xl:gap-20 lg:z-auto child:drop-shadow-xl"
	>
		<a href="/user" class="hover:text-blue-500 duration-200 transition-all">Kezdőlap</a>
		<a href="/user/help" class="hover:text-blue-500 duration-200 transition-all">Segédlet</a>
		<a href="/user/potlekok" class="hover:text-blue-500 duration-200 transition-all">Pótlékok</a>
		<a href="/user/leintesek" class="hover:text-blue-500 duration-200 transition-all">Leintések</a>
		<a href="/user/szamlak" class="hover:text-blue-500 duration-200 transition-all"
			>Szereltetési számlák</a
		>
		{#if data.layout?.admin}
			<a href="/user/admin" class="hover:text-blue-500 duration-200 transition-all">Műszakvezetés</a
			>
		{/if}
		<div class="relative w-full group hidden xl:block">
			<button class="hover:text-blue-500 duration-200 transition-all peer"
				><MaterialSymbolsMenu /></button
			>
			<div
				class="absolute z-[99] top-[100%] left-[50%] translate-x-[-50%] rounded-md overflow-hidden shadow-lg min-w-[200px] w-max peer-focus:visible peer-focus:opacity-100 opacity-0 invisible duration-200 p-1 bg-gray-100 dark:bg-gray-800 border border-dimmed text-xs md:text-sm"
			>
				<a href="/user/leintesek" class="hover:text-blue-500 duration-200 transition-all"
					>Leintések</a
				>
			</div>
		</div>
	</div>
</nav>

<slot />
<h2 class="fixed bottom-0 bg-red-600 text-center w-screen font-bold text-white">
	Jelenleg az oldal még fejlesztés alatt áll, nem minden funkció elérhető. Ha hibát tapasztalsz <a
		target="_blank"
		href="https://github.com/SCKK-APMS-Dev/TaxiWeb/issues/new/choose"
		class="text-yellow-400">itt</a
	> jelentheted
</h2>
