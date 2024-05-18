<script lang="ts">
	export let data;
</script>

<div class="child:p-2 md:child:p-16 grid grid-cols-1 grid-rows-1 text-center text-white">
	{#if !data.error}
		<div class="ml-16 mr-16 mt-16 rounded-lg bg-emerald-600">
			<h1 class="text-2xl drop-shadow-lg md:mb-4 md:text-5xl md:font-bold">
				Üdv {data.layout?.name}!
			</h1>
			{#if !data.layout.am}
				<h2 class="text-xl drop-shadow-lg md:text-2xl">
					Hívásaid (app+leintés): {data.calls?.app}+{data.calls?.leintes}={Number(data.calls.app) +
						Number(data.calls?.leintes)}
				</h2>
			{/if}
			<!-- <h2 class="text-xl md:text-2xl drop-shadow-lg">
				Elfogadott pótlékaid (délelőtti+éjszakai): {data.calls?.potlek.de}+{data.calls?.potlek.éj}
			</h2> -->
		</div>
	{/if}
</div>
{#if data.error}
	<h2 class="text-center text-3xl font-bold text-white">Sikertelen API lekérdezés</h2>
	<h2 class="text-center text-xl text-gray-300">{data.error}</h2>
{/if}
