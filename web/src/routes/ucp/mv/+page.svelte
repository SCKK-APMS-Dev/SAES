<script lang="ts">
	import { beforeNavigate } from '$app/navigation';
	import { socket } from '$lib/socket';
	export let data;
	let count = 0;
	$socket?.emit('JoinEvent', { event_name: 'socketppl' });
	$socket?.on('socketppl-update', (data) => {
		count = data;
	});
	beforeNavigate(() => {
		$socket?.emit('LeaveEvent', { event_name: 'socketppl' });
	});
</script>

<div class="mt-5 text-center">
	<h1 class="mb-2 text-3xl font-bold text-white">Műszakvezetési statisztika</h1>
	<h2 class="mb-2 mt-5 text-xl text-white">Weboldal használók száma: {count}</h2>
	<div class="child:p-2 md:child:p-4 ml-5 mr-5 grid grid-cols-3 gap-5 text-center text-white">
		<div class="rounded-lg bg-red-600">
			<h1 class="text-2xl font-bold">Új pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.feltoltve}</h2>
		</div>
		<div class="rounded-lg bg-red-600">
			<h1 class="text-2xl font-bold">Új leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.feltoltve}</h2>
		</div>
		<div class="rounded-lg bg-red-600">
			<h1 class="text-2xl font-bold">Új számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.feltoltve}</h2>
		</div>
		<div class="rounded-lg bg-white text-black">
			<h1 class="text-2xl font-bold">Elfogadott pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.elfogadva}</h2>
		</div>
		<div class="rounded-lg bg-white text-black">
			<h1 class="text-2xl font-bold">Elfogadott leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.elfogadva}</h2>
		</div>
		<div class="rounded-lg bg-white text-black">
			<h1 class="text-2xl font-bold">Elfogadott számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.elfogadva}</h2>
		</div>
		<div class="rounded-lg bg-green-600">
			<h1 class="text-2xl font-bold">Elutasított pótlékok száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.potlek.elutasitva}</h2>
		</div>
		<div class="rounded-lg bg-green-600">
			<h1 class="text-2xl font-bold">Elutasított leintések száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.leintes.elutasitva}</h2>
		</div>
		<div class="rounded-lg bg-green-600">
			<h1 class="text-2xl font-bold">Elutasított számlák száma:</h1>
			<h2 class="text-3xl font-bold">{data.stat.szamla.elutasitva}</h2>
		</div>
	</div>
</div>
