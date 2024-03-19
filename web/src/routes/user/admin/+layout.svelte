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

<nav class="bg-emerald-700 grid grid-cols-2 lg:flex text-white justify-between items-center">
	<div class="flex items-center gap-2 ml-[10vw]">
		<h1 class="font-bold text-3xl drop-shadow-xl">Műszakvezetés</h1>
	</div>
	<button
		class="cursor-pointer self-center justify-self-end text-3xl font-semibold hover:text-emerald-500 duration-200 transition-all lg:hidden mr-[10vw]"
		on:click={tognav}
	>
		<MaterialSymbolsMenu />
	</button>
	<div
		bind:this={nav}
		class="gap-2 text-xl col-span-2 lg:col-span-1 flex-col lg:!flex items-center hidden text-center lg:mr-[10vw] lg:flex-row lg:gap-5 xl:gap-10 2xl:gap-20 lg:z-auto child:drop-shadow-xl"
	>
		<a href="/user/admin" class="hover:text-emerald-500 duration-200 transition-all"
			>Jelenlegi hét</a
		>
		<a href="/user/admin/prev" class="hover:text-emerald-500 duration-200 transition-all"
			>Előző hét</a
		>
		<a href="/user/admin/potlekok" class="hover:text-emerald-500 duration-200 transition-all"
			>Pótlékok</a
		>
		<a href="/user/admin/leintesek" class="hover:text-emerald-500 duration-200 transition-all"
			>Leintések</a
		>
		<a href="/user/admin/szamlak" class="hover:text-emerald-500 duration-200 transition-all"
			>Szereltetési számlák</a
		>
	</div>
</nav>
<slot />
