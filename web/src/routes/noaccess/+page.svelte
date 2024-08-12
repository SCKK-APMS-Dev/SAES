<script lang="ts">
	import { goto } from '$app/navigation';

	function quit() {
		goto('/logout');
	}
</script>

<div class="flex h-screen">
	<div class="m-auto text-center">
		<h1 class="text-3xl font-bold text-red-600">Nincs jogosultságod!</h1>
		<button
			on:click={() => quit()}
			class="mt-5 rounded-xl bg-green-500 px-2 text-xl font-bold text-white drop-shadow-xl hover:bg-green-800"
			>Kijelentkezés</button
		>
	</div>
</div>
