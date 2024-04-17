<script lang="ts">
	export let data: {
		api?: string | undefined;
		layout: { am: boolean };
		error?: string | undefined;
	};
</script>

<div class="text-center text-white">
	<h1 class="text-3xl font-bold">Segítség az oldal használatához</h1>
	<h2 class="text-2xl font-bold mt-3">Alap információk</h2>
	<h2 class="text-gray-300 mb-3">
		Ez a weboldal a {#if data.layout.am}
			vontatósok
		{:else}
			taxi
		{/if} papírmunkáját könnyíti meg. Ide kell feltölteni a leintéseket, számlákat illetve a műszakon
		kívüli pótlékokat.
	</h2>
	<div class="align-bottom">
		<h1 class="text-3xl font-bold text-left ml-[20%] mt-5">1. Pótlékok</h1>
		<ul class="child:mx-[20%]">
			<li class="flex items-center gap-2">
				<h2 class="text-xl text-left font-bold">1.1.</h2>
				<h2 class="text-xl">
					{#if data.layout.am}
						A fotónak tartalmaznia kell a kiadott számlát és a teljes játékot
					{:else}
						A fotónak tartalmaznia kell a nyugtát és a teljes játékot
					{/if}
				</h2>
			</li>
			<li class="flex items-center gap-2">
				<h2 class="text-xl text-left font-bold">1.2.</h2>
				<h2 class="text-xl">
					Pótlékok a következő időben érvényesek: {#if data.layout.am}
						8-16 óra (délelőtti), 20-4 (éjszakai)
					{:else}
						8-15 óra (délelőtti), 22-4 (éjszakai)
					{/if}
				</h2>
			</li>
		</ul>
		<h1 class="text-3xl font-bold text-left ml-[20%] mt-5">
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
						A második képen a fizetésről kell képet készíteni (chat)
					{:else}
						A második képen a közép fent található "xy kifizette az utazást" szövegnek kell
						látszódnia vagy a kék dobozban, vagy a chaten
					{/if}
				</h2>
			</li>
		</ul>
		<h1 class="text-3xl font-bold text-left ml-[20%] mt-5">3. Számlák</h1>
		<ul class="child:mx-[20%] child:text-left">
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">3.1.</h2>
				<h2 class="text-xl">
					Csak is saját (te vagy az ügyfél a számlán) szereléseket tölthetsz fel, autó függetlenül
				</h2>
			</li>
			<li class="flex gap-2">
				<h2 class="text-xl font-bold">2.2.</h2>
				<h2 class="text-xl">Csak az APMS általi szerelésekért tudjuk visszaadni a pénzt</h2>
			</li>
		</ul>
	</div>
</div>
