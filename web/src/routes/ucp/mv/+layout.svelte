<script lang="ts">
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

<svelte:head>
	<meta content="SCKK Weboldal" property="og:title" />
	<meta content="https://sckk.hu/ucp/mv" property="og:url" />
	<meta content="SCKK Műszakvezetői felület" property="og:description" />
	<meta content="https://sckk.hu/favicon.png" property="og:image" />
	<meta content="#fece01" data-react-helmet="true" name="theme-color" />
</svelte:head>

<nav class="grid grid-cols-2 items-center justify-between bg-emerald-700 text-white lg:flex">
	<div class="ml-[10vw] flex items-center gap-2">
		<h1 class="hidden text-3xl font-bold drop-shadow-xl xl:block">Műszakvezetés</h1>
		<h1 class="text-3xl font-bold drop-shadow-xl xl:hidden">Műszakv.</h1>
	</div>
	<button
		class="mr-[10vw] cursor-pointer self-center justify-self-end text-3xl font-semibold transition-all duration-200 hover:text-emerald-500 lg:hidden"
		on:click={tognav}
	>
		<MaterialSymbolsMenu />
	</button>
	<div
		bind:this={nav}
		class="child:px-2 child:rounded-lg child:drop-shadow-xl col-span-2 hidden flex-col items-center justify-center text-center text-xl md:flex-row lg:z-auto lg:col-span-1 lg:mr-[10vw] lg:!flex"
	>
		<a href="/ucp/mv" class="transition-all duration-200 hover:bg-emerald-600">Főoldal</a>
		<a href="/ucp/mv/stat/current" class="transition-all duration-200 hover:bg-emerald-600"
			>Jelenlegi hét</a
		>
		<a href="/ucp/mv/stat/previous" class="transition-all duration-200 hover:bg-emerald-600"
			>Előző hét</a
		>
		<a href="/ucp/mv/call" class="transition-all duration-200 hover:bg-emerald-600">Hívásszámláló</a
		>
		<a href="/ucp/mv/potlekok" class="transition-all duration-200 hover:bg-emerald-600">Pótlékok</a>
		<a href="/ucp/mv/leintesek" class="transition-all duration-200 hover:bg-emerald-600"
			>Leintések</a
		>
		<a href="/ucp/mv/szamlak" class="transition-all duration-200 hover:bg-emerald-600"
			>Szereltetési számlák</a
		>
	</div>
</nav>
<slot />
