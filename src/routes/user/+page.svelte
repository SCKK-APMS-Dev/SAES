<script lang="ts">
	export let data;
</script>

<div class="text-center text-white mt-5">
	<h1 class="text-3xl">Üdv {data.layout?.doksi.name}!</h1>
	<h2 class="text-xl">
		Hívásaid (app+leintés): {data.page?.calls.app}+{data.page?.calls.leint}={data.page?.calls.all}
	</h2>
	<h2 class="text-xl">
		Pótlékaid (délelőtti+éjszakai): {data.page?.potlek.délelőtti}+{data.page?.potlek.éjszakai}
	</h2>
</div>
