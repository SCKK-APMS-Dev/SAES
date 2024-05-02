<script lang="ts">
	import { onMount } from 'svelte';

	export let data;
	onMount(() => {});
</script>

<div class="h-screen flex">
	<div class="m-auto text-center">
		{#if data?.noauth}
			<h1 class="text-black dark:text-white text-3xl font-bold">
				Az oldal használatához kérlek lépj be
			</h1>
			<button
				class="relative group overflow-hidden px-6 h-12 rounded-full flex space-x-2 items-center bg-gradient-to-r from-white to-red-600 hover:to-red-700 m-auto"
			>
				<a href="user">
					<span class="relative text-sm text-black font-bold text-xl">Belépés Discordal</span>
				</a>
				<div class="flex items-center -space-x-3 translate-x-3">
					<div
						class="w-2.5 h-[1.6px] rounded bg-white origin-left scale-x-0 transition duration-300 group-hover:scale-x-100"
					></div>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5 stroke-white -translate-x-2 transition duration-300 group-hover:translate-x-0"
						fill="none"
						viewBox="0 0 24 24"
						stroke="currentColor"
						stroke-width="2"
					>
						<path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7" />
					</svg>
				</div>
			</button>
		{/if}
	</div>
</div>

<!--
			RÉGI GOMB
			<a
						href="user"
						class="text-white bg-green-500 text-2xl font-bold px-3 rounded-xl mt-2 hover:bg-green-800 transition-all duration-200"
						>Belépés Discordal</a>
			-->
