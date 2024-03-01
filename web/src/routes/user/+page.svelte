<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-2 child:p-2 md:child:p-16">
	{#if !data.error}
		<div class="bg-gray-400">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Üdv {data.layout?.doksi.name}!</h1>
			<h2 class="text-xl md:text-2xl">
				Hívásaid (app+leintés): {data.page?.calls.app}+{data.page?.calls.leint}={data.page?.calls
					.all}
			</h2>
			<h2 class="text-xl md:text-2xl">
				Pótlékaid (délelőtti+éjszakai): {data.page?.potlek.délelőtti.all}+{data.page?.potlek
					.éjszakai.all}
			</h2>
		</div>
		<div class="bg-green-400">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Leintéseid</h1>
			<h2 class="text-xl md:text-2xl">
				Leintéseid: {data.page?.calls.leint}
			</h2>
			<h2 class="text-xl md:text-2xl">Elfogadott: 0</h2>
		</div>
		<div class="bg-yellow-400">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Délelőtti pótlék</h1>
			<h2 class="text-xl md:text-2xl">
				Összesen: {data.page?.potlek.délelőtti.all}
			</h2>
			<h2 class="text-xl md:text-2xl">Elfogadott: {data.page?.potlek.délelőtti.elfogadott}</h2>
		</div>
		<div class="bg-blue-900">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Éjszakai pótlék</h1>
			<h2 class="text-xl md:text-2xl">
				Összesen: {data.page?.potlek.éjszakai.all}
			</h2>
			<h2 class="text-xl md:text-2xl">Elfogadott: {data.page?.potlek.éjszakai.elfogadott}</h2>
		</div>
	{/if}
</div>
{#if data.layout?.doksi.rang === 'admin'}
	<div class="bg-emerald-400 w-screen grid place-content-center text-white font-bold p-2">
		<h1 class="text-3xl">Műszakvezetés</h1>
	</div>
	<div class="text-center text-white grid grid-cols-2 child:p-2 md:child:p-16">
		<div class="bg-orange-400">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Mostani hét</h1>
			<h2 class="text-xl md:text-2xl">
				Leintéseid: {data.page?.calls.leint}
			</h2>
			<h2 class="text-xl md:text-2xl">Elfogadott: 0</h2>
		</div>
		<div class="bg-cyan-400">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Előző hét</h1>
			<h2 class="text-xl md:text-2xl">
				Leintéseid: {data.page?.calls.leint}
			</h2>
			<h2 class="text-xl md:text-2xl">Elfogadott: 0</h2>
		</div>
	</div>
{/if}
{#if data.error}
	<h2 class="text-center text-white font-bold text-3xl">Sikertelen API lekérdezés</h2>
{/if}
