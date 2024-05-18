<script lang="ts">
	export let data;
</script>

<div class="flex h-screen">
	<div class="m-auto text-center">
		{#if data?.noauth}
			<h1 class="text-3xl font-bold text-black dark:text-white">
				Az oldal használatához kérlek lépj be
			</h1>
			<button
				class="group relative m-auto flex h-12 items-center space-x-2 overflow-hidden rounded-full bg-gradient-to-r from-white to-red-600 px-6 hover:to-red-700"
			>
				<a href="user">
					<span class="relative text-xl font-bold text-black">Belépés Discordal</span>
				</a>
				<div class="flex translate-x-3 items-center -space-x-3">
					<div
						class="h-[1.6px] w-2.5 origin-left scale-x-0 rounded bg-white transition duration-300 group-hover:scale-x-100"
					></div>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						class="h-5 w-5 -translate-x-2 stroke-white transition duration-300 group-hover:translate-x-0"
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
