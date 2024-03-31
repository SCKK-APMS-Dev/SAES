<script lang="ts">
	export let data;
	import MaterialSymbolsMenu from '~icons/material-symbols/menu';
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
	Nem vagy biztos valamiben? Nézd meg a <a href="/user/help" class="text-yellow-300 font-bold"
		>segédletet</a
	>!
</h2>
<nav class="bg-gray-700 grid lg:flex grid-cols-2 text-white justify-between items-center py-2">
	<div class="flex items-center gap-2 ml-[10vw] align-middle content-center">
		<img
			src="/favicon.png"
			class="pointer-events-none drop-shadow-xl"
			width="40"
			height="40"
			alt="SCKK Logó"
		/>
		<h1 class="font-bold text-3xl drop-shadow-xl">SCKK</h1>
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
		<a href="/user/leintesek" class="hover:bg-slate-600 duration-200 transition-all">Leintések</a>
		<a href="/user/szamlak" class="hover:bg-slate-600 duration-200 transition-all"
			>Szereltetési számlák</a
		>
		{#if data.layout?.admin}
			<a href="/user/admin" class="hover:bg-slate-600 duration-200 transition-all">Műszakvezetés</a>
		{/if}
	</div>
</nav>

<slot />
