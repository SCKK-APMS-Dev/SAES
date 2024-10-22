<script lang="ts">
	export let data;
	const supportCountries = ['HU', 'SK', 'RO'];
	let color = supportCountries.includes(data.country as string) ? data.country : 'HU';
</script>

<div class="mt-5 text-center">
	<h1 class="mb-2 text-3xl font-bold text-white">Műszakvezetési statisztika</h1>
	<div class="child:p-2 md:child:p-4 ml-5 mr-5 grid grid-cols-3 gap-5 text-center text-white">
		<div
			class="rounded-lg"
			class:bg-red-700={color === 'HU'}
			class:bg-white={color === 'SK'}
			class:text-black={color === 'SK'}
			class:bg-blue-900={color === 'RO'}
		>
			<h1 class="text-2xl font-bold">Új pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.feltoltve}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-red-700={color === 'HU'}
			class:bg-white={color === 'SK'}
			class:text-black={color === 'SK'}
			class:bg-yellow-400={color === 'RO'}
		>
			<h1 class="text-2xl font-bold">Új leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.feltoltve}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-red-700={color === 'HU' || color === 'RO'}
			class:bg-white={color === 'SK'}
			class:text-black={color === 'SK'}
		>
			<h1 class="text-2xl font-bold">Új számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.feltoltve}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-white={color === 'HU'}
			class:text-black={color === 'HU'}
			class:bg-blue-700={color === 'SK'}
			class:bg-blue-900={color === 'RO'}
		>
			<h1 class="text-2xl font-bold">Elfogadott pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.elfogadva}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-white={color === 'HU'}
			class:text-black={color === 'HU'}
			class:bg-blue-700={color === 'SK'}
			class:bg-yellow-400={color === 'RO'}
		>
			<h1 class="text-2xl font-bold">Elfogadott leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.elfogadva}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-white={color === 'HU'}
			class:text-black={color === 'HU'}
			class:bg-blue-700={color === 'SK'}
			class:bg-red-700={color === 'RO'}
		>
			<h1 class="text-2xl font-bold">Elfogadott számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.elfogadva}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-blue-900={color === 'RO'}
			class:bg-red-600={color === 'SK'}
			class:bg-green-600={color === 'HU'}
		>
			<h1 class="text-2xl font-bold">Elutasított pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.elutasitva}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-red-600={color === 'SK'}
			class:bg-yellow-400={color === 'RO'}
			class:bg-green-600={color === 'HU'}
		>
			<h1 class="text-2xl font-bold">Elutasított leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.elutasitva}</h2>
		</div>
		<div
			class="rounded-lg"
			class:bg-red-600={color === 'SK'}
			class:bg-red-700={color === 'RO'}
			class:bg-green-600={color === 'HU'}
		>
			<h1 class="text-2xl font-bold">Elutasított számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.elutasitva}</h2>
		</div>
	</div>
	<div class="mt-4 text-white">
		<h1 class="text-2xl">Mivel nincs ilyen menő cuccom erre így ide kiírom a változásokat:</h1>
		<h2>- Weboldal némítása ideiglenes kikerült megint, addig a zene se szól.</h2>
		<h2>- Hívásszámláló átkerült az eszközökhöz.</h2>
		<h1 class="text-xl font-bold">Érdemes a "Műszakvezetés" címtől jobbra nézni! 😉</h1>
	</div>
</div>
