<script lang="ts">
	export let data;
</script>

<nav class="bg-red-700 flex text-white justify-around items-center py-2 flex-col md:flex-row">
	<div class="flex items-center gap-2">
		<img
			src="/favicon.png"
			class="pointer-events-none drop-shadow-xl"
			width="40"
			height="40"
			alt="SCKK Logó"
		/>
		<a href="info" class="font-bold text-3xl drop-shadow-xl">SCKK A műszak</a>
	</div>
	<div
		class="flex gap-2 text-xl flex-col text-center md:flex-row md:gap-10 child:w-screen child:md:w-auto child:border-y-2 child:md:border-none child:drop-shadow-xl"
	>
		<a href="/user" class="hover:font-bold duration-200 transition-all">Kezdőlap</a>
		<a href="/user/potlekok" class="hover:font-bold duration-200 transition-all">Pótlékok</a>
		<a href="/user/leintesek" class="hover:font-bold duration-200 transition-all">Leintések</a>
		<a href="/user" class="hover:font-bold duration-200 transition-all">Szereltetési számlák</a>
		{#if data.layout?.doksi.rang === 'admin'}
			<a href="/user/admin" class="hover:font-bold duration-200 transition-all">Műszakvezetés</a>
		{/if}
	</div>
</nav>
<slot />
<h2 class="sticky float-right text-red-600 text-center w-screen font-bold">
	Jelenleg az oldal még fejlesztés alatt áll, nem minden funkció elérhető.
</h2>
