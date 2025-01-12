<script lang="ts">
	import { page } from '$app/state';
	import { Tooltip } from 'flowbite-svelte';
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
	let nav: HTMLDivElement = $state()!;
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

<nav class="grid grid-cols-2 items-center justify-between bg-emerald-700 text-white lg:flex">
	<div class="ml-2 flex flex-shrink items-center gap-2 xl:ml-[10vw]">
		<h1 class="hidden text-3xl font-bold drop-shadow-xl md:block">Műszakvezetés</h1>
		<h1 class="text-3xl font-bold drop-shadow-xl md:hidden">Műszakv.</h1>
		<a
			href="/ucp/sm/tools"
			aria-label="tools"
			class="icon-[tabler--tool] hover:text-taxi min-h-6 min-w-6 transition-colors duration-500"
		></a>
		<Tooltip placement="bottom">Eszközök megnyitása</Tooltip>
	</div>
	<button
		aria-label="mv-menu"
		class="mr-[10vw] flex cursor-pointer self-center justify-self-end text-3xl font-semibold transition-all duration-200 hover:text-emerald-500 lg:hidden"
		onclick={tognav}
	>
		<span class="icon-[material-symbols--menu]"></span>
	</button>
	<div
		bind:this={nav}
		class="child:px-2 child:rounded-lg child:drop-shadow-xl col-span-2 hidden flex-col items-center justify-center text-center text-xl md:flex-row lg:z-auto lg:col-span-1 lg:!flex xl:mr-[10vw]"
	>
		<a href="/ucp/sm" class="transition-all duration-200 hover:bg-emerald-600">Főoldal</a>
		<a href="/ucp/sm/stat/current" class="transition-all duration-200 hover:bg-emerald-600"
			>Jelenlegi hét</a
		>
		<a href="/ucp/sm/stat/previous" class="transition-all duration-200 hover:bg-emerald-600"
			>Előző hét</a
		>
		<a href="/ucp/sm/potlekok" class="transition-all duration-200 hover:bg-emerald-600">Pótlékok</a>
		<a href="/ucp/sm/leintesek" class="transition-all duration-200 hover:bg-emerald-600"
			>Leintések</a
		>
		<a href="/ucp/sm/szamlak" class="transition-all duration-200 hover:bg-emerald-600"
			>Szereltetési számlák</a
		>
	</div>
</nav>
{@render children?.()}
