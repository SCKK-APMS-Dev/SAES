<script lang="ts">
	import { goto } from '$app/navigation';

	function quit() {
		goto('/logout');
	}
</script>

<div class="h-screen flex">
	<div class="m-auto text-center">
		<h1 class="text-3xl text-red-600 font-bold">Nincs jogosultságod!</h1>
		<button
			on:click={() => quit()}
			class="text-white font-bold bg-green-500 px-2 mt-5 text-xl drop-shadow-xl rounded-xl hover:bg-green-800"
			>Kijelentkezés</button
		>
	</div>
</div>
