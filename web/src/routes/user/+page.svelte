<script lang="ts">
	export let data;
</script>

<div class="text-center text-white grid grid-cols-1 grid-rows-1 child:p-2 md:child:p-16">
	{#if !data.error}
		<div class="bg-green-600">
			<h1 class="text-2xl md:text-5xl md:font-bold md:mb-4">Üdv {data.layout?.name}!</h1>
			<h2 class="text-xl md:text-2xl">
				Hívásaid (app+leintés): {data.calls.app}+{data.calls.leintes.elfogadott}={Number(
					data.calls.app
				) + Number(data.calls.leintes.elfogadott)}
			</h2>
			<h2 class="text-xl md:text-2xl">
				Elfogadott pótlékaid (délelőtti+éjszakai): {data.calls?.potlek.de}+{data.calls?.potlek.éj}
			</h2>
		</div>
	{/if}
</div>
{#if data.error}
	<h2 class="text-center text-white font-bold text-3xl">Sikertelen API lekérdezés</h2>
{/if}
