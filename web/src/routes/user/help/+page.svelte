<script lang="ts">
	export let data: {
		api?: string | undefined;
		layout: { am: boolean };
		error?: string | undefined;
	};
</script>

<div class="text-center text-black dark:text-white">
	<h1 class="text-3xl font-bold">Segítség az oldal használatához</h1>
	<h2 class="mt-3 text-2xl font-bold">Alap információk</h2>
	<h2 class="mb-3 text-black dark:text-white">
		Ez a weboldal a {#if data.layout.am}
			vontatósok
		{:else}
			taxi
		{/if} papírmunkáját könnyíti meg. Ide kell feltölteni a leintéseket, számlákat illetve a műszakon
		kívüli pótlékokat.
	</h2>
	<div class="text-left align-bottom">
		<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">1. Pótlékok</h1>
		<ul class="child:mx-[20%]">
			<li class="flex items-center gap-2">
				<h2 class="text-left text-xl font-bold">1.1.</h2>
				<h2 class="text-xl">
					{#if data.layout.am}
						A fotónak tartalmaznia kell a kiadott számlát és a teljes játékot.
					{:else}
						A fuvar igazolásáról kell egy teljes képernyős képet készíteni. (4-es pont)
					{/if}
				</h2>
			</li>
			<li class="flex items-center gap-2">
				<h2 class="text-left text-xl font-bold">1.2.</h2>
				<h2 class="text-xl">
					Pótlékok a következő időben érvényesek: {#if data.layout.am}
						8-16 óra (délelőtti), 20-4 (éjszakai)
					{:else}
						8-14 óra (délelőtti), 23-4 (éjszakai)
					{/if}
				</h2>
			</li>
		</ul>
		<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">
			2. Leintések {#if data.layout.am}/ Bejelentések
			{/if}
		</h1>
		<ul class="child:mx-[20%] child:text-left">
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">2.1.</h2>
				<h2 class="text-xl">Egy érvényes leintés 2 képből áll</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">2.2.</h2>
				<h2 class="text-xl">
					{#if data.layout.am}
						Az első képen a kiadott számlának kell látszódnia
					{:else}
						Az első képen a 10-12 rádiózásának, vagy hívás átvételnél a rádió rp-nek kell látszódnia
					{/if}
				</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">2.3.</h2>
				<h2 class="text-xl">
					{#if data.layout.am}
						A második képen a fizetésről kell képet készíteni. (chat)
					{:else}
						A második képen a hívást kell igazolni (4-es pont).
					{/if}
				</h2>
			</li>
		</ul>
		<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">3. Számlák</h1>
		<ul class="child:mx-[20%] child:text-left">
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">3.1.</h2>
				<h2 class="text-xl">
					Csak is saját (te vagy az ügyfél a számlán) szereléseket tölthetsz fel, autó függetlenül
				</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">3.2.</h2>
				<h2 class="text-xl">Csak az APMS általi szerelésekért tudjuk visszaadni a pénzt</h2>
			</li>
		</ul>
		<h1 class="ml-[20%] mt-5 text-left text-3xl font-bold">4. Hívások igazolása</h1>
		<ul class="child:mx-[20%] child:text-left">
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">4.1.</h2>
				<h2 class="text-xl">Hívásokat kétféleképpen lehet igazolni, más módot nem fogadunk el!</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">4.2.</h2>
				<h2 class="text-xl">Lehetséges igazolni a fuvar végén kinyomtatott nyugta fotózásával.</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">4.3.</h2>
				<h2 class="text-xl">
					Lehetséges igazolni a fuvar kifizetésénél felbukkanó kék dobozzal, vagy a chaten látható
					azonos "kifizette az utazást" szöveget tartalmaztó chatsorral.
				</h2>
			</li>
		</ul>
	</div>
</div>
