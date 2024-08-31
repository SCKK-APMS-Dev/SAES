<script lang="ts">
	import { socket } from '$lib/socket';
	let count = 0;
	$socket?.emit('JoinEvent', { event_name: 'socketppl' });
	$socket?.on('socketppl-update', (data) => {
		count = data;
	});
</script>

<div class="mt-5 text-center">
	<h1 class=" mb-2 text-3xl font-bold text-white">Hamarosan! 😉</h1>
	<h2 class="text-gray-400">
		Eddig, a Műszakvezetés gomb az eheti statisztikához vitt, most viszont az ún. Főoldalra.
	</h2>
	<h2 class="text-gray-400">
		A Főoldalon majd egy fokkal másabb statisztikát láthattok, erről majd később.
	</h2>
	<h2 class="text-xl font-bold text-gray-400">
		A Főoldalra vonatkozik a hamarosan, nem a Műszakvezetésre. 🙂
	</h2>
	<h2 class="mt-5 text-xl text-white">Jelenlegi aktív weboldal használók száma: {count}</h2>
</div>
