<script lang="ts">
	export let data;
	import MaterialSymbolsMenu from '~icons/material-symbols/menu';
	import { Ripple, initTWE } from 'tw-elements';

	initTWE({ Ripple });

	// Get the button
	const mybutton = document.getElementById('btn-back-to-top');

	// When the user scrolls down 20px from the top of the document, show the button

	const scrollFunction = () => {
		if (document.body.scrollTop > 20 || document.documentElement.scrollTop > 20) {
			mybutton.classList.remove('hidden');
		} else {
			mybutton.classList.add('hidden');
		}
	};
	const backToTop = () => {
		window.scrollTo({ top: 0, behavior: 'smooth' });
	};

	// When the user clicks on the button, scroll to the top of the document
	mybutton.addEventListener('click', backToTop);

	window.addEventListener('scroll', scrollFunction);s
</script>

<div class="relative z-20 border-b bg-white dark:bg-slate-900 dark:text-white">
	<div class="px-6 md:px-12 lg:container lg:mx-auto lg:px-6 lg:py-4">
		<div class="flex items-center justify-between">
			<div class="relative z-20">
				<img
					src="/favicon.png"
					class="pointer-events-none drop-shadow-xl ml-5"
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

			<div class="flex items-center justify-end border-l lg:border-l-0">
				<input type="checkbox" name="hamburger" id="hamburger" class="peer opacity-0" hidden />
				<label
					for="hamburger"
					class="peer-checked:hamburger block relative z-20 p-6 -mr-6 cursor-pointer lg:hidden"
				>
					<div
						aria-hidden="true"
						class="m-auto h-0.5 w-6 rounded bg-sky-900 transition duration-300"
					></div>
					<div
						aria-hidden="true"
						class="m-auto mt-2 h-0.5 w-6 rounded bg-sky-900 transition duration-300"
					></div>
				</label>

				<div
					class="peer-checked:translate-x-0 fixed inset-0 w-[calc(100%-4.5rem)] translate-x-[-100%] bg-white dark:bg-slate-900 border-r shadow-xl transition duration-300 lg:border-r-0 lg:w-auto lg:static lg:shadow-none lg:translate-x-0"
				>
					<div class="flex flex-col h-full justify-between lg:items-center lg:flex-row">
						<ul
							class="px-6 pt-32 text-gray-700 space-y-8 md:px-12 lg:space-y-0 lg:flex lg:space-x-12 lg:pt-0"
						>
							<li>
								<a
									href="/user"
									class="group relative before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:bg-red-800"
								>
									<span class="relative text-black dark:text-white">Kezdőlap</span>
								</a>
							</li>
							<li>
								<a
									href="/user/help"
									class="group relative before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-red-800 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
								>
									<span class="relative text-black dark:text-white">Segédlet</span>
								</a>
							</li>
							<li>
								<a
									href="/user/potlekok"
									class="group relative before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-red-800 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
								>
									<span class="relative text-black dark:text-white">Pótlékok</span>
								</a>
							</li>
							<li>
								<a
									href="/user/leintesek"
									class="group relative before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-red-800 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
								>
									<span class="relative text-black dark:text-white"
										>Leintések {#if data.layout.am}/ Bejelentések{/if}</span
									>
								</a>
							</li>
							<li>
								<a
									href="/user/szamlak"
									class="group relative before:absolute before:inset-x-0 before:bottom-0 before:h-2 before:origin-right before:scale-x-0 before:bg-red-800 before:transition before:duration-200 hover:before:origin-left hover:before:scale-x-100"
								>
									<span class="relative text-black dark:text-white">Szereltetési számlák</span>
								</a>
							</li>
						</ul>

						<div
							class="border-t py-8 px-6 md:px-12 md:py-16 lg:border-t-0 lg:border-l lg:py-0 lg:pr-0 lg:pl-6"
						>
							{#if data.layout?.admin}
								<a
									href="/user/admin"
									class="block px-6 py-3 rounded-full bg-gradient-to-r from-sky-600 to-cyan-400 text-center text-black font-bold"
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
	<h2 class="text-center bg-blue-600 text-white py-1 text-xl">
		Nem vagy biztos valamiben? Nézd meg a <a href="/user/help" class="text-yellow-300 font-bold"
			>segédletet</a
		>!
	</h2>
</footer>

<slot />
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
