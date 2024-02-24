<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-1">
	<a href="/user/potlekok/upload" class="bg-green-500 text-xl">Feltöltés</a>
	<div>
		<h1 class="text-5xl font-bold">Pótlékaid:</h1>
		<img src={data.potlek} alt="Aha" />
	</div>
</div>
