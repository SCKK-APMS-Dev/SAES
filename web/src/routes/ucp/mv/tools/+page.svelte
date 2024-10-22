<script lang="ts">
	export let data;
</script>

<div class="flex items-center text-center text-white">
	<div class="m-auto">
		<h1 class="mb-5 mt-5 text-4xl font-bold">Eszközök</h1>
		<div class="flex w-screen items-center justify-center gap-5">
			{#if !data.layout.am}
				<a
					href="/ucp/mv/tools/app"
					class="border-taxi hover:bg-taxi w-1/2 rounded-3xl border-4 p-5 transition-colors duration-500 lg:w-1/4"
				>
					<h1 class="text-2xl font-bold">APP Feldolgozó</h1>
					<h2>
						Az SCKK Application kódját felhasználva meg lehet nézni, ki, mikor vitt hívást az adott
						napon, illetve a műszakok hogyan teljesítettek.
					</h2>
				</a>
			{/if}
			<a
				href="/ucp/mv/tools/call"
				class="col-span-2 w-1/2 rounded-3xl border-4 border-blue-600 p-5 transition-colors duration-500 hover:bg-blue-600 lg:w-1/4"
			>
				<h1 class="text-2xl font-bold">Hívásszámláló</h1>
				<h2>
					Akár több .log fájlból meg lehet nézni, ki hány hívást vitt el, akár több napra nézve!
				</h2>
			</a>
		</div>
	</div>
</div>
